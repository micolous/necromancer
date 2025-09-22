use binrw::binrw;
#[cfg(feature = "clap")]
use clap::ValueEnum;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Video input source.
///
/// Not all inputs are available on all switchers.
#[binrw]
#[brw(repr = u16, big)]
#[derive(Default, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum VideoSource {
    Black = 0,

    Input1 = 1,
    Input2 = 2,
    Input3 = 3,
    Input4 = 4,
    Input5 = 5,
    Input6 = 6,
    Input7 = 7,
    Input8 = 8,
    Input9 = 9,
    Input10 = 10,
    Input11 = 11,
    Input12 = 12,
    Input13 = 13,
    Input14 = 14,
    Input15 = 15,
    Input16 = 16,
    Input17 = 17,
    Input18 = 18,
    Input19 = 19,
    Input20 = 20,
    Input21 = 21,
    Input22 = 22,
    Input23 = 23,
    Input24 = 24,
    Input25 = 25,
    Input26 = 26,
    Input27 = 27,
    Input28 = 28,
    Input29 = 29,
    Input30 = 30,
    Input31 = 31,
    Input32 = 32,
    Input33 = 33,
    Input34 = 34,
    Input35 = 35,
    Input36 = 36,
    Input37 = 37,
    Input38 = 38,
    Input39 = 39,
    Input40 = 40,

    ColourBars = 1000,
    Colour1 = 2001,
    Colour2 = 2002,
    // TODO: Colour3 - Colour8 haven't actually been seen, these are just a
    // guess.
    Colour3 = 2003,
    Colour4 = 2004,
    Colour5 = 2005,
    Colour6 = 2006,
    Colour7 = 2007,
    Colour8 = 2008,

    MediaPlayer1 = 3010,
    MediaPlayer1Key = 3011,
    MediaPlayer2 = 3020,
    MediaPlayer2Key = 3021,
    MediaPlayer3 = 3030,
    MediaPlayer3Key = 3031,
    MediaPlayer4 = 3040,
    MediaPlayer4Key = 3041,

    Key1Mask = 4010,
    Key2Mask = 4020,
    Key3Mask = 4030,
    Key4Mask = 4040,

    DSK1Mask = 5010,
    DSK2Mask = 5020,

    SuperSource = 6000,

    CleanFeed1 = 7001,
    CleanFeed2 = 7002,

    Auxilary1 = 8001,
    Auxilary2 = 8002,
    Auxilary3 = 8003,
    Auxilary4 = 8004,
    Auxilary5 = 8005,
    Auxilary6 = 8006,

    ME1Prog = 10010,
    ME1Prev = 10011,
    ME2Prog = 10020,
    ME2Prev = 10021,

    Input1Direct = 11001,
    /// Internal value: unknown video source state.
    #[default]
    Unknown = 0xffff,
}
