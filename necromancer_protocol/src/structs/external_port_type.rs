use binrw::{BinRead, BinWrite};
use modular_bitfield::{bitfield, specifiers::B4};

/// The external port type of the video switcher.
///
/// ## Format
///
/// The value is a bitmask, and uses the network port type numbering, which
/// is different from `BMDSwitcherExternalPortType` in the official SDK:
///
/// API value | Network value | Notes
/// --------- | ------------- | -----------
/// `0x0e1f`  | `0x0e1f`      | Same in both
/// `0x01c0`  | `0x00e0`      | `API = Network << 1`
/// `0x0020`  | `0x0100`      | `API = Network >> 3`
/// `0xf000`  | `0xf000`      | unused[^1]
///
/// [^1]: `0x1000` (RJ45) is unused by the BM SDK.
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Specifier, BinRead, BinWrite, Debug, Default, PartialEq, Eq, Clone, Copy)]
#[br(map = From::<u16>::from)]
#[bw(map = |&x| Into::<u16>::into(x))]
pub struct ExternalPortType {
    /// Serial digital interface (SDI)
    pub sdi: bool,

    /// High-definition multimedia interface (HDMI)
    pub hdmi: bool,

    /// Component video
    pub component: bool,

    /// Composite video
    pub composite: bool,

    /// S-Video
    pub svideo: bool,

    /// XLR audio connection
    pub xlr: bool,

    /// AES EBU audio connection
    pub aes_ebu: bool,

    /// RCA audio connection
    pub rca: bool,

    /// Internal port
    pub internal: bool,

    /// TS audio connection
    pub ts_jack: bool,

    /// MADI audio connection
    pub madi: bool,

    /// TRS audio connection
    pub trs: bool,

    // RJ45 not supported on network layer?
    #[skip]
    __: B4,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn external_port_types() {
        assert_eq!(
            ExternalPortType::new().with_sdi(true),
            ExternalPortType::from(0x0001u16)
        );
    }
}
