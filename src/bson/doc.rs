use std::fmt;
use std::error;
use std::result;
use std::iter::{FromIterator, Map};

use linked_hash_map::{self, LinkedHashMap};
use chrono::{DateTime, Utc};

use object_id::ObjectId;
use super::bson::Bson;
use super::bson::Array;
use super::spec::BinarySubtype;


#[derive(PartialEq)]
pub enum Error {
    NotPresent,
    UnexpectedType,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotPresent => write!(f, "ValueAccessError: field is not present"),
            Error::UnexpectedType => {
                write!(f, "ValueAccessError: field does not have the expected type")
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotPresent => write!(f, "Field is not present"),
            Error::UnexpectedType => write!(f, "Field does not have the expected type"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Error to indicate that either a value was empty or it contained an unexpected type"
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct Document {
    inner: LinkedHashMap<String, Bson>
}

impl Document {
    pub fn new() -> Document {
        Document {
            inner: LinkedHashMap::new()
        }
    }

    pub fn iter(&self) -> DocumentIterator<'_> {
        self.into_iter()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn get(&self, key: &str) -> Option<&Bson> {
        self.inner.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Bson> {
        self.inner.get_mut(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    pub fn keys<'a>(&'a self) -> Keys<'a> {
        fn first<A, B>((a, _): (A, B)) -> A {
            a
        }

        let first: fn((&'a String, &'a Bson)) -> &'a String = first;

        Keys {
            inner: self.iter().map(first)
        }
    }

    pub fn values<'a>(&'a self) -> Values<'a> {
        fn second<A, B>((_, b): (A, B)) -> B {
            b
        }
        let second: fn((&'a String, &'a Bson)) -> &'a Bson = second;

        Values { 
            inner: self.iter().map(second)
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn insert<K: Into<String>, V: Into<Bson>>(&mut self, key: K, value: V) -> Option<Bson> {
        self.insert_bson(key.into(), value.into())
    }

    pub fn insert_bson(&mut self, key: String, value: Bson) -> Option<Bson> {
        self.inner.insert(key, value)
    }

    pub fn remove(&mut self, key: &str) -> Option<Bson> {
        self.inner.remove(key)
    }

    pub fn get_f64(&self, key: &str) -> Result<f64> {
        match self.get(key) {
            Some(&Bson::Double(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_str(&self, key: &str) -> Result<&str> {
        match self.get(key) {
            Some(&Bson::String(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_array(&self, key: &str) -> Result<&Array> {
        match self.get(key) {
            Some(&Bson::Array(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_document(&self, key: &str) -> Result<&Document> {
        match self.get(key) {
            Some(&Bson::Document(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_bool(&self, key: &str) -> Result<bool> {
        match self.get(key) {
            Some(&Bson::Boolean(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn is_null(&self, key: &str) -> bool {
        self.get(key) == Some(&Bson::Null)
    }

    pub fn get_i32(&self, key: &str) -> Result<i32> {
        match self.get(key) {
            Some(&Bson::Int32(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_i64(&self, key: &str) -> Result<i64> {
        match self.get(key) {
            Some(&Bson::Int64(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    pub fn get_time_stamp(&self, key: &str) -> Result<i64> {
        match self.get(key) {
            Some(&Bson::TimeStamp(v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    /// Get a generic binary value for this key if it exists and has the correct type.
    pub fn get_binary_generic(&self, key: &str) -> Result<&Vec<u8>> {
        match self.get(key) {
            Some(&Bson::Binary(BinarySubtype::Generic, ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    /// Get an object id value for this key if it exists and has the correct type.
    pub fn get_object_id(&self, key: &str) -> Result<&ObjectId> {
        match self.get(key) {
            Some(&Bson::ObjectId(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    /// Get a UTC datetime value for this key if it exists and has the correct type.
    pub fn get_utc_datetime(&self, key: &str) -> Result<&DateTime<Utc>> {
        match self.get(key) {
            Some(&Bson::UTCDatetime(ref v)) => Ok(v),
            Some(_) => Err(Error::UnexpectedType),
            None => Err(Error::NotPresent),
        }
    }

    /// Extends an other document.
    pub fn extend<I: Into<Document>>(&mut self, iter: I) {
        self.inner.extend(iter.into());
    }
}

impl fmt::Display for Document {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{{")?;

        let mut first = true;
        for (k, v) in self.iter() {
            if first {
                first = false;
                write!(fmt, " ")?;
            } else {
                write!(fmt, ", ")?;
            }

            write!(fmt, "{}: {}", k, v)?;
        }

        write!(fmt, "{}}}", if !first { " " } else { "" })?;

        Ok(())
    }
}

impl fmt::Debug for Document {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Document({:?})", self.inner)
    }
}

pub struct DocumentIntoIterator {
    inner: LinkedHashMap<String, Bson>,
}

pub struct DocumentIterator<'a> {
    inner: linked_hash_map::Iter<'a, String, Bson>,
}

pub struct Keys<'a> {
    inner: Map<DocumentIterator<'a>, fn((&'a String, &'a Bson)) -> &'a String>,
}

pub struct Values<'a> {
    inner: Map<DocumentIterator<'a>, fn((&'a String, &'a Bson)) -> &'a Bson>,
}

impl<'a> Iterator for Keys<'a> {
    type Item = &'a String;
    fn next(&mut self) -> Option<(&'a String)> {
        self.inner.next()
    }
}

impl<'a> Iterator for Values<'a> {
    type Item = &'a Bson;
    fn next(&mut self) -> Option<(&'a Bson)> {
        self.inner.next()
    }
}

impl IntoIterator for Document {
    type Item = (String, Bson);
    type IntoIter = DocumentIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        DocumentIntoIterator { inner: self.inner }
    }
}

impl<'a> IntoIterator for &'a Document {
    type Item = (&'a String, &'a Bson);
    type IntoIter = DocumentIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DocumentIterator { inner: self.inner.iter() }
    }
}


impl FromIterator<(String, Bson)> for Document {
    fn from_iter<T: IntoIterator<Item = (String, Bson)>>(iter: T) -> Self {
        let mut doc = Document::new();
        for (k, v) in iter {
            doc.insert(k, v);
        }
        doc
    }
}

impl<'a> Iterator for DocumentIntoIterator {
    type Item = (String, Bson);
    fn next(&mut self) -> Option<(String, Bson)> {
        self.inner.pop_front()
    }
}

impl<'a> Iterator for DocumentIterator<'a> {
    type Item = (&'a String, &'a Bson);
    fn next(&mut self) -> Option<(&'a String, &'a Bson)> {
        self.inner.next()
    }
}

impl From<LinkedHashMap<String, Bson>> for Document {
    fn from(tree: LinkedHashMap<String, Bson>) -> Document {
        Document { inner: tree }
    }
}
