//! Converters for bit-packed 10-bit YUVA 4:2:2:4 ("ay10")
//!
//! The switchers store still images in this format,
//! [but `ffmpeg` doesn't support it][0]. 😿
//!
//! This transforms to/from `yuva422p10be` (planar-format), which `ffmpeg`
//! supports, and we can handle losslessly.
//!
//! This module does not handle any colour space related issues.
//!
//! ## `ay10` format
//!
//! **Reference:** <https://forum.blackmagicdesign.com/viewtopic.php?f=12&t=87152>
//!
//! Every two pixels is represented by two `u32` values:
//!
//! * First value:
//!   * `y0`: 10 bits (LSB)
//!   * `cb`: 10 bits (subsampled)
//!   * `a0`: 10 bits
//!   * 2 bits padding (MSB)
//! * Second value:
//!   * `y1`: 10 bits (LSB)
//!   * `cr`: 10 bits (subsampled)
//!   * `a1`: 10 bits
//!   * 2 bits padding (MSB)
//!
//! The total data size is `(width * height * 4)` bytes.
//!
//! ## `yuva422p10be` format
//!
//! This is the closest comparable format currently supported by FFmpeg. The
//! frame is split into 4 planes, which are concatenated:
//!
//! 1. `y` value of every pixel
//! 2. `cb` value of every 2 pixels (subsampled)
//! 3. `cr` value of every 2 pixels (subsampled)
//! 4. `a` value of every pixel
//!
//! Every value is stored using the lower 10 bits of a `u16`.
//!
//! The total data size is `(width * height * 6)` bytes.
//!
//! [0]: https://trac.ffmpeg.org/ticket/10577
use crate::{Error, Result};

/// Convert bit-packed 10-bit YUVA 4:2:2:4 into a planar format (`yuva422p10be`)
/// suitable for FFmpeg.
pub fn ay10be_to_yuva422p10be(b: &[u64]) -> Vec<u8> {
    // 8 bytes into 12
    // _AUY _AVY => YY YY | UU | VV | AA AA
    let new_len = b.len() * 12;
    let mut o = vec![0; new_len];

    let mut yp = 0usize;
    let mut up = b.len() * 4;
    let mut vp = b.len() * 6;
    let mut ap = b.len() * 8;

    for w in b.iter().copied() {
        let cr = ((w >> 10) & 0x3ff) as u16;

        let y = (((w >> 32) & 0x3ff) as u32) << 16 | (w & 0x3ff) as u32;
        let cb = ((w >> 42) & 0x3ff) as u16;
        let a = (((w >> 52) & 0x3ff) as u32) << 16 | ((w >> 20) & 0x3ff) as u32;

        o[yp..yp + 4].copy_from_slice(&y.to_be_bytes());
        yp += 4;

        o[up..up + 2].copy_from_slice(&cb.to_be_bytes());
        up += 2;

        o[vp..vp + 2].copy_from_slice(&cr.to_be_bytes());
        vp += 2;

        o[ap..ap + 4].copy_from_slice(&a.to_be_bytes());
        ap += 4;
    }

    o
}

/// Convert a `ffmpeg` `yuva422p10be` frame into bit-packed 10-bit YUVA 4:2:2:4
/// format.
pub fn yuva422p10be_to_ay10be(b: &[u8]) -> Result<Vec<u64>> {
    // TODO: make this return an iterator instead

    // 12 bytes into 8
    // YY YY | UU | VV | AA AA => _AUY _AVY
    if b.len() % 12 != 0 {
        error!(
            "unexpected buffer size ({}), expected to be a multiple of 12 bytes",
            b.len()
        );
        return Err(Error::InvalidLength);
    }

    let new_len = b.len() / 12;
    let mut o = Vec::with_capacity(new_len);

    // Size of the larger planes. U and V together take up the space of 1 plane.
    let ps = b.len() / 3;

    // Split view by plane
    let (yp, b) = b.split_at(ps);
    let (up, b) = b.split_at(ps / 2);
    let (vp, ap) = b.split_at(ps / 2);

    // Chunked reader for each of the 4 planes (y, u, v, a)
    let yp = yp.chunks_exact(4);
    let up = up.chunks_exact(2);
    let vp = vp.chunks_exact(2);
    let ap = ap.chunks_exact(4);

    for (((y, u), v), a) in yp.zip(up).zip(vp).zip(ap) {
        let y = u32::from_be_bytes(y.try_into().map_err(|_| Error::Internal)?);
        let y0 = ((y >> 16) & 0x3ff) as u64;
        let y1 = (y & 0x3ff) as u64;
        let u = (u16::from_be_bytes(u.try_into().map_err(|_| Error::Internal)?) & 0x3ff) as u64;
        let v = (u16::from_be_bytes(v.try_into().map_err(|_| Error::Internal)?) & 0x3ff) as u64;
        let a = u32::from_be_bytes(a.try_into().map_err(|_| Error::Internal)?);
        let a0 = ((a >> 16) & 0x3ff) as u64;
        let a1 = (a & 0x3ff) as u64;

        o.push(a0 << 52 | u << 42 | y0 << 32 | a1 << 20 | v << 10 | y1);
    }
    assert_eq!(o.len(), new_len);

    Ok(o)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{rle::RleDecompressor, IntReader};
    use flate2::bufread::GzDecoder;
    use std::io::Read;

    const COLOUR_BARS_RLE_GZ: &[u8] = include_bytes!("./testdata/colourbars.rle.gz");
    const COLOUR_BARS_PLANAR_GZ: &[u8] = include_bytes!("./testdata/colourbars.yuva422p10be.gz");

    #[test]
    fn convert_round() -> Result {
        let mut colour_bars_planar = Vec::new();
        GzDecoder::new(COLOUR_BARS_PLANAR_GZ)
            .read_to_end(&mut colour_bars_planar)
            .unwrap();

        let colour_bars = GzDecoder::new(COLOUR_BARS_RLE_GZ);
        let colour_bars: Vec<u64> =
            RleDecompressor::new(IntReader::<_, u64>::new(colour_bars)).collect();
        let planar = ay10be_to_yuva422p10be(&colour_bars);
        assert_eq!(colour_bars_planar, planar);

        let packed = yuva422p10be_to_ay10be(&planar)?;
        assert_eq!(colour_bars, packed);
        Ok(())
    }
}
