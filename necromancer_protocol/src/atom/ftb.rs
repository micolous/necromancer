//! # Fade to black
//!
//! ## Unimplemented atoms
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `FEna` | `FtbEnabled` | 0xc
use binrw::binrw;

/// `FtbA`: fade to black (auto/transition) (`DoFtbAuto`)
///
/// ## Packet format
///
/// * `u8`: me
/// * 3 bytes padding
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct FadeToBlackAuto {
    #[brw(pad_size_to = 4)]
    pub me: u8,
}

/// `FCut`: cut to black (`DoFtbCut`)
///
/// ## Packet format
///
/// * `u8`: me
/// * `bool`: black
/// * 2 bytes padding
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct CutToBlack {
    pub me: u8,
    #[brw(pad_after = 2)]
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub black: bool,
}

/// `FtbC`: change fade-to-black parameters (`ChangeFtbParameters`)
///
/// ## Packet format
///
/// * `bool`: set rate; if 0, acts as a cut
/// * `u8`: me
/// * `u8`: rate
/// * 1 byte padding
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct SetFadeToBlackParams {
    #[br(temp, map = |v: u8| v != 0)]
    #[bw(calc(rate.is_some()), map = Into::<u8>::into)]
    set_rate: bool,
    pub me: u8,
    #[brw(pad_after = 1)]
    #[br(map = |v: u8| if set_rate { Some(v) } else { None })]
    #[bw(map = |v: &Option<u8>| v.unwrap_or_default())]
    pub rate: Option<u8>,
}

/// `FtbP`: fade-to-black parameter change event (`FtbConfigParameters`)
///
/// ## Packet format
///
/// * `u8`: me
/// * `u8`: rate
/// * 2 bytes padding
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct FadeToBlackParams {
    pub me: u8,
    #[brw(pad_after = 2)]
    pub rate: u8,
}

/// `FtbS`: fade-to-black status event (`FtbCurrentState`)
///
/// ## Packet format
///
/// * `u8`: me
/// * `bool`: fully black
/// * `bool`: in transition
/// * `u8`: frames remaining
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct FadeToBlackStatus {
    pub me: u8,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub fully_black: bool,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub in_transition: bool,
    pub frames_remaining: u8,
}
