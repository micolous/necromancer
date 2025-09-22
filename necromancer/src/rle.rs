use crate::protocol::rle::RleDecompressor;

/// Computes the MD5 of the uncompressed frame, and its size (in bytes).
///
/// This requires a full scan of the file.
pub fn rle_md5_size(i: impl Iterator<Item = u64>) -> ([u8; 16], u64) {
    let mut md5 = md5::Context::new();
    let rle = RleDecompressor::new(i);
    let mut s = 0;

    for w in rle {
        md5.consume(w.to_be_bytes());
        s += 8;
    }

    (md5.finalize().into(), s)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Result;

    fn rle_md5_size_from_bytes(b: &[u8]) -> ([u8; 16], u64) {
        rle_md5_size(
            b.chunks(8)
                .map(|c| u64::from_be_bytes(c.try_into().unwrap())),
        )
    }

    #[test]
    fn rle_black() -> Result {
        let black = hex::decode("fefefefefefefefe00000000000fd2003ac800403ac80040")?;
        assert_eq!(
            (
                [
                    0x52, 0x99, 0x71, 0x75, 0x9f, 0xd7, 0x8a, 0x24, 0xbd, 0x0e, 0x24, 0x82, 0x69,
                    0xe9, 0xe5, 0xe5
                ],
                1920 * 1080 * 4
            ),
            rle_md5_size_from_bytes(&black)
        );
        Ok(())
    }

    #[test]
    fn rle_red() -> Result<()> {
        let red = hex::decode("fefefefefefefefe00000000000fd2003ac668f93acefcf9")?;
        assert_eq!(
            (
                [
                    0xe7, 0xe0, 0xaa, 0x0e, 0x50, 0x28, 0x72, 0xd1, 0xc2, 0x2c, 0x34, 0x87, 0xbb,
                    0x8e, 0xb1, 0xc5
                ],
                1920 * 1080 * 4
            ),
            rle_md5_size_from_bytes(&red)
        );
        Ok(())
    }
}
