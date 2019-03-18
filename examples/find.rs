use mongors::*;
use bsonrs::doc;

fn main() {
    let client = MongoClient::new("mongodb://localhost:27017/?maxpoolsize=10").unwrap();

    let db = client.db("test");

    let collection = db.collection("test");


    let mut cursor = collection.find(doc!{}, None, None).unwrap();

    for doc in &mut cursor {
        println!("{:?}", doc);
    }

    cursor.error_document();

    let mut cursor = collection.find(
        doc!{},
        Some(doc!{
            //"limit": 1,
            "awaitData": true,
            "tailable": true
        }),
        None
    ).unwrap();

    
    for doc in &mut cursor {
        println!("{:?}", doc);
    }

    cursor.error_document();
}

