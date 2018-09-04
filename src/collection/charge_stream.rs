use bson::Document;
use collection::Collection;
use collection::options::{ChangeStreamOptions, AggregateOptions};
use cursor::Cursor;
use read_concern::ReadConcern;
use error::{Result, Error};

#[derive(Debug)]
pub struct ChargeStream {
    pub collection: Collection,
    pub pipeline: Vec<Document>,
    pub options: Option<ChangeStreamOptions>,
    pub resume_token: Document,
    pub cursor: Cursor,
}

impl ChargeStream {
    pub fn new(collection: Collection, pipeline: Vec<Document>, options: Option<ChangeStreamOptions>) -> Result<Self> {
        let mut full_options = Document::new();
        let mut aggregate_options = AggregateOptions::new();

        if let Some(options) = options.clone() {
            if let Some(full_document) = options.full_document {
                full_options.insert("fullDocument", full_document);
            }

            if let Some(resume_after) = options.resume_after {
                full_options.insert("resumeAfter", resume_after);
            }

            aggregate_options.batch_size = options.batch_size;
            aggregate_options.max_time_ms = options.max_await_time_ms;
            aggregate_options.collation = options.collation;
            // `read_concern` must be `ReadConcern("majority")` in order to use the `$changeStream` stage.
            aggregate_options.read_concern = Some(ReadConcern::new());
            aggregate_options.read_preference = options.read_preference;
        }

        let mut full_pipeline = vec![doc!{"$changeStream": full_options}];
        full_pipeline.extend(pipeline.clone());

        let cursor = collection.aggregate(full_pipeline, Some(aggregate_options))?;

        let charge_stream = ChargeStream {
            collection,
            pipeline,
            options,
            resume_token: Document::new(),
            cursor
        };

        Ok(charge_stream)
    }
}

impl Iterator for ChargeStream {
    type Item = Result<Document>;

    fn next(&mut self) -> Option<Result<Document>> {
        loop {
            if let Some(charge) = self.cursor.next() {
                match charge {
                    Ok(doc) => {

                        if let Ok(resume_token) = doc.get_document("_id") {
                            self.resume_token = resume_token.clone();
                        } else {
                            return Some(Err(Error::OperationError("Cannot provide resume functionality when the resume token is missing.".to_owned())))
                        }

                        return Some(Ok(doc))

                    }
                    Err(err) => return Some(Err(err))
                }
            }
        }
    }
}
