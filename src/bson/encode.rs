use std::io::{self, Write};
use std::fmt;
use std::error;
use std::mem;

use serde::ser::{self, Serialize};
use chrono::Timelike;
use byteorder::{LittleEndian, WriteBytesExt};

use super::bson::Bson;
use super::serde::encode::Encoder;

#[derive(Debug)]
pub enum EncodeError {
    IoError(io::Error),
    InvalidMapKeyType(Bson),
    Unknown(String),
    UnsupportedUnsignedType
}

impl From<io::Error> for EncodeError {
    fn from(err: io::Error) -> EncodeError {
        EncodeError::IoError(err)
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncodeError::IoError(ref inner) => inner.fmt(fmt),
            EncodeError::InvalidMapKeyType(ref bson) => {
                write!(fmt, "Invalid map key type: {:?}", bson)
            }
            EncodeError::Unknown(ref inner) => inner.fmt(fmt),
            EncodeError::UnsupportedUnsignedType => write!(fmt, "BSON does not support unsigned type"),
        }
    }
}

impl error::Error for EncodeError {
    fn description(&self) -> &str {
        match *self {
            EncodeError::IoError(ref inner) => inner.description(),
            EncodeError::InvalidMapKeyType(_) => "Invalid map key type",
            EncodeError::Unknown(ref inner) => inner,
            EncodeError::UnsupportedUnsignedType => "BSON does not support unsigned type",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EncodeError::IoError(ref inner) => Some(inner),
            _ => None,
        }
    }
}

impl ser::Error for EncodeError {
    fn custom<T: fmt::Display>(msg: T) -> EncodeError {
        EncodeError::Unknown(msg.to_string())
    }
}

pub type EncodeResult<T> = Result<T, EncodeError>;

fn write_string<W>(writer: &mut W, s: &str) -> EncodeResult<()> 
    where W: Write + ?Sized
{
    writer.write_i32::<LittleEndian>(s.len() as i32 + 1)?;
    writer.write_all(s.as_bytes())?;
    writer.write_u8(0)?;
    Ok(())
}

fn write_cstring<W>(writer: &mut W, s: &str) -> EncodeResult<()>
    where W: Write + ?Sized
{
    writer.write_all(s.as_bytes())?;
    writer.write_u8(0)?;
    Ok(())
}

#[inline]
fn write_i32<W>(writer: &mut W, val: i32) -> EncodeResult<()> 
    where W: Write + ?Sized
{
    writer.write_i32::<LittleEndian>(val).map_err(From::from)
}

#[inline]
fn write_i64<W>(writer: &mut W, val: i64) -> EncodeResult<()>
    where W: Write + ?Sized
{
    writer.write_i64::<LittleEndian>(val).map_err(From::from)
}

#[inline]
fn write_f64<W>(writer: &mut W, val: f64) -> EncodeResult<()>
    where W: Write + ?Sized
{
    writer.write_f64::<LittleEndian>(val).map_err(From::from)
}

fn encode_array<W>(writer: &mut W, arr: &[Bson]) -> EncodeResult<()>
    where W: Write + ?Sized
{
    let mut buf = Vec::new();
    for (key, val) in arr.iter().enumerate() {
        encode_bson(&mut buf, &key.to_string(), val)?;
    }

    write_i32(
        writer,
        (buf.len() + mem::size_of::<i32>() + mem::size_of::<u8>()) as i32
    )?;

    writer.write_all(&buf)?;
    writer.write_u8(0)?;
    Ok(())
}

pub fn encode_document<'a, S, W, D> (writer: &mut W, doc: D) -> EncodeResult<()>
    where S: AsRef<str> + 'a, W: Write + ?Sized, D: IntoIterator<Item = (&'a S, &'a Bson)>
{
    let mut buf = Vec::new();
    for (key, val) in doc {
        encode_bson(&mut buf, key.as_ref(), val)?;
    }

    write_i32(
        writer,
        (buf.len() + mem::size_of::<i32>() + mem::size_of::<u8>()) as i32
    )?;

    writer.write_all(&buf)?;
    writer.write_u8(0)?;
    Ok(())
}

fn encode_bson<W>(writer: &mut W, key: &str, val: &Bson) -> EncodeResult<()>
    where W: Write + ?Sized
{
    writer.write_u8(val.element_type() as u8)?;
    write_cstring(writer, key)?;

    match *val {
        Bson::Double(v) => write_f64(writer, v),
        Bson::String(ref v) => write_string(writer, &v),
        Bson::Array(ref v) => encode_array(writer, &v),
        Bson::Document(ref v) => encode_document(writer, v),
        Bson::Boolean(v) => writer.write_u8(if v { 0x01 } else { 0x00 }).map_err(From::from),
        Bson::RegExp(ref pat, ref opt) => {
            write_cstring(writer, pat)?;
            write_cstring(writer, opt)
        }
        Bson::JavaScriptCode(ref code) => write_string(writer, &code),
        Bson::ObjectId(ref id) => writer.write_all(&id.bytes()).map_err(From::from),
        Bson::JavaScriptCodeWithScope(ref code, ref scope) => {
            let mut buf = Vec::new();
            write_string(&mut buf, code)?;
            encode_document(&mut buf, scope)?;

            write_i32(writer, buf.len() as i32 + 4)?;
            writer.write_all(&buf).map_err(From::from)
        }
        Bson::Int32(v) => write_i32(writer, v),
        Bson::Int64(v) => write_i64(writer, v),
        Bson::TimeStamp(v) => write_i64(writer, v),
        Bson::Binary(subtype, ref data) => {
            write_i32(writer, data.len() as i32)?;
            writer.write_u8(From::from(subtype))?;
            writer.write_all(data).map_err(From::from)
        }
        Bson::UTCDatetime(ref v) => {
            write_i64(
                writer,
                v.timestamp() * 1000 + i64::from(v.nanosecond() / 1_000_000)
            )
        }
        Bson::Null => Ok(()),
        Bson::Symbol(ref v) => write_string(writer, &v),
    }
}

/// Encode a `T` Serializable into a BSON `Value`.
pub fn to_bson<T: ?Sized>(value: &T) -> EncodeResult<Bson>
    where T: Serialize
{
    let ser = Encoder::new();
    value.serialize(ser)
}
