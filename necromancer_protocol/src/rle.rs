//! Run-length frame encoding ("Simple RLE")
//!
//! ## Data format
//!
//! Data is stored as an array of `u64`.
//!
//! If an entry is set to the RLE marker (`0xfefefefefefefefe`), the following
//! two `u64`s are RLE instructions:
//!
//! * `u64`: number of times to repeat the following block
//! * `u64`: the block data to repeat
//!
//! Otherwise, the entry is passed as-is.
//!
//! The frame pixel format (`ay10`) [is described `yuva` module docs][crate::yuva].

use crate::{Error, Result};

/// Maximum number of repeats (in blocks).
const MAX_REPEATS: u64 = 7860 * 4680;

/// Marker for the start of an RLE sequence.
pub const RLE_MARKER: u64 = 0xfefefefefefefefe;

/// Decompressor for "Simple RLE".
///
/// This takes a Simple RLE stream as [`Iterator`] of [`u64`], and itself
/// implements an [`Iterator`] of [`u64`].
pub struct RleDecompressor<T: Iterator<Item = u64>> {
    /// Iterator over the file
    i: T,
    /// Number of times remaining to repeat the sequence `p`
    c: u64,
    /// Repeated sequence
    p: u64,
    /// End the stream, regardless of whether the iterator was consumed
    e: bool,
}

impl<T: Iterator<Item = u64>> RleDecompressor<T> {
    pub fn new(i: T) -> Self {
        Self {
            i,
            c: 0,
            p: 0,
            e: false,
        }
    }
}

impl<T: Iterator<Item = u64>> Iterator for RleDecompressor<T> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: implement FalliableIterator instead, so that it propagates
        // errors.
        if self.e {
            return None;
        }

        if self.c > 0 {
            self.c -= 1;
            return Some(self.p);
        }

        loop {
            let Some(n) = self.i.next() else {
                self.e = true;
                return None;
            };

            if n == RLE_MARKER {
                self.c = self.i.next()? - 1;
                self.p = self.i.next()?;

                if self.c == 0 {
                    warn!("c = 0?");
                    continue;
                }

                if self.c > MAX_REPEATS {
                    error!("RLE repeat {:#08x} > {MAX_REPEATS:#08x}, aborting - likely data corruption!", self.c);
                    self.c = 0;
                    self.e = true;
                    return None;
                }
                return Some(self.p);
            } else {
                return Some(n);
            }
        }
    }
}

/// Find the size of an RLE sequence in elements.
///
/// Each element corresponds to 2 pixels. Multiply by 8 to get the size in bytes.
///
/// ## Comparison with [RleDecompressor]
///
/// * this function is a little more efficient than using `.count()` on an
///   [RleDecompressor], because this does not create copies of repeated data.
/// * this function will return errors for invalid RLE sequences, whereas
///   [RleDecompressor] will just stop part way.
pub fn rle_size_elements(mut i: impl Iterator<Item = u64>) -> Result<u64> {
    let mut o = 0;
    while let Some(t) = i.next() {
        if t == RLE_MARKER {
            let Some(c) = i.next() else {
                error!("EOF when expecting RLE sequence!");
                return Err(Error::UnexpectedState);
            };

            if c > MAX_REPEATS {
                error!(
                    "RLE repeat {c:#08x} > {MAX_REPEATS:#08x}, aborting - likely data corruption!"
                );
                return Err(Error::InvalidLength);
            }
            o += c;

            // skip the sequence
            if i.next().is_none() {
                error!("EOF when expecting RLE sequence!");
                return Err(Error::UnexpectedState);
            };
        } else {
            o += 1;
        }
    }

    Ok(o)
}

/// Compressor for "Simple RLE".
///
/// This takes an image stream as an [`Iterator`] of [`u64`] and itself
/// implements an [`Iterator`] of [`u64`] for RLE-encoded data.
pub struct RleCompressor<T: Iterator<Item = u64>> {
    /// Iterator over the file
    i: T,
    /// Number of times we saw the sequence `p`
    c: u64,
    /// Do we need to emit the item count on the next pass?
    m: bool,
    /// Repeated sequence
    p: u64,
    /// Next item, if we've peeked ahead.
    n: Option<u64>,
}

impl<T: Iterator<Item = u64>> RleCompressor<T> {
    pub fn new(i: T) -> Self {
        Self {
            i,
            m: false,
            c: 0,
            p: 0,
            n: None,
        }
    }
}

impl<T> Iterator for RleCompressor<T>
where
    T: Iterator<Item = u64>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.c != 0 {
            if self.m {
                self.m = false;
                return Some(self.c);
            } else {
                self.c = 0;
                return Some(self.p);
            }
        }

        // Check if we have a stashed peek
        self.p = if let Some(n) = self.n.take() {
            n
        } else {
            self.i.next()?
        };
        self.m = true;
        self.c = 1;

        loop {
            // Peek at the next item
            if let Some(n) = self.i.next() {
                if n == self.p {
                    self.c += 1;

                    if self.c < MAX_REPEATS {
                        continue;
                    } else {
                        // We'd overflow, so stash this for later.
                        self.n = Some(n);
                    }
                } else {
                    // Item changed, stash that peek for later, and return the
                    // marker
                    self.n = Some(n);
                }
            }

            if self.p != RLE_MARKER {
                // Item doesn't need escaping
                if self.c == 1 {
                    // Item was alone, and not the marker.
                    self.c = 0;
                    return Some(self.p);
                }

                if self.c == 2 {
                    // There were two of the same item in a row, but compressing
                    // this would increase its size.
                    self.c = 1;
                    self.m = false;
                    return Some(self.p);
                }
            }

            // Item wasn't alone. Start a RLE.
            return Some(RLE_MARKER);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Result;

    /// Simple RLE stream size, using an input byte array, and returning the size in bytes.
    fn rle_size_from_bytes_in_bytes(b: &[u8]) -> Result<u64> {
        rle_size_elements(
            b.chunks(8)
                .map(|c| u64::from_be_bytes(c.try_into().unwrap())),
        )
        .map(|e| e * 8)
    }

    #[test]
    fn rle_black() -> Result {
        let black = hex::decode("fefefefefefefefe00000000000fd2003ac800403ac80040")?;
        assert_eq!((1920 * 1080 * 4), rle_size_from_bytes_in_bytes(&black)?);
        let mut c = 0;
        let i = RleDecompressor::new(
            black
                .chunks(8)
                .map(|c| u64::from_be_bytes(c.try_into().unwrap())),
        )
        .map(|w| {
            assert_eq!(0x3ac800403ac80040, w);
            c += 1;
            w
        });

        let compressor = RleCompressor::new(i);
        let mut out = Vec::with_capacity(black.len());
        for w in compressor {
            out.extend_from_slice(&w.to_be_bytes());
        }

        assert_eq!(1036800, c);
        assert_eq!(out, black);

        Ok(())
    }

    #[test]
    fn rle_red() -> Result {
        let red = hex::decode("fefefefefefefefe00000000000fd2003ac668f93acefcf9")?;
        assert_eq!((1920 * 1080 * 4), rle_size_from_bytes_in_bytes(&red)?);
        let mut c = 0;
        let i = RleDecompressor::new(
            red.chunks(8)
                .map(|c| u64::from_be_bytes(c.try_into().unwrap())),
        )
        .map(|w| {
            assert_eq!(0x3ac668f93acefcf9, w);
            c += 1;
            w
        });

        let compressor = RleCompressor::new(i);
        let mut out = Vec::with_capacity(red.len());
        for w in compressor {
            out.extend_from_slice(&w.to_be_bytes());
        }

        assert_eq!(1036800, c);
        assert_eq!(out, red);

        Ok(())
    }

    #[test]
    fn rle_optimisation() -> Result {
        // All unique entries should not result in RLE.
        let d = [0x1234, 0x3456, 0x5678];
        let compressor = RleCompressor::new(d.iter().copied());
        let out: Vec<u64> = compressor.collect();
        assert_eq!(d.as_slice(), out);
        assert_eq!(d.len() as u64, rle_size_elements(out.iter().copied())?);

        // Only two entries of the same value in a row should not result in RLE.
        let d = [0x1234, 0x1234, 0x3456];
        let compressor = RleCompressor::new(d.iter().copied());
        let out: Vec<u64> = compressor.collect();
        assert_eq!(d.as_slice(), out);
        assert_eq!(d.len() as u64, rle_size_elements(out.iter().copied())?);

        // Three entries of the same value in a row results in RLE
        let d = [0x1234, 0x1234, 0x1234];
        let expected = [0xfefefefefefefefe, 0x3, 0x1234];
        let compressor = RleCompressor::new(d.iter().copied());
        let out: Vec<u64> = compressor.collect();
        assert_eq!(expected.as_slice(), out);
        assert_eq!(d.len() as u64, rle_size_elements(out.iter().copied())?);

        Ok(())
    }

    #[test]
    fn rle_escape() -> Result {
        // Presence of the RLE marker should be escaped
        let d = [0xfefefefefefefefe, 0x1234, 0x1234, 0x1234];
        let expected = [
            0xfefefefefefefefe,
            0x1,
            0xfefefefefefefefe,
            0xfefefefefefefefe,
            0x3,
            0x1234,
        ];
        let compressor = RleCompressor::new(d.iter().copied());
        let out: Vec<u64> = compressor.collect();
        assert_eq!(expected.as_slice(), out);
        assert_eq!(d.len() as u64, rle_size_elements(out.iter().copied())?);

        Ok(())
    }
}
