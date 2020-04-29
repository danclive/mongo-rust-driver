//! ObjectId
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::ffi::CStr;
use std::{io, fmt, result, error};

use byteorder::{ByteOrder, BigEndian, LittleEndian};
use libc;
use rand::{self, Rng};
use rand::rngs::OsRng;

use crate::util::md5;
use crate::util::hex::{ToHex, FromHex, FromHexError};

static mut MACHINE_BYTES: Option<[u8; 3]> = None;
static OID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ObjectId {
    bytes: [u8; 12]
}

pub type Result<T> = result::Result<T, Error>;

impl ObjectId {
    /// Generate a new ObjectId
    ///
    /// # Examples
    ///
    /// ```
    /// use bsonrs::object_id::ObjectId;
    ///
    /// let id = ObjectId::new();
    ///
    /// println!("{:?}", id);
    /// ```
    pub fn new() -> ObjectId {
        let timestamp = timestamp();
        let machine_id = machine_id();
        let process_id = process_id();
        let counter = gen_count();

        let mut buf: [u8; 12] = [0; 12];

        buf[0] = timestamp[0];
        buf[1] = timestamp[1];
        buf[2] = timestamp[2];
        buf[3] = timestamp[3];

        buf[4] = machine_id[0];
        buf[5] = machine_id[1];
        buf[6] = machine_id[2];

        buf[7] = process_id[0];
        buf[8] = process_id[1];

        buf[9] = counter[0];
        buf[10] = counter[1];
        buf[11] = counter[2];

        ObjectId {
            bytes: buf
        }
    }

    /// Generate an ObjectId with bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use bsonrs::object_id::ObjectId;
    ///
    /// let id = ObjectId::with_bytes([90, 167, 114, 110, 99, 55, 51, 218, 65, 162, 186, 71]);
    ///
    /// assert_eq!(format!("{}", id), "5aa7726e633733da41a2ba47")
    /// ```
    pub fn with_bytes(bytes: [u8; 12]) -> Self {
        ObjectId { bytes }
    }

    /// Generate an ObjectId with string.
    /// Provided string must be a 12-byte hexadecimal string
    ///
    /// # Examples
    ///
    /// ```
    /// use bsonrs::object_id::ObjectId;
    ///
    /// let id = ObjectId::with_string("5932a005b4b4b4ac168cd9e4").unwrap();
    ///
    /// assert_eq!(format!("{}", id), "5932a005b4b4b4ac168cd9e4")
    /// ```
    pub fn with_string(str: &str) -> Result<ObjectId> {
        let bytes: Vec<u8> = FromHex::from_hex(str.as_bytes())?;
        if bytes.len() != 12 {
            return Err(Error::ArgumentError("Provided string must be a 12-byte hexadecimal string.".to_string()))
        }

        let mut buf = [0u8; 12];
        buf[..].copy_from_slice(&bytes);

        Ok(ObjectId {
            bytes: buf
        })
    }

    /// 12-byte binary representation of this ObjectId.
    pub fn bytes(&self) -> [u8; 12] {
        self.bytes
    }

    /// Timstamp of this ObjectId
    pub fn timestamp(&self) -> u32 {
        BigEndian::read_u32(&self.bytes)
    }

    /// Machine ID of this ObjectId
    pub fn machine_id(&self) -> u32 {
        let mut buf: [u8; 4] = [0; 4];
        buf[..3].clone_from_slice(&self.bytes[4..7]);
        LittleEndian::read_u32(&buf)
    }

    /// Process ID of this ObjectId
    pub fn process_id(&self) -> u16 {
        LittleEndian::read_u16(&self.bytes[7..9])
    }

    /// Convert this ObjectId to a 12-byte hexadecimal string.
    pub fn to_hex(&self) -> String {
        self.bytes.to_hex()
    }
}

impl Default for ObjectId {
    fn default() -> Self {
        ObjectId::new()
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("ObjectId({})", self.to_hex()))
    }
}

#[inline]
fn timestamp() -> [u8; 4] {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_secs() as u32;

    let mut buf: [u8; 4] = [0; 4];
    BigEndian::write_u32(&mut buf, time);
    buf
}

#[inline]
fn hosename() -> Option<String> {
    let mut buf = [0u8; 255];
    let ptr = buf.as_mut_ptr() as *mut libc::c_char;

    unsafe {
        if libc::gethostname(ptr, buf.len() as libc::size_t) != 0 {
            return None;
        }

        Some(CStr::from_ptr(ptr).to_string_lossy().to_string())
    }
}

#[inline]
fn machine_id() -> [u8; 3] {
    unsafe {
        if let Some(bytes) = MACHINE_BYTES.as_ref() {
            return *bytes;
        }
    }

    let hostname = hosename().expect("Can't get hostname!");

    let bytes = format!("{:x}", md5::compute(hostname.as_bytes()));
    let bytes = bytes.as_bytes();

    let mut buf = [0u8; 3];
    buf[0] = bytes[0];
    buf[1] = bytes[1];
    buf[2] = bytes[2];

    unsafe {
        MACHINE_BYTES = Some(buf);
    }

    buf
}

#[inline]
fn process_id() -> [u8; 2] {
    let pid = unsafe {
        libc::getpid() as u16
    };
    let mut buf: [u8; 2] = [0; 2];
    LittleEndian::write_u16(&mut buf, pid);
    buf
}

#[inline]
fn gen_count() -> [u8; 3] {

    const MAX_U24: usize = 0x00FF_FFFF;

    if OID_COUNTER.load(Ordering::SeqCst) == 0 {
        let mut rng = OsRng;
        let start = rng.gen_range(0, MAX_U24 + 1);
        OID_COUNTER.store(start, Ordering::SeqCst);
    }

    let count = OID_COUNTER.fetch_add(1, Ordering::SeqCst);

    let u = count % MAX_U24;

    let mut buf: [u8; 8] = [0; 8];
    BigEndian::write_u64(&mut buf, u as u64);

    [buf[5], buf[6], buf[7]]
}

#[derive(Debug)]
pub enum Error {
    ArgumentError(String),
    FromHexError(FromHexError),
    IoError(io::Error),
    RandError(rand::Error)
}

impl From<FromHexError> for Error {
    fn from(err: FromHexError) -> Error {
        Error::FromHexError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<rand::Error> for Error {
    fn from(err: rand::Error) -> Error {
        Error::RandError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ArgumentError(ref err) => err.fmt(fmt),
            Error::FromHexError(ref err) => err.fmt(fmt),
            Error::IoError(ref inner) => inner.fmt(fmt),
            Error::RandError(ref inner) => inner.fmt(fmt)
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::ArgumentError(_) => None,
            Error::FromHexError(ref err) => Some(err),
            Error::IoError(ref err) => Some(err),
            Error::RandError(ref err) => Some(err)
        }
    }
}

#[test]
fn test_display() {
    let id = ObjectId::with_string("53e37d08776f724e42000000").unwrap();

    assert_eq!(format!("{}", id), "53e37d08776f724e42000000")
}

#[test]
fn test_debug() {
    let id = ObjectId::with_string("53e37d08776f724e42000000").unwrap();

    assert_eq!(format!("{:?}", id), "ObjectId(53e37d08776f724e42000000)")
}
