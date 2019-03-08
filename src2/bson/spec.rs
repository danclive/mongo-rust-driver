
//! Constants derived from the [BSON Specification Version 1.1](http://bsonspec.org/spec.html).

// ElementType
pub const DOUBLE: u8                     = 0x01;
pub const UTF8_STRING: u8                = 0x02;
pub const DOCUMENT: u8                   = 0x03;
pub const ARRAY: u8                      = 0x04;
pub const BINARY: u8                     = 0x05;
pub const UNDEFINED: u8                  = 0x06;
pub const OBJECT_ID: u8                  = 0x07;
pub const BOOLEAN: u8                    = 0x08;
pub const UTC_DATETIME: u8               = 0x09;
pub const NULL_VALUE: u8                 = 0x0A;
pub const REGULAR_EXPRESSION: u8         = 0x0B;
pub const DBPOINTER: u8                  = 0x0C;
pub const JAVASCRIPT_CODE: u8            = 0x0D;
pub const SYMBOL: u8                     = 0x0E;
pub const JAVASCRIPT_CODE_WITH_SCOPE: u8 = 0x0F;
pub const INT_32BIT: u8                  = 0x10;
pub const TIMESTAMP: u8                  = 0x11;
pub const INT_64BIT: u8                  = 0x12;
pub const MINKEY: u8                     = 0xFF;
pub const MAXKEY: u8                     = 0x7F;

// BinarySubtype
pub const GENERIC: u8                    = 0x00;
pub const FUNCTION: u8                   = 0x01;
pub const BINARY_OLD: u8                 = 0x02;
pub const UUID_OLD: u8                   = 0x03;
pub const UUID: u8                       = 0x04;
pub const MD5: u8                        = 0x05;
// pub const USER_DEFINED: u8               = 0x80;

#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum ElementType {
    Double                  = DOUBLE,
    Utf8String              = UTF8_STRING,
    Document                = DOCUMENT,
    Array                   = ARRAY,
    Binary                  = BINARY,
    Undefiend               = UNDEFINED, // Deprecated
    ObjectId                = OBJECT_ID,
    Boolean                 = BOOLEAN,
    UTCDatetime             = UTC_DATETIME,
    NullValue               = NULL_VALUE,
    RegularExpression       = REGULAR_EXPRESSION,
    DBPointer               = DBPOINTER, // Deprecated
    JavaScriptCode          = JAVASCRIPT_CODE,
    Symbol                  = SYMBOL, // Deprecated
    JavaScriptCodeWithScope = JAVASCRIPT_CODE_WITH_SCOPE,
    Int32                   = INT_32BIT,
    TimeStamp               = TIMESTAMP,
    Int64                   = INT_64BIT,
    MinKey                  = MINKEY,
    MaxKey                  = MAXKEY,
}

impl ElementType {
    pub fn from(tag: u8) -> Option<ElementType> {
        Some(match tag {
            DOUBLE                     => ElementType::Double,
            UTF8_STRING                => ElementType::Utf8String,
            DOCUMENT                   => ElementType::Document,
            ARRAY                      => ElementType::Array,
            BINARY                     => ElementType::Binary,
            UNDEFINED                  => ElementType::Undefiend,
            OBJECT_ID                  => ElementType::ObjectId,
            BOOLEAN                    => ElementType::Boolean,
            UTC_DATETIME               => ElementType::UTCDatetime,
            NULL_VALUE                 => ElementType::NullValue,
            REGULAR_EXPRESSION         => ElementType::RegularExpression,
            DBPOINTER                  => ElementType::DBPointer,
            JAVASCRIPT_CODE            => ElementType::JavaScriptCode,
            SYMBOL                     => ElementType::Symbol,
            JAVASCRIPT_CODE_WITH_SCOPE => ElementType::JavaScriptCodeWithScope,
            INT_32BIT                  => ElementType::Int32,
            TIMESTAMP                  => ElementType::TimeStamp,
            INT_64BIT                  => ElementType::Int64,
            MINKEY                     => ElementType::MinKey,
            MAXKEY                     => ElementType::MaxKey,
            _                          => return None,
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BinarySubtype {
    Generic,
    Function,
    BinaryOld,
    UuidOld,
    Uuid,
    Md5,
    UserDefined(u8),
}

impl From<BinarySubtype> for u8 {
    fn from(t: BinarySubtype) -> u8 {
        match t {
            BinarySubtype::Generic => GENERIC,
            BinarySubtype::Function => FUNCTION,
            BinarySubtype::BinaryOld => BINARY_OLD,
            BinarySubtype::UuidOld => UUID_OLD,
            BinarySubtype::Uuid => UUID,
            BinarySubtype::Md5 => MD5,
            BinarySubtype::UserDefined(x) => x,
        }
    }
}

impl From<u8> for BinarySubtype {
    #[inline]
    fn from(t: u8) -> BinarySubtype {
        match t {
            GENERIC => BinarySubtype::Generic,
            FUNCTION => BinarySubtype::Function,
            BINARY_OLD => BinarySubtype::BinaryOld,
            UUID_OLD => BinarySubtype::UuidOld,
            UUID => BinarySubtype::Uuid,
            MD5 => BinarySubtype::Md5,
            _ => BinarySubtype::UserDefined(t),
        }
    }
}
