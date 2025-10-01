//! Camera control commands
//!
//! This is similar to the "Abstract Packet Format" described in
//! [Blackmagic SDI Camera Control Protocol v1.3][bmsdi].
//!
//! The general value format is as follows:
//!
//! * `uint8`: data type
//! * padding to align to a 2 byte boundary of the command (varies)
//! * `uint16`: number of values/bytes, for `boolean`, `byte` and `utf8` types
//! * `uint16`: number of values, for `int16` and `fixed5.11` types
//! * `uint16`: number of values, for `int32` data type
//! * `uint16`: number of values, for `int64` data type
//! * padding to align to an 8 byte boundary of the command (varies)
//! * data payload
//! * padding:
//!   * if `CCdP`: align to 8 byte boundary
//!   * if `CCmd`: `0.max(20 - payload.len())` bytes
//!
//! *All* length fields are *always* included. When longer types are used, the
//! shorter types *can* be used for some offset to the payload, but this doesn't
//! seem to be used by the switcher, and doesn't make a lot of sense. Have a
//! look at `AtomCameraControlCommand::GetPtr_data{32,64}` in a
//! disassembler and you'll see what I mean. :)
//!
//! ## Unimplemented atoms
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `CCdo` | `CameraControlCommandOptions` |
//! `CCts` | `CameraControlSettings` | 0xc
//! `InMp` | `InputCameraModel` | 0x10
//!
//! [bmsdi]: https://documents.blackmagicdesign.com/DeveloperManuals/BlackmagicCameraControl.pdf
use crate::{Error, Result};
use binrw::binrw;
use fixed::types::I5F11;
use std::ops::{Deref, DerefMut};

/// Container type for [CameraParameterValue].
///
/// This bundles up the type and length fields before passing them down to the
/// inner [CameraParameterValue].
///
/// [CameraParameterValueContainer] implements `From<CameraParameterValue>`,
/// `Deref<Target = CameraParameterValue>` and `DerefMut`, so you should prefer
/// using that type.
#[binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
#[brw(big)]
struct CameraParameterValueContainer {
    #[br(temp)]
    #[bw(calc = value.typ())]
    typ: u8,

    #[brw(align_before = 2)]
    #[br(temp)]
    #[bw(try_calc = value.len8())]
    len8: u16,

    #[br(temp)]
    #[bw(try_calc = value.len16())]
    len16: u16,

    #[br(temp)]
    #[bw(try_calc = value.len32())]
    len32: u16,

    #[br(temp)]
    #[bw(try_calc = value.len64())]
    len64: u16,

    #[brw(align_before = 8)]
    #[br(args { typ: typ, len8: len8, len16: len16, len32: len32, len64: len64 })]
    value: CameraParameterValue,
}

/// Camera Parameter Value, used by `CCdP` and `CCmd` messages.
///
/// **Note:** while this implements `BinRead` and `BinWrite`, this needs to be
/// mapped to [CameraParameterValueContainer].
#[binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
#[br(import { typ: u8, len8: u16, len16: u16, len32: u16, len64: u16 })]
pub enum CameraParameterValue {
    /// Boolean or void type.
    ///
    /// When the inner `Vec` is empty, this is a void type.
    #[br(pre_assert(typ == 0))]
    Bool(
        #[br(count = len8, map = |v: Vec<u8>| v.into_iter().map(|i| i != 0).collect())]
        #[bw(map = |v: &Vec<bool>| v.iter().map(|i| Into::<u8>::into(*i)).collect::<Vec<u8>>())]
        Vec<bool>,
    ),

    #[br(pre_assert(typ == 1))]
    I8(#[br(count = len8)] Vec<i8>),

    #[br(pre_assert(typ == 2))]
    I16(#[br(count = len16)] Vec<i16>),

    #[br(pre_assert(typ == 3))]
    I32(#[br(count = len32)] Vec<i32>),

    #[br(pre_assert(typ == 4))]
    I64(#[br(count = len64)] Vec<i64>),

    // todo: string
    #[br(pre_assert(typ == 128))]
    I5F11(
        #[br(count = len16, map = |v: Vec<i16>| v.into_iter().map(I5F11::from_bits).collect())]
        #[bw(map = |v: &Vec<I5F11>| v.iter().map(|i| i.to_bits()).collect::<Vec<i16>>())]
        Vec<I5F11>,
    ),
}

impl CameraParameterValue {
    const fn typ(&self) -> u8 {
        use CameraParameterValue::*;
        match self {
            Bool(_) => 0,
            I8(_) => 1,
            I16(_) => 2,
            I32(_) => 3,
            I64(_) => 4,
            I5F11(_) => 128,
        }
    }

    fn len8(&self) -> Result<u16> {
        use CameraParameterValue::*;
        match self {
            Bool(i) => i.len().try_into(),
            I8(i) => i.len().try_into(),
            _ => Ok(0),
        }
        .map_err(|_| Error::InvalidLength)
    }

    fn len16(&self) -> Result<u16> {
        use CameraParameterValue::*;
        match self {
            I16(i) => i.len().try_into(),
            I5F11(i) => i.len().try_into(),
            _ => Ok(0),
        }
        .map_err(|_| Error::InvalidLength)
    }

    fn len32(&self) -> Result<u16> {
        if let CameraParameterValue::I32(i) = self {
            i.len().try_into().map_err(|_| Error::InvalidLength)
        } else {
            Ok(0)
        }
    }

    fn len64(&self) -> Result<u16> {
        if let CameraParameterValue::I64(i) = self {
            i.len().try_into().map_err(|_| Error::InvalidLength)
        } else {
            Ok(0)
        }
    }

    /// Shortcut to [Clone] and convert `&CameraParameterValue` to
    /// [CameraParameterValueContainer] for `#[bw(map())]`.
    fn as_container(&self) -> CameraParameterValueContainer {
        self.clone().into()
    }
}

impl From<CameraParameterValue> for CameraParameterValueContainer {
    fn from(value: CameraParameterValue) -> Self {
        Self { value }
    }
}

impl From<CameraParameterValueContainer> for CameraParameterValue {
    fn from(value: CameraParameterValueContainer) -> Self {
        value.value
    }
}

impl Deref for CameraParameterValueContainer {
    type Target = CameraParameterValue;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for CameraParameterValueContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

// TODO: make this a trait or something
macro_rules! to_camera_parameter_value {
    (
        $variant:ident, $inner_type:ty
    ) => {
        impl From<Vec<$inner_type>> for CameraParameterValue {
            fn from(v: Vec<$inner_type>) -> Self {
                Self::$variant(v)
            }
        }
    };
}

to_camera_parameter_value!(Bool, bool);
to_camera_parameter_value!(I8, i8);
to_camera_parameter_value!(I16, i16);
to_camera_parameter_value!(I32, i32);
to_camera_parameter_value!(I64, i64);
to_camera_parameter_value!(I5F11, I5F11);

// Here be dragons:
// This uses the naming/numbering according to BM's SDI protocol docs
// https://documents.blackmagicdesign.com/UserManuals/ATEM_Production_Studio_Switchers_Manual.pdf
//
// This is different to a lot of the reverse engineered docs
#[binrw]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[brw(big)]
pub enum CameraParameterID {
    #[brw(magic = 0u8)]
    Lens(LensParam),
    #[brw(magic = 1u8)]
    Video(VideoParam),
    #[brw(magic = 2u8)]
    Audio(AudioParam),
    #[brw(magic = 3u8)]
    Output(OutputParam),
    #[brw(magic = 4u8)]
    Display(DisplayParam),
    #[brw(magic = 5u8)]
    Tally(TallyParam),
    #[brw(magic = 6u8)]
    Reference(ReferenceParam),
    #[brw(magic = 7u8)]
    Config(ConfigParam),
    #[brw(magic = 8u8)]
    ColourCorrection(ColourCorrectionParam),
    #[brw(magic = 10u8)]
    Media(MediaParam),
    #[brw(magic = 11u8)]
    PtzControl(PtzControlParam),
    Unknown(u8, u8),
}

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum LensParam {
    Focus = 0,
    AutoFocus = 1,
    ApertureFStop = 2,
    ApertureNormalised = 3,
    ApertureOrdinal = 4,
    AutoAperture = 5,
    OpticalImageStabilisation = 6,
    ZoomMillimetres = 7,
    ZoomNormalised = 8,
    ZoomSpeed = 9,
}

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum VideoParam {
    VideoMode = 0,
    GainISO = 1,
    ManualWhiteBalance = 2,
    SetAutoWhiteBalance = 3,
    RestoreAutoWhiteBalance = 4,
    ExposureMicroSeconds = 5,
    ExposureOrdinal = 6,
    DynamicRangeMode = 7,
    VideoSharpeningLevel = 8,
    RecordingFormat = 9,
    SetAutoExposure = 10,
    ShutterAngle = 11,
    ShutterSpeed = 12,
    GainDb = 13,
    CameraIso = 14,
}

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum AudioParam {
    MicLevel = 0,
    HeadphoneLevel = 1,
    HeadphoneProgramMix = 2,
    SpeakerLevel = 3,
    InputType = 4,
    InputLevels = 5,
    PhantomPower = 6,
}

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputParam {
    OverlayEnables = 0,
    FrameGuidesStyle3 = 1,
    FrameGuidesOpacity3 = 2,
    FrameOverlays4 = 3,
}

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum DisplayParam {
    Brightness = 0,
    OverlayEnables = 1,
    ZebraLevel = 2,
    PeakingLevel = 3,
    ColourBarsDisplayTime = 4,
    FocusAssist = 5,
}

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
#[allow(clippy::enum_variant_names)]
pub enum TallyParam {
    TallyBrightness = 0,
    FrontTallyBrightness = 1,
    RearTallyBrightness = 2,
}

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum ReferenceParam {
    Source = 0,
    Offset = 1,
}

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum ConfigParam {
    RealTimeClock = 0,
    SystemLanguage = 1,
    Timezone = 2,
    Location = 3,
}

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum ColourCorrectionParam {
    LiftAdjust = 0,
    GammaAdjust = 1,
    GainAdjust = 2,
    OffsetAdjust = 3,
    ContrastAdjust = 4,
    LumaMix = 5,
    ColourAdjust = 6,
    CorrectionResetDefault = 7,
}

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum MediaParam {
    Codec = 0,
    TransportMode = 1,
}

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum PtzControlParam {
    PanTiltVelocity = 0,
    MemoryPreset = 1,
}

/// `CCdP`: Camera Control Command Properties
///
/// ## Format
///
/// * `u8`: input source
/// * `u8[2]`: parameter ID
/// * parameter value
#[binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
#[brw(big)]
pub struct CameraControl {
    pub input: u8,
    pub parameter: CameraParameterID,
    #[brw(align_after = 8)]
    #[br(map(|v: CameraParameterValueContainer| v.into()))]
    #[bw(map(CameraParameterValue::as_container))]
    pub value: CameraParameterValue,
}

/// `CCmd`: Camera Control Command (change parameter)
///
/// ## Format
///
/// * `u8`: input
/// * `u8[2]`: parameter ID
/// * `bool`: relative mode (true) or absolute (false)
/// * value
#[binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
#[brw(big)]
pub struct CameraCommand {
    pub input: u8,
    pub parameter: CameraParameterID,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub relative: bool,

    #[brw(pad_size_to = 20)]
    #[br(map(|v: CameraParameterValueContainer| v.into()))]
    #[bw(map(CameraParameterValue::as_container))]
    pub value: CameraParameterValue,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        atom::{Atom, Payload},
        packet::AtemPacket,
        Result,
    };
    use binrw::{BinRead, BinWrite};
    use fixed::types::I5F11;
    use std::io::Cursor;

    #[test]
    fn white_balance() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        let expected_pl = CameraControl {
            input: 1,
            parameter: CameraParameterID::Video(VideoParam::ManualWhiteBalance),
            value: CameraParameterValue::I16(vec![5600, 0]),
        };

        let cmd = hex::decode("00200000434364500101020200000002000000000000000015e0000000000000")?;
        let ccdp = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::CameraControl(pl) = ccdp.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected_pl, pl);

        // re-encode from sources
        let o = Atom::new(pl);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        // re-encode from scratch
        let o = Atom::new(expected_pl);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;

        assert_eq!(cmd, out.into_inner());

        Ok(())
    }

    #[test]
    fn auto_focus() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        let expected_pl = CameraControl {
            input: 1,
            parameter: CameraParameterID::Lens(LensParam::AutoFocus),
            value: CameraParameterValue::Bool(vec![]),
        };

        let cmd = hex::decode("001800004343645001000100000000000000000000000000")?;
        let ccdp = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::CameraControl(pl) = ccdp.payload else {
            panic!("wrong command type");
        };

        assert_eq!(expected_pl, pl);

        // re-encode from sources
        let o = Atom::new(pl);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        // re-encode from scratch
        let o = Atom::new(expected_pl);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;

        assert_eq!(cmd, out.into_inner());

        Ok(())
    }

    #[test]
    fn gain() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        let f2048 = I5F11::from_num(1);
        let expected_pl = CameraControl {
            input: 1,
            parameter: CameraParameterID::ColourCorrection(ColourCorrectionParam::GainAdjust),
            value: CameraParameterValue::I5F11(vec![f2048, f2048, f2048, f2048]),
        };

        // Removed uninitialised memory
        let cmd = hex::decode("0020000043436450010802800000000400000000000000000800080008000800")?;
        let ccdp = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::CameraControl(pl) = ccdp.payload else {
            panic!("wrong command type");
        };

        assert_eq!(expected_pl, pl);

        // re-encode from sources
        let o = Atom::new(pl);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        // re-encode from scratch
        let o = Atom::new(expected_pl);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;

        assert_eq!(cmd, out.into_inner());

        Ok(())
    }

    #[test]
    fn luminance() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        let f2048 = I5F11::from_num(1);
        let expected_pl = CameraControl {
            input: 1,
            parameter: CameraParameterID::ColourCorrection(ColourCorrectionParam::LumaMix),
            value: CameraParameterValue::I5F11(vec![f2048]),
        };

        // Removed uninitialised memory
        let cmd = hex::decode("0020000043436450010805800000000100000000000000000800000000000000")?;
        let ccdp = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::CameraControl(pl) = ccdp.payload else {
            panic!("wrong command type");
        };

        assert_eq!(expected_pl, pl);

        // re-encode from sources
        let o = Atom::new(pl);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        // re-encode from scratch
        let o = Atom::new(expected_pl);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;

        assert_eq!(cmd, out.into_inner());

        Ok(())
    }

    #[test]
    fn ccdp() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();

        let cmd = hex::decode("8d8c80010004000000000009002000004343645001000080000000010000000000000000000000000000000000200000434364500100028000000001000000000000000028000000000000000020000043436450010110800000000100000000010000000000000000000000002000004343645001010d010001000000000000010000000000000000000000002000004343645001010101000100000000000001000000020000000000000000200000434364500101020200000002000000000100000015e000000000000000200000434364500101050300000000000100000100000000004e20000000000020000043436450010108010001000000000000010000000100000000000000002000004343645001040401000100000000000001000000000000000000000000200000434364500108008000000004000000000100000000000000000000000020000043436450010801800000000400000000010000000000000000000000002000004343645001080280000000040000000001000000080008000800080000200000434364500108038000000004000000000100000000000000000000000020000043436450010804800000000200000000010000000400080000000000002000004343645001080580000000010000000001000000080000000000000000200000434364500108068000000002000000000100000000000800000000000020000043436450010b00800000000200000000000000000000000000000000002000004343645002000080000000010000000000000000000000000000000000200000434364500200028000000001000000000000000028000000000000000020000043436450020110800000000100000000010000000000000000000000002000004343645002010d010001000000000000010000000000000000000000002000004343645002010101000100000000000001000000020000000000000000200000434364500201020200000002000000000100000015e000000000000000200000434364500201050300000000000100000100000000004e20000000000020000043436450020108010001000000000000010000000100000000000000002000004343645002040401000100000000000001000000000000000000000000200000434364500208008000000004000000000100000000000000000000000020000043436450020801800000000400000000010000000000000000000000002000004343645002080280000000040000000001000000080008000800080000200000434364500208038000000004000000000100000000000000000000000020000043436450020804800000000200000000010000000400080000000000002000004343645002080580000000010000000001000000080000000000000000200000434364500208068000000002000000000100000000000800000000000020000043436450020b00800000000200000000000000000000000000000000002000004343645003000080000000010000000000000000000000000000000000200000434364500300028000000001000000000000000028000000000000000020000043436450030110800000000100000000010000000000000000000000002000004343645003010d010001000000000000010000000000000000000000002000004343645003010101000100000000000001000000020000000000000000200000434364500301020200000002000000000100000015e000000000000000200000434364500301050300000000000100000100000000004e2000000000002000004343645003010801000100000000000001000000010000000000000000200000434364500304040100010000000000000100000000000000000000000020000043436450030800800000000400000000010000000000000000000000").unwrap();
        let pkt = AtemPacket::read(&mut Cursor::new(&cmd))?;

        let cmds = pkt.atoms().expect("wrong payload type");

        assert_eq!(44, cmds.len());
        // println!("commands:");
        // for cmd in cmds.iter() {
        //     let Payload::CameraControl(pl) = &cmd.payload else {
        //         continue;
        //     };
        //     println!("cmd: {:?}", pl);
        // }

        // Re-encode
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        pkt.write(&mut out)?;

        // Because the original had uninitialised memory, just try to read it again
        out.set_position(0);
        let pkt2 = AtemPacket::read(&mut out)?;
        assert_eq!(pkt, pkt2);

        Ok(())
    }

    #[test]
    fn colour_bar_display_time() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();
        let expected = CameraCommand {
            input: 1,
            parameter: CameraParameterID::Display(DisplayParam::ColourBarsDisplayTime),
            relative: false,
            value: CameraParameterValue::I8(vec![0]),
        };
        let cmd = hex::decode("0020000043436d64010404000100000100000000000000000000000000000000")?;
        let cam = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::CameraCommand(cam) = cam.payload else {
            panic!("unexpected payload");
        };

        assert_eq!(expected, cam);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn shutter_speed() -> Result<()> {
        let _ = tracing_subscriber::fmt().try_init();

        let expected = CameraCommand {
            input: 1,
            parameter: CameraParameterID::Video(VideoParam::ExposureMicroSeconds),
            relative: false,
            value: CameraParameterValue::I32(vec![20000]),
        };
        // Contains extra padding
        let cmd = hex::decode("0020000043436d640101050003000000000000010000000000004e2000000000")?;
        let cam = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::CameraCommand(cam) = cam.payload else {
            panic!("unexpected payload");
        };

        assert_eq!(expected, cam);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn ccmd() -> Result<()> {
        // TODO: test just CCmd, and add in expected values
        let _ = tracing_subscriber::fmt().try_init();

        // Contains uninitialised memory
        let cmd = hex::decode("0c2c800100000000000100020020000043436d640100000180000000000100000000000000000000000000000020000043436d640100020080000000000100000000000028000000000000000020000043436d640101100080000000000100000000000000000000000000000020000043436d6401010d0001000001000000000000000000000000000000000020000043436d640101010001000001000000000000000002000000000000000020000043436d640101020002000000000200000000000015e00000000000000020000043436d640101050003000000000000010000000000004e20000000000020000043436d640101080001000001000000000000000001000000000000000020000043436d640104040001000001000000000000000000000000000000000020000043436d640108000080000000000400000000000000000000000000000020000043436d640108010080000000000400000000000000000000000000000020000043436d640108020080000000000400000000000008000800080008000020000043436d640108030080000000000400000000000000000000000000000020000043436d640108040080000000000200000000000004000800000000000020000043436d640108050080000000000100000000000008000000000000000020000043436d640108060080000000000200000000000000000800000000000020000043436d64010b00008000000000020000000000000000000000000000001000004343646f0101010d01000000001000004343646f0101010101000000001000004343646f0101011001000000001000004343646f0101010201000000001000004343646f0101010501000000001000004343646f0101010801000000001000004343646f0101040401000000001000004343646f0101080001000000001000004343646f0101080101000000001000004343646f0101080201000000001000004343646f0101080301000000001000004343646f0101080401000000001000004343646f0101080501000000001000004343646f01010806017f00000020000043436d640200000180000000000100000000000000000000000000000020000043436d640200020080000000000100000000000028000000000000000020000043436d640201100080000000000100000000000000000000000000000020000043436d6402010d0001000001000000000000000000000000000000000020000043436d640201010001000001000000000000000002000000000000000020000043436d640201020002000000000200000000000015e00000000000000020000043436d640201050003000000000000010000000000004e20000000000020000043436d640201080001000001000000000000000001000000000000000020000043436d64020404000100000100000000000000000000000000000000")?;
        let pkt = AtemPacket::read(&mut Cursor::new(&cmd))?;
        let cmds = pkt.atoms().expect("wrong payload type");

        // This ensures that the alignment rules for CameraCommand and
        // CameraParameterValue are working.
        assert_eq!(40, cmds.len());

        // println!("commands:");
        // for cmd in cmds.iter() {
        //     println!("cmd: {cmd:?}");
        // }

        // Re-encode
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        pkt.write(&mut out)?;

        // Because the original had uninitialised memory, just try to read it again
        out.set_position(0);
        let pkt2 = AtemPacket::read(&mut out)?;
        assert_eq!(pkt, pkt2);
        Ok(())
    }
}
