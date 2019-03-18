use mongors::*;
use bsonrs::doc;

fn main() {
    let client = MongoClient::new("mongodb://localhost:27017/?maxpoolsize=10").unwrap();

    let db = client.db("test");

    let collection = db.collection("test");


    let doc = doc!{
        "id": 123
    };

    let result = collection.insert_one(doc, None);
    println!("{:?}", result);

    let docs = vec![
        doc!{
            "a": 1
        },
        doc!{
            "a": 2
        }
    ];

    let result = collection.insert_many(docs, None);
    println!("{:?}", result);
}

