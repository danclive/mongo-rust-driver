use bson::Document;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WriteConcern {
    /// Write replication
    pub w: W,
    /// Used in conjunction with 'w'. Propagation timeout in ms.
    pub w_timeout: i32,
    /// If true, will block until write operations have been committed to journal.
    pub j: bool
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum W {
    Int32(i32),
    Tag(String),
    Majority
}

impl Default for W {
    fn default() -> Self {
        W::Int32(1)
    }
}

impl WriteConcern {
    pub fn new() -> WriteConcern {
        WriteConcern {
            w: W::Int32(1),
            w_timeout: 0,
            j: false
        }
    }

    pub fn to_document(&self) -> Document {
        let mut doc = doc!{
            "wtimeout": self.w_timeout,
            "j": self.j
        };

        match self.w {
            W::Int32(n) => { doc.insert("w", n); },
            W::Tag(ref s) => { doc.insert("w", s); },
            W::Majority => { doc.insert("w", "majority"); }
        }

        doc
    }
}
