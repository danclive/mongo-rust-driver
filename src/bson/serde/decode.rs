use std::vec;
use std::fmt;
use std::result;
use std::marker::PhantomData;

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess, SeqAccess, VariantAccess,
                DeserializeSeed, EnumAccess};
use serde::de::{Error, Expected, Unexpected};
use linked_hash_map::LinkedHashMap;

use object_id::ObjectId;
use bson::bson::Bson;
use bson::bson::UTCDateTime;
use bson::doc::Document;
use bson::doc::DocumentIntoIterator;
use bson::decode::DecodeError;
use bson::decode::DecodeResult;

impl de::Error for DecodeError {
    fn custom<T: fmt::Display>(msg: T) -> DecodeError {
        DecodeError::Unknown(msg.to_string())
    }

    fn invalid_type(_unexp: Unexpected, exp: &Expected) -> DecodeError {
        DecodeError::InvalidType(exp.to_string())
    }

    fn invalid_value(_unexp: Unexpected, exp: &Expected) -> DecodeError {
        DecodeError::InvalidValue(exp.to_string())
    }

    fn invalid_length(len: usize, exp: &Expected) -> DecodeError {
        DecodeError::InvalidLength(len, exp.to_string())
    }

    fn unknown_variant(variant: &str, _expected: &'static [&'static str]) -> DecodeError {
        DecodeError::UnknownVariant(variant.to_string())
    }

    fn unknown_field(field: &str, _expected: &'static [&'static str]) -> DecodeError {
        DecodeError::UnknownField(field.to_string())
    }

    fn missing_field(field: &'static str) -> DecodeError {
        DecodeError::ExpectedField(field)
    }

    fn duplicate_field(field: &'static str) -> DecodeError {
        DecodeError::DuplicatedField(field)
    }
}

pub struct BsonVisitor;

impl<'de> Deserialize<'de> for ObjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer
            .deserialize_map(BsonVisitor)
            .and_then(|bson|
                if let Bson::ObjectId(oid) = bson {
                    Ok(oid)
                } else {
                    let err = format!("expected objectId extended document, found {}", bson);
                    Err(de::Error::invalid_type(Unexpected::Map, &&*err))
            })
    }
}

impl<'de> Deserialize<'de> for Document {
    /// Deserialize this value given this `Deserializer`.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer
            .deserialize_map(BsonVisitor)
            .and_then(|bson|
                if let Bson::Document(doc) = bson {
                    Ok(doc)
                } else {
                    let err = format!("expected document, found extended JSON data type: {}", bson);
                    Err(de::Error::invalid_type(Unexpected::Map, &&*err))
            })
    }
}

impl<'de> Deserialize<'de> for Bson {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Bson, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_any(BsonVisitor)
    }
}

impl<'de> Visitor<'de> for BsonVisitor {
    type Value = Bson;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expecting a Bson")
    }

    #[inline]
    fn visit_bool<E>(self, value: bool) -> Result<Bson, E>
        where E: Error
    {
        Ok(Bson::Boolean(value))
    }

    #[inline]
    fn visit_i8<E>(self, value: i8) -> Result<Bson, E>
        where E: Error
    {
        Ok(Bson::Int32(value as i32))
    }

    #[inline]
    fn visit_u8<E>(self, value: u8) -> Result<Bson, E>
        where E: Error
    {
        Err(Error::invalid_type(Unexpected::Unsigned(value as u64), &"a signed integer"))
    }

    #[inline]
    fn visit_i16<E>(self, value: i16) -> Result<Bson, E>
        where E: Error
    {
        Ok(Bson::Int32(value as i32))
    }

    #[inline]
    fn visit_u16<E>(self, value: u16) -> Result<Bson, E>
        where E: Error
    {
        Err(Error::invalid_type(Unexpected::Unsigned(value as u64), &"a signed integer"))
    }

    #[inline]
    fn visit_i32<E>(self, value: i32) -> Result<Bson, E>
        where E: Error
    {
        Ok(Bson::Int32(value))
    }

    #[inline]
    fn visit_u32<E>(self, value: u32) -> Result<Bson, E>
        where E: Error
    {
        Err(Error::invalid_type(Unexpected::Unsigned(value as u64), &"a signed integer"))
    }

    #[inline]
    fn visit_i64<E>(self, value: i64) -> Result<Bson, E>
        where E: Error
    {
        Ok(Bson::Int64(value))
    }

    #[inline]
    fn visit_u64<E>(self, value: u64) -> Result<Bson, E>
        where E: Error
    {
        Err(Error::invalid_type(Unexpected::Unsigned(value), &"a signed integer"))
    }

    #[inline]
    fn visit_f64<E>(self, value: f64) -> Result<Bson, E> {
        Ok(Bson::Double(value))
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Bson, E>
        where E: de::Error
    {
        self.visit_string(value.to_string())
    }

    #[inline]
    fn visit_string<E>(self, value: String) -> Result<Bson, E> {
        Ok(Bson::String(value))
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Bson, E> {
        Ok(Bson::Null)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Bson, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_any(self)
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Bson, E> {
        Ok(Bson::Null)
    }

    #[inline]
    fn visit_seq<V>(self, mut visitor: V) -> Result<Bson, V::Error>
        where V: SeqAccess<'de>
    {
        let mut values = Vec::new();

        while let Some(elem) = visitor.next_element()? {
            values.push(elem);
        }

        Ok(Bson::Array(values))
    }

    #[inline]
    fn visit_map<V>(self, visitor: V) -> Result<Bson, V::Error>
        where V: MapAccess<'de>
    {
        let values = DocumentVisitor::new().visit_map(visitor)?;
        Ok(Bson::from_extended_document(values.into()))
    }
}

pub struct DocumentVisitor {
    marker: PhantomData<Document>
}

impl DocumentVisitor {
    pub fn new() -> DocumentVisitor {
        DocumentVisitor { marker: PhantomData }
    }
}

impl<'de> Visitor<'de> for DocumentVisitor {
    type Value = Document;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expecting ordered document")
    }

    #[inline]
    fn visit_unit<E>(self) -> result::Result<Document, E>
        where E: de::Error
    {
        Ok(Document::new())
    }

    #[inline]
    fn visit_map<V>(self, mut visitor: V) -> result::Result<Document, V::Error>
        where V: MapAccess<'de>
    {
        let mut inner = match visitor.size_hint() {
            Some(size) => LinkedHashMap::with_capacity(size),
            None => LinkedHashMap::new(),
        };

        while let Some((key, value)) = visitor.next_entry()? {
            inner.insert(key, value);
        }

        Ok(inner.into())
    }
}

/// Serde Decoder
pub struct Decoder {
    value: Option<Bson>,
}

impl Decoder {
    pub fn new(value: Bson) -> Decoder {
        Decoder { value: Some(value) }
    }
}

macro_rules! forward_to_deserialize {
    ($(
        $name:ident ( $( $arg:ident : $ty:ty ),* );
    )*) => {
        $(
            forward_to_deserialize!{
                func: $name ( $( $arg: $ty ),* );
            }
        )*
    };

    (func: deserialize_enum ( $( $arg:ident : $ty:ty ),* );) => {
        fn deserialize_enum<V>(
            self,
            $(_: $ty,)*
            _visitor: V,
        ) -> ::std::result::Result<V::Value, Self::Error>
            where V: ::serde::de::Visitor<'de>
        {
            Err(::serde::de::Error::custom("unexpected Enum"))
        }
    };

    (func: $name:ident ( $( $arg:ident : $ty:ty ),* );) => {
        #[inline]
        fn $name<V>(
            self,
            $(_: $ty,)*
            visitor: V,
        ) -> ::std::result::Result<V::Value, Self::Error>
            where V: ::serde::de::Visitor<'de>
        {
            self.deserialize_any(visitor)
        }
    };
}

impl<'de> Deserializer<'de> for Decoder {
    type Error = DecodeError;

    #[inline]
    fn deserialize_any<V>(mut self, visitor: V) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        let value = match self.value.take() {
            Some(value) => value,
            None => return Err(DecodeError::EndOfStream),
        };

        match value {
            Bson::Double(v) => visitor.visit_f64(v),
            Bson::String(v) => visitor.visit_string(v),
            Bson::Array(v) => {
                let len = v.len();
                visitor.visit_seq(
                    SeqDecoder {
                        iter: v.into_iter(),
                        len: len,
                    }
                )
            }
            Bson::Document(v) => {
                let len = v.len();
                visitor.visit_map(
                    MapDecoder {
                        iter: v.into_iter(),
                        value: None,
                        len: len,
                    }
                )
            }
            Bson::Boolean(v) => visitor.visit_bool(v),
            Bson::Null => visitor.visit_unit(),
            Bson::Int32(v) => visitor.visit_i32(v),
            Bson::Int64(v) => visitor.visit_i64(v),
            Bson::Binary(_, v) => visitor.visit_bytes(&v),
            _ => {
                let doc = value.to_extended_document();
                let len = doc.len();
                visitor.visit_map(
                    MapDecoder {
                        iter: doc.into_iter(),
                        value: None,
                        len: len,
                    }
                )
            }
        }
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        match self.value {
            Some(Bson::Null) => visitor.visit_none(),
            Some(_) => visitor.visit_some(self),
            None => Err(DecodeError::EndOfStream),
        }
    }

    #[inline]
    fn deserialize_enum<V>(
        mut self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V
    ) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        let value = match self.value.take() {
            Some(Bson::Document(value)) => value,
            Some(Bson::String(variant)) => {
                return visitor.visit_enum(
                    EnumDecoder {
                        val: Bson::String(variant),
                        decoder: VariantDecoder { val: None },
                    }
                );
            }
            Some(_) => {
                return Err(DecodeError::InvalidType("expected an enum".to_string()));
            }
            None => {
                return Err(DecodeError::EndOfStream);
            }
        };

        let mut iter = value.into_iter();

        let (variant, value) = match iter.next() {
            Some(v) => v,
            None => return Err(DecodeError::SyntaxError("expected a variant name".to_string())),
        };

        // enums are encoded in json as maps with a single key:value pair
        match iter.next() {
            Some(_) => {
                Err(DecodeError::InvalidType("expected a single key:value pair".to_string()))
            }
            None => {
                visitor.visit_enum(
                    EnumDecoder {
                        val: Bson::String(variant),
                        decoder: VariantDecoder { val: Some(value) },
                    }
                )
            }
        }
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V
    ) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_newtype_struct(self)
    }

    forward_to_deserialize!{
        deserialize_bool();
        deserialize_u8();
        deserialize_u16();
        deserialize_u32();
        deserialize_u64();
        deserialize_i8();
        deserialize_i16();
        deserialize_i32();
        deserialize_i64();
        deserialize_f32();
        deserialize_f64();
        deserialize_char();
        deserialize_str();
        deserialize_string();
        deserialize_unit();
        deserialize_seq();
        deserialize_bytes();
        deserialize_map();
        deserialize_unit_struct(name: &'static str);
        deserialize_tuple_struct(name: &'static str, len: usize);
        deserialize_struct(name: &'static str, fields: &'static [&'static str]);
        deserialize_tuple(len: usize);
        deserialize_identifier();
        deserialize_ignored_any();
        deserialize_byte_buf();
    }
}

struct EnumDecoder {
    val: Bson,
    decoder: VariantDecoder,
}

impl<'de> EnumAccess<'de> for EnumDecoder {
    type Error = DecodeError;
    type Variant = VariantDecoder;
    fn variant_seed<V>(self, seed: V) -> DecodeResult<(V::Value, Self::Variant)>
        where V: DeserializeSeed<'de>
    {
        let dec = Decoder::new(self.val);
        let value = seed.deserialize(dec)?;
        Ok((value, self.decoder))
    }
}

struct VariantDecoder {
    val: Option<Bson>,
}

impl<'de> VariantAccess<'de> for VariantDecoder {
    type Error = DecodeError;

    fn unit_variant(mut self) -> DecodeResult<()> {
        match self.val.take() {
            None => Ok(()),
            Some(val) => {
                Bson::deserialize(Decoder::new(val)).map(|_| ())
            }
        }
    }

    fn newtype_variant_seed<T>(mut self, seed: T) -> DecodeResult<T::Value>
        where T: DeserializeSeed<'de>
    {
        let dec = Decoder::new(self.val.take().ok_or(DecodeError::EndOfStream)?);
        seed.deserialize(dec)
    }

    fn tuple_variant<V>(mut self, _len: usize, visitor: V) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        if let Bson::Array(fields) = self.val.take().ok_or(DecodeError::EndOfStream)? {

            let de = SeqDecoder {
                len: fields.len(),
                iter: fields.into_iter(),
            };
            de.deserialize_any(visitor)
        } else {
            return Err(DecodeError::InvalidType("expected a tuple".to_string()));
        }
    }

    fn struct_variant<V>(
        mut self,
        _fields: &'static [&'static str],
        visitor: V
    ) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        if let Bson::Document(fields) = self.val.take().ok_or(DecodeError::EndOfStream)? {
            let de = MapDecoder {
                len: fields.len(),
                iter: fields.into_iter(),
                value: None,
            };
            de.deserialize_any(visitor)
        } else {
            return Err(DecodeError::InvalidType("expected a struct".to_string()));
        }
    }
}

struct SeqDecoder {
    iter: vec::IntoIter<Bson>,
    len: usize,
}

impl<'de> Deserializer<'de> for SeqDecoder {
    type Error = DecodeError;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        if self.len == 0 {
            visitor.visit_unit()
        } else {
            visitor.visit_seq(self)
        }
    }

    forward_to_deserialize!{
        deserialize_bool();
        deserialize_u8();
        deserialize_u16();
        deserialize_u32();
        deserialize_u64();
        deserialize_i8();
        deserialize_i16();
        deserialize_i32();
        deserialize_i64();
        deserialize_f32();
        deserialize_f64();
        deserialize_char();
        deserialize_str();
        deserialize_string();
        deserialize_unit();
        deserialize_option();
        deserialize_seq();
        deserialize_bytes();
        deserialize_map();
        deserialize_unit_struct(name: &'static str);
        deserialize_newtype_struct(name: &'static str);
        deserialize_tuple_struct(name: &'static str, len: usize);
        deserialize_struct(name: &'static str, fields: &'static [&'static str]);
        deserialize_tuple(len: usize);
        deserialize_enum(name: &'static str, variants: &'static [&'static str]);
        deserialize_identifier();
        deserialize_ignored_any();
        deserialize_byte_buf();
    }
}

impl<'de> SeqAccess<'de> for SeqDecoder {
    type Error = DecodeError;

    fn next_element_seed<T>(&mut self, seed: T) -> DecodeResult<Option<T::Value>>
        where T: DeserializeSeed<'de>
    {
        match self.iter.next() {
            None => Ok(None),
            Some(value) => {
                self.len -= 1;
                let de = Decoder::new(value);
                match seed.deserialize(de) {
                    Ok(value) => Ok(Some(value)),
                    Err(err) => Err(err),
                }
            }
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct MapDecoder {
    iter: DocumentIntoIterator,
    value: Option<Bson>,
    len: usize,
}

impl<'de> MapAccess<'de> for MapDecoder {
    type Error = DecodeError;

    fn next_key_seed<K>(&mut self, seed: K) -> DecodeResult<Option<K::Value>>
        where K: DeserializeSeed<'de>
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.len -= 1;
                self.value = Some(value);

                let de = Decoder::new(Bson::String(key));
                match seed.deserialize(de) {
                    Ok(val) => Ok(Some(val)),
                    Err(DecodeError::UnknownField(_)) => Ok(None),
                    Err(e) => Err(e),
                }
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> DecodeResult<V::Value>
        where V: DeserializeSeed<'de>
    {
        let value = self.value.take().ok_or(DecodeError::EndOfStream)?;
        let de = Decoder::new(value);
        seed.deserialize(de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'de> Deserializer<'de> for MapDecoder {
    type Error = DecodeError;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> DecodeResult<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize!{
        deserialize_bool();
        deserialize_u8();
        deserialize_u16();
        deserialize_u32();
        deserialize_u64();
        deserialize_i8();
        deserialize_i16();
        deserialize_i32();
        deserialize_i64();
        deserialize_f32();
        deserialize_f64();
        deserialize_char();
        deserialize_str();
        deserialize_string();
        deserialize_unit();
        deserialize_option();
        deserialize_seq();
        deserialize_bytes();
        deserialize_map();
        deserialize_unit_struct(name: &'static str);
        deserialize_newtype_struct(name: &'static str);
        deserialize_tuple_struct(name: &'static str, len: usize);
        deserialize_struct(name: &'static str, fields: &'static [&'static str]);
        deserialize_tuple(len: usize);
        deserialize_enum(name: &'static str, variants: &'static [&'static str]);
        deserialize_identifier();
        deserialize_ignored_any();
        deserialize_byte_buf();
    }
}

impl<'de> Deserialize<'de> for UTCDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;

        match Bson::deserialize(deserializer)? {
            Bson::UTCDatetime(dt) => Ok(UTCDateTime(dt)),
            _ => Err(D::Error::custom("expecting UtcDateTime")),
        }
    }
}
