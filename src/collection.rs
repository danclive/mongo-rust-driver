use crate::db::DB;

pub struct Collection {
    pub db: DB,
    pub name: String
}

impl Collection {
    pub fn new(db: DB, name: &str) -> Collection {
        Collection {
            db,
            name: name.to_owned()
        }
    }
}
