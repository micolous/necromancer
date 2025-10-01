//! # Internal utilities
use std::{
    io::{Seek, SeekFrom, Write},
    marker::PhantomData,
};

/// Wrapper for [Write] streams which records the range of bytes written,
/// including any seeks.
pub struct OffsetCounter<T> {
    first_byte: u64,
    last_byte: u64,
    inner: T,
}

impl<T> OffsetCounter<T> {
    pub const fn total(&self) -> u64 {
        assert!(self.first_byte <= self.last_byte);
        self.last_byte - self.first_byte
    }
}

impl<T: Seek> OffsetCounter<T> {
    pub fn new(mut inner: T) -> Self {
        // TODO: check why this would fail
        let first_byte = inner.stream_position().unwrap_or_default();
        Self {
            first_byte,
            inner,
            last_byte: first_byte,
        }
    }
}

impl<T: Write + Seek> Write for OffsetCounter<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let w: usize = self.inner.write(buf)?;
        let _ = self.stream_position()?;
        Ok(w)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl<T: Seek> Seek for OffsetCounter<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let p = self.inner.seek(pos)?;
        if p > self.last_byte {
            self.last_byte = p;
        }
        if p < self.first_byte {
            self.first_byte = p;
        }
        Ok(p)
    }
}

/// Reads a file as an iterator of integer values.
pub struct IntReader<T, S> {
    f: T,
    m: PhantomData<S>,
}

impl<T, S> IntReader<T, S> {
    pub fn new(f: T) -> Self {
        Self { f, m: PhantomData }
    }
}

macro_rules! intreader_iterator {
    (
        $($type:ty)*
    ) => {
        $(
            impl<T: std::io::Read> Iterator for IntReader<T, $type> {
                type Item = $type;

                fn next(&mut self) -> Option<Self::Item> {
                    let mut b = [0; std::mem::size_of::<$type>()];
                    self.f.read_exact(&mut b).ok()?;
                    Some(<$type>::from_be_bytes(b))
                }
            }
        )*
    };
}

intreader_iterator!(u64);
