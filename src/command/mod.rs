use std::time::Instant;

use client::MongoClient;
use pool::PooledStream;
use bson::{Document, Bson};
use message::{OpMsg, Section};
use apm::{CommandStarted, CommandResult, EventRunner};
use error::{Result, Error, CodedError};

macro_rules! try {
    (
        $client:expr,
        $duration:expr,
        $command_name:expr,
        $request_id:expr,
        $connstring:expr,
        $result:expr
    ) => {
        match $result {
            Ok(result) => result,
            Err(err) => {
                if let Err(_) = $client.run_completion_hooks(&CommandResult::Failure {
                    duration: $duration,
                    command_name: $command_name.clone(),
                    failure: &err,
                    request_id: $request_id,
                    connection_string: $connstring.clone()
                }) {
                    return Err(Error::EventListenerError(Some(Box::new(err))));
                }

                return Err(err)
            }
        }
    }
}

pub fn base_command(
    client: &MongoClient,
    stream: &mut PooledStream,
    command: &Document
) -> Result<Document> {
    let mut msg_builder = OpMsg::builder();
    let request_msg = msg_builder
        .request_id(client.get_request_id())
        .push_section(Section::from_document(command)?)
        .build();

    let socket = stream.get_socket();
    let connstring = format!("{}", socket.get_ref().peer_addr()?);
    let command_name = {
        if let Some((ref command_name, _)) = command.front() {
            command_name.to_string()
        } else {
            return Err(Error::ArgumentError("Then command must include command name".to_string()))
        }
    };

    let db_name = if let Some(&Bson::String(ref db_name)) = command.get("$db") {
        db_name.to_owned()
    } else {
        return Err(Error::ArgumentError("The command must include '$db'".to_string()))
    };

    let init_time = Instant::now();

    if let Err(_) = client.run_start_hooks(&CommandStarted {
        command: command.clone(),
        database_name: db_name,
        command_name: command_name.clone(),
        request_id: i64::from(request_msg.header.request_id),
        connection_string: connstring.clone()
    }) {
        return Err(Error::EventListenerError(None))
    }

    //request_msg.write(socket)?;
    try!(
        client,
        u64::from((Instant::now() - init_time).subsec_nanos()),
        command_name,
        i64::from(request_msg.header.request_id),
        connstring,
        request_msg.write(socket)
    );

    //let response_msg = OpMsg::read(socket)?;
    let response_msg = try!(
        client,
        u64::from((Instant::now() - init_time).subsec_nanos()),
        command_name,
        i64::from(request_msg.header.request_id),
        connstring,
        OpMsg::read(socket)
    );

    //let doc = response_msg.get_document()?;
    let doc = try!(
        client,
        u64::from((Instant::now() - init_time).subsec_nanos()),
        command_name,
        i64::from(request_msg.header.request_id),
        connstring,
        response_msg.get_document()
    );

    if let Some(&Bson::Int32(code)) = doc.get("code") {
        if let Some(&Bson::String(ref msg)) = doc.get("errmsg") {
            return Err(Error::CodedError(CodedError{code: code, errmsg: msg.to_string()}))
        }
    }

    if let Err(_) = client.run_completion_hooks(&CommandResult::Success {
        duration: u64::from((Instant::now() - init_time).subsec_nanos()),
        reply: &doc,
        command_name,
        request_id: i64::from(request_msg.header.request_id),
        connection_string: connstring
    }) {
        return Err(Error::EventListenerError(None))
    }

    Ok(doc)
}

pub fn base_command_wihtout_hook(
    client: &MongoClient,
    stream: &mut PooledStream,
    command: &Document
) -> Result<Document> {
    let mut msg_builder = OpMsg::builder();
    let request_msg = msg_builder
        .request_id(client.get_request_id())
        .push_section(Section::from_document(command)?)
        .build();

    let socket = stream.get_socket();

    request_msg.write(socket)?;

    let response_msg = OpMsg::read(socket)?;

    let doc = response_msg.get_document()?;

    if let Some(&Bson::Int32(code)) = doc.get("code") {
        if let Some(&Bson::String(ref msg)) = doc.get("errmsg") {
            return Err(Error::CodedError(CodedError{code: code, errmsg: msg.to_string()}))
        }
    }

    Ok(doc)
}

