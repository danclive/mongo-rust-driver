use mongors::*;
use bsonrs::doc;

fn main() {
    let client = MongoClient::new("mongodb://localhost:27017/?maxpoolsize=10").unwrap();

    let db = client.db("admin");

    let collection = db.collection("$cmd");

    let result = collection.command_simple(doc!{"ping": 1}, None).unwrap();

    assert_eq!(result, doc!{"ok": 1.0});

    println!("{:?}", result);
}
