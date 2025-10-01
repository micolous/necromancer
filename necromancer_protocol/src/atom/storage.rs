//! # Media pool and file transfers
//!
//! There are four file types:
//!
//! * Still [ay10 frames][crate::ay10], [compressed with "Simple RLE"][crate::rle]
//! * Audio clips
//! * Multi-view labels (client-rendered text)
//! * Macros
//!
//! ## File download (client to switcher) process
//!
//! 1. Client obtains a priority lock (`PLCK`)
//! 1. Switcher responds with:
//!    * `LKST`: [lock status][MediaPoolLockStatus]
//!    * `CapA`: ??
//!    * `CCST`: ??
//! 1. Client [starts a file download][SetupFileDownload] (`FTSD`)
//! 1. Switcher [indicates how to chunk the data][FileTransferChunkParams] (`FTCD`), and how many
//!    data chunks to send before waiting for an acknowledgement.
//! 1. Client [sends data chunks][TransferChunk] (`FTDa`)
//! 1. Switcher periodically acknowledges chunks (on a [packet level][crate::packet])
//! 1. Client [sends file description][FinishFileDownload] (`FTFD`)
//! 1. Switcher [indicates the transfer was completed][TransferCompleted] (`FTDC`)
//! 1. Switcher [sends file description update][MediaPlayerFrameDescription] (`MPfe`)
//! 1. Client [unlocks the file][MediaPoolLock] (`LOCK`)
//!
//! ## File upload (switcher to client) process
//!
//! 1. Client [sends lock request][MediaPoolLock] (`LOCK`)
//! 1. Client [sends upload request][SetupFileUpload] (`FTSU`)
//! 1. Switcher [sends data chunks][TransferChunk] (`FTDa`)
//! 1. Client [periodically sends acknowledgement atoms][TransferAck] (`FTUA`)
//! 1. Switcher [indicates the transfer was completed][TransferCompleted] (`FTDC`)
//!
//! ## Unimplemented atoms
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `PLCK` | `MediaPoolPriorityLock` | 0x10
//! `FTAD` | `FileTransferCancelDownload` | 0xc
//! `CLMP` | `ClearMediaPool` | 0x8

// colour format may be defined in BMDSwitcherPixelFormat

use super::{str_from_utf8_null, Atom};
use binrw::binrw;
use std::fmt::Debug;

/// File type for [DownloadRequest] and [SetupFileUpload]
#[binrw]
#[brw(big, repr = u8)]
#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum FileType {
    StillFrame = 0x00,
    Audio = 0x01,
    MultiViewLabel = 0x02,
    Macro = 0x03,
}

/// `FTSU`: File Transfer Setup Upload
///
/// Used by the client to setup a data download from the switcher.
///
/// ## Packet format
///
/// * `u16`: transfer ID
/// * `u16`: store ID
/// * `u32`: storage index / slot
/// * `u8`: type
/// * 3 bytes padding
#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SetupFileUpload {
    pub id: u16,
    pub store_id: u16,
    pub index: u32,
    #[brw(pad_after = 3)]
    pub typ: FileType,
}

/// `FTCD`: file transfer chunk parameters (`FileTransferCanDownload`)
///
/// Used by the switcher to indicate to the client how to chunk and burst a transfer.
///
/// The switcher will periodically sends this message, even if the client hasn't
/// used up all the [`chunk_count`][Self::chunk_count] from the previous
/// [`FileTransferChunkParams`]. This means asynchronous transfers need to be
/// careful to keep everything in order.
///
/// However, this doesn't appear to measure the only bottleneck of the
/// switcher's file pipeline. The ATEM Mini reports `chunk_count = 320`, and
/// will wait about 20ms to batch-acknowledge upto 32 [`TransferChunk`]s – and
/// easily fall behind.
///
/// ## Packet format
///
/// * `u16`: transfer ID
/// * 2 bytes padding
/// * `u32`: chunk size
/// * `u16`: chunk count
/// * 2 bytes padding
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct FileTransferChunkParams {
    /// Transfer session ID
    #[brw(pad_after = 2)]
    pub id: u16,

    /// Maximum size of each chunk.
    ///
    /// On the wire, this is a `u32`, but we can't actually fit that many bytes
    /// in the payload (the maximum is [`TransferChunk::MAX_PAYLOAD_LENGTH`]),
    /// so this is coerced into a `u16`.
    #[br(try_map(TryFrom::<u32>::try_from))]
    #[bw(map(|v| u32::from(*v)))]
    pub chunk_size: u16,

    /// Client may send this many chunks before needing to wait for another
    /// [FileTransferChunkParams].
    #[brw(pad_after = 2)]
    pub chunk_count: u16,
}

/// `FTSD`: File Transfer Setup Download
///
/// Used by the client to setup a data upload to the switcher.
///
/// ## Packet format
///
/// * `u16`: transfer ID
/// * `u16`: store ID, (0 = still images)
/// * `u32`: storage index / slot
/// * `u32`: uncompressed payload size
/// * `u8`: storage type
/// * `u8`: possibly RLE status:
///   * `0x00`: default(implicit), macros
///   * `0x01`: still images, labels
/// * 2 bytes padding
#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SetupFileDownload {
    /// Transfer ID
    pub id: u16,
    pub store_id: u16,
    pub index: u32,
    pub size: u32,
    pub typ: FileType,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    #[brw(pad_after = 2)]
    pub is_rle: bool,
}

/// `FTFD`: finish file download
///
/// Used by the client to set file metadata on the switcher for a newly-uploaded
/// file.
///
/// This seems to be able to come before all `FTDa` chunks are sent – the
/// switcher keeps track of how much of the frame it's expecting from the `FTSD`
/// message.
///
/// ## Packet format
///
/// * `u16`: transfer ID
/// * `char[64]`: file name, maybe null terminated
/// * `char[128]`: description, maybe null terminated
/// * `char[16]`: MD5 hash of file
/// * 2 bytes padding
#[binrw]
#[brw(big)]
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct FinishFileDownload {
    /// Transfer ID
    pub id: u16,
    /// Name of the file, up to 64 bytes. This normally doesn't contain a file
    /// extension.
    #[br(try_map = |v: [u8; 64]| str_from_utf8_null(&v).map(str::to_string))]
    #[bw(assert(name.len() <= 64), pad_size_to = 64, map = |v: &String| { v.as_bytes().to_vec() })]
    pub name: String,
    /// Description of the file.
    #[br(try_map = |v: [u8; 128]| str_from_utf8_null(&v).map(str::to_string))]
    #[bw(assert(description.len() <= 128), pad_size_to = 128, map = |v: &String| { v.as_bytes().to_vec() })]
    pub description: String,
    /// MD5 hash of the _uncompressed_ frame.
    ///
    /// The switcher and ATEM's library don't appear to verify this. ATEM
    /// Software Control will use this to determine whether its cache of a
    /// frame should be updated.
    #[brw(pad_after = 2)]
    pub md5: [u8; 16],
}

/// `FTDa`: transfer chunk (`FileTransferData`)
///
/// This represents a chunk of data in a frame transfer. The packets are
/// sequenced by [`AtemPacket::sender_packet_id`][0].
///
/// ## Packet format
///
/// * `u16`: transfer ID
/// * `u16`: chunk length
/// * chunk data
///
/// [0]: crate::packet::AtemPacket::sender_packet_id
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone)]
#[bw(assert(payload.len() <= usize::from(Self::MAX_PAYLOAD_LENGTH)))]
pub struct TransferChunk {
    /// Transfer ID
    pub id: u16,

    #[br(temp, assert(length <= Self::MAX_PAYLOAD_LENGTH))]
    #[bw(try_calc(u16::try_from(payload.len())))]
    length: u16,

    /// Chunk payload. This must be at most [`MAX_PAYLOAD_LENGTH`] bytes.
    ///
    /// RLE-compressed streams must be a multiple of 0x8 bytes; this is not yet
    /// enforced. It's unknown whether this applies to non-RLE streams.
    ///
    /// [`MAX_PAYLOAD_LENGTH`]: TransferChunk::MAX_PAYLOAD_LENGTH
    #[br(count = length)]
    pub payload: Vec<u8>,
}

impl TransferChunk {
    /// Length of [TransferChunk] headers (id + length)
    const HEADERS_LENGTH: u16 = 4;

    /// Maximum [`TransferChunk::payload`] length.
    pub const MAX_PAYLOAD_LENGTH: u16 = (Atom::MAX_PAYLOAD_LENGTH - Self::HEADERS_LENGTH) & !0x7;

    pub fn new(id: u16, capacity: usize) -> Self {
        Self {
            id,
            payload: Vec::with_capacity(capacity),
        }
    }
}

/// `FTUA`: File Transfer Upload Ack
///
/// Acknowledgement of [`TransferChunk`]
///
/// ## Packet format
///
/// * `u16`: transfer ID
/// * 2 bytes padding
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct TransferAck {
    #[brw(pad_after = 2)]
    pub id: u16,
}

/// `FTDC`: File transfer complete
///
/// ## Packet format
///
/// * `u16`: transfer ID
/// * `u16`: unknown - probably uninitialised memory
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct TransferCompleted {
    pub id: u16,
    unknown: u16,
}

/// `FTDE`: file transfer error
///
/// ## Packet format
///
/// * `u16`: transfer ID
/// * `u8`: error code
/// * 1 byte padding
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct FileTransferError {
    /// Transfer ID
    pub id: u16,
    /// Error code
    #[brw(pad_after = 1)]
    pub code: u8,
}

/// `MPfe`: media player frame description
///
/// ## Packet format
///
/// Packets are padded to 4 byte boundaries.
///
/// * `u8`: store ID
/// * 1 byte padding
/// * `u16`: storage index / slot
/// * `u8`: is valid
/// * `char[16]`: MD5 hash of image frame
/// * 1 byte padding
/// * `u16`: image name length
/// * image name
/// * 0 - 3 bytes of padding
#[binrw]
#[brw(big)]
#[derive(Default, PartialEq, Eq, Clone)]
pub struct MediaPlayerFrameDescription {
    #[brw(pad_after = 1)]
    pub store_id: u8,
    pub index: u16,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub is_valid: bool,
    #[brw(pad_after = 1)]
    pub md5: [u8; 16],

    #[br(temp)]
    #[bw(try_calc(u16::try_from(name.len())))]
    name_length: u16,

    #[br(count = name_length, try_map = |v: Vec<u8>| str_from_utf8_null(&v).map(str::to_string))]
    #[bw(map = |v: &String| { v.as_bytes().to_vec() })]
    #[brw(align_after = 4)]
    pub name: String,
}

impl Debug for MediaPlayerFrameDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut b = f.debug_struct("MediaPlayerFrameDescription");

        b.field("store_id", &self.store_id)
            .field("index", &self.index)
            .field("is_valid", &self.is_valid);
        if self.is_valid {
            b.field("md5", &hex::encode(self.md5))
                .field("name", &self.name);
        }
        b.finish()
    }
}

/// `LOCK`: obtain media pool lock
///
/// ## Packet format
///
/// * `u16`: store ID (0 = stills, 1..255 = clip ID)
/// * `bool`: lock state
/// * 1 byte padding
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct MediaPoolLock {
    store_id: u16,
    #[brw(pad_after = 1)]
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    state: bool,
}

impl MediaPoolLock {
    /// Create a new storage lock request.
    #[inline]
    pub const fn lock(store_id: u16) -> Self {
        Self {
            store_id,
            state: true,
        }
    }

    /// Create a new storage unlock request.
    #[inline]
    pub const fn unlock(store_id: u16) -> Self {
        Self {
            store_id,
            state: false,
        }
    }
}

/// `LKOB`: media pool / storage lock obtained
///
/// ## Packet format
///
/// * `u16`: store ID
/// * 2 bytes padding
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct LockObtained {
    #[brw(pad_after = 2)]
    pub store_id: u16,
}

impl LockObtained {
    pub const fn new(store_id: u16) -> Self {
        Self { store_id }
    }
}

/// `LKST`: storage lock state changed
///
/// ## Packet format
///
/// * `u16`: store ID
/// * `u8`: lock state
/// * 1 byte padding
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct MediaPoolLockStatus {
    pub store_id: u16,
    #[brw(pad_after = 1)]
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub locked: bool,
}

#[cfg(test)]
mod test {
    use binrw::{BinRead, BinWrite};
    use std::io::Cursor;

    use super::*;
    use crate::{atom::Payload, Result};

    #[test]
    fn complete_chunk() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        let expected = TransferChunk {
            id: 10420,
            payload: hex::decode("fefefefefefefefe00000000000000783ac803ac3ac803ac3ac69b9f3ac823933ac3cf863ac8677afefefefefefefefe00000000000000763ac1036d3ac8a76d3ac2bb543ac71f3c3ac62f233ac4130bfefefefefefefefe00000000000000763ac99ef23ac102f23ac836e53ac122d93ac56acc3ac166c0fefefefefefefefe00000000000000763ac29eb33ac1a6b33ac4c6673ac4321c3ac915d03ac94585fefefefefefefefe00000000000000763acd65393ace5d393acbfd2c3ace7d203ac931133acec107fefefefefefefefe00000000000000763ac664fa3acf00fa3ac81ce13acd78c93acb90b03aca6c98fefefefefefefefe00000000000000763acf007f3ac75c7f3acd98723ac77c663acacc593ac7c04dfefefefefefefefe00000000000000763ac800403ac80040fefefefefefefefe00000000000000783ac803ac3ac803ac3ac69b9f3ac823933ac3cf863ac8677afefefefefefefefe00000000000000763ac1036d3ac8a76d3ac2bb543ac71f3c3ac62f233ac4130bfefefefefefefefe00000000000000763ac99ef23ac102f23ac836e53ac122d93ac56acc3ac166c0fefefefefefefefe00000000000000763ac29eb33ac1a6b33ac4c6673ac4321c3ac915d03ac94585fefefefefefefefe00000000000000763acd65393ace5d393acbfd2c3ace7d203ac931133acec107fefefefefefefefe00000000000000763ac664fa3acf00fa3ac81ce13acd78c93acb90b03aca6c98fefefefefefefefe00000000000000763acf007f3ac75c7f3acd98723ac77c663acacc593ac7c04dfefefefefefefefe00000000000000763ac800403ac80040fefefefefefefefe00000000000000783ac803ac3ac803ac3ac69b9f3ac823933ac3cf863ac8677afefefefefefefefe00000000000000763ac1036d3ac8a76d3ac2bb543ac71f3c3ac62f233ac4130bfefefefefefefefe00000000000000763ac99ef23ac102f23ac836e53ac122d93ac56acc3ac166c0fefefefefefefefe00000000000000763ac29eb33ac1a6b33ac4c6673ac4321c3ac915d03ac94585fefefefefefefefe00000000000000763acd65393ace5d393acbfd2c3ace7d203ac931133acec107fefefefefefefefe00000000000000763ac664fa3acf00fa3ac81ce13acd78c93acb90b03aca6c98fefefefefefefefe00000000000000763acf007f3ac75c7f3acd98723ac77c663acacc593ac7c04dfefefefefefefefe00000000000000763ac800403ac80040fefefefefefefefe00000000000000783ac803ac3ac803ac3ac69b9f3ac823933ac3cf863ac8677afefefefefefefefe00000000000000763ac1036d3ac8a76d3ac2bb543ac71f3c3ac62f233ac4130bfefefefefefefefe00000000000000763ac99ef23ac102f23ac836e53ac122d93ac56acc3ac166c0fefefefefefefefe00000000000000763ac29eb33ac1a6b33ac4c6673ac4321c3ac915d03ac94585fefefefefefefefe00000000000000763acd65393ace5d393acbfd2c3ace7d203ac931133acec107fefefefefefefefe00000000000000763ac664fa3acf00fa3ac81ce13acd78c93acb90b03aca6c98fefefefefefefefe00000000000000763acf007f3ac75c7f3acd98723ac77c663acacc593ac7c04dfefefefefefefefe00000000000000763ac800403ac80040fefefefefefefefe00000000000000783ac803ac3ac803ac3ac69b9f3ac823933ac3cf863ac8677afefefefefefefefe00000000000000763ac1036d3ac8a76d3ac2bb543ac71f3c3ac62f233ac4130bfefefefefefefefe00000000000000763ac99ef23ac102f23ac836e53ac122d93ac56acc3ac166c0fefefefefefefefe00000000000000763ac29eb33ac1a6b33ac4c6673ac4321c3ac915d03ac94585fefefefefefefefe00000000000000763acd6539")?,
        };
        let cmd: Vec<u8> = hex::decode(
            "058000004654446128b40574fefefefefefefefe00000000000000783ac803ac3ac803ac3ac69b9f3ac823933ac3cf863ac8677afefefefefefefefe00000000000000763ac1036d3ac8a76d3ac2bb543ac71f3c3ac62f233ac4130bfefefefefefefefe00000000000000763ac99ef23ac102f23ac836e53ac122d93ac56acc3ac166c0fefefefefefefefe00000000000000763ac29eb33ac1a6b33ac4c6673ac4321c3ac915d03ac94585fefefefefefefefe00000000000000763acd65393ace5d393acbfd2c3ace7d203ac931133acec107fefefefefefefefe00000000000000763ac664fa3acf00fa3ac81ce13acd78c93acb90b03aca6c98fefefefefefefefe00000000000000763acf007f3ac75c7f3acd98723ac77c663acacc593ac7c04dfefefefefefefefe00000000000000763ac800403ac80040fefefefefefefefe00000000000000783ac803ac3ac803ac3ac69b9f3ac823933ac3cf863ac8677afefefefefefefefe00000000000000763ac1036d3ac8a76d3ac2bb543ac71f3c3ac62f233ac4130bfefefefefefefefe00000000000000763ac99ef23ac102f23ac836e53ac122d93ac56acc3ac166c0fefefefefefefefe00000000000000763ac29eb33ac1a6b33ac4c6673ac4321c3ac915d03ac94585fefefefefefefefe00000000000000763acd65393ace5d393acbfd2c3ace7d203ac931133acec107fefefefefefefefe00000000000000763ac664fa3acf00fa3ac81ce13acd78c93acb90b03aca6c98fefefefefefefefe00000000000000763acf007f3ac75c7f3acd98723ac77c663acacc593ac7c04dfefefefefefefefe00000000000000763ac800403ac80040fefefefefefefefe00000000000000783ac803ac3ac803ac3ac69b9f3ac823933ac3cf863ac8677afefefefefefefefe00000000000000763ac1036d3ac8a76d3ac2bb543ac71f3c3ac62f233ac4130bfefefefefefefefe00000000000000763ac99ef23ac102f23ac836e53ac122d93ac56acc3ac166c0fefefefefefefefe00000000000000763ac29eb33ac1a6b33ac4c6673ac4321c3ac915d03ac94585fefefefefefefefe00000000000000763acd65393ace5d393acbfd2c3ace7d203ac931133acec107fefefefefefefefe00000000000000763ac664fa3acf00fa3ac81ce13acd78c93acb90b03aca6c98fefefefefefefefe00000000000000763acf007f3ac75c7f3acd98723ac77c663acacc593ac7c04dfefefefefefefefe00000000000000763ac800403ac80040fefefefefefefefe00000000000000783ac803ac3ac803ac3ac69b9f3ac823933ac3cf863ac8677afefefefefefefefe00000000000000763ac1036d3ac8a76d3ac2bb543ac71f3c3ac62f233ac4130bfefefefefefefefe00000000000000763ac99ef23ac102f23ac836e53ac122d93ac56acc3ac166c0fefefefefefefefe00000000000000763ac29eb33ac1a6b33ac4c6673ac4321c3ac915d03ac94585fefefefefefefefe00000000000000763acd65393ace5d393acbfd2c3ace7d203ac931133acec107fefefefefefefefe00000000000000763ac664fa3acf00fa3ac81ce13acd78c93acb90b03aca6c98fefefefefefefefe00000000000000763acf007f3ac75c7f3acd98723ac77c663acacc593ac7c04dfefefefefefefefe00000000000000763ac800403ac80040fefefefefefefefe00000000000000783ac803ac3ac803ac3ac69b9f3ac823933ac3cf863ac8677afefefefefefefefe00000000000000763ac1036d3ac8a76d3ac2bb543ac71f3c3ac62f233ac4130bfefefefefefefefe00000000000000763ac99ef23ac102f23ac836e53ac122d93ac56acc3ac166c0fefefefefefefefe00000000000000763ac29eb33ac1a6b33ac4c6673ac4321c3ac915d03ac94585fefefefefefefefe00000000000000763acd6539",
        )?;
        let chunk = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::TransferChunk(chunk) = chunk.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, chunk);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn invalid_chunk() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        // Transfer payload (0xff0) is a multiple of 0x8 bytes
        let mut cmd = hex::decode("0ffc00004654446128b40ff0")?;
        cmd.resize(0xffc, 0);

        // Atom > MAX_COMMAND_LENGTH
        let err = Atom::read(&mut Cursor::new(&cmd)).unwrap_err();
        assert!(matches!(err, binrw::Error::AssertFail { .. }));

        // Parsing with TransferChunk should error more specifically
        let err = TransferChunk::read(&mut Cursor::new(&cmd[8..])).unwrap_err();
        assert!(matches!(err, binrw::Error::AssertFail { .. }));

        // Write paths should also fail
        let bad_chunk = TransferChunk {
            id: 10240,
            payload: vec![0; 0xff0],
        };

        let o = Atom::new(bad_chunk);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        let err = o.write(&mut out).unwrap_err();
        assert!(matches!(err, binrw::Error::AssertFail { .. }));
        Ok(())
    }

    #[test]
    fn transfer_ack() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        let expected = TransferAck { id: 10420 };
        // removed uninitialized memory
        let cmd: Vec<u8> = hex::decode("000c00004654554128b40000")?;
        let ack = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::TransferAck(ack) = ack.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, ack);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn transfer_completed() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        let expected = TransferCompleted {
            id: 10420,
            // probably uninitialised memory
            unknown: 0x103,
        };
        // uninitialized memory removed, grumble grumble
        let cmd: Vec<u8> = hex::decode("000c00004654444328b40103")?;
        let completed = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::TransferCompleted(completed) = completed.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, completed);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn file_transfer_chunk_params() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        let expected = FileTransferChunkParams {
            id: 9183,
            chunk_size: 1396,
            chunk_count: 320,
        };
        // uninitialized memory removed, grumble grumble
        let cmd: Vec<u8> = hex::decode("001400004654434423df00000000057401400000")?;
        let ftcd = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::FileTransferChunkParams(ftcd) = ftcd.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, ftcd);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn setup_file_upload() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        let expected = SetupFileDownload {
            id: 9183,
            store_id: 0,
            index: 2,
            size: 8294400,
            typ: FileType::StillFrame,
            is_rle: true,
        };
        // removed uninitialized memory
        let cmd: Vec<u8> = hex::decode("001800004654534423df000000000002007e900000010000")?;
        let ftsd = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::SetupFileDownload(ftsd) = ftsd.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, ftsd);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn finish_file_upload() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        let expected = FinishFileDownload {
            id: 35522,
            name: "_DSC1248".to_string(),
            description: String::new(),
            md5: hex::decode("61cd8041cf830b9ddff96ee5207c2dc4")?
                .try_into()
                .unwrap(),
        };
        // uninitialized memory removed, grumble grumble
        let cmd: Vec<u8> = hex::decode(concat!(
            "00dc0000",           // length
            "46544644",           // cmd
            "8ac2",               // transfer ID
            "5f4453433132343800", // name: "_DSC1248\0"
            // padding
            "0000000000000000000000000000000000000000000000000000000000000000",
            "0000000000000000000000000000000000000000000000",
            "00", // description: "\0"
            // padding
            "0000000000000000000000000000000000000000000000000000000000000000",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "00000000000000000000000000000000000000000000000000000000000000",
            "61cd8041cf830b9ddff96ee5207c2dc4", // MD5
            // padding
            "0000",
        ))?;
        let ftfd = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::FinishFileDownload(ftfd) = ftfd.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, ftfd);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn media_player_frame_description() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        // All of these samples had uninitialised memory :(
        // Occupied slot
        let expected = MediaPlayerFrameDescription {
            store_id: 0,
            index: 5,
            is_valid: true,
            md5: [
                0xb1, 0xa6, 0x19, 0x4d, 0x4f, 0x52, 0xb4, 0x49, 0xfd, 0x51, 0x98, 0x70, 0xa6, 0x3c,
                0xb3, 0xc2,
            ],
            name: "tram-1080p.rle".to_string(),
        };
        let cmd: Vec<u8> = hex::decode("003000004d5066650000000501b1a6194d4f52b449fd519870a63cb3c200000e7472616d2d31303830702e726c650000")?;
        let mpfe = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::MediaPlayerFrameDescription(mpfe) = mpfe.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, mpfe);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        // Empty slot
        let expected = MediaPlayerFrameDescription {
            store_id: 0,
            index: 2,
            is_valid: false,
            md5: [0; 16],
            name: "".to_string(),
        };
        let cmd: Vec<u8> =
            hex::decode("002000004d506665000000020000000000000000000000000000000000000000")?;
        let mpfe = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::MediaPlayerFrameDescription(mpfe) = mpfe.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, mpfe);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        // Checking word alignment, 1 byte name
        let expected = MediaPlayerFrameDescription {
            store_id: 0,
            index: 0,
            is_valid: true,
            md5: [
                0xb1, 0xa6, 0x19, 0x4d, 0x4f, 0x52, 0xb4, 0x49, 0xfd, 0x51, 0x98, 0x70, 0xa6, 0x3c,
                0xb3, 0xc2,
            ],
            name: "A".to_string(),
        };
        let cmd: Vec<u8> = hex::decode(
            "002400004d5066650000000001b1a6194d4f52b449fd519870a63cb3c200000141000000",
        )?;
        let mpfe = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::MediaPlayerFrameDescription(mpfe) = mpfe.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, mpfe);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        // Checking word alignment, 1 byte name
        let expected = MediaPlayerFrameDescription {
            store_id: 0,
            index: 1,
            is_valid: true,
            md5: [
                0xb1, 0xa6, 0x19, 0x4d, 0x4f, 0x52, 0xb4, 0x49, 0xfd, 0x51, 0x98, 0x70, 0xa6, 0x3c,
                0xb3, 0xc2,
            ],
            name: "AB".to_string(),
        };
        let cmd: Vec<u8> = hex::decode(
            "002400004d5066650000000101b1a6194d4f52b449fd519870a63cb3c200000241420000",
        )?;
        let mpfe = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::MediaPlayerFrameDescription(mpfe) = mpfe.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, mpfe);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        // Checking word alignment, 3 byte name
        let expected = MediaPlayerFrameDescription {
            store_id: 0,
            index: 2,
            is_valid: true,
            md5: [
                0xb1, 0xa6, 0x19, 0x4d, 0x4f, 0x52, 0xb4, 0x49, 0xfd, 0x51, 0x98, 0x70, 0xa6, 0x3c,
                0xb3, 0xc2,
            ],
            name: "ABC".to_string(),
        };
        let cmd: Vec<u8> = hex::decode(
            "002400004d5066650000000201b1a6194d4f52b449fd519870a63cb3c200000341424300",
        )?;
        let mpfe = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::MediaPlayerFrameDescription(mpfe) = mpfe.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, mpfe);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        // Checking word alignment, 4 byte name
        let expected = MediaPlayerFrameDescription {
            store_id: 0,
            index: 3,
            is_valid: true,
            md5: [
                0xb1, 0xa6, 0x19, 0x4d, 0x4f, 0x52, 0xb4, 0x49, 0xfd, 0x51, 0x98, 0x70, 0xa6, 0x3c,
                0xb3, 0xc2,
            ],
            name: "ABCD".to_string(),
        };
        let cmd: Vec<u8> = hex::decode(
            "002400004d5066650000000301b1a6194d4f52b449fd519870a63cb3c200000441424344",
        )?;
        let mpfe = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::MediaPlayerFrameDescription(mpfe) = mpfe.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, mpfe);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        // Checking word alignment, 5 byte name
        let expected = MediaPlayerFrameDescription {
            store_id: 0,
            index: 4,
            is_valid: true,
            md5: [
                0xb1, 0xa6, 0x19, 0x4d, 0x4f, 0x52, 0xb4, 0x49, 0xfd, 0x51, 0x98, 0x70, 0xa6, 0x3c,
                0xb3, 0xc2,
            ],
            name: "ABCDE".to_string(),
        };
        let cmd: Vec<u8> = hex::decode(
            "002800004d5066650000000401b1a6194d4f52b449fd519870a63cb3c20000054142434445000000",
        )?;
        let mpfe = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::MediaPlayerFrameDescription(mpfe) = mpfe.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, mpfe);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        Ok(())
    }

    #[test]
    fn transfer_error() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        let expected = FileTransferError {
            id: 0x0c0c,
            // attempting to download data while storage is not locked
            code: 5,
        };
        let cmd: Vec<u8> = hex::decode("000c0000465444450c0c0500")?;
        let ftde = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::FileTransferError(ftde) = ftde.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, ftde);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn lock() -> Result<()> {
        let expected = MediaPoolLock::lock(0);
        // modified to remove uninitialised memory
        let cmd: Vec<u8> = hex::decode("000c00004c4f434b00000100")?;
        let lock = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::MediaPoolLock(lock) = lock.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, lock);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn lock_obtained() -> Result<()> {
        let expected = LockObtained::new(0);
        // modified to remove uninitialised memory
        let cmd: Vec<u8> = hex::decode("000c00004c4b4f4200000000")?;
        let lkob = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::LockObtained(lkob) = lkob.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, lkob);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn lock_state_changed() -> Result<()> {
        // Still storage locked
        let expected = MediaPoolLockStatus {
            store_id: 0,
            locked: true,
        };
        // modified to remove uninitialised memory
        let cmd: Vec<u8> = hex::decode("000c00004c4b535400000100")?;
        let lkst = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::MediaPoolLockStatus(lkst) = lkst.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, lkst);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        // Still storage unlocked
        let expected = MediaPoolLockStatus {
            store_id: 0,
            locked: false,
        };
        // modified to remove uninitialised memory
        let cmd: Vec<u8> = hex::decode("000c00004c4b535400000000")?;
        let lkst = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::MediaPoolLockStatus(lkst) = lkst.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, lkst);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;

        assert_eq!(cmd, out.into_inner());
        Ok(())
    }
}
