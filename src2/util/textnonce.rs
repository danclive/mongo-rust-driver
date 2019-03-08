use std::io::Cursor;
use std::ops::Deref;
use std::fmt;
use std::result;
use std::error;

use data_encoding::{BASE64, BASE64URL};
use chrono;
use rand::{self, RngCore};
use byteorder::{LittleEndian, WriteBytesExt};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Short,
    Divisable,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Short => fmt.write_str("Length must be >= 16"),
            Error::Divisable => fmt.write_str("Length must be divisable by 4")
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Short => "Length must be >= 16.",
            Error::Divisable => "Length must be divisable by 4."
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct TextNonce(pub String);

impl TextNonce {

    pub fn new() -> TextNonce {
        TextNonce::sized(32).unwrap()
    }

    pub fn sized(length: usize) -> Result<TextNonce> {
        Ok(TextNonce(BASE64.encode(&TextNonce::sized_configured(length)?)))
    }

    pub fn sized_urlsafe(length: usize) -> Result<TextNonce> {
        Ok(TextNonce(BASE64URL.encode(&TextNonce::sized_configured(length)?)))
    }

    pub fn sized_configured(length: usize) -> Result<Vec<u8>> {
        if length < 16 {
            return Err(Error::Short);
        }

        if length % 4 != 0 {
            return Err(Error::Divisable);
        }

        let bytelength: usize = (length / 4) * 3;

        let mut raw: Vec<u8> = Vec::with_capacity(bytelength);
        unsafe { raw.set_len(bytelength); }

        {
            let now = chrono::Utc::now();
            let secs: i64 = now.timestamp();
            let nsece: u32 = now.timestamp_subsec_nanos();

            let mut cursor = Cursor::new(&mut *raw);
            cursor.write_u32::<LittleEndian>(nsece).unwrap();
            cursor.write_i64::<LittleEndian>(secs).unwrap();
        }

        match rand::OsRng::new() {
            Ok(mut g) => g.fill_bytes(&mut raw[12..bytelength]),
            Err(_) => rand::thread_rng().fill_bytes(&mut raw[12..bytelength])
        }

        Ok(raw)
    }

    pub fn into_string(self) -> String {
        let TextNonce(s) = self;
        s
    }
}

impl fmt::Display for TextNonce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Deref for TextNonce {
    type Target = str;
    fn deref(&self) -> &str {
        &*self.0
    }
}
