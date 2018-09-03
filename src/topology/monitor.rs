use std::collections::BTreeMap;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use chrono::{self, DateTime, Utc};

use object_id::ObjectId;
use client::MongoClient;
use error::Result;
use error::Error::{self, ArgumentError, OperationError};
use bson::{Bson, Document};
//use db::Database;
//use command_type::CommandType;
use connstring::{self, Host};
use pool::ConnectionPool;
use stream::StreamConnector;
use command::base_command;

use super::server::{ServerDescription, ServerType};
use super::{DEFAULT_HEARTBEAT_FREQUENCY_MS, TopologyDescription};

const DEFAULT_MAX_BSON_OBJECT_SIZE: i64 = 16 * 1024 * 1024;
const DEFAULT_MAX_MESSAGE_SIZE_BYTES: i64 = 48_000_000;

/// The result of an isMaster operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IsMasterResult {
    pub ok: bool,
    pub is_master: bool,
    pub max_bson_object_size: i64,
    pub max_message_size_bytes: i64,
    pub local_time: Option<DateTime<Utc>>,
    pub min_wire_version: i64,
    pub max_wire_version: i64,

    /// Shard-specific. mongos instances will add this field to the
    /// isMaster reply, and it will contain the value "isdbgrid".
    pub msg: String,

    // Replica Set specific
    pub is_replica_set: bool,
    pub is_secondary: bool,
    pub me: Option<Host>,
    pub hosts: Vec<Host>,
    pub passives: Vec<Host>,
    pub arbiters: Vec<Host>,
    pub arbiter_only: bool,
    pub tags: BTreeMap<String, String>,
    pub set_name: String,
    pub election_id: Option<ObjectId>,
    pub primary: Option<Host>,
    pub hidden: bool,
    pub set_version: Option<i64>,
}

/// Monitors and updates server and topology information.
pub struct Monitor {
    // Host being monitored.
    host: Host,
    // Connection pool for the host.
    server_pool: Arc<ConnectionPool>,
    // Topology description to update.
    top_description: Arc<RwLock<TopologyDescription>>,
    // Server description to update.
    server_description: Arc<RwLock<ServerDescription>>,
    // Client reference.
    client: MongoClient,
    // string, single-threaded pool.
    personal_pool: Arc<ConnectionPool>,
    // string copy of the topology's heartbeat frequency.
    heartbeat_frequency_ms: AtomicUsize,
    // Used for condvar functionality.
    dummy_lock: Mutex<()>,
    // To allow servers to request an immediate update, this
    // condvar can be notified to wake up the monitor.
    condvar: Condvar,
    /// While true, the monitor will check server connection health
    /// at the topology's heartbeat frequency rate.
    pub running: Arc<AtomicBool>,
}

impl IsMasterResult {
    /// Parses an isMaster response document from the server.
    pub fn new(doc: &Document) -> Result<IsMasterResult> {
        let ok = match doc.get("ok") {
            Some(&Bson::Int32(v)) => v != 0,
            Some(&Bson::Int64(v)) => v != 0,
            Some(&Bson::Double(v)) => v != 0.0,
            _ => return Err(ArgumentError("result does not contain `ok`.".to_string())),
        };

        let mut result = IsMasterResult {
            ok,
            is_master: false,
            max_bson_object_size: DEFAULT_MAX_BSON_OBJECT_SIZE,
            max_message_size_bytes: DEFAULT_MAX_MESSAGE_SIZE_BYTES,
            local_time: None,
            min_wire_version: -1,
            max_wire_version: -1,
            msg: String::new(),
            is_secondary: false,
            is_replica_set: false,
            me: None,
            hosts: Vec::new(),
            passives: Vec::new(),
            arbiters: Vec::new(),
            arbiter_only: false,
            tags: BTreeMap::new(),
            set_name: String::new(),
            election_id: None,
            primary: None,
            hidden: false,
            set_version: None,
        };

        if let Some(&Bson::Boolean(b)) = doc.get("ismaster") {
            result.is_master = b;
        }

        if let Some(&Bson::UTCDatetime(ref datetime)) = doc.get("localTime") {
            result.local_time = Some(*datetime);
        }

        if let Some(&Bson::Int64(v)) = doc.get("minWireVersion") {
            result.min_wire_version = v;
        }

        if let Some(&Bson::Int64(v)) = doc.get("maxWireVersion") {
            result.max_wire_version = v;
        }

        if let Some(&Bson::String(ref s)) = doc.get("msg") {
            result.msg = s.to_string();
        }

        if let Some(&Bson::Boolean(ref b)) = doc.get("secondary") {
            result.is_secondary = *b;
        }

        if let Some(&Bson::Boolean(ref b)) = doc.get("isreplicaset") {
            result.is_replica_set = *b;
        }

        if let Some(&Bson::String(ref s)) = doc.get("setName") {
            result.set_name = s.to_string();
        }

        if let Some(&Bson::String(ref s)) = doc.get("me") {
            result.me = Some(connstring::parse_host(s)?);
        }

        if let Some(&Bson::Array(ref arr)) = doc.get("hosts") {
            result.hosts = arr.iter()
                .filter_map(|bson| match *bson {
                    Bson::String(ref s) => connstring::parse_host(s).ok(),
                    _ => None,
                })
                .collect();
        }

        if let Some(&Bson::Array(ref arr)) = doc.get("passives") {
            result.passives = arr.iter()
                .filter_map(|bson| match *bson {
                    Bson::String(ref s) => connstring::parse_host(s).ok(),
                    _ => None,
                })
                .collect();
        }

        if let Some(&Bson::Array(ref arr)) = doc.get("arbiters") {
            result.arbiters = arr.iter()
                .filter_map(|bson| match *bson {
                    Bson::String(ref s) => connstring::parse_host(s).ok(),
                    _ => None,
                })
                .collect();
        }

        if let Some(&Bson::String(ref s)) = doc.get("primary") {
            result.primary = Some(connstring::parse_host(s)?);
        }

        if let Some(&Bson::Boolean(ref b)) = doc.get("arbiterOnly") {
            result.arbiter_only = *b;
        }

        if let Some(&Bson::Boolean(ref h)) = doc.get("hidden") {
            result.hidden = *h;
        }

        if let Some(&Bson::Int64(v)) = doc.get("setVersion") {
            result.set_version = Some(v);
        }

        if let Some(&Bson::Document(ref doc)) = doc.get("tags") {
            for (k, v) in doc {
                if let Bson::String(ref tag) = *v {
                    result.tags.insert(k.to_string(), tag.to_string());
                }
            }
        }

        match doc.get("electionId") {
            Some(&Bson::ObjectId(ref id)) => result.election_id = Some(id.clone()),
            Some(&Bson::Document(ref doc)) => {
                if let Some(&Bson::String(ref s)) = doc.get("$oid") {
                    result.election_id = Some(ObjectId::with_string(s)?);
                }
            }
            _ => (),
        }

        Ok(result)
    }
}

impl Monitor {
    /// Returns a new monitor connected to the server.
    pub fn new(
        client: MongoClient,
        host: Host,
        pool: Arc<ConnectionPool>,
        top_description: Arc<RwLock<TopologyDescription>>,
        server_description: Arc<RwLock<ServerDescription>>,
        connector: StreamConnector
    ) -> Monitor {
        Monitor {
            client,
            host: host.clone(),
            server_pool: pool,
            personal_pool: Arc::new(ConnectionPool::with_size(host, connector, 1)),
            top_description,
            server_description,
            heartbeat_frequency_ms: AtomicUsize::new(DEFAULT_HEARTBEAT_FREQUENCY_MS as usize),
            dummy_lock: Mutex::new(()),
            condvar: Condvar::new(),
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    // Set server description error field.
    fn set_err(&self, err: Error) {
        {
            let mut server_description = self.server_description.write().unwrap();
            server_description.set_err(err);
        }

        self.update_top_description(&self.server_description);
    }

    /// Returns an isMaster server response using an string monitor socket.
    pub fn is_master(&self) -> Result<(Document, i64)> {
        let mut stream = self.personal_pool.acquire_stream()?;

        let time = chrono::Local::now();
        let start_ms = time.timestamp() * 1000 + i64::from(time.timestamp_subsec_millis());

        let command = doc!{
            "isMaster": 1,
            "$db": "local"
        };

        let doc = base_command(&self.client, &mut stream, command)?;

        let end_time = chrono::Local::now();
        let end_ms = end_time.timestamp() * 1000 + i64::from(end_time.timestamp_subsec_millis());

        let round_trip_time = end_ms - start_ms;
        Ok((doc, round_trip_time))
    }

    pub fn request_update(&self) {
        self.condvar.notify_one();
    }

    // Updates the server description associated with this monitor using an isMaster server
    // response.
    fn update_server_description(
        &self,
        doc: &Document,
        round_trip_time: i64
    ) -> Result<Arc<RwLock<ServerDescription>>> {

        let ismaster_result = IsMasterResult::new(&doc);
        {
            let mut server_description = self.server_description.write().unwrap();
            match ismaster_result {
                Ok(ismaster) => server_description.update(ismaster, round_trip_time),
                Err(err) => {
                    server_description.set_err(err);
                    return Err(OperationError("Failed to parse ismaster result.".to_string()));
                }
            }
        }

        Ok(self.server_description.clone())
    }

    // Updates the topology description associated with this monitor using a new server description.
    fn update_top_description(&self, description: &Arc<RwLock<ServerDescription>>) {
        let mut top_description = self.top_description.write().unwrap();
        top_description.update(
            &self.host,
            &description,
            &self.client,
            &self.top_description
        );
    }

    // Updates server and topology descriptions using a successful isMaster cursor result.
    fn update_with_is_master_cursor(&self, doc: &Document, round_trip_time: i64) {
        if let Ok(description) = self.update_server_description(&doc, round_trip_time) {
            self.update_top_description(&description);
        }
    }

    /// Execute isMaster and update the server and topology.
    fn execute_update(&self) {
        match self.is_master() {
            Ok((doc, rtt)) => self.update_with_is_master_cursor(&doc, rtt),
            Err(err) => {
                // Refresh all connections
                self.server_pool.clear();
                self.personal_pool.clear();

                if self.server_description.read().unwrap().server_type == ServerType::Unknown {
                    self.set_err(err);
                }

                // Retry once
                match self.is_master() {
                    Ok((mut doc, rtt)) => self.update_with_is_master_cursor(&doc, rtt),
                    Err(err) => self.set_err(err)
                }
            }
        }
    }

    /// Starts server monitoring.
    pub fn run(&self) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }

        self.running.store(true, Ordering::SeqCst);

        let mut guard = self.dummy_lock.lock().unwrap();

        loop {
            if !self.running.load(Ordering::SeqCst) {
                break;
            }

            self.execute_update();

            if let Ok(description) = self.top_description.read() {
                self.heartbeat_frequency_ms.store(description.heartbeat_frequency_ms as usize,
                                                  Ordering::SeqCst);
            }

            let frequency = self.heartbeat_frequency_ms.load(Ordering::SeqCst) as u64;
            guard = self.condvar.wait_timeout(guard, Duration::from_millis(frequency)).unwrap().0;
        }
    }
}
