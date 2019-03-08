use bson::Document;

/// Specifies a level of isolation for read operations. For example, you can use read concern to
/// only read data that has propagated to a majority of nodes in a replica set.
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
