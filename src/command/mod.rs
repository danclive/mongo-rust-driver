use client::MongoClient;
use pool::PooledStream;
use bson::Document;
use common::{ReadPreference, WriteConcern, ReadConcern};
use message::{OpMsg, OpMsgBuilder, Section};
use error::{Result, ErrorCode};

pub fn command(
	client: &MongoClient,
	stream: PooledStream,
	command: Document
) -> Result<Document> {
	//let read_preference = read_preference.unwrap_or_else(||client.inner.options.read_preference.clone());
	let mut msg_builder = OpMsg::builder();
	let request_msg = msg_builder
		.request_id(client.get_request_id())
		.push_section(Section::from_document(command)?)
		.build();

	let mut stream = stream;
	let stream = stream.get_socket();

	request_msg.write(stream)?;

	let response_msg = OpMsg::read(stream)?;

	Ok(response_msg.get_document()?)
}
