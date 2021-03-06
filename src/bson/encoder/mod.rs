//! Encoder

mod error;
mod serde;

pub use self::{
    error::{EncoderError, EncoderResult},
    serde::Encoder,
};

use std::{io::Write, iter::IntoIterator, mem};

use byteorder::{LittleEndian, WriteBytesExt};
use chrono::Timelike;

use crate::bson::{Binary, Bson, JavaScriptCodeWithScope, Regex};
#[cfg(feature = "decimal128")]
use crate::bson::decimal128::Decimal128;
use ::serde::Serialize;

fn write_string<W: Write + ?Sized>(writer: &mut W, s: &str) -> EncoderResult<()> {
    writer.write_i32::<LittleEndian>(s.len() as i32 + 1)?;
    writer.write_all(s.as_bytes())?;
    writer.write_u8(0)?;
    Ok(())
}

fn write_cstring<W: Write + ?Sized>(writer: &mut W, s: &str) -> EncoderResult<()> {
    writer.write_all(s.as_bytes())?;
    writer.write_u8(0)?;
    Ok(())
}

#[inline]
fn write_i32<W: Write + ?Sized>(writer: &mut W, val: i32) -> EncoderResult<()> {
    writer.write_i32::<LittleEndian>(val).map_err(From::from)
}

#[inline]
fn write_i64<W: Write + ?Sized>(writer: &mut W, val: i64) -> EncoderResult<()> {
    writer.write_i64::<LittleEndian>(val).map_err(From::from)
}

#[inline]
fn write_f64<W: Write + ?Sized>(writer: &mut W, val: f64) -> EncoderResult<()> {
    writer.write_f64::<LittleEndian>(val).map_err(From::from)
}

#[cfg(feature = "decimal128")]
#[inline]
fn write_f128<W: Write + ?Sized>(writer: &mut W, val: Decimal128) -> EncoderResult<()> {
    let raw = val.to_raw_bytes_le();
    writer.write_all(&raw).map_err(From::from)
}

fn encode_array<W: Write + ?Sized>(writer: &mut W, arr: &[Bson]) -> EncoderResult<()> {
    let mut buf = Vec::new();
    for (key, val) in arr.iter().enumerate() {
        encode_bson(&mut buf, &key.to_string(), val)?;
    }

    write_i32(
        writer,
        (buf.len() + mem::size_of::<i32>() + mem::size_of::<u8>()) as i32,
    )?;
    writer.write_all(&buf)?;
    writer.write_u8(0)?;
    Ok(())
}

/// Attempt to encode a `Document` into a byte stream.
///
/// Can encode any type which is iterable as `(key: &str, value: &Bson)` pairs,
/// which generally means most maps.
pub fn encode_document<
    'a,
    S: AsRef<str> + 'a,
    W: Write + ?Sized,
    D: IntoIterator<Item = (&'a S, &'a Bson)>,
>(
    writer: &mut W,
    doc: D,
) -> EncoderResult<()> {
    let mut buf = Vec::new();
    for (key, val) in doc.into_iter() {
        encode_bson(&mut buf, key.as_ref(), val)?;
    }

    write_i32(
        writer,
        (buf.len() + mem::size_of::<i32>() + mem::size_of::<u8>()) as i32,
    )?;
    writer.write_all(&buf)?;
    writer.write_u8(0)?;
    Ok(())
}

fn encode_bson<W: Write + ?Sized>(writer: &mut W, key: &str, val: &Bson) -> EncoderResult<()> {
    writer.write_u8(val.element_type() as u8)?;
    write_cstring(writer, key)?;

    match *val {
        Bson::FloatingPoint(v) => write_f64(writer, v),
        Bson::String(ref v) => write_string(writer, &v),
        Bson::Array(ref v) => encode_array(writer, &v),
        Bson::Document(ref v) => encode_document(writer, v),
        Bson::Boolean(v) => writer
            .write_u8(if v { 0x01 } else { 0x00 })
            .map_err(From::from),
        Bson::Regex(Regex {
            ref pattern,
            ref options,
        }) => {
            write_cstring(writer, pattern)?;
            write_cstring(writer, options)
        }
        Bson::JavaScriptCode(ref code) => write_string(writer, &code),
        Bson::ObjectId(ref id) => writer.write_all(&id.bytes()).map_err(From::from),
        Bson::JavaScriptCodeWithScope(JavaScriptCodeWithScope {
            ref code,
            ref scope,
        }) => {
            let mut buf = Vec::new();
            write_string(&mut buf, code)?;
            encode_document(&mut buf, scope)?;

            write_i32(writer, buf.len() as i32 + 4)?;
            writer.write_all(&buf).map_err(From::from)
        }
        Bson::I32(v) => write_i32(writer, v),
        Bson::I64(v) => write_i64(writer, v),
        Bson::TimeStamp(ts) => write_i64(writer, ts.to_le_i64()),
        Bson::Binary(Binary { subtype, ref bytes }) => {
            write_i32(writer, bytes.len() as i32)?;
            writer.write_u8(From::from(subtype))?;
            writer.write_all(bytes).map_err(From::from)
        }
        Bson::UtcDatetime(ref v) => write_i64(
            writer,
            (v.timestamp() * 1000) + (v.nanosecond() / 1_000_000) as i64,
        ),
        Bson::Null => Ok(()),
        Bson::Symbol(ref v) => write_string(writer, &v),
        #[cfg(feature = "decimal128")]
        Bson::Decimal128(ref v) => write_f128(writer, v.clone()),
    }
}

/// Encode a `T` Serializable into a BSON `Value`.
pub fn to_bson<T: ?Sized>(value: &T) -> EncoderResult<Bson>
where
    T: Serialize,
{
    let ser = Encoder::new();
    value.serialize(ser)
}
