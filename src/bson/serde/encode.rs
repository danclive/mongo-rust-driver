use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeTuple, SerializeTupleStruct,
                 SerializeTupleVariant, SerializeMap, SerializeStruct, SerializeStructVariant};

use object_id::ObjectId;
use bson::bson::Bson;
use bson::bson::Array;
use bson::bson::UTCDateTime;
use bson::doc::Document;
use bson::encode::EncodeError;
use bson::encode::EncodeResult;
use bson::encode::to_bson;
use bson::spec::BinarySubtype;

impl Serialize for ObjectId {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_key("$oid")?;
        map.serialize_value(&self.to_string())?;
        map.end()
    }
}

impl Serialize for Document {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map.serialize_key(k)?;
            map.serialize_value(v)?;
        }
        map.end()
    }
}

impl Serialize for Bson {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            Bson::Double(v) => serializer.serialize_f64(v),
            Bson::String(ref v) => serializer.serialize_str(v),
            Bson::Array(ref v) => v.serialize(serializer),
            Bson::Document(ref v) => v.serialize(serializer),
            Bson::Boolean(v) => serializer.serialize_bool(v),
            Bson::Null => serializer.serialize_unit(),
            Bson::Int32(v) => serializer.serialize_i32(v),
            Bson::Int64(v) => serializer.serialize_i64(v),
            _ => {
                let doc = self.to_extended_document();
                doc.serialize(serializer)
            }
        }
    }
}

/// Serde Encoder
pub struct Encoder;

impl Encoder {
    /// Construct a new `Serializer`.
    pub fn new() -> Encoder {
        Encoder
    }
}

impl Serializer for Encoder {
    type Ok = Bson;
    type Error = EncodeError;

    type SerializeSeq = ArraySerializer;
    type SerializeTuple = TupleSerializer;
    type SerializeTupleStruct = TupleStructSerializer;
    type SerializeTupleVariant = TupleVariantSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = StructSerializer;
    type SerializeStructVariant = StructVariantSerializer;

    #[inline]
    fn serialize_bool(self, value: bool) -> EncodeResult<Bson> {
        Ok(Bson::Boolean(value))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> EncodeResult<Bson> {
        self.serialize_i32(value as i32)
    }

    #[inline]
    fn serialize_u8(self, _value: u8) -> EncodeResult<Bson> {
        Err(EncodeError::UnsupportedUnsignedType)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> EncodeResult<Bson> {
        self.serialize_i32(value as i32)
    }

    #[inline]
    fn serialize_u16(self, _value: u16) -> EncodeResult<Bson> {
        Err(EncodeError::UnsupportedUnsignedType)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> EncodeResult<Bson> {
        Ok(Bson::Int32(value))
    }

    #[inline]
    fn serialize_u32(self, _value: u32) -> EncodeResult<Bson> {
        Err(EncodeError::UnsupportedUnsignedType)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> EncodeResult<Bson> {
        Ok(Bson::Int64(value))
    }

    #[inline]
    fn serialize_u64(self, _value: u64) -> EncodeResult<Bson> {
        Err(EncodeError::UnsupportedUnsignedType)
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> EncodeResult<Bson> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> EncodeResult<Bson> {
        Ok(Bson::Double(value))
    }

    #[inline]
    fn serialize_char(self, value: char) -> EncodeResult<Bson> {
        let mut s = String::new();
        s.push(value);
        self.serialize_str(&s)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> EncodeResult<Bson> {
        Ok(Bson::String(value.to_string()))
    }

    fn serialize_bytes(self, value: &[u8]) -> EncodeResult<Bson> {
        Ok(Bson::Binary(BinarySubtype::Generic, value.into()))
    }

    #[inline]
    fn serialize_none(self) -> EncodeResult<Bson> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<V: ?Sized>(self, value: &V) -> EncodeResult<Bson>
        where V: Serialize
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> EncodeResult<Bson> {
        Ok(Bson::Null)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> EncodeResult<Bson> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str
    ) -> EncodeResult<Bson> {
        Ok(Bson::String(variant.to_string()))
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T
    ) -> EncodeResult<Bson>
        where T: Serialize
    {
        let mut ser = TupleStructSerializer { inner: Array::new() };
        ser.serialize_field(value)?;
        ser.end()
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T
    ) -> EncodeResult<Bson>
        where T: Serialize
    {
        let mut ser = TupleVariantSerializer {
            inner: Array::new(),
            name: variant,
        };
        ser.serialize_field(value)?;
        ser.end()
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> EncodeResult<Self::SerializeSeq> {
        Ok(ArraySerializer { inner: Array::with_capacity(len.unwrap_or(0)) })
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> EncodeResult<Self::SerializeTuple> {
        Ok(TupleSerializer { inner: Array::with_capacity(len) })
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize
    ) -> EncodeResult<Self::SerializeTupleStruct> {
        Ok(TupleStructSerializer { inner: Array::with_capacity(len) })
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize
    ) -> EncodeResult<Self::SerializeTupleVariant> {
        Ok(TupleVariantSerializer {
            inner: Array::with_capacity(len),
            name: variant,
        })
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> EncodeResult<Self::SerializeMap> {
        Ok(MapSerializer {
            inner: Document::new(),
            next_key: None,
        })
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize
    ) -> EncodeResult<Self::SerializeStruct> {
        Ok(StructSerializer { inner: Document::new() })
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize
    ) -> EncodeResult<Self::SerializeStructVariant> {
        Ok(StructVariantSerializer {
            name: variant,
            inner: Document::new(),
        })
    }
}

#[doc(hidden)]
pub struct ArraySerializer {
    inner: Array
}

impl SerializeSeq for ArraySerializer {
    type Ok = Bson;
    type Error = EncodeError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        self.inner.push(to_bson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Bson> {
        Ok(Bson::Array(self.inner))
    }
}

#[doc(hidden)]
pub struct TupleSerializer {
    inner: Array
}

impl SerializeTuple for TupleSerializer {
    type Ok = Bson;
    type Error = EncodeError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        self.inner.push(to_bson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Bson> {
        Ok(Bson::Array(self.inner))
    }
}

#[doc(hidden)]
pub struct TupleStructSerializer {
    inner: Array
}

impl SerializeTupleStruct for TupleStructSerializer {
    type Ok = Bson;
    type Error = EncodeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        self.inner.push(to_bson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Bson> {
        Ok(Bson::Array(self.inner))
    }
}

#[doc(hidden)]
pub struct TupleVariantSerializer {
    inner: Array,
    name: &'static str
}

impl SerializeTupleVariant for TupleVariantSerializer {
    type Ok = Bson;
    type Error = EncodeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        self.inner.push(to_bson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Bson> {
        let mut tuple_variant = Document::new();
        if self.inner.len() == 1 {
            tuple_variant.insert(self.name, self.inner.into_iter().next().unwrap());
        } else {
            tuple_variant.insert(self.name, Bson::Array(self.inner));
        }

        Ok(Bson::Document(tuple_variant))
    }
}

#[doc(hidden)]
pub struct MapSerializer {
    inner: Document,
    next_key: Option<String>
}

impl SerializeMap for MapSerializer {
    type Ok = Bson;
    type Error = EncodeError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> EncodeResult<()> {
        self.next_key = match to_bson(&key)? {
            Bson::String(s) => Some(s),
            other => return Err(EncodeError::InvalidMapKeyType(other)),
        };
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<()> {
        let key = self.next_key.take().unwrap_or_else(|| "".to_string());
        self.inner.insert(key, to_bson(&value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Bson> {
        Ok(Bson::from_extended_document(self.inner))
    }
}

#[doc(hidden)]
pub struct StructSerializer {
    inner: Document
}

impl SerializeStruct for StructSerializer {
    type Ok = Bson;
    type Error = EncodeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T
    ) -> EncodeResult<()> {
        self.inner.insert(key, to_bson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Bson> {
        Ok(Bson::from_extended_document(self.inner))
    }
}

#[doc(hidden)]
pub struct StructVariantSerializer {
    inner: Document,
    name: &'static str
}

impl SerializeStructVariant for StructVariantSerializer {
    type Ok = Bson;
    type Error = EncodeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T
    ) -> EncodeResult<()> {
        self.inner.insert(key, to_bson(value)?);
        Ok(())
    }

    fn end(self) -> EncodeResult<Bson> {
        let var = Bson::from_extended_document(self.inner);

        let mut struct_variant = Document::new();
        struct_variant.insert(self.name, var);

        Ok(Bson::Document(struct_variant))
    }
}

impl Serialize for UTCDateTime {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        // Cloning a `DateTime` is extremely cheap
        let doc = Bson::UTCDatetime(self.0);
        doc.serialize(serializer)
    }
}

