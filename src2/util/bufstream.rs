use std::io::{self, Read, Write};
use std::io::{Error, ErrorKind};
use std::cmp;

pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub struct BufStream<S: Read + Write> {
    inner: Option<S>,
    reader: Box<[u8]>,
    writer: Vec<u8>,
    pos: usize,
    cap: usize,
    panicked: bool
}

impl <S: Read + Write> BufStream<S> {
    pub fn new(inner: S) -> BufStream<S> {
        BufStream::with_capacity(DEFAULT_BUF_SIZE, inner)
    }

    pub fn with_capacity(cap: usize, inner: S) -> BufStream<S> {
        BufStream {
            inner: Some(inner),
            reader: vec![0; cap].into_boxed_slice(),
            writer: Vec::with_capacity(cap),
            pos: 0,
            cap: 0,
            panicked: false
        }
    }

    pub fn get_ref(&self) -> &S {
        &self.inner.as_ref().unwrap()
    }

    pub fn get_mut(&mut self) -> &mut S {
        self.inner.as_mut().unwrap()
    }

    pub fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.pos >= self.cap {
            debug_assert!(self.pos == self.cap);
            self.cap = self.inner.as_mut().unwrap().read(&mut self.reader)?;
            self.pos = 0;
        }

        Ok(&self.reader[self.pos..self.cap])
    }

    pub fn consume(&mut self, amt: usize) {
        self.pos = cmp::min(self.pos + amt, self.cap);
    }

    pub fn flush_buf(&mut self) -> io::Result<()> {
        let mut written = 0;
        let len = self.writer.len();
        let mut ret = Ok(());

        while written < len {
            self.panicked = true;
            let r = self.inner.as_mut().unwrap().write(&self.writer[written..]);
            self.panicked = false;

            match r {
                Ok(0) => {
                    ret = Err(Error::new(ErrorKind::WriteZero, "failed to write the buffered data"));
                    break;
                }
                Ok(n) => written += n,
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => { ret = Err(e); break }
            }
        }

        if written > 0 {
            self.writer.drain(..written);
        }

        ret
    }
}

impl <S: Read + Write> Read for BufStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos == self.cap && buf.len() >= self.reader.len() {
            return self.inner.as_mut().unwrap().read(buf);
        }

        let nread = {
            let mut rem = self.fill_buf()?;
            rem.read(buf)?
        };

        self.pos = cmp::min(self.pos + nread, self.cap);

        Ok(nread)
    }
}

impl <S: Read + Write> Write for BufStream<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.writer.len() + buf.len() > self.writer.capacity() {
            self.flush_buf()?;
        }

        if buf.len() >= self.writer.capacity() {
            self.panicked = true;
            let r = self.inner.as_mut().unwrap().write(buf);
            self.panicked = false;
            r
        } else {
            Write::write(&mut self.writer, buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush_buf().and_then(|()| self.get_mut().flush())
    }
}

impl <S: Read + Write> Drop for BufStream<S> {
    fn drop(&mut self) {
        if !self.panicked {
            let _r = self.flush_buf();
        }
    }
}
