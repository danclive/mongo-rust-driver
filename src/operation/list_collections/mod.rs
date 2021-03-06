#[cfg(test)]
mod test;

use crate::{
    cmap::{Command, CommandResponse, StreamDescription},
    cursor::CursorSpecification,
    error::Result,
    operation::{append_options, CursorBody, Operation},
    options::{ListCollectionsOptions, ReadPreference, SelectionCriteria},
};
use crate::bson::Document;
use crate::doc;

#[derive(Debug)]
pub(crate) struct ListCollections {
    db: String,
    filter: Option<Document>,
    name_only: bool,
    options: Option<ListCollectionsOptions>,
}

impl ListCollections {
    #[cfg(test)]
    fn empty() -> Self {
        Self::new(String::new(), None, false, None)
    }

    pub(crate) fn new(
        db: String,
        filter: Option<Document>,
        name_only: bool,
        options: Option<ListCollectionsOptions>,
    ) -> Self {
        Self {
            db,
            filter,
            name_only,
            options,
        }
    }
}

impl Operation for ListCollections {
    type O = CursorSpecification;
    const NAME: &'static str = "listCollections";

    fn build(&self, description: &StreamDescription) -> Result<Command> {
        let mut body = doc! {
            Self::NAME: 1,
        };

        let mut name_only = self.name_only;
        if let Some(ref filter) = self.filter {
            body.insert("filter", filter.clone());

            if name_only && filter.keys().any(|k| k != "name") {
                name_only = false;
            }
        }
        body.insert("nameOnly", name_only);

        append_options(&mut body, self.options.as_ref())?;

        Ok(Command::new(Self::NAME.to_string(), self.db.clone(), body))
    }

    fn handle_response(&self, response: CommandResponse) -> Result<Self::O> {
        let body: CursorBody = response.body()?;

        Ok(CursorSpecification {
            ns: body.cursor.ns,
            address: response.source_address().clone(),
            id: body.cursor.id,
            batch_size: self.options.as_ref().and_then(|opts| opts.batch_size),
            max_time: None,
            buffer: body.cursor.first_batch,
        })
    }

    fn selection_criteria(&self) -> Option<&SelectionCriteria> {
        Some(SelectionCriteria::ReadPreference(ReadPreference::Primary)).as_ref()
    }
}
