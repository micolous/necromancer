use binrw::{binrw, BinRead, BinWrite};
use modular_bitfield::{
    bitfield,
    specifiers::{B2, B4},
};
use std::ops::RangeInclusive;

/// Equaliser frequency band range (`BMDSwitcherFairlightAudioEqualizerBandFrequencyRange`).
///
/// The switcher's frequency limits are defined in
/// [`FairlightEqualiserBandRangeCapabilities`][crate::atom::FairlightEqualiserBandRangeCapabilities].
#[binrw]
#[brw(big, repr = u8)]
#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum EqualiserRange {
    /// Low frequency range
    Low = 0x01,

    /// Mid-low frequency range
    MidLow = 0x02,

    /// Mid-high frequency range
    MidHigh = 0x04,

    /// High frequency range
    High = 0x08,
}

impl EqualiserRange {
    /// Number of supported frequency ranges.
    pub const COUNT: usize = 4;
}

impl From<EqualiserRange> for SupportedEqualiserRanges {
    fn from(value: EqualiserRange) -> Self {
        SupportedEqualiserRanges::from_bytes([value as u8])
    }
}

/// Supported equaliser frequency band ranges.
#[bitfield(bits = 8)]
#[repr(u8)]
#[derive(Specifier, BinRead, BinWrite, Debug, Default, PartialEq, Eq, Clone, Copy)]
#[br(map = From::<u8>::from)]
#[bw(map = |&x| Into::<u8>::into(x))]
pub struct SupportedEqualiserRanges {
    /// Supports [the low frequency range][EqualiserRange::Low].
    pub low: bool,

    /// Supports [the mid-low frequency range][EqualiserRange::MidLow].
    pub mid_low: bool,

    /// Supports [the mid-high frequency range][EqualiserRange::MidHigh].
    pub mid_high: bool,

    /// Supports [the high frequency range][EqualiserRange::High].
    pub high: bool,

    #[skip]
    __: B4,
}

impl SupportedEqualiserRanges {
    /// Returns `true` if this [SupportedEqualiserRanges] contains the given [EqualiserRange].
    pub fn contains(&self, range: EqualiserRange) -> bool {
        self.bytes[0] & (range as u8) != 0
    }
}

/// Equaliser shape (`BMDSwitcherFairlightAudioEqualizerBandShape`)
#[binrw]
#[brw(big, repr = u8)]
#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum EqualiserShape {
    /// Low shelf filter. >─
    LowShelf = 0x01,

    /// Low pass filter. ‾‾╲
    LowPass = 0x02,

    /// Band pass filter. ─<>─
    BandPass = 0x04,

    /// Notch filter. ╶╮╭╴
    Notch = 0x08,

    /// High pass filter. ╱‾‾
    HighPass = 0x10,

    /// High shelf filter. ─<
    HighShelf = 0x20,
}

impl From<EqualiserShape> for SupportedEqualiserShapes {
    fn from(value: EqualiserShape) -> Self {
        SupportedEqualiserShapes::from_bytes([value as u8])
    }
}

/// Supported equaliser shapes.
#[bitfield(bits = 8)]
#[repr(u8)]
#[derive(Specifier, BinRead, BinWrite, Debug, Default, PartialEq, Eq, Clone, Copy)]
#[br(map = From::<u8>::from)]
#[bw(map = |&x| Into::<u8>::into(x))]
pub struct SupportedEqualiserShapes {
    /// Supports [low shelf filter][EqualiserShape::LowShelf].
    pub low_shelf: bool,

    /// Supports [low pass filter][EqualiserShape::LowPass].
    pub low_pass: bool,

    /// Supports [band pass filter][EqualiserShape::BandPass].
    pub band_pass: bool,

    /// Supports [notch filter][EqualiserShape::Notch].
    pub notch: bool,

    /// Supports [high pass filter][EqualiserShape::HighPass].
    pub high_pass: bool,

    /// Supports [high shelf filter][EqualiserShape::HighShelf].
    pub high_shelf: bool,

    #[skip]
    __: B2,
}

impl SupportedEqualiserShapes {
    /// Returns `true` if this [SupportedEqualiserShapes] contains the given [EqualiserShape].
    pub fn contains(&self, range: EqualiserShape) -> bool {
        self.bytes[0] & (range as u8) != 0
    }
}

/// Equaliser band frequency range limits
/// (`BEPStructFairlightEqualiserBandRangeFrequencyLimits`).
///
/// ## Packet format
///
/// * `u8`: [EqualiserRange][] the limits apply to
/// * 3 bytes padding
/// * `u32`: minimum frequency, in hertz
/// * `u32`: maximum frequency, in hertz
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct EqualiserRangeLimit {
    /// Equaliser range the limits apply to
    #[brw(pad_size_to = 4)]
    pub range: EqualiserRange,

    /// Minimum band frequency, in hertz
    pub min_freq: u32,

    /// Maximum band frequency, in hertz
    pub max_freq: u32,
}

impl From<EqualiserRangeLimit> for (EqualiserRange, RangeInclusive<u32>) {
    fn from(value: EqualiserRangeLimit) -> Self {
        (value.range, value.min_freq..=value.max_freq)
    }
}

impl From<EqualiserRangeLimit> for RangeInclusive<u32> {
    fn from(value: EqualiserRangeLimit) -> Self {
        value.min_freq..=value.max_freq
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Result;
    use num_traits::FromPrimitive as _;

    /// [SupportedEqualiserRanges][] and [EqualiserRange][] need to use the same values in their
    /// byte representations.
    #[test]
    fn supported_ranges_consistency() -> Result {
        let v = [
            (
                SupportedEqualiserRanges::new().with_low(true),
                EqualiserRange::Low,
            ),
            (
                SupportedEqualiserRanges::new().with_mid_low(true),
                EqualiserRange::MidLow,
            ),
            (
                SupportedEqualiserRanges::new().with_mid_high(true),
                EqualiserRange::MidHigh,
            ),
            (
                SupportedEqualiserRanges::new().with_high(true),
                EqualiserRange::High,
            ),
        ];

        for (supported_ranges, range) in v {
            let r2sr = SupportedEqualiserRanges::from_bytes([range as u8]);
            assert_eq!(r2sr, supported_ranges);

            let from = SupportedEqualiserRanges::from(range);
            assert_eq!(from, supported_ranges);

            let sr2r = EqualiserRange::from_u8(supported_ranges.into_bytes()[0]);
            assert_eq!(sr2r, Some(range));

            assert!(supported_ranges.contains(range));
        }

        Ok(())
    }

    /// [SupportedEqualiserShapes][] and [EqualiserShape][] need to use the same values in their
    /// byte representations.
    #[test]
    fn supported_shapes_consistency() -> Result {
        let v = [
            (
                SupportedEqualiserShapes::new().with_low_shelf(true),
                EqualiserShape::LowShelf,
            ),
            (
                SupportedEqualiserShapes::new().with_low_pass(true),
                EqualiserShape::LowPass,
            ),
            (
                SupportedEqualiserShapes::new().with_band_pass(true),
                EqualiserShape::BandPass,
            ),
            (
                SupportedEqualiserShapes::new().with_notch(true),
                EqualiserShape::Notch,
            ),
            (
                SupportedEqualiserShapes::new().with_high_pass(true),
                EqualiserShape::HighPass,
            ),
            (
                SupportedEqualiserShapes::new().with_high_shelf(true),
                EqualiserShape::HighShelf,
            ),
        ];

        for (supported_shapes, shape) in v {
            let s2ss = SupportedEqualiserShapes::from_bytes([shape as u8]);
            assert_eq!(s2ss, supported_shapes);

            let from = SupportedEqualiserShapes::from(shape);
            assert_eq!(from, supported_shapes);

            let ss2s = EqualiserShape::from_u8(supported_shapes.into_bytes()[0]);
            assert_eq!(ss2s, Some(shape));

            assert!(supported_shapes.contains(shape));
        }

        Ok(())
    }
}
