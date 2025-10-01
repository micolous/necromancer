//! # ATEM commands / atoms
//!
//! Structures here map to `BEPAtom*` and `BEPStruct*` classes in `BMDSwitcherAPI`.
//!
//! Atoms are grouped into modules by their functional area, and re-exported here.
//!
//! Blackmagic's protocol is based on `repr(C)` of various structs (ie: _not packed_), and the
//! alignment is baked into the protocol in strange ways. Both the ATEM hardware and software
//! (`BMDSwitcherAPI.bundle`) leak uninitialised memory into "padding" fields, making round-trip
//! testing _annoying_. These structures are written in machine-native byte order before being
//! converted to big-endian.
//!
//! A lot of the publicly-documented reverse engineering efforts use slightly different terminology,
//! and are incomplete. This implementation is no different in that regard. ;)
//!
//! `necromancer` will _generally_ favour Blackmagic's terminology (unless it's clunky or wrong),
//! and mention third-party names for things where relevant and accurate.
//!
//! ## Progress
//!
//! In BMDSwitcherAPI 9.8.3: implemented 58 of 253 atoms
//!
//! ## Uncategorised unimplemented atoms
//!
//! <div class="warning">
//!
//! **LIST IS NOT YET COMPLETE**
//!
//! </div>
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `KeOn` | `KeyOnAir` | 0xc
//! `DDsA` | `DoDskAuto` | 0xc
//! `RFIK` | `RunFlyKey` | 0x10
//! `KeFS` | `KeyFlyState` | 0x10
//! `Powr` | `PowerStatus` | 0xc
//! `CDsC` | `ChangeDskCut` | 0xc
//! `CDsT` | `ChangeDskTie` | 0xc
//! `CKeC` | `ChangeKeyCut` | 0xc
//! `RILo` | `RemoteSource` | 0x90
//! `CDsF` | `ChangeDskFill` | 0xc
//! `CDsL` | `ChangeDskLive` | 0xc
//! `CDsM` | `ChangeDskMask` | 0x14
//! `CDsR` | `ChangeDskRate` | 0xc
//! `CKeF` | `ChangeKeyFill` | 0xc
//! `CKMs` | `ChangeKeyMask` | 0x14
//! `CKTp` | `ChangeKeyType` | 0x10
//! `DKOn` | `DelayKeyOnAir` | 0x10
//! `CKOn` | `ChangeKeyOnAir` | 0xc
//! `CLMP` | `ClearMediaPool` | 0x8
//! `RXCE` | `HyperDeckError` | 0xc
//! `RXHI` | `HyperDeckModel` | 0x4c
//! `RXMS` | `HyperDeckSetup` | 0x1c
//! `Warn` | `WarningMessage` | 0x34
//! `_DVE` | `CapabilitiesDVE` | variable
//! `ClrM` | `ColorimetryMode` | 0xc
//! `DskS` | `DskCurrentState` | 0x10
//! `RInL` | `ResetInputLabels` | 0xc
//! `CKfT` | `ChangeKeyTypeFill` | 0x10
//! `DskB` | `DskInputSelection` | 0x10
//! `KeLm` | `KeyLumaParameters` | 0x14
//! `SFKF` | `SetKeyFlyKeyFrame` | 0xc
//! `RXCC` | `HyperDeckClipCount` | 0xc
//! `CXSS` | `HyperDeckSetSource` | 0x10
//! `KeBP` | `KeyBasicParameters` | 0x1c
//! `SPtM` | `SerialPortFunction` | 0xc
//! `DskP` | `DskConfigParameters` | 0x1c
//! `RXCS` | `HyperDeckClipStatus` | 0x5c
//! `Whol` | `IdentityInformation` | 0xb8
//! `KeCk` | `KeyChromaParameters` | 0x14
//! `RFKF` | `ResetKeyFlyKeyFrame` | 0xc
//! `AMPP` | `AudioMixerProperties` | 0xc
//! `CXMS` | `ChangeHyperDeckSetup` | 0x18
//! `KePt` | `KeyPatternParemeters` | 0x18
//! `SKPV` | `SetKeyPatternVelocity` | 0x18
//! `C3sl` | `ChangeSDI3GOutputLevel` | 0xc
//! `RXSS` | `HyperDeckCurrentSource` | 0x28
//! `KeDV` | `KeyDVEAndFlyParameters` | 0x44
//! `RACK` | `ResetKeyAdvancedChroma` | 0xc
//! `V3sl` | `CurrentSDI3GOutputLevel` | 0xc
//! `RXCP` | `HyperDeckClipPlayStatus` | 0x1c
//! `KACC` | `KeyAdvancedChromaCursor` | 0x18
//! `RXML` | `RemoteSourceExternalXML` | 0x40c
//! `SKDV` | `SetKeyDVEAndFlyVelocity` | 0x1c
//! `DcOt` | `CurrentDownConvertedMode` | 0xc
//! `DHVm` | `DownConvertedHDVideoMode` | 0xc
//! `KKFP` | `KeyFlyKeyFrameParemeters` | 0x3c
//! `MMOP` | `MixMinusOutputProperties` | 0x14
//! `RSca` | `RemoteSourceCapabilities` | 0xc
//! `RSDs` | `RemoteSourceDiscoverable` | 0xc
//! `RXHS` | `HyperDeckStorageMediaInfo` | 0x8c
//! `RIMa` | `RemoteSourceExternalAdded` | 0x8c
//! `RIMd` | `RemoteSourceExternalRemove` | 0xc

mod audio;
#[path = "aux_.rs"]
mod aux;
mod camera;
mod colour;
mod cut;
mod fairlight;
mod ftb;
mod initialisation;
mod inpr;
mod macros;
mod media_player;
mod mfg_test;
mod mix_effect;
mod multiview;
mod network;
mod recording;
mod settings;
mod storage;
mod stream;
mod super_source;
mod talkback;
mod tally;
mod time;
mod topology;
mod ver;
mod video_mode;
mod visca;

use crate::{packet::AtemPacket, util::OffsetCounter, Result};
use std::{fmt::Debug, io::SeekFrom};

use binrw::{binrw, helpers::until_eof, io::TakeSeekExt};

pub use self::{
    camera::{CameraCommand, CameraControl},
    colour::{ColourGeneratorParams, SetColourGeneratorParams},
    cut::{Auto, Cut, TransitionPosition},
    fairlight::{FairlightAudioMixerInputSourceProperties, FairlightAudioMixerTally},
    ftb::{
        CutToBlack, FadeToBlackAuto, FadeToBlackParams, FadeToBlackStatus, SetFadeToBlackParams,
    },
    initialisation::InitialisationComplete,
    inpr::InputProperties,
    media_player::{
        CaptureStill, MediaPlayerCapabilities, MediaPlayerSource, MediaPlayerSourceID,
        SetMediaPlayerSource, CAPTURE_STILL,
    },
    mfg_test::{MfgTest, MfgTestResult},
    mix_effect::{
        MixEffectBlockCapabilities, PreviewInput, ProgramInput, SetPreviewInput, SetProgramInput,
    },
    recording::{
        RecordToMedia, RecordToMediaDurationRequest, RecordToMediaRecordingTimecode,
        RecordToMediaStatus,
    },
    settings::{
        ClearSettings, RestoreSettings, SaveSettings, CLEAR_STARTUP_SETTINGS,
        RESTORE_STARTUP_SETTINGS, SAVE_STARTUP_SETTINGS,
    },
    storage::{
        DownloadRequest, FileTransferChunkParams, FileTransferError, FileType, FinishFileUpload,
        LockObtained, MediaPlayerFrameDescription, MediaPoolLock, MediaPoolLockStatus,
        SetupFileUpload, TransferAck, TransferChunk, TransferCompleted,
    },
    tally::TalliedSources,
    time::{SetTimeOfDay, SetTimecodeConfig, Time, TimeMode, TimecodeConfig, TimecodeRequest},
    topology::Topology,
    ver::{ProductName, Version},
    video_mode::{CoreVideoMode, SetVideoMode, SupportedVideoModes},
};

/// Structure for BEP atoms: commands sent by the SDK to the switcher, and events from the
/// switcher sent to the SDK.
///
/// ## Format
///
/// * `u16`: command length
/// * 2 bytes padding
/// * 4 bytes: command identifier
/// * `(command length - 8)` bytes: payload
///
/// The command identifier is parsed as `magic` in [Payload].
#[binrw]
#[derive(Clone, PartialEq, Eq)]
#[brw(big)]
#[bw(stream = r, map_stream = OffsetCounter::new)]
pub struct Atom {
    // Length for the read path
    #[br(temp, pad_after = 2)]
    #[br(assert((Self::HEADERS_LENGTH..=Self::MAX_COMMAND_LENGTH).contains(&length)))]
    #[bw(ignore)]
    length: u16,

    /// Command payload.
    // On read, length includes 4 extra bytes (length field and padding).
    #[br(map_stream = |reader| { reader.take_seek(u64::from(length) - 4) }, pad_size_to = length - 4)]
    // On write, we haven't written the `length` field yet, and we'll come back to it.
    #[bw(pad_before = 4)]
    pub payload: Payload,

    // Length field for the write path
    #[br(ignore)]
    // On write, r.total() includes all headers
    #[bw(assert(r.total() <= (Self::MAX_COMMAND_LENGTH as u64)))]
    #[bw(try_calc(u16::try_from(r.total())))]
    #[bw(seek_before = SeekFrom::Current(-(r.total() as i64)), restore_position)]
    length: u16,
}

impl std::fmt::Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Atom")
            // .field("length", &self.length)
            // .field("id", &self.id.escape_ascii().to_string())
            // .field("payload", &hex::encode(&self.payload))
            .field("payload", &self.payload)
            .finish()
    }
}

#[binrw]
#[brw(big)]
#[derive(Clone, PartialEq, Eq)]
pub enum Payload {
    #[brw(magic = b"DAut")]
    Auto(Auto),
    #[brw(magic = b"CCmd")]
    CameraCommand(CameraCommand),
    #[brw(magic = b"CCdP")]
    CameraControl(CameraControl),
    #[brw(magic = b"Capt")]
    CaptureStill(CaptureStill),
    #[brw(magic = b"SRcl")]
    ClearSettings(ClearSettings),
    #[brw(magic = b"ColV")]
    ColourGeneratorParams(ColourGeneratorParams),
    #[brw(magic = b"DCut")]
    Cut(Cut),
    #[brw(magic = b"FCut")]
    CutToBlack(CutToBlack),
    #[brw(magic = b"FTSU")]
    DownloadRequest(DownloadRequest),
    #[brw(magic = b"FtbA")]
    FadeToBlackAuto(FadeToBlackAuto),
    #[brw(magic = b"FtbC")]
    SetFadeToBlackParams(SetFadeToBlackParams),
    #[brw(magic = b"FtbP")]
    FadeToBlackParams(FadeToBlackParams),
    #[brw(magic = b"FtbS")]
    FadeToBlackStatus(FadeToBlackStatus),
    #[brw(magic = b"FASP")]
    FairlightAudioMixerInputSourceProperties(FairlightAudioMixerInputSourceProperties),
    #[brw(magic = b"FMTl")]
    FairlightAudioMixerTally(FairlightAudioMixerTally),
    #[brw(magic = b"FTCD")]
    FileTransferChunkParams(FileTransferChunkParams),
    #[brw(magic = b"FTFD")]
    FinishFileUpload(FinishFileUpload),
    #[brw(magic = b"VidM")]
    CoreVideoMode(CoreVideoMode),
    #[brw(magic = b"InCm")]
    InitialisationComplete(InitialisationComplete),
    #[brw(magic = b"InPr")]
    InputProperties(InputProperties),
    #[brw(magic = b"LKOB")]
    LockObtained(LockObtained),
    #[brw(magic = b"LOCK")]
    MediaPoolLock(MediaPoolLock),
    #[brw(magic = b"LKST")]
    MediaPoolLockStatus(MediaPoolLockStatus),
    #[brw(magic = b"_mpl")]
    MediaPlayerCapabilities(MediaPlayerCapabilities),
    #[brw(magic = b"MPfe")]
    MediaPlayerFrameDescription(MediaPlayerFrameDescription),
    #[brw(magic = b"MPCE")]
    MediaPlayerSource(MediaPlayerSource),
    #[brw(magic = b"_MeC")]
    MixEffectBlockCapabilities(MixEffectBlockCapabilities),
    #[brw(magic = b"MfgT")]
    MfgTest(MfgTest),
    #[brw(magic = b"MfgR")]
    MfgTestResult(MfgTestResult),
    #[brw(magic = b"PrvI")]
    PreviewInput(PreviewInput),
    #[brw(magic = b"_pin")]
    ProductName(ProductName),
    #[brw(magic = b"PrgI")]
    ProgramInput(ProgramInput),
    #[brw(magic = b"RcTM")]
    RecordToMedia(RecordToMedia),
    #[brw(magic = b"RTMS")]
    RecordToMediaStatus(RecordToMediaStatus),
    #[brw(magic = b"RMDR")]
    RecordToMediaDurationRequest(RecordToMediaDurationRequest),
    #[brw(magic = b"RTMR")]
    RecordToMediaRecordingTimecode(RecordToMediaRecordingTimecode),
    #[brw(magic = b"SRrs")]
    RestoreSettings(RestoreSettings),
    #[brw(magic = b"SRsv")]
    SaveSettings(SaveSettings),
    #[brw(magic = b"CClV")]
    SetColourGeneratorParams(SetColourGeneratorParams),
    #[brw(magic = b"MPSS")]
    SetMediaPlayerSource(SetMediaPlayerSource),
    #[brw(magic = b"CPvI")]
    SetPreviewInput(SetPreviewInput),
    #[brw(magic = b"CPgI")]
    SetProgramInput(SetProgramInput),
    #[brw(magic = b"CTCC")]
    SetTimecodeConfig(SetTimecodeConfig),
    #[brw(magic = b"SToD")]
    SetTimeOfDay(SetTimeOfDay),
    #[brw(magic = b"CVdM")]
    SetVideoMode(SetVideoMode),
    #[brw(magic = b"FTSD")]
    SetupFileUpload(SetupFileUpload),
    #[brw(magic = b"_VMC")]
    SupportedVideoModes(SupportedVideoModes),
    #[brw(magic = b"TlSr")]
    TalliedSources(TalliedSources),
    #[brw(magic = b"Time")]
    Time(Time),
    #[brw(magic = b"TCCc")]
    TimecodeConfig(TimecodeConfig),
    #[brw(magic = b"TiRq")]
    TimecodeRequest(TimecodeRequest),
    #[brw(magic = b"_top")]
    Topology(Topology),
    #[brw(magic = b"FTUA")]
    TransferAck(TransferAck),
    #[brw(magic = b"FTDa")]
    TransferChunk(TransferChunk),
    #[brw(magic = b"FTDC")]
    TransferCompleted(TransferCompleted),
    #[brw(magic = b"FTDE")]
    FileTransferError(FileTransferError),
    #[brw(magic = b"TrPs")]
    TransitionPosition(TransitionPosition),
    #[brw(magic = b"_ver")]
    Version(Version),
    Unknown([u8; 4], #[br(parse_with = until_eof)] Vec<u8>),
}

macro_rules! atom_payloads {
    (
        $($variant:ident,)*
    ) => {
        $(
            impl From<$variant> for Payload {
                fn from(p: $variant) -> Payload {
                    Payload::$variant(p)
                }
            }
        )*

        impl Debug for Payload {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant(v) => f
                            .debug_tuple(stringify!($variant))
                            .field(v)
                            .finish(),
                    )*
                    Self::Unknown(cmd, payload) => f
                        .debug_tuple("Unknown")
                        .field(&hex::encode(cmd))
                        .field(&hex::encode(payload))
                        .finish(),
                }
            }
        }
    }
}

atom_payloads!(
    Auto,
    CameraCommand,
    CameraControl,
    CaptureStill,
    ClearSettings,
    ColourGeneratorParams,
    Cut,
    CutToBlack,
    DownloadRequest,
    FadeToBlackAuto,
    SetFadeToBlackParams,
    FadeToBlackParams,
    FadeToBlackStatus,
    FairlightAudioMixerInputSourceProperties,
    FairlightAudioMixerTally,
    FileTransferChunkParams,
    FinishFileUpload,
    CoreVideoMode,
    InitialisationComplete,
    InputProperties,
    LockObtained,
    MediaPoolLock,
    MediaPoolLockStatus,
    MediaPlayerCapabilities,
    MediaPlayerFrameDescription,
    MediaPlayerSource,
    MixEffectBlockCapabilities,
    MfgTest,
    MfgTestResult,
    PreviewInput,
    ProductName,
    ProgramInput,
    RecordToMedia,
    RecordToMediaDurationRequest,
    RecordToMediaRecordingTimecode,
    RecordToMediaStatus,
    RestoreSettings,
    SaveSettings,
    SetColourGeneratorParams,
    SetMediaPlayerSource,
    SetPreviewInput,
    SetProgramInput,
    SetTimecodeConfig,
    SetTimeOfDay,
    SetupFileUpload,
    SetVideoMode,
    SupportedVideoModes,
    TalliedSources,
    Time,
    TimecodeConfig,
    TimecodeRequest,
    Topology,
    TransferAck,
    TransferChunk,
    TransferCompleted,
    FileTransferError,
    TransitionPosition,
    Version,
);

impl Atom {
    /// Minimum size of an [Atom], including all headers (length + padding + magic).
    const HEADERS_LENGTH: u16 = 8;

    /// Maximum size of an [Atom], including all headers (length + padding + magic).
    const MAX_COMMAND_LENGTH: u16 = AtemPacket::MAX_PAYLOAD_LENGTH;

    /// Maximum command payload size (minus [Atom] headers).
    const MAX_PAYLOAD_LENGTH: u16 = Self::MAX_COMMAND_LENGTH - Self::HEADERS_LENGTH;

    pub fn new(payload: impl Into<Payload>) -> Self {
        Self {
            payload: payload.into(),
        }
    }
}

/// Parses a byte slice as an _optionally_-null-terminated, UTF-8-encoded
/// string, ignoring all bytes after the first null.
///
/// Returns [`Error::Utf8`][1] on UTF-8 encoding errors.
///
/// This is similar to [`CStr::from_bytes_until_nul`][0], but does not require a
/// null terminator.
///
/// [0]: std::ffi::CStr::from_bytes_until_nul
/// [1]: crate::Error::Utf8
fn str_from_utf8_null(p: &[u8]) -> Result<&str> {
    let p = p.split(|c| *c == 0).next().unwrap_or(p);
    Ok(std::str::from_utf8(p)?)
}
