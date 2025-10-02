use binrw::binrw;

/// Equaliser frequency band range (`BMDSwitcherFairlightAudioEqualizerBandFrequencyRange`)
#[binrw]
#[brw(big, repr = u8)]
#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum EqualiserRange {
    Low = 0x01,
    MidLow = 0x02,
    MidHigh = 0x04,
    High = 0x08,
}

#[binrw]
#[brw(big, repr = u8)]
#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum EqualiserShape {
    LowShelf = 0x01,
    LowPass = 0x02,
    BandPass = 0x04,
    Notch = 0x08,
    HighPass = 0x10,
    HighShelf = 0x20,
}

/// `BEPStructFairlightEqualiserBandRangeFrequencyLimits`
///
/// ## Packet format
///
/// * `u8`: ???
/// * 3 bytes likely padding
/// * `u32`: minimum frequency, in hertz
/// * `u32`: maximum frequency, in hertz
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FairlightEqualiserBandRangeFrequencyLimits {
    /// Band range
    #[brw(pad_size_to = 4)]
    pub range: EqualiserRange,

    /// Minimum band frequency, in hertz
    pub min_freq: u32,

    /// Maximum band frequency, in hertz
    pub max_freq: u32,
}
