use std::str::FromStr;
use std::collections::BTreeMap;

use bson::{Bson, Document};
use error::Error::{self, ArgumentError};
use error::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadMode {
    Primary,
    PrimaryPreferred,
    Secondary,
    SecondaryPreferred,
    Nearest,
}

impl FromStr for ReadMode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "Primary" => ReadMode::Primary,
            "PrimaryPreferred" => ReadMode::PrimaryPreferred,
            "Secondary" => ReadMode::Secondary,
            "SecondaryPreferred" => ReadMode::SecondaryPreferred,
            "Nearest" => ReadMode::Nearest,
            _ => return Err(ArgumentError(format!("Could not convert '{}' to ReadMode.", s))),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ReadPreference {
    /// Indicates how a server should be selected during read operations.
    pub mode: ReadMode,
    pub max_staleness_seconds: Option<i32>,
    /// Filters servers based on the first tag set that matches at least one server.
    pub tag_sets: Vec<BTreeMap<String, String>>,
}

impl ReadPreference {
    pub fn new(mode: ReadMode, max_staleness_seconds: Option<i32>, tag_sets: Option<Vec<BTreeMap<String, String>>>) -> ReadPreference {
        ReadPreference {
            mode,
            max_staleness_seconds,
            tag_sets: tag_sets.unwrap_or_else(Vec::new),
        }
    }

    pub fn to_document(&self) -> Document {
        let mut doc = doc!{ "mode": stringify!(self.mode).to_ascii_lowercase() };
        let bson_tag_sets: Vec<_> = self.tag_sets
            .iter()
            .map(|map| {
                let mut bson_map = Document::new();
                for (key, val) in map.iter() {
                    bson_map.insert(&**key, Bson::String(val.to_string()));
                }
                Bson::Document(bson_map)
            })
            .collect();

        if let Some(seconds) = self.max_staleness_seconds {
            doc.insert("maxStalenessSeconds", seconds);
        }

        if bson_tag_sets.len() > 0 {
            doc.insert("tag_sets", Bson::Array(bson_tag_sets));
        }

        doc
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WriteConcern {
    /// Write replication
    pub w: i32,
    pub w_str: String,
    /// Used in conjunction with 'w'. Propagation timeout in ms.
    pub w_timeout: i32,
    /// If true, will block until write operations have been committed to journal.
    pub j: bool
}

impl WriteConcern {
    pub fn new() -> WriteConcern {
        WriteConcern {
            w: 1,
            w_str: String::new(),
            w_timeout: 0,
            j: false
        }
    }

    pub fn to_document(&self) -> Document {
        let mut doc = doc!{
            "wtimeout": self.w_timeout,
            "j": self.j
        };

        if self.w_str != "" {
            doc.insert("w", self.w_str.clone());
        } else {
            doc.insert("w", self.w);
        }

        doc
    }
}

/// Specifies a level of isolation for read operations. For example, you can use read concern to
/// only read data that has propagated to a majority of nodes in a replica set.
/// 
// Todo: document: https://docs.mongodb.com/manual/reference/read-concern/
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ReadConcern {
    pub level: ReadConcernLevel
}

impl ReadConcern {
    pub fn new() -> Self {
        ReadConcern::default()
    }

    pub fn to_document(&self) -> Document {
        doc!{
            "level": self.level.to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadConcernLevel {
    Local,
    Available,
    Majority,
    Linearizable
}

impl ReadConcernLevel {
    pub fn to_string(&self) -> String {
        match *self {
            ReadConcernLevel::Local => "local".to_owned(),
            ReadConcernLevel::Available => "available".to_owned(),
            ReadConcernLevel::Majority => "majority".to_owned(),
            ReadConcernLevel::Linearizable => "linearizable".to_owned()
        }
    }
}

impl Default for ReadConcernLevel {
    fn default() -> Self {
        ReadConcernLevel::Majority
    }
}

#[test]
fn read_concern() {
    assert_eq!(ReadConcern::new().to_document(), doc!{"level": "majority"});
}
