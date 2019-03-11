use std::error;
use std::fmt;
use std::result;

use crate::core::error::MongocError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MongocError(MongocError)
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::MongocError(ref inner) => inner.fmt(fmt)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MongocError(ref inner) => inner.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::MongocError(ref inner) => Some(inner)
        }
    }
}

impl From<MongocError> for Error {
    fn from(err: MongocError) -> Error {
        Error::MongocError(err)
    }
}
