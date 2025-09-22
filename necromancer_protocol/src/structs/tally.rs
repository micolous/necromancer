use binrw::{BinRead, BinWrite};
use modular_bitfield::{bitfield, specifiers::B6};

/// Source tally status.
#[bitfield(bits = 8)]
#[repr(u8)]
#[derive(Specifier, BinRead, BinWrite, Debug, Default, PartialEq, Eq, Clone, Copy)]
#[br(map = From::<u8>::from)]
#[bw(map = |&x| Into::<u8>::into(x))]
pub struct TallyFlags {
    /// The source is currently in use as a program output.
    pub program: bool,
    /// The source is currently in use as a preview output.
    pub preview: bool,
    #[skip]
    __: B6,
}
