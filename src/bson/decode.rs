use std::{io, error, fmt, string};
use std::io::Read;

use serde::de::Deserialize;
use byteorder::{LittleEndian, ReadBytesExt};
use chrono::Utc;
use chrono::offset::TimeZone;

use object_id::ObjectId;
use super::doc::Document;
use super::bson::Bson;
use super::bson::Array;
use super::spec::ElementType;
use super::spec::BinarySubtype;
use super::serde::decode::Decoder;

#[derive(Debug)]
pub enum DecodeError {
    IoError(io::Error),
    FromUtf8Error(string::FromUtf8Error),
    UnrecognizedElementType(u8),
    InvalidArrayKey(usize, String),
    ExpectedField(&'static str),
    UnknownField(String),
    SyntaxError(String),
    EndOfStream,
    InvalidType(String),
    InvalidLength(usize, String),
    DuplicatedField(&'static str),
    UnknownVariant(String),
    InvalidValue(String),
    Unknown(String)
}

impl From<io::Error> for DecodeError {
    fn from(err: io::Error) -> DecodeError {
        DecodeError::IoError(err)
    }
}

impl From<string::FromUtf8Error> for DecodeError {
    fn from(err: string::FromUtf8Error) -> DecodeError {
        DecodeError::FromUtf8Error(err)
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodeError::IoError(ref inner) => inner.fmt(fmt),
            DecodeError::FromUtf8Error(ref inner) => inner.fmt(fmt),
            DecodeError::UnrecognizedElementType(tag) => {
                write!(fmt, "Unrecognized element type `{}`", tag)
            }
            DecodeError::InvalidArrayKey(ref want, ref got) => {
                write!(fmt, "Invalid array key: expected `{}`, got `{}`", want, got)
            }
            DecodeError::ExpectedField(field_type) => {
                write!(fmt, "Expected a field of type `{}`", field_type)
            }
            DecodeError::UnknownField(ref field) => write!(fmt, "Unknown field `{}`", field),
            DecodeError::SyntaxError(ref inner) => inner.fmt(fmt),
            DecodeError::EndOfStream => write!(fmt, "End of stream"),
            DecodeError::InvalidType(ref desc) => desc.fmt(fmt),
            DecodeError::InvalidLength(ref len, ref desc) => {
                write!(fmt, "Expecting length {}, {}", len, desc)
            }
            DecodeError::DuplicatedField(ref field) => write!(fmt, "Duplicated field `{}`", field),
            DecodeError::UnknownVariant(ref var) => write!(fmt, "Unknown variant `{}`", var),
            DecodeError::InvalidValue(ref desc) => desc.fmt(fmt),
            DecodeError::Unknown(ref inner) => inner.fmt(fmt),
        }
    }
}

impl error::Error for DecodeError {
    fn description(&self) -> &str {
        match *self {
            DecodeError::IoError(ref inner) => inner.description(),
            DecodeError::FromUtf8Error(ref inner) => inner.description(),
            DecodeError::UnrecognizedElementType(_) => "Unrecognized element type",
            DecodeError::InvalidArrayKey(_, _) => "Invalid array key",
            DecodeError::ExpectedField(_) => "Expected a field",
            DecodeError::UnknownField(_) => "Found an unknown field",
            DecodeError::SyntaxError(ref inner) => inner,
            DecodeError::EndOfStream => "End of stream",
            DecodeError::InvalidType(ref desc) => desc,
            DecodeError::InvalidLength(_, ref desc) => desc,
            DecodeError::DuplicatedField(_) => "Duplicated field",
            DecodeError::UnknownVariant(_) => "Unknown variant",
            DecodeError::InvalidValue(ref desc) => desc,
            DecodeError::Unknown(ref inner) => inner,
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DecodeError::IoError(ref inner) => Some(inner),
            DecodeError::FromUtf8Error(ref inner) => Some(inner),
            _ => None,
        }
    }
}

pub type DecodeResult<T> = Result<T, DecodeError>;

fn read_string<R: Read + ?Sized>(reader: &mut R) -> DecodeResult<String> {
    let len = reader.read_i32::<LittleEndian>()?;

    let mut s = String::with_capacity(len as usize - 1);
    reader.take(len as u64 - 1).read_to_string(&mut s)?;
    reader.read_u8()?; // The last 0x00

    Ok(s)
}

fn read_cstring<R: Read + ?Sized>(reader: &mut R) -> DecodeResult<String> {
    let mut v = Vec::new();

    loop {
        let c = reader.read_u8()?;
        if c == 0 {
            break;
        }
        v.push(c);
    }

    Ok(String::from_utf8(v)?)
}

#[inline]
fn read_i32<R: Read + ?Sized>(reader: &mut R) -> DecodeResult<i32> {
    reader.read_i32::<LittleEndian>().map_err(From::from)
}

#[inline]
fn read_i64<R: Read + ?Sized>(reader: &mut R) -> DecodeResult<i64> {
    reader.read_i64::<LittleEndian>().map_err(From::from)
}

/// Attempt to decode a `Document` from a byte stream.
pub fn decode_document<R: Read + ?Sized>(reader: &mut R) -> DecodeResult<Document> {
    let mut doc = Document::new();

    // disregard the length: using Read::take causes infinite type recursion
    read_i32(reader)?;

    loop {
        let tag = reader.read_u8()?;

        if tag == 0 {
            break;
        }

        let key = read_cstring(reader)?;
        let val = decode_bson(reader, tag)?;

        doc.insert(key, val);
    }

    Ok(doc)
}

fn decode_array<R: Read + ?Sized>(reader: &mut R) -> DecodeResult<Array> {
    let mut arr = Array::new();

    // disregard the length: using Read::take causes infinite type recursion
    read_i32(reader)?;

    loop {
        let tag = reader.read_u8()?;
        if tag == 0 {
            break;
        }

        // check that the key is as expected
        let key = read_cstring(reader)?;
        match key.parse::<usize>() {
            Err(..) => return Err(DecodeError::InvalidArrayKey(arr.len(), key)),
            Ok(idx) => {
                if idx != arr.len() {
                    return Err(DecodeError::InvalidArrayKey(arr.len(), key));
                }
            }
        }

        let val = decode_bson(reader, tag)?;
        arr.push(val)
    }

    Ok(arr)
}

fn decode_bson<R: Read + ?Sized>(reader: &mut R, tag: u8) -> DecodeResult<Bson> {
    match ElementType::from(tag) {
        Some(ElementType::Double) => {
            Ok(Bson::Double(reader.read_f64::<LittleEndian>()?))
        }
        Some(ElementType::Utf8String) => {
            read_string(reader).map(Bson::String)
        }
        Some(ElementType::Document) => {
            decode_document(reader).map(Bson::Document)
        }
        Some(ElementType::Array) => {
            decode_array(reader).map(Bson::Array)
        }
        Some(ElementType::Binary) => {
            let len = read_i32(reader)?;
            let subtype = BinarySubtype::from(reader.read_u8()?);
            let mut data = Vec::with_capacity(len as usize);
            
            reader.take(len as u64).read_to_end(&mut data)?;
            
            Ok(Bson::Binary(subtype, data))
        }
        Some(ElementType::ObjectId) => {
            let mut objid = [0; 12];
            
            for x in &mut objid {
                *x = reader.read_u8()?;
            }
            
            Ok(Bson::ObjectId(ObjectId::with_bytes(objid)))
        }
        Some(ElementType::Boolean) => {
            Ok(Bson::Boolean(reader.read_u8()? != 0))
        }
        Some(ElementType::NullValue) => {
            Ok(Bson::Null)
        }
        Some(ElementType::RegularExpression) => {
            let pat = read_cstring(reader)?;
            let opt = read_cstring(reader)?;
            
            Ok(Bson::RegExp(pat, opt))
        }
        Some(ElementType::JavaScriptCode) => {
            read_string(reader).map(Bson::JavaScriptCode)
        }
        Some(ElementType::JavaScriptCodeWithScope) => {
            // disregard the length:
            //     using Read::take causes infinite type recursion
            read_i32(reader)?;

            let code = read_string(reader)?;
            let scope = decode_document(reader)?;
            
            Ok(Bson::JavaScriptCodeWithScope(code, scope))
        }
        Some(ElementType::Int32) => {
            read_i32(reader).map(Bson::Int32)
        }
        Some(ElementType::Int64) => {
            read_i64(reader).map(Bson::Int64)
        }
        Some(ElementType::TimeStamp) => {
            read_i64(reader).map(Bson::TimeStamp)
        }
        Some(ElementType::UTCDatetime) => {
            let time = read_i64(reader)?;
            Ok(Bson::UTCDatetime(Utc.timestamp(time / 1000, (time % 1000) as u32 * 1000000)))
        }
        Some(ElementType::Symbol) => {
            read_string(reader).map(Bson::Symbol)
        }
        Some(ElementType::Undefiend) | Some(ElementType::DBPointer) | Some(ElementType::MaxKey) | Some(ElementType::MinKey) | None => {
            Err(DecodeError::UnrecognizedElementType(tag))
        }
    }
}

/// Decode a BSON `Value` into a `T` Deserializable.
pub fn from_bson<'de, T>(bson: Bson) -> DecodeResult<T>
    where T: Deserialize<'de>
{
    let de = Decoder::new(bson);
    Deserialize::deserialize(de)
}
