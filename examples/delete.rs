use mongors::*;
use bsonrs::doc;

fn main() {
    let client = MongoClient::new("mongodb://localhost:27017/?maxpoolsize=10").unwrap();

    let db = client.db("test");

    let collection = db.collection("test");

    let selector = doc!{
        "a": 2
    };

    let result = collection.delete_one(selector, None);
    println!("{:?}", result);

    let selector = doc!{
        "id": 123 
    };

    let result = collection.delete_many(selector, None);
    println!("{:?}", result);
}

