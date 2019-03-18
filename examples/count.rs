use mongors::*;
use bsonrs::doc;

fn main() {
    let client = MongoClient::new("mongodb://localhost:27017/?maxpoolsize=10").unwrap();

    let db = client.db("local");

    let collection = db.collection("startup_log");

    let result = collection.count_documents(doc!{}, None, None).unwrap();

    println!("{:?}", result);

    let result = collection.estimated_document_count(None, None).unwrap();

    println!("{:?}", result);
}
