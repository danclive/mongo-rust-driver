#[derive(PartialEq, Eq, Clone)]
pub enum CommandType {
    Aggregate,
    BuildInfo,
    Count,
    CreateCollection,
    CreateIndexes,
    CreateUser,
    Delete,
    Distinct,
    DropAllUsers,
    DropCollection,
    DropDatabase,
    DropIndexes,
    DropUser,
    Find,
    FindAndModify,
    FindOneAndDelete,
    FindOneAndReplace,
    FindOneAndUpdate,
    GetUser,
    GetUsers,
    GetMore,
    Insert,
    IsMaster,
    ListCollections,
    ListDatabases,
    ListIndexes,
    Suppressed,
    Update
}

impl CommandType {
    pub fn to_str(&self) -> &str {
        match *self {
            CommandType::Aggregate => "aggregate",
            CommandType::BuildInfo => "buildinfo",
            CommandType::Count => "count",
            CommandType::CreateCollection => "create_collection",
            CommandType::CreateIndexes => "create_indexes",
            CommandType::CreateUser => "create_user",
            CommandType::Delete => "delete",
            CommandType::Distinct => "distinct",
            CommandType::DropAllUsers => "drop_all_users",
            CommandType::DropCollection => "drop_collection",
            CommandType::DropDatabase => "drop_database",
            CommandType::DropIndexes => "drop_indexes",
            CommandType::DropUser => "drop_user",
            CommandType::Find => "find",
            CommandType::FindAndModify => "find_and_modify",
            CommandType::FindOneAndDelete => "find_one_and_delete",
            CommandType::FindOneAndReplace => "find_one_and_replace",
            CommandType::FindOneAndUpdate => "find_one_and_update",
            CommandType::GetUser => "get_user",
            CommandType::GetUsers => "get_users",
            CommandType::GetMore => "get_more",
            CommandType::Insert => "insert",
            CommandType::IsMaster => "is_master",
            CommandType::ListCollections => "list_collections",
            CommandType::ListDatabases => "list_databases",
            CommandType::ListIndexes => "list_indexes",
            CommandType::Suppressed => "suppressed",
            CommandType::Update => "update"
        }
    }

    pub fn is_write_command(&self) -> bool {
        match *self {
            CommandType::CreateCollection |
            CommandType::CreateIndexes |
            CommandType::CreateUser |
            CommandType::Delete |
            CommandType::DropAllUsers |
            CommandType::DropCollection |
            CommandType::DropDatabase |
            CommandType::DropIndexes |
            CommandType::DropUser |
            CommandType::FindAndModify |
            CommandType::FindOneAndDelete |
            CommandType::FindOneAndReplace |
            CommandType::FindOneAndUpdate |
            CommandType::Insert |
            CommandType::Update |
            CommandType::Aggregate |
            CommandType::BuildInfo |
            CommandType::Count |
            CommandType::Distinct |
            CommandType::Find |
            CommandType::GetUser |
            CommandType::GetUsers |
            CommandType::GetMore |
            CommandType::IsMaster |
            CommandType::ListCollections |
            CommandType::ListDatabases |
            CommandType::ListIndexes |
            CommandType::Suppressed => false,
        }
    }
}
