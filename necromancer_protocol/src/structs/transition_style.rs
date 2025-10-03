use binrw::binrw;

#[binrw]
#[brw(big, repr = u8)]
#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DVETransitionStyle {
    SwooshTopLeft = 0x0,
    SwooshTop = 0x1,
    SwooshTopRight = 0x2,
    SwooshLeft = 0x3,
    SwooshRight = 0x4,
    SwooshBottomLeft = 0x5,
    SwooshBottom = 0x6,
    SwooshBottomRight = 0x7,

    SpinCWTopLeft = 0x8,
    SpinCWTopRigtt = 0x9,
    SpinCWBottomLeft = 0xa,
    SpinCWBottomRight = 0xb,
    SpinCCWTopLeft = 0xc,
    SpinCCWTopRigtt = 0xd,
    SpinCCWBottomLeft = 0xe,
    SpinCCWBottomRight = 0xf,

    SqueezeTopLeft = 0x10,
    SqueezeTop = 0x11,
    SqueezeTopRight = 0x12,
    SqueezeLeft = 0x13,
    SqueezeRight = 0x14,
    SqueezeBottomLeft = 0x15,
    SqueezeBottom = 0x16,
    SqueezeBottomRight = 0x17,

    PushTopLeft = 0x18,
    PushTop = 0x19,
    PushTopRight = 0x1a,
    PushLeft = 0x1b,
    PushRight = 0x1c,
    PushBottomLeft = 0x1d,
    PushBottom = 0x1e,
    PushBottomRight = 0x1f,

    GraphicCWSpin = 0x20,
    GraphicCCVSpin = 0x21,
    GraphicLogoWipe = 0x22,
}
