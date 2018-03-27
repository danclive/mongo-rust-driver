//! ObjectId
use std::ffi::CStr;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::fmt;
use std::error;
use std::result;
use std::io;

use chrono;
use byteorder::{ByteOrder, BigEndian, LittleEndian};
use libc;
use rand::{Rng, OsRng};

use util::hex::{ToHex, FromHex, FromHexError};
use util::md5;

extern "C" {
    fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
}

const TIMESTAMP_SIZE: usize = 4;
const MACHINE_ID_SIZE: usize = 3;
const PROCESS_ID_SIZE: usize = 2;
const COUNTER_SIZE: usize = 3;
const TIMESTAMP_OFFSET: usize = 0;
const MACHINE_ID_OFFSET: usize = TIMESTAMP_OFFSET + TIMESTAMP_SIZE;
const PROCESS_ID_OFFSET: usize = MACHINE_ID_OFFSET + MACHINE_ID_SIZE;
const COUNTER_OFFSET: usize = PROCESS_ID_OFFSET + PROCESS_ID_SIZE;
const MAX_U24: usize = 0xFFFFFF;

static OID_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
static mut MACHINE_BYTES: Option<[u8; 3]> = None;

#[derive(Debug)]
pub enum Error {
    ArgumentError(String),
    HostnameError,
    FromHexError(FromHexError),
    IoError(io::Error)
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

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ArgumentError(ref err) => err.fmt(fmt),
            Error::HostnameError => write!(fmt, "Failed to retrieve hostname for OID generation."),
            Error::FromHexError(ref err) => err.fmt(fmt),
            Error::IoError(ref inner) => inner.fmt(fmt),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ArgumentError(ref err) => &err,
            Error::HostnameError => "Failed to retrieve hostname for OID generation.",
            Error::FromHexError(ref err) => err.description(),
            Error::IoError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::ArgumentError(_) => None,
            Error::HostnameError => None,
            Error::FromHexError(ref err) => Some(err),
            Error::IoError(ref err) => Some(err)
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

/// A MongoDB ObjectId
///
/// A ObjectId is a 12-byte unique identifier consisting of:
/// - a 4-byte value representing the seconds since the Unix epoch,
/// - a 3-byte value machine identifier,
/// - a 2-byte process id, and
/// - a 3-byte counter, starting with a random value.
///
/// You can use `ObjectId::new()` generate a new unique identifer.
/// And the `ObjectId::with_bytes()` or `ObjectId::with_string()` from
/// with byte or string.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ObjectId {
    id: [u8; 12]
}

impl ObjectId {
    /// Generate a new ObjectId
    /// 
    /// # Examples
    ///
    /// ```
    /// use mongors::object_id::ObjectId;
    ///
    /// let id = ObjectId::new().unwrap();
    ///
    /// println!("{:?}", id);
    /// ```
    pub fn new() -> Result<ObjectId> {
        let timestamp = gen_timestamp();
        let machine_id = gen_machine_id()?;
        let process_id = gen_process_id();
        let counter = gen_count()?;
        let mut buf: [u8; 12] = [0; 12];

        for i in 0..TIMESTAMP_SIZE {
            buf[TIMESTAMP_OFFSET + i] = timestamp[i];
        }
        for i in 0..MACHINE_ID_SIZE {
            buf[MACHINE_ID_OFFSET + i] = machine_id[i];
        }
        for i in 0..PROCESS_ID_SIZE {
            buf[PROCESS_ID_OFFSET + i] = process_id[i];
        }
        for i in 0..COUNTER_SIZE {
            buf[COUNTER_OFFSET + i] = counter[i];
        }

        Ok(ObjectId::with_bytes(buf))
    }

    /// Generate an ObjectId with bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use mongors::object_id::ObjectId;
    ///
    /// let id = ObjectId::with_bytes([90, 167, 114, 110, 99, 55, 51, 218, 65, 162, 186, 71]);
    ///
    /// assert_eq!(format!("{}", id), "5aa7726e633733da41a2ba47")
    /// ```
    pub fn with_bytes(bytes: [u8; 12]) -> ObjectId {
        ObjectId { id: bytes }
    }

    /// Generate an ObjectId with string.
    /// Provided string must be a 12-byte hexadecimal string
    ///
    /// # Examples
    ///
    /// ```
    /// use mongors::object_id::ObjectId;
    ///
    /// let id = ObjectId::with_string("5932a005b4b4b4ac168cd9e4").unwrap();
    ///
    /// assert_eq!(format!("{}", id), "5932a005b4b4b4ac168cd9e4")
    /// ```
    pub fn with_string(s: &str) -> Result<ObjectId> {
        let bytes: Vec<u8> = FromHex::from_hex(s.as_bytes())?;
        if bytes.len() != 12 {
            Err(Error::ArgumentError("Provided string must be a 12-byte hexadecimal string.".to_string()))
        } else {
            let mut byte_array: [u8; 12] = [0; 12];
            for i in 0..12 {
                byte_array[i] = bytes[i];
            }
            Ok(ObjectId::with_bytes(byte_array))
        }
    }

    /// 12-byte binary representation of this ObjectId.
    pub fn bytes(&self) -> [u8; 12] {
        self.id
    }

    /// Timstamp of this ObjectId
    pub fn timestamp(&self) -> u32 {
        BigEndian::read_u32(&self.id)
    }

    /// Machine ID of this ObjectId
    pub fn machine_id(&self) -> u32 {
        let mut buf: [u8; 4] = [0; 4];
        for i in 0..MACHINE_ID_SIZE {
            buf[i] = self.id[MACHINE_ID_OFFSET + i];
        }
        LittleEndian::read_u32(&buf)
    }

    /// Process ID of this ObjectId
    pub fn process_id(&self) -> u16 {
        LittleEndian::read_u16(&self.id[PROCESS_ID_OFFSET..])
    }

    /// Convert this ObjectId to a 12-byte hexadecimal string.
    pub fn to_hex(&self) -> String {
        self.id.to_hex()
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
fn gen_timestamp() -> [u8; 4] {
    let time = chrono::Local::now().timestamp() as u32;

    let mut buf: [u8; 4] = [0; 4];
    BigEndian::write_u32(&mut buf, time);
    buf
}

#[inline]
fn gen_machine_id() -> Result<[u8; 3]> {
    
    unsafe {
        if let Some(bytes) = MACHINE_BYTES.as_ref() {
            return Ok(bytes.clone());
        }
    }

    let hostname = get_hosename();
    if hostname.is_none() {
        return Err(Error::HostnameError);
    }

    let bytes = format!("{:x}", md5::compute(hostname.unwrap().as_bytes()));

    let mut bytes = bytes.as_bytes().iter();

    let mut vec: [u8; 3] = [0; 3];

    for i in 0..MACHINE_ID_SIZE {
        match bytes.next() {
            Some(b) => vec[i] = *b,
            None => break
        }
    }

    unsafe {
        MACHINE_BYTES = Some(vec);
    }

    Ok(vec)

}

#[inline]
fn gen_process_id() -> [u8; 2] {
    let pid = unsafe {
        libc::getpid() as u16
    };
    let mut buf: [u8; 2] = [0; 2];
    LittleEndian::write_u16(&mut buf, pid);
    buf
}

#[inline]
fn gen_count() -> Result<[u8; 3]> {
    if OID_COUNTER.load(Ordering::SeqCst) == 0 {
        let mut rng = OsRng::new()?;
        let start = rng.gen_range(0, MAX_U24 + 1);
        OID_COUNTER.store(start, Ordering::SeqCst); 
    }

    let count = OID_COUNTER.fetch_add(1, Ordering::SeqCst);

    let u = count % MAX_U24;

    let mut buf: [u8; 8] = [0; 8];
    BigEndian::write_u64(&mut buf, u as u64);
    let buf_u24: [u8; 3] = [buf[5], buf[6], buf[7]];
    Ok(buf_u24)
}

#[inline]
fn get_hosename() -> Option<String> {

    let len = 255;
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_ptr() as *mut libc::c_char;

    unsafe {
        if gethostname(ptr, len as libc::size_t) != 0 {
            return None;
        }

        return Some(CStr::from_ptr(ptr).to_string_lossy().to_string());
    }
}

#[test]
fn test_display() {
    let id = ObjectId::with_string("5932a005b4b4b4ac168cd9e4").unwrap();

    assert_eq!(format!("{}", id), "5932a005b4b4b4ac168cd9e4")
}

#[test]
fn test_debug() {
    let id = ObjectId::with_string("5932a005b4b4b4ac168cd9e4").unwrap();

    assert_eq!(format!("{:?}", id), "ObjectId(5932a005b4b4b4ac168cd9e4)")
}
