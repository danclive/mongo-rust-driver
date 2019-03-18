use std::error;
use std::fmt;
use std::result;

use bsonrs::encode::EncodeError;
use bsonrs::decode::DecodeError;

use crate::core::error::MongocError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MongocError(MongocError),
    EncodeError(EncodeError),
    DecodeError(DecodeError)
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::MongocError(ref inner) => inner.fmt(fmt),
            Error::EncodeError(ref inner) => inner.fmt(fmt),
            Error::DecodeError(ref inner) => inner.fmt(fmt)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MongocError(ref inner) => inner.description(),
            Error::EncodeError(ref inner) => inner.description(),
            Error::DecodeError(ref inner) => inner.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::MongocError(ref inner) => Some(inner),
            Error::EncodeError(ref inner) => Some(inner),
            Error::DecodeError(ref inner) => Some(inner)

        }
    }
}

impl From<MongocError> for Error {
    fn from(err: MongocError) -> Error {
        Error::MongocError(err)
    }
}

impl From<EncodeError> for Error {
    fn from(err: EncodeError) -> Error {
        Error::EncodeError(err)
    }
}

impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Error {
        Error::DecodeError(err)
    }
}
