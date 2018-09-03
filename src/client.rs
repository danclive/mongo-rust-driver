use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicIsize, Ordering, ATOMIC_ISIZE_INIT};
use std::fs::{File, OpenOptions};
use std::ops::DerefMut;
use std::io::Write;

use common::{ReadPreference, WriteConcern, ReadConcern, ReadMode};
use apm::Listener;
use topology::{Topology, TopologyDescription, TopologyType, DEFAULT_HEARTBEAT_FREQUENCY_MS, 
    DEFAULT_SERVER_SELECTION_TIMEOUT_MS, DEFAULT_LOCAL_THRESHOLD_MS};
use topology::server::Server;
use stream::StreamConnector;
use connstring::{self, ConnectionString};
use apm::{CommandStarted, CommandResult};
use pool::PooledStream;
use pool::DEFAULT_POOL_SIZE;
use database::Database;
use bson::{self, Bson};
use error::Result;
use error::Error::ResponseError;

#[derive(Clone)]
pub struct MongoClient {
    pub inner: Arc<ClientInner>,
}

pub struct ClientInner {
    request_id: AtomicIsize,
    topology: Topology,
    pub listener: Listener,
    pub options: ClientOptions,
    log_file: Option<Mutex<File>>
}

#[derive(Clone, Debug)]
pub struct ClientOptions {
    pub log_file: Option<String>,
    pub read_preference: ReadPreference,
    pub read_concern: ReadConcern,
    pub write_concern: WriteConcern,
    pub heartbeat_frequency_ms: u32,
    pub server_selection_timeout_ms: i64,
    pub local_threshold_ms: i64,
    pub stream_connector:  StreamConnector,
    pub pool_size: usize
}

impl ClientOptions {
    pub fn new() -> ClientOptions {
        ClientOptions::default()
    }
}

impl Default for ClientOptions {
    fn default() -> Self {
        ClientOptions {
            log_file: None,
            read_preference: ReadPreference::new(ReadMode::Primary, None, None),
            read_concern: ReadConcern::new(),
            write_concern: WriteConcern::new(),
            heartbeat_frequency_ms: DEFAULT_HEARTBEAT_FREQUENCY_MS,
            server_selection_timeout_ms: DEFAULT_SERVER_SELECTION_TIMEOUT_MS,
            local_threshold_ms: DEFAULT_LOCAL_THRESHOLD_MS,
            pool_size: DEFAULT_POOL_SIZE,
            stream_connector: StreamConnector::default()
        }
    }
}

fn is_send<T: Send>() {}
fn is_sync<T: Sync>() {}

impl MongoClient {
    pub fn connect(host: &str, port: u16) -> Result<MongoClient> {
        let config = ConnectionString::new(host, port);
        let mut description = TopologyDescription::new(StreamConnector::Tcp);
        description.topology_type = TopologyType::Single;
        MongoClient::with_config(&config, None, Some(description))
    }

    pub fn connect_with_options(host: &str, port: u16, options: ClientOptions) -> Result<MongoClient> {
        let config = ConnectionString::new(host, port);
        let mut description = TopologyDescription::new(options.stream_connector.clone());

        description.topology_type = TopologyType::Single;
        MongoClient::with_config(&config, Some(options), Some(description))
    }

    pub fn with_uri(uri: &str) -> Result<MongoClient> {
        let config = connstring::parse(uri)?;
        MongoClient::with_config(&config, None, None)
    }

    pub fn with_uri_and_options(uri: &str, options: ClientOptions) -> Result<MongoClient> {
        let config = connstring::parse(uri)?;
        MongoClient::with_config(&config, Some(options), None)
    }

    pub fn with_config(config: &ConnectionString, options: Option<ClientOptions>, description: Option<TopologyDescription>) -> Result<MongoClient> {
        is_send::<MongoClient>();
        is_sync::<MongoClient>();

        let client_options = options.unwrap_or_else(ClientOptions::default);

        // Todo parse options
        // https://docs.mongodb.com/manual/reference/connection-string/index.html

        let listener = Listener::new();
        let file = match client_options.log_file {
            Some(ref string) => {
                let _ = listener.add_start_hook(log_command_started);
                let _ = listener.add_completion_hook(log_command_completed);
                Some(Mutex::new(OpenOptions::new().write(true).append(true).create(true).open(string)?))
            }
            None => None
        };

        let client = MongoClient {
            inner: Arc::new(ClientInner {
                request_id: ATOMIC_ISIZE_INIT,
                topology: Topology::new(config.clone(), description, client_options.stream_connector.clone())?,
                listener,
                options: client_options.clone(),
                log_file: file
            })
        };

        {
            let top_description = &client.inner.topology.description;
            let mut top = top_description.write()?;
            top.heartbeat_frequency_ms = client_options.heartbeat_frequency_ms;
            top.server_selection_timeout_ms = client_options.server_selection_timeout_ms;
            top.local_threshold_ms = client_options.local_threshold_ms;

            for host in &config.hosts {
                let server = Server::new(
                    client.clone(),
                    host.clone(),
                    top_description.clone(),
                    true,
                    client_options.stream_connector.clone()
                );

                top.servers.insert(host.clone(), server);
            }
        }

        Ok(client)
    }

    pub fn get_request_id(&self) -> i32 {
        self.inner.request_id.fetch_add(1, Ordering::SeqCst) as i32
    }

    pub fn acquire_stream(&self, read_preference: ReadPreference)
        -> Result<(PooledStream, bool)> {
            self.inner.topology.acquire_stream(read_preference)
        }

    pub fn acquire_write_stream(&self) -> Result<PooledStream> {
        self.inner.topology.acquire_write_stream()
    }

    pub fn db(&self, db_name: &str) -> Database {
        Database::open(self.clone(), db_name, None, None, None)
    }

    pub fn database_names(&self) -> Result<Vec<String>> {
        let mut doc = bson::Document::new();
        doc.insert("listDatabases", Bson::Int32(1));

        let db = self.db("admin");
        let res = db.command(doc, None)?;
        if let Some(&Bson::Array(ref batch)) = res.get("databases") {
            // Extract database names
            let map = batch.iter()
                .filter_map(|bdoc| {
                    if let Bson::Document(ref doc) = *bdoc {
                        if let Some(&Bson::String(ref name)) = doc.get("name") {
                            return Some(name.to_string());
                        }
                    }
                    None
                })
            .collect();
            return Ok(map);
        }

        Err(ResponseError("Server reply does not contain 'databases'.".to_string()))
    }
}

fn log_command_started(client: &MongoClient, command_started: &CommandStarted) {
    let mutex = match client.inner.log_file {
        Some(ref mutex) => mutex,
        None => return,
    };

    let mut guard = match mutex.lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };

    let _ = writeln!(guard.deref_mut(), "{}", command_started);
}

fn log_command_completed(client: &MongoClient, command_result: &CommandResult) {
    let mutex = match client.inner.log_file {
        Some(ref mutex) => mutex,
        None => return,
    };

    let mut guard = match mutex.lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };

    let _ = writeln!(guard.deref_mut(), "{}", command_result);
}
