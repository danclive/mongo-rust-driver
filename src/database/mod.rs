use std::sync::Arc;
use std::fmt;
use std::error::Error as stdError;

use bson::{self, Bson, Document};
use client::MongoClient;
use read_preference::ReadPreference;
use read_concern::ReadConcern;
use write_concern::WriteConcern;
use semver::Version;
use error::{Result, Error};
use command::{base_command, is_write_command};
use auth::Authenticator;
use collection::Collection;
use cursor::{Cursor, DEFAULT_BATCH_SIZE};
use self::options::{CreateCollectionOptions, CreateUserOptions, UserInfoOptions};


pub mod options;
pub mod roles;

#[derive(Clone)]
pub struct Database {
    pub inner: Arc<DatabaseInner>
}

pub struct DatabaseInner {
    /// The database name.
    pub name: String,
    /// A reference to the client that spawned this database.
    pub client: MongoClient,
    /// Indicates how a server should be selected for read operations.
    pub read_preference: ReadPreference,
    /// The readConcern query option for replica sets and replica set shards determines
    /// which data to return from a query.
    pub read_concern: ReadConcern,
    /// Describes the guarantees provided by MongoDB when reporting the success of a write
    /// operation.
    pub write_concern: WriteConcern,
}

fn is_send<T: Send>() {}
fn is_sync<T: Sync>() {}

impl Database {
    pub fn open(
        client: MongoClient,
        name: &str,
        read_preference: Option<ReadPreference>,
        read_concern: Option<ReadConcern>,
        write_concern: Option<WriteConcern>
    ) -> Database {
        is_send::<Database>();
        is_sync::<Database>();

        let rp = read_preference.unwrap_or_else(|| client.inner.options.read_preference.clone());
        let rc = read_concern.unwrap_or_else(|| client.inner.options.read_concern.clone());
        let wc = write_concern.unwrap_or_else(|| client.inner.options.write_concern.clone());

        Database {
            inner: Arc::new(DatabaseInner {
                name: name.to_string(),
                client,
                read_preference: rp,
                read_concern: rc,
                write_concern: wc,
            })
        }
    }

    pub fn command(&self, command: Document, read_preference: Option<ReadPreference>) -> Result<Document> {
        let read_preference = read_preference.unwrap_or_else(|| self.inner.read_preference.clone());
        let command_name = {
            if let Some((ref command_name, _)) = command.front() {
                command_name.to_string()
            } else {
                return Err(Error::ArgumentError("Then command must include command name".to_string()))
            }
        };

        let (mut stream, send_read_pref) = if is_write_command(&command_name) {
            (self.inner.client.acquire_write_stream()?, false)
        } else {
            self.inner.client.acquire_stream(read_preference.clone())?
        };

        let mut command = command;

        command.insert("$db", self.inner.name.clone());

        if send_read_pref {
            command.insert("$read_preference", read_preference.to_document());
        }

        let mut doc = base_command(&self.inner.client, &mut stream, command)?;

        if let Some(Bson::Document(doc)) = doc.remove("cursor") {
            return Ok(doc)
        }

        Ok(doc)
    }

    pub fn auth(&self, user: &str, password: &str) -> Result<()> {
        let authenticator = Authenticator::new(self.clone());
        authenticator.auth(user, password)
    }

    pub fn drop_database(&self) -> Result<()> {
        let spec = doc!{ "dropDatabase": 1 };
        self.command(spec, None)?;
        Ok(())
    }

    pub fn collection(&self, coll_name: &str) -> Collection {
        Collection::new(&self, coll_name, false, None, None, None)
    }

    pub fn collection_with_prefs(
        &self,
        coll_name: &str,
        create: bool,
        read_preference: Option<ReadPreference>,
        read_concern: Option<ReadConcern>,
        write_concern: Option<WriteConcern>
    ) -> Collection {
        Collection::new(
            &self,
            coll_name,
            create,
            read_preference,
            read_concern,
            write_concern
        )
    }

    pub fn version(&self) -> Result<Version> {
        let doc = doc!{ "buildinfo": 1 };
        let out = self.command(doc, None)?;

        match out.get("version") {
            Some(&Bson::String(ref s)) => {
                match Version::parse(s) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::ResponseError(e.description().to_string())),
                }
            }
            _ => Err(Error::ResponseError("No version received from server".to_string())),
        }
    }

    // Todo: nameOnly
    // https://docs.mongodb.com/manual/reference/command/listCollections/#dbcmd.listCollections
    pub fn list_collections(&self, filter: Option<bson::Document>) -> Result<Cursor> {
        self.list_collections_with_batch_size(filter, DEFAULT_BATCH_SIZE)
    }

    pub fn list_collections_with_batch_size(
        &self,
        filter: Option<bson::Document>,
        batch_size: i32
    ) -> Result<Cursor> {
        let mut list_collections_command = doc!{ "listCollections": 1 };

        if filter.is_some() {
            list_collections_command.insert("filter", Bson::Document(filter.unwrap()));
        }

        Cursor::command(self.clone(), list_collections_command, Some(batch_size), None, None)
    }

    pub fn create_collection(
        &self,
        name: &str,
        options: Option<CreateCollectionOptions>
    ) -> Result<()> {
        let mut doc = doc!{ "create": name };

        if let Some(create_collection_options) = options {
            doc.extend(create_collection_options);
        }

        self.command(doc, None).map(|_| ())
    }

    pub fn drop_collection(&self, name: &str) -> Result<()> {
        // let mut spec = bson::Document::new();
        // spec.insert("drop", Bson::String(name.to_string()));
        let spec = doc!{
            "drop": name
        };

        self.command(spec, None)?;

        Ok(())
    }

    pub fn create_user(
        &self,
        name: &str,
        password: &str,
        options: Option<CreateUserOptions>
    ) -> Result<()> {
        let mut doc = doc!{
            "createUser": name,
            "pwd": password
        };

        match options {
            Some(user_options) => {
                doc.extend(user_options);
            }
            None => {
                doc.insert("roles", Bson::Array(Vec::new()));
            }
        };

        self.command(doc, None).map(|_| ())
    }

    pub fn drop_user(&self, name: &str, write_concern: Option<WriteConcern>) -> Result<()> {
        let mut doc = doc!{ "dropUser": name };

        if let Some(concern) = write_concern {
            doc.insert("writeConcern", concern.to_document());
        }

        self.command(doc, None).map(|_| ())
    }

    pub fn drop_all_users(&self, write_concern: Option<WriteConcern>) -> Result<(i32)> {
        let mut doc = doc!{ "dropAllUsersFromDatabase": 1 };

        if let Some(concern) = write_concern {
            doc.insert("writeConcern", concern.to_document());
        }

        let response = self.command(doc, None)?;

        match response.get("n") {
            Some(&Bson::Int32(i)) => Ok(i),
            Some(&Bson::Int64(i)) => Ok(i as i32),
            _ => Err(Error::CursorNotFoundError),
        }
    }

    pub fn get_users(
        &self,
        users: Vec<&str>,
        options: Option<UserInfoOptions>
    ) -> Result<Vec<bson::Document>> {
        let vec: Vec<_> = users.into_iter()
            .map(|user| {
                let doc = doc!{
                    "user": user,
                    "db": self.inner.name.to_string()
                };
                Bson::Document(doc)
            })
            .collect();

        let mut doc = doc!{ "usersInfo": vec };

        if let Some(user_info_options) = options {
            doc.extend(user_info_options);
        }

        let out = self.command(doc, None)?;
        let vec = match out.get("users") {
            Some(&Bson::Array(ref vec)) => vec.clone(),
            _ => return Err(Error::CursorNotFoundError),
        };

        let mut users = vec![];

        for bson in vec {
            match bson {
                Bson::Document(doc) => users.push(doc),
                _ => return Err(Error::CursorNotFoundError),
            };
        }

        Ok(users)
    }

    pub fn get_user(&self, user: &str, options: Option<UserInfoOptions>) -> Result<bson::Document> {
        let mut doc = doc!{
            "usersInfo": {
                "user": user,
                "db": self.inner.name.to_string()
            }
        };

        if let Some(user_info_options) = options {
            doc.extend(user_info_options);
        }

        let out = match self.command(doc, None) {
            Ok(doc) => doc,
            Err(e) => return Err(e),
        };

        let users = match out.get("users") {
            Some(&Bson::Array(ref v)) => v.clone(),
            _ => return Err(Error::CursorNotFoundError),
        };

        match users.first() {
            Some(&Bson::Document(ref doc)) => Ok(doc.clone()),
            _ => Err(Error::CursorNotFoundError),
        }
    }

    pub fn get_all_users(&self, show_credentials: bool) -> Result<Vec<bson::Document>> {
        let doc = doc!{
            "usersInfo": 1,
            "showCredentials": show_credentials
        };

        let out = self.command(doc, None)?;
        let vec = match out.get("users") {
            Some(&Bson::Array(ref vec)) => vec.clone(),
            _ => return Err(Error::CursorNotFoundError),
        };

        let mut users = vec![];

        for bson in vec {
            match bson {
                Bson::Document(doc) => users.push(doc),
                _ => return Err(Error::CursorNotFoundError),
            };
        }

        Ok(users)
    }
}

impl fmt::Debug for Database {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.inner.name)
    }
}
