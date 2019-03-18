use mongors::*;
use bsonrs::doc;

fn main() {
    let client = MongoClient::new("mongodb://localhost:27017/?maxpoolsize=10").unwrap();

    let db = client.db("test");

    let collection = db.collection("test");

    let selector = doc!{
        "a": 1
    };

    let update = doc!{
        "$set": {
            "a": 3
        }
    };

    let result = collection.update_one(selector, update, None);
    println!("{:?}", result);

    let selector = doc!{
        "id": 123 
    };

    let update = doc!{
        "$set": {
            "la": 3
        }
    };

    let result = collection.update_many(selector, update, None);
    println!("{:?}", result);
}

