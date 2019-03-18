use mongors::*;
use bsonrs::doc;

fn main() {
    let client = MongoClient::new("mongodb://localhost:27017/?maxpoolsize=10").unwrap();

    let db = client.db("test");

    let collection = db.collection("test");

    let selector = doc!{
        "a": 3
    };

    let update = doc!{
        "b": 3,
        "c": 4
    };

    let result = collection.replace_one(selector, update, None);
    println!("{:?}", result);
}

