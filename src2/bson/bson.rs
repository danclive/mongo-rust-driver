use std::fmt;
use std::ops::{Deref, DerefMut};

use chrono::{DateTime, Timelike, Utc};
use chrono::offset::TimeZone;
use serde_json::Value;

use object_id::ObjectId;
use util::hex::{ToHex, FromHex};
use super::doc::Document;
use super::spec::BinarySubtype;
use super::spec::ElementType;

#[derive(Clone, PartialEq)]
pub enum Bson {
    Double(f64),
    String(String),
    Array(Array),
    Document(Document),
    Boolean(bool),
    Null,
    RegExp(String, String),
    JavaScriptCode(String),
    JavaScriptCodeWithScope(String, Document),
    Int32(i32),
    Int64(i64),
    TimeStamp(i64),
    Binary(BinarySubtype, Vec<u8>),
    ObjectId(ObjectId),
    UTCDatetime(DateTime<Utc>),
    Symbol(String)
}

pub type Array = Vec<Bson>;

impl fmt::Debug for Bson {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Bson::Double(p) => write!(fmt, "Double({:?})", p),
            Bson::String(ref s) => write!(fmt, "String({})", s),
            Bson::Array(ref vec) => write!(fmt, "Array({:?})", vec),
            Bson::Document(ref doc) => write!(fmt, "{:?}", doc),
            Bson::Boolean(b) => write!(fmt, "Boolean({:?})", b),
            Bson::Null => write!(fmt, "Null"),
            Bson::RegExp(ref pat, ref opt) => write!(fmt, "RegExp(/{:?}/{:?})", pat, opt),
            Bson::JavaScriptCode(ref s) => write!(fmt, "JavaScriptCode({:?})", s),
            Bson::JavaScriptCodeWithScope(ref s, ref scope) => {
                write!(fmt, "JavaScriptCodeWithScope({:?}, {:?})", s, scope)
            }
            Bson::Int32(v) => write!(fmt, "Int32({:?})", v),
            Bson::Int64(v) => write!(fmt, "Int64({:?})", v),
            Bson::TimeStamp(i) => {
                let time = (i >> 32) as i32;
                let inc = (i & 0xFFFF_FFFF) as i32;

                write!(fmt, "TimeStamp({}, {})", time, inc)
            }
            Bson::Binary(t, ref vec) => write!(fmt, "BinData({}, 0x{})", u8::from(t), vec.to_hex()),
            Bson::ObjectId(ref id) => write!(fmt, "ObjectId({})", id),
            Bson::UTCDatetime(date_time) => write!(fmt, "UTCDatetime({:?})", date_time),
            Bson::Symbol(ref sym) => write!(fmt, "Symbol({:?})", sym),
        }
    }
}

impl fmt::Display for Bson {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Bson::Double(f) => write!(fmt, "{}", f),
            Bson::String(ref s) => write!(fmt, "\"{}\"", s),
            Bson::Array(ref vec) => {
                write!(fmt, "[")?;

                let mut first = true;
                for bson in vec.iter() {
                    if !first {
                        write!(fmt, ", ")?;
                    }

                    write!(fmt, "{}", bson)?;
                    first = false;
                }

                write!(fmt, "]")
            }
            Bson::Document(ref doc) => write!(fmt, "{}", doc),
            Bson::Boolean(b) => write!(fmt, "{}", b),
            Bson::Null => write!(fmt, "null"),
            Bson::RegExp(ref pat, ref opt) => write!(fmt, "/{}/{}", pat, opt),
            Bson::JavaScriptCode(ref s) |
            Bson::JavaScriptCodeWithScope(ref s, _) => fmt.write_str(&s),
            Bson::Int32(i) => write!(fmt, "{}", i),
            Bson::Int64(i) => write!(fmt, "{}", i),
            Bson::TimeStamp(i) => {
                let time = (i >> 32) as i32;
                let inc = (i & 0xFFFF_FFFF) as i32;

                write!(fmt, "Timestamp({}, {})", time, inc)
            }
            Bson::Binary(t, ref vec) => {
                write!(fmt, "BinData({}, 0x{})", u8::from(t), vec.to_hex())
            }
            Bson::ObjectId(ref id) => write!(fmt, "ObjectId(\"{}\")", id),
            Bson::UTCDatetime(date_time) => write!(fmt, "Date(\"{}\")", date_time),
            Bson::Symbol(ref sym) => write!(fmt, "Symbol(\"{}\")", sym),
        }
    }
}

impl From<f32> for Bson {
    fn from(f: f32) -> Bson {
        Bson::Double(f64::from(f))
    }
}

impl From<f64> for Bson {
    fn from(f: f64) -> Bson {
        Bson::Double(f)
    }
}

impl<'a> From<&'a str> for Bson {
    fn from(s: &str) -> Bson {
        Bson::String(s.to_owned())
    }
}

impl From<String> for Bson {
    fn from(s: String) -> Bson {
        Bson::String(s)
    }
}

impl<'a> From<&'a String> for Bson {
    fn from(s: &'a String) -> Bson {
        Bson::String(s.to_owned())
    }
}

impl From<Document> for Bson {
    fn from(d: Document) -> Bson {
        Bson::Document(d)
    }
}

impl From<bool> for Bson {
    fn from(b: bool) -> Bson {
        Bson::Boolean(b)
    }
}

impl From<(String, String)> for Bson {
    fn from(r: (String, String)) -> Bson {
        let (r1, r2) = r;
        Bson::RegExp(r1, r2)
    }
}

impl From<(String, Document)> for Bson {
    fn from(j: (String, Document)) -> Bson {
        let (j1, j2) = j;
        Bson::JavaScriptCodeWithScope(j1, j2)
    }
}

impl From<(BinarySubtype, Vec<u8>)> for Bson {
    fn from(b: (BinarySubtype, Vec<u8>)) -> Bson {
        let (b1, b2) = b;
        Bson::Binary(b1, b2)
    }
}

impl From<i32> for Bson {
    fn from(i: i32) -> Bson {
        Bson::Int32(i)
    }
}

impl From<i64> for Bson {
    fn from(i: i64) -> Bson {
        Bson::Int64(i)
    }
}

impl From<u32> for Bson {
    fn from(a: u32) -> Bson {
        Bson::Int32(a as i32)
    }
}

impl From<u64> for Bson {
    fn from(a: u64) -> Bson {
        Bson::Int64(a as i64)
    }
}
impl From<[u8; 12]> for Bson {
    fn from(o: [u8; 12]) -> Bson {
        Bson::ObjectId(ObjectId::with_bytes(o))
    }
}

impl From<ObjectId> for Bson {
    fn from(o: ObjectId) -> Bson {
        Bson::ObjectId(o)
    }
}

impl From<DateTime<Utc>> for Bson {
    fn from(d: DateTime<Utc>) -> Bson {
        Bson::UTCDatetime(d)
    }
}

impl<T> From<Vec<T>> for Bson where Bson: From<T> {
    fn from(vec: Vec<T>) -> Bson {
        Bson::Array(vec.into_iter().map(|v| Bson::from(v)).collect::<Vec<Bson>>())
    }
}

impl Bson {
    pub fn element_type(&self) -> ElementType {
        match *self {
            Bson::Double(..) => ElementType::Double,
            Bson::String(..) => ElementType::Utf8String,
            Bson::Array(..) => ElementType::Array,
            Bson::Document(..) => ElementType::Document,
            Bson::Boolean(..) => ElementType::Boolean,
            Bson::Null => ElementType::NullValue,
            Bson::RegExp(..) => ElementType::RegularExpression,
            Bson::JavaScriptCode(..) => ElementType::JavaScriptCode,
            Bson::JavaScriptCodeWithScope(..) => ElementType::JavaScriptCodeWithScope,
            Bson::Int32(..) => ElementType::Int32,
            Bson::Int64(..) => ElementType::Int64,
            Bson::TimeStamp(..) => ElementType::TimeStamp,
            Bson::Binary(..) => ElementType::Binary,
            Bson::ObjectId(..) => ElementType::ObjectId,
            Bson::UTCDatetime(..) => ElementType::UTCDatetime,
            Bson::Symbol(..) => ElementType::Symbol,
        }
    }

    /// If `Bson` is `Double`, return its value. Returns `None` otherwise
    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Bson::Double(ref v) => Some(*v),
            _ => None,
        }
    }

    /// If `Bson` is `String`, return its value. Returns `None` otherwise
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Bson::String(ref s) => Some(s),
            _ => None,
        }
    }

    /// If `Bson` is `Array`, return its value. Returns `None` otherwise
    pub fn as_array(&self) -> Option<&Array> {
        match *self {
            Bson::Array(ref v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `Document`, return its value. Returns `None` otherwise
    pub fn as_document(&self) -> Option<&Document> {
        match *self {
            Bson::Document(ref v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `Boolean`, return its value. Returns `None` otherwise
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Bson::Boolean(ref v) => Some(*v),
            _ => None,
        }
    }

    /// If `Bson` is `I32`, return its value. Returns `None` otherwise
    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            Bson::Int32(ref v) => Some(*v),
            _ => None,
        }
    }

    /// If `Bson` is `I64`, return its value. Returns `None` otherwise
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Bson::Int64(ref v) => Some(*v),
            _ => None,
        }
    }

    /// If `Bson` is `Objectid`, return its value. Returns `None` otherwise
    pub fn as_object_id(&self) -> Option<&ObjectId> {
        match *self {
            Bson::ObjectId(ref v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `UtcDateTime`, return its value. Returns `None` otherwise
    pub fn as_utc_date_time(&self) -> Option<&DateTime<Utc>> {
        match *self {
            Bson::UTCDatetime(ref v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `Symbol`, return its value. Returns `None` otherwise
    pub fn as_symbol(&self) -> Option<&str> {
        match *self {
            Bson::Symbol(ref v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `TimeStamp`, return its value. Returns `None` otherwise
    pub fn as_timestamp(&self) -> Option<i64> {
        match *self {
            Bson::TimeStamp(v) => Some(v),
            _ => None,
        }
    }

    /// If `Bson` is `Null`, return its value. Returns `None` otherwise
    pub fn as_null(&self) -> Option<()> {
        match *self {
            Bson::Null => Some(()),
            _ => None,
        }
    }

    pub fn to_json(&self) -> Value {
        self.clone().into()
    }

    pub fn into_json(self) -> Value {
        self.into()
    }

    pub fn from_json(val: Value) -> Bson {
        val.into()
    }

    pub fn to_extended_document(&self) -> Document {
        match *self {
            Bson::RegExp(ref pat, ref opt) => {
                doc!{
                    "$regex": pat.clone(),
                    "$options": opt.clone()
                }
            }
            Bson::JavaScriptCode(ref code) => {
                doc!{
                    "$code": code.clone()
                }
            }
            Bson::JavaScriptCodeWithScope(ref code, ref scope) => {
                doc!{
                    "$code": code.clone(),
                    "$scope": scope.clone()
                }
            }
            Bson::TimeStamp(v) => {
                let time = (v >> 32) as i32;
                let inc = (v & 0xFFFF_FFFF) as i32;

                doc!{
                    "t": time,
                    "i": inc
                }
            }
            Bson::Binary(t, ref v) => {
                let tval: u8 = From::from(t);
                doc!{
                    "$binary": v.to_hex(),
                    "type": i64::from(tval)
                }
            }
            Bson::ObjectId(ref v) => {
                doc!{
                    "$oid": v.to_string()
                }
            }
            Bson::UTCDatetime(ref v) => {
                doc!{
                    "$date": {
                        "$numberLong": v.timestamp() * 1000 + i64::from(v.nanosecond()) / 1_000_000
                    }
                }
            }
            Bson::Symbol(ref v) => {
                doc!{
                    "$symbol": v.to_owned()
                }
            }
            _ => panic!("Attempted conversion of invalid data type: {}", self)
        }
    }

    /// Converts from extended format.
    /// This function mainly used for [extended JSON format](https://docs.mongodb.com/manual/reference/mongodb-extended-json/).
    #[doc(hidden)]
    pub fn from_extended_document(values: Document) -> Bson {
        if values.len() == 2 {
            if let (Ok(pat), Ok(opt)) = (values.get_str("$regex"), values.get_str("$options")) {
                return Bson::RegExp(pat.to_owned(), opt.to_owned());

            } else if let (Ok(code), Ok(scope)) =
                (values.get_str("$code"), values.get_document("$scope")) {
                return Bson::JavaScriptCodeWithScope(code.to_owned(), scope.clone());

            } else if let (Ok(t), Ok(i)) = (values.get_i32("t"), values.get_i32("i")) {
                let timestamp = (i64::from(t) << 32) + i64::from(i);
                return Bson::TimeStamp(timestamp);

            } else if let (Ok(t), Ok(i)) = (values.get_i64("t"), values.get_i64("i")) {
                let timestamp = (t << 32) + i;
                return Bson::TimeStamp(timestamp);

            } else if let (Ok(hex), Ok(t)) = (values.get_str("$binary"), values.get_i64("type")) {
                let ttype = t as u8;
                return Bson::Binary(From::from(ttype), FromHex::from_hex(hex.as_bytes()).unwrap());
            }

        } else if values.len() == 1 {
            if let Ok(code) = values.get_str("$code") {
                return Bson::JavaScriptCode(code.to_string());

            } else if let Ok(hex) = values.get_str("$oid") {
                return Bson::ObjectId(ObjectId::with_string(hex).unwrap());

            } else if let Ok(long) = values.get_document("$date").and_then(|inner| inner.get_i64("$numberLong")) {
                return Bson::UTCDatetime(Utc.timestamp(long / 1000, ((long % 1000) * 1_000_000) as u32));
            } else if let Ok(sym) = values.get_str("$symbol") {
                return Bson::Symbol(sym.to_string());
            }
        }

        Bson::Document(values)
    }

}


impl From<Value> for Bson {
    fn from(a: Value) -> Bson {
        match a {
            Value::Number(x) => {
                x.as_i64().map(Bson::from)
                    .or_else(|| x.as_u64().map(Bson::from))
                    .or_else(|| x.as_f64().map(Bson::from))
                    .unwrap_or_else(|| panic!("Invalid number value: {}", x))
            }
            Value::String(x) => x.into(),
            Value::Bool(x) => x.into(),
            Value::Array(x) => Bson::Array(x.into_iter().map(Bson::from).collect()),
            Value::Object(x) => {
                Bson::from_extended_document(
                    x.into_iter().map(|(k, v)| (k.clone(), v.into())).collect()
                )
            }
            Value::Null => Bson::Null,
        }
    }
}

impl Into<Value> for Bson {
    fn into(self) -> Value {
        match self {
            Bson::Double(v) => json!(v),
            Bson::String(v) => json!(v),
            Bson::Array(v) => json!(v),
            Bson::Document(v) => json!(v),
            Bson::Boolean(v) => json!(v),
            Bson::Null => Value::Null,
            Bson::RegExp(pat, opt) => {
                json!({
                    "$regex": pat,
                    "$options": opt
                })
            }
            Bson::JavaScriptCode(code) => json!({"$code": code}),
            Bson::JavaScriptCodeWithScope(code, scope) => {
                json!({
                    "$code": code,
                    "scope": scope
                })
            }
            Bson::Int32(v) => v.into(),
            Bson::Int64(v) => v.into(),
            Bson::TimeStamp(v) => {
                let time = v >> 32;
                let inc = v & 0x0000_FFFF;
                json!({
                    "t": time,
                    "i": inc
                })
            }
            Bson::Binary(t, ref v) => {
                let tval: u8 = From::from(t);
                json!({
                    "type": tval,
                    "$binary": v.to_hex()
                })
            }
            Bson::ObjectId(v) => json!({"$oid": v.to_string()}),
            Bson::UTCDatetime(v) => {
                json!({
                    "$date": {
                        "$numberLong": (v.timestamp() * 1000) + i64::from(v.nanosecond() / 1_000_000)
                    }
                })
            }
            // FIXME: Don't know what is the best way to encode Symbol type
            Bson::Symbol(v) => json!({"$symbol": v}),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct UTCDateTime(pub DateTime<Utc>);

impl Deref for UTCDateTime {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UTCDateTime {
    fn deref_mut(&mut self) -> &mut DateTime<Utc> {
        &mut self.0
    }
}

impl Into<DateTime<Utc>> for UTCDateTime {
    fn into(self) -> DateTime<Utc> {
        self.0
    }
}

impl From<DateTime<Utc>> for UTCDateTime {
    fn from(x: DateTime<Utc>) -> Self {
        UTCDateTime(x)
    }
}
