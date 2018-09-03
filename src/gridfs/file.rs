//! Lower-level file and chunk representations in GridFS.
use std::{cmp, io, thread};
use std::error::Error as ErrorTrait;
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::sync::atomic::{AtomicIsize, ATOMIC_ISIZE_INIT, Ordering};

use chrono::{DateTime, Utc};

use object_id::ObjectId;
use bson::{self, Bson};
use bson::spec::BinarySubtype;
use error::Error::{self, ArgumentError, OperationError, PoisonLockError};
use error::Result;
use collection::options::IndexOptions;
use util::md5;
use super::Store;

pub const DEFAULT_CHUNK_SIZE: i32 = 255 * 1024;
pub const MEGABYTE: usize = 1024 * 1024;

/// File modes.
#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Closed,
    Read,
    Write,
}

// Helper class to implement a threaded mutable error.
struct InnerError {
    inner: Option<Error>,
}

/// A writable or readable file stream within GridFS.
pub struct File {
    // The file lock.
    mutex: Arc<Mutex<()>>,
    // A condition variable to coordinate asynchronous operations.
    condvar: Arc<Condvar>,
    // The associated GridFS.
    gfs: Store,
    // The current chunk index.
    chunk_num: i32,
    // The current file byte offset.
    offset: i64,
    // The number of writes in progress.
    wpending: Arc<AtomicIsize>,
    // The write buffer.
    wbuf: Vec<u8>,
    // The file md5 hash builder.
    //wsum: Md5,
    wsum: md5::Context,
    // The read buffer.
    rbuf: Vec<u8>,
    // Holds a pre-cached chunk.
    rcache: Option<Arc<Mutex<CachedChunk>>>,
    // The file read/write mode.
    mode: Mode,
    // Holds an error, if one occurred during a threaded operation.
    err: Arc<RwLock<InnerError>>,
    /// The file document associated with this stream.
    pub doc: GfsFile,
}

/// A one-to-one representation of a file document within GridFS.
pub struct GfsFile {
    // The byte length.
    len: i64,
    // The md5 hash.
    md5: String,
    // The unique object id.
    pub id: ObjectId,
    // The chunk size.
    pub chunk_size: i32,
    // An array of alias strings.
    pub aliases: Vec<String>,
    // The filename of the document.
    pub name: Option<String>,
    // The date the document was first stored in GridFS.
    pub upload_date: Option<DateTime<Utc>>,
    // The content type of the file.
    pub content_type: Option<String>,
    // Any additional metadata provided by the user.
    pub metadata: Option<Vec<u8>>,
}

// A pre-loaded chunk.
struct CachedChunk {
    // The file chunk index.
    n: i32,
    // The binary chunk data.
    data: Vec<u8>,
    // The error that occurred during reading, if any occurred.
    err: Option<Error>,
}

impl Deref for File {
    type Target = GfsFile;

    fn deref(&self) -> &Self::Target {
        &self.doc
    }
}

impl DerefMut for File {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.doc
    }
}

impl File {
    /// A new file stream with an id-referenced GridFS file.
    pub fn new(gfs: Store, id: ObjectId, mode: Mode) -> File {
        File::with_gfs_file(gfs, GfsFile::new(id), mode)
    }

    /// A new file stream with a name-and-id-referenced GridFS file.
    pub fn with_name(gfs: Store, name: String, id: ObjectId, mode: Mode) -> File {
        File::with_gfs_file(gfs, GfsFile::with_name(name, id), mode)
    }

    /// A new file stream from a read file document.
    pub fn with_doc(gfs: Store, doc: &bson::Document) -> File {
        File::with_gfs_file(gfs, GfsFile::with_doc(doc), Mode::Read)
    }

    // Generic new file stream.
    fn with_gfs_file(gfs: Store, file: GfsFile, mode: Mode) -> File {
        File {
            mutex: Arc::new(Mutex::new(())),
            condvar: Arc::new(Condvar::new()),
            mode,
            gfs,
            chunk_num: 0,
            offset: 0,
            wpending: Arc::new(ATOMIC_ISIZE_INIT),
            wbuf: Vec::new(),
            wsum: md5::Context::new(),
            rbuf: Vec::new(),
            rcache: None,
            doc: file,
            err: Arc::new(RwLock::new(InnerError { inner: None })),
        }
    }

    /// Returns the byte length of the file.
    pub fn len(&self) -> i64 {
        self.len
    }

    /// Returns true if the file contains no bytes.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Retrieves the description of the threaded error, if one occurred.
    pub fn err_description(&self) -> Result<Option<String>> {
        let err = self.err.read()?;
        let inner = &err.deref().inner;
        let description = match *inner {
            Some(ref err) => Some(err.description().to_string()),
            None => None,
        };
        Ok(description)
    }

    /// Ensures the file mode matches the desired mode.
    pub fn assert_mode(&self, mode: &Mode) -> Result<()> {
        if &self.mode != mode {
            return match self.mode {
                Mode::Read => Err(ArgumentError("File is open for reading.".to_string())),
                Mode::Write => Err(ArgumentError("File is open for writing.".to_string())),
                Mode::Closed => Err(ArgumentError("File is closed.".to_string())),
            };
        }
        Ok(())
    }

    /// Completes writing or reading and closes the file. This will be called when the
    /// file is dropped, but errors will be ignored. Therefore, this method should
    /// be called manually.
    pub fn close(&mut self) -> Result<()> {

        // Flush chunks
        if self.mode == Mode::Write {
            self.flush()?;
        }

        let _ = self.mutex.lock()?;

        // Complete file write
        if self.mode == Mode::Write {
            if self.err_description()?.is_none() {
                if self.doc.upload_date.is_none() {
                    self.doc.upload_date = Some(Utc::now());
                }
                self.doc.md5 = format!("{:x}", self.wsum.compute());
                self.gfs.inner.files.insert_one(self.doc.to_bson(), None)?;

                // Ensure indexes
                self.gfs.inner.files.create_index(doc!{ "filename": 1 }, None, None)?;

                let mut opts = IndexOptions::new();
                opts.unique = Some(true);
                self.gfs.inner.chunks.create_index(doc!{ "files_id": 1, "n": 1}, Some(opts), None)?;
            } else {
                self.gfs.inner.chunks.delete_many(doc!{ "files_id": self.doc.id.clone() }, None)?;
            }
        }

        // Complete pending chunk pre-load and wipe cache
        if self.mode == Mode::Read && self.rcache.is_some() {
            {
                let cache = self.rcache.as_ref().unwrap();
                let _ = cache.lock()?;
            }
            self.rcache = None;
        }

        self.mode = Mode::Closed;

        let description = self.err_description()?;
        if description.is_some() {
            Err(OperationError(description.unwrap()))
        } else {
            Ok(())
        }
    }

    /// Inserts a file chunk into GridFS.
    fn insert_chunk(&self, n: i32, buf: &[u8]) -> Result<()> {

        // Start a pending write and copy the buffer and metadata into a bson document
        self.wpending.fetch_add(1, Ordering::SeqCst);
        let mut vec_buf = Vec::with_capacity(buf.len());
        vec_buf.extend_from_slice(&buf);

        let document = doc!{
            "_id": ObjectId::new()?,
            "files_id": self.doc.id.clone(),
            "n": n,
            "data": (BinarySubtype::Generic, vec_buf)
        };

        // Insert chunk asynchronously into the database.
        let arc_gfs = self.gfs.clone();
        let arc_mutex = self.mutex.clone();
        let arc_wpending = self.wpending.clone();
        let cvar = self.condvar.clone();
        let err = self.err.clone();

        thread::spawn(move || {
            let result = arc_gfs.inner.chunks.insert_one(document, None);

            // Complete pending write
            let _ = arc_mutex.lock();
            arc_wpending.fetch_sub(1, Ordering::SeqCst);
            if result.is_err() {
                if let Ok(mut err_mut) = err.write() {
                    err_mut.inner = Some(result.err().unwrap());
                }
            }
            cvar.notify_all();
        });

        Ok(())
    }

    // Retrieves a binary file chunk from GridFS.
    pub fn find_chunk(&mut self, id: ObjectId, chunk_num: i32) -> Result<Vec<u8>> {
        let filter = doc!{"files_id": id, "n": chunk_num };
        match self.gfs.inner.chunks.find_one(filter, None)? {
            Some(doc) => {
                match doc.get("data") {
                    Some(&Bson::Binary(_, ref buf)) => Ok(buf.clone()),
                    _ => Err(OperationError("Chunk contained no data".to_string())),
                }
            }
            None => Err(OperationError("Chunk not found".to_string())),
        }
    }

    // Retrieves a binary file chunk and asynchronously pre-loads the next chunk.
    fn get_chunk(&mut self) -> Result<Vec<u8>> {
        let id = self.doc.id.clone();
        let curr_chunk_num = self.chunk_num;

        // Find the file chunk from the cache or from GridFS.
        let data = if let Some(lock) = self.rcache.take() {
            let cache = lock.lock()?;
            if cache.n == curr_chunk_num && cache.err.is_none() {
                cache.data.clone()
            } else {
                self.find_chunk(id, curr_chunk_num)?
            }
        } else {
            self.find_chunk(id, curr_chunk_num)?
        };

        self.chunk_num += 1;

        // Pre-load the next file chunk for GridFS.
        if i64::from(self.chunk_num) * i64::from(self.doc.chunk_size) < self.doc.len {

            let cache = Arc::new(Mutex::new(CachedChunk::new(self.chunk_num)));

            let arc_cache = cache.clone();
            let arc_gfs = self.gfs.clone();
            let id = self.doc.id.clone();
            let next_chunk_num = self.chunk_num;

            thread::spawn(move || {
                let mut cache = match arc_cache.lock() {
                    Ok(cache) => cache,
                    Err(_) => {
                        // Cache lock is poisoned; abandon caching mechanism.
                        return;
                    }
                };

                let result = arc_gfs.inner.chunks.find_one(doc!{"files_id": id, "n": next_chunk_num}, None);

                match result {
                    Ok(Some(doc)) => {
                        match doc.get("data") {
                            Some(&Bson::Binary(_, ref buf)) => {
                                cache.data = buf.clone();
                                cache.err = None;
                            }
                            _ => {
                                cache.err = Some(OperationError("Chunk contained no data.".to_string()))
                            }
                        }
                    }
                    Ok(None) => cache.err = Some(OperationError("Chunk not found.".to_string())),
                    Err(err) => cache.err = Some(err),
                }
            });

            self.rcache = Some(cache);
        }

        Ok(data)
    }
}

impl io::Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.assert_mode(&Mode::Write)?;

        let mut guard = match self.mutex.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, PoisonLockError)),
        };

        let description = self.err_description()?;
        if description.is_some() {
            return Err(io::Error::new(io::ErrorKind::Other, OperationError(description.unwrap())));
        }

        let mut data = buf;
        let n = data.len();
        let chunk_size = self.doc.chunk_size as usize;

        self.doc.len += data.len() as i64;

        // If the total local buffer is below chunk_size, return.
        if self.wbuf.len() + data.len() < chunk_size {
            self.wbuf.extend_from_slice(data);
            return Ok(n);
        }

        // Otherwise, form a chunk with the current buffer + data and flush to GridFS.
        if !self.wbuf.is_empty() {

            // Split data
            let missing = cmp::min(chunk_size - self.wbuf.len(), data.len());
            let (part1, part2) = data.split_at(missing);

            // Extend local buffer into a chunk
            self.wbuf.extend_from_slice(part1);
            data = part2;

            let curr_chunk_num = self.chunk_num;
            self.chunk_num += 1;
            self.wsum.consume(buf);

            // If over a megabyte is being written at once, wait for the load to reduce.
            while self.doc.chunk_size * self.wpending.load(Ordering::SeqCst) as i32 >= MEGABYTE as i32 {
                guard = match self.condvar.wait(guard) {
                    Ok(guard) => guard,
                    Err(_) => return Err(io::Error::new(io::ErrorKind::Other, PoisonLockError)),
                };

                let description = self.err_description()?;
                if description.is_some() {
                    return Err(io::Error::new(io::ErrorKind::Other,
                                              OperationError(description.unwrap())));
                }
            }

            // Flush chunk to GridFS
            let chunk = self.wbuf.clone();
            self.insert_chunk(curr_chunk_num, &chunk)?;
            self.wbuf.clear();
        }

        // Continuously write full chunks of data to GridFS.
        while data.len() > chunk_size as usize {
            let size = cmp::min(chunk_size, data.len());
            let (part1, part2) = data.split_at(size);

            let curr_chunk_num = self.chunk_num;
            self.chunk_num += 1;
            self.wsum.consume(buf);

            // Pending megabyte
            while self.doc.chunk_size * self.wpending.load(Ordering::SeqCst) as i32 >=
                  MEGABYTE as i32 {
                guard = match self.condvar.wait(guard) {
                    Ok(guard) => guard,
                    Err(_) => return Err(io::Error::new(io::ErrorKind::Other, PoisonLockError)),
                };

                let description = self.err_description()?;
                if description.is_some() {
                    return Err(io::Error::new(io::ErrorKind::Other,
                                              OperationError(description.unwrap())));
                }
            }

            self.insert_chunk(curr_chunk_num, part1)?;
            data = part2;
        }

        // Store unfinished chunk to local buffer and return.
        self.wbuf.extend_from_slice(data);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.assert_mode(&Mode::Write)?;

        let mut guard = match self.mutex.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, PoisonLockError)),
        };

        // Flush local buffer to GridFS
        if !self.wbuf.is_empty() && self.err_description()?.is_none() {
            let chunk_num = self.chunk_num;
            self.chunk_num += 1;
            //self.wsum.input(&self.wbuf);
            self.wsum.consume(&self.wbuf);

            // Pending megabyte
            while self.doc.chunk_size * self.wpending.load(Ordering::SeqCst) as i32 >=
                  MEGABYTE as i32 {
                guard = match self.condvar.wait(guard) {
                    Ok(guard) => guard,
                    Err(_) => return Err(io::Error::new(io::ErrorKind::Other, PoisonLockError)),
                }
            }

            // Flush and clear local buffer.
            if self.err_description()?.is_none() {
                let chunk = self.wbuf.clone();
                self.insert_chunk(chunk_num, &chunk)?;
                self.wbuf.clear();
            }
        }

        // Block until all pending write ares complete.
        while self.wpending.load(Ordering::SeqCst) > 0 {
            guard = match self.condvar.wait(guard) {
                Ok(guard) => guard,
                Err(_) => return Err(io::Error::new(io::ErrorKind::Other, PoisonLockError)),
            }
        }

        let description = self.err_description()?;
        if description.is_some() {
            return Err(io::Error::new(io::ErrorKind::Other, OperationError(description.unwrap())));
        }

        Ok(())
    }
}

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.assert_mode(&Mode::Read)?;

        let _ = match self.mutex.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, PoisonLockError)),
        };

        // End of File (EOF)
        if self.offset == self.doc.len {
            return Ok(0);
        }

        // Read chunks into memory
        if self.rbuf.len() < buf.len() && i64::from(self.chunk_num) * i64::from(self.doc.chunk_size) < self.doc.len {
            let chunk = self.get_chunk()?;
            self.rbuf.extend_from_slice(&chunk);
        }

        // Write into buf
        let i = (&mut *buf).write(&self.rbuf)?;
        self.offset += i as i64;

        // Save unread chunk portion into local read buffer
        let mut new_rbuf = Vec::with_capacity(self.rbuf.len() - i);
        {
            let (_, p2) = self.rbuf.split_at(i);
            new_rbuf.extend_from_slice(p2);
        }

        self.rbuf = new_rbuf;

        Ok(i)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        // This ignores errors during closing; instead, the close function should be
        // used explicitly to handle errors.
        let _ = self.close();
    }
}

impl GfsFile {
    /// Create a new GfsFile by ObjectId.
    pub fn new(id: ObjectId) -> GfsFile {
        GfsFile {
            id,
            chunk_size: DEFAULT_CHUNK_SIZE,
            name: None,
            len: 0,
            md5: String::new(),
            aliases: Vec::new(),
            upload_date: None,
            content_type: None,
            metadata: None,
        }
    }

    /// Create a new GfsFile by filename and ObjectId.
    pub fn with_name(name: String, id: ObjectId) -> GfsFile {
        GfsFile {
            id,
            chunk_size: DEFAULT_CHUNK_SIZE,
            name: Some(name),
            len: 0,
            md5: String::new(),
            aliases: Vec::new(),
            upload_date: None,
            content_type: None,
            metadata: None,
        }
    }

    /// Read a GridFS file document into a new GfsFile.
    pub fn with_doc(doc: &bson::Document) -> GfsFile {
        let mut file: GfsFile;

        if let Some(&Bson::ObjectId(ref id)) = doc.get("_id") {
            file = GfsFile::new(id.clone())
        } else {
            panic!("Document has no _id!");
        }

        if let Some(&Bson::String(ref name)) = doc.get("filename") {
            file.name = Some(name.to_string());
        }

        if let Some(&Bson::Int32(ref chunk_size)) = doc.get("chunkSize") {
            file.chunk_size = *chunk_size;
        }

        if let Some(&Bson::UTCDatetime(ref datetime)) = doc.get("uploadDate") {
            file.upload_date = Some(*datetime);
        }

        if let Some(&Bson::Int64(ref length)) = doc.get("length") {
            file.len = *length;
        }

        if let Some(&Bson::String(ref hash)) = doc.get("md5") {
            file.md5 = hash.to_string();
        }

        if let Some(&Bson::String(ref content_type)) = doc.get("contentType") {
            file.content_type = Some(content_type.to_string());
        }

        if let Some(&Bson::Binary(_, ref metadata)) = doc.get("metadata") {
            file.metadata = Some(metadata.clone());
        }

        file
    }

    /// Converts a GfsFile into a bson document.
    pub fn to_bson(&self) -> bson::Document {
        let mut doc = doc!{
            "_id": self.id.clone(),
            "chunkSize": self.chunk_size,
            "length": self.len,
            "md5": self.md5.to_string(),
            "uploadDate": *self.upload_date.as_ref().unwrap()
        };

        if self.name.is_some() {
            doc.insert("filename", self.name.as_ref().unwrap().to_string());
        }

        if self.content_type.is_some() {
            doc.insert("contentType", self.content_type.as_ref().unwrap().to_string());
        }

        if self.metadata.is_some() {
            doc.insert(
                "metadata",
                (BinarySubtype::Generic, self.metadata.as_ref().unwrap().clone())
            );
        }

        doc
    }
}

impl CachedChunk {
    // Create a new cached chunk to be post-populated with the binary data.
    pub fn new(n: i32) -> CachedChunk {
        CachedChunk {
            n,
            data: Vec::new(),
            err: Some(Error::DefaultError("Chunk has not yet been initialized".to_string())),
        }
    }
}
