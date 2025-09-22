#![allow(rustdoc::private_intra_doc_links)]

use crate::{atom::*, util::OffsetCounter, Error, Result};
use binrw::{binrw, helpers::until_eof, io::TakeSeekExt, BinRead, BinWrite};
use modular_bitfield::{bitfield, specifiers::B11, Specifier};
use std::io::SeekFrom;

/// Packet flags and length value.
///
/// ## Format
///
/// This is a big-endian `u16` bitfield. Fields from LSB to MSB:
///
/// * `u11 0x07ff`: length
/// * `bit 0x0800`: ACK (needs ACK)
/// * `bit 0x1000`: Control
/// * `bit 0x2000`: Retransmission
/// * `bit 0x4000`: Hello
/// * `bit 0x8000`: Response (to ACK)
#[bitfield(bits = 5)]
#[derive(Specifier, BinRead, BinWrite, Debug, Default, PartialEq, Eq, Clone, Copy)]
#[brw(big)]
pub struct AtemPacketFlags {
    /// The packet's sender requests an acknowledgement of this packet from the
    /// recipient (using `response = true`).
    pub ack: bool,

    /// Control payload.
    ///
    /// When `true`, the payload is [`AtemPacketPayload::Control`], otherwise
    /// it is [`AtemPacketPayload::Atom`].
    pub control: bool,

    /// The packet is a retransmission of a prior packet (probably because there
    /// was no response to an `ack`).
    pub retransmission: bool,

    pub hello: bool,

    /// The packet is acknowledging a prior `ack = true` packet.
    pub response: bool,
}

/// Packet flags and length value.
///
/// ## Format
///
/// This is a big-endian `u16` bitfield. Fields from LSB to MSB:
///
/// * `u11 0x07ff`: length
/// * `bit 0x0800`: ACK (needs ACK)
/// * `bit 0x1000`: Control
/// * `bit 0x2000`: Retransmission
/// * `bit 0x4000`: Hello
/// * `bit 0x8000`: Response (to ACK)
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Specifier, BinRead, BinWrite, Debug, Default, PartialEq, Eq, Clone, Copy)]
#[brw(big)]
#[br(map = From::<u16>::from)]
#[bw(map = |&x| Into::<u16>::into(x))]
pub struct AtemPacketFlagsLength {
    /// The total length of the packet including headers, in bytes.
    ///
    /// When set to exactly `12`, the packet has no payload
    /// ([`AtemPacketPayload::None`]).
    length: B11,
    flags: AtemPacketFlags,
}

/// Control command.
///
/// It is used for [AtemPacket] with [AtemPacketFlags::control] set.
///
/// ## Packet format
///
/// * `u8`: control code
/// * likely padding
/// * `u16`: for ConnectAck, this is the session ID.
/// * `u32`: ???
#[binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[brw(big)]
pub enum AtemControl {
    #[brw(magic = 0x01u8)]
    Connect,
    #[brw(magic = 0x02u8)]
    ConnectAck {
        #[brw(pad_before = 1)]
        session_id: u16,
    },
    #[brw(magic = 0x03u8)]
    ConnectNack,
    #[brw(magic = 0x04u8)]
    Disconnect,
    #[brw(magic = 0x05u8)]
    DisconnectAck,
}

impl AtemControl {
    const LENGTH: u16 = 8;
}

/// [AtemPacket] is the basic unit of communication with ATEM video switchers
/// over UDP.
///
/// Everything else is built on top of it.
///
/// ## Packet format
///
/// * `u16`: [flags] (5 high bits) + [packet length] (11 low bits)
/// * `u16`: [session ID]
/// * `u16`: [acked packet ID]
/// * 2 unknown bytes
/// * `u16`: [client packet ID] - probably a misnomer
/// * `u16`: [sender packet ID]
/// * [payload] (optional)
///
/// [flags]: AtemPacketFlagsLength::flags
/// [packet length]: AtemPacketFlagsLength::length
/// [session ID]: Self::session_id
/// [acked packet ID]: Self::acked_packet_id
/// [client packet ID]: Self::client_packet_id
/// [sender packet ID]: Self::sender_packet_id
/// [payload]: Self::payload
#[binrw]
#[derive(Debug, Clone, PartialEq, Eq)]
#[brw(big, stream = s)]
#[bw(map_stream = OffsetCounter::new)]
pub struct AtemPacket {
    // Read path for AtemPacketFlagsLength
    #[br(temp, assert(flags_length.length() >= 12))]
    #[bw(ignore)]
    flags_length: AtemPacketFlagsLength,

    #[br(calc(flags_length.flags()))]
    #[bw(ignore)]
    pub flags: AtemPacketFlags,

    /// Packet session ID.
    ///
    /// IDs with the highest bit set (`0x8000`) are assigned by the switcher.
    #[bw(pad_before = 2)]
    pub session_id: u16,

    /// If non-zero, all [sender packet IDs] with this ID and lower should be
    /// considered acknowledged.
    ///
    /// [sender packet IDs]: Self::sender_packet_id
    pub acked_packet_id: u16,
    unknown: u16,

    /// Client packet identifier; probably a misnomer. This may actually
    /// indicate a protocol version, or it might just be more uninitialized
    /// memory.
    ///
    /// Observed values:
    ///
    /// * `0xb1` when the controller sends an `AtemInitPayload`
    /// * `0xd4` when the controller requests initial switcher status
    /// * `0xd2` when the controller acknowledges any switcher messages
    /// * `0x00` or `0x01` when the controller sends commands to the switcher
    /// * `0x1e` when the switcher acknowledges any controller commands
    /// * `0x00` when the switcher sends any `ACK | RESPONSE` message
    pub client_packet_id: u16,

    /// Packet identifier according to the packet's *sender*, which can be the
    /// switcher or the controller.
    ///
    /// The switcher overflows at `0x7fff` back to `0x0000`.
    ///
    /// Acknowledgements to this packet will be sent in the [acked packet ID].
    ///
    /// Also known as "switcher packet ID" - but this is a misnomer.
    ///
    /// [acked packet ID]: Self::acked_packet_id
    pub sender_packet_id: u16,

    // #[br(if(!flags_length.control(), Vec::new()))]
    // #[br(map_stream = |reader| reader.take_seek(u64::from(flags_length.length()) - 12))]
    // pub commands: Vec<Atom>,
    #[br(args(flags_length), map_stream = |reader| { reader.take_seek(u64::from(flags_length.length() - Self::HEADERS_LENGTH)) })]
    #[bw(args(flags))]
    payload: AtemPacketPayload,

    // Write path for AtemPacketFlagsLength
    #[br(ignore)]
    #[bw(try_calc(
        s.total()
            .try_into()
            .map_err(|_| Error::InvalidLength)
            .and_then(|l|
                AtemPacketFlagsLength::new()
                    .with_flags(self.flags)
                    .with_length_checked(l)
                    .map_err(|_| Error::InvalidLength)
            )
    ), seek_before = SeekFrom::Current(-(s.total() as i64)), restore_position)]
    flags_length: AtemPacketFlagsLength,
}

/// The [payload][AtemPacket::payload] of an [AtemPacket].
///
/// The `bw(assert)` rules for this structure do not check the length.
#[binrw]
#[derive(Default, Debug, Clone, PartialEq, Eq)]
// #[br(import(control: bool, length: B11))]
#[br(import(flags_length: AtemPacketFlagsLength))]
#[bw(import(flags: &AtemPacketFlags))]
enum AtemPacketPayload {
    /// The packet payload is 0 or more atoms.
    #[br(pre_assert(!flags_length.flags().control() && flags_length.length() > AtemPacket::HEADERS_LENGTH))]
    #[bw(assert(!flags.control()))]
    Atom(#[br(parse_with = until_eof)] Vec<Atom>),

    /// The packet payload contains control commands.
    #[br(pre_assert(flags_length.flags().control() && flags_length.length() == AtemPacket::HEADERS_LENGTH + AtemControl::LENGTH))]
    #[bw(assert(flags.control()))]
    Control(#[brw(pad_size_to = 8)] AtemControl),

    /// The packet has no valid payload data.
    ///
    /// This is used as a ping or keep-alive message.
    #[default]
    #[br(pre_assert(!flags_length.flags().control() && flags_length.length() == AtemPacket::HEADERS_LENGTH))]
    None,
}

impl AtemPacket {
    const HEADERS_LENGTH: u16 = 12;
    /// Maximum packet size, including headers.
    pub(crate) const MAX_PACKET_LENGTH: u16 = 0x7ff;
    /// Maximum packet payload size (minus [AtemPacket] headers)
    pub(crate) const MAX_PAYLOAD_LENGTH: u16 = Self::MAX_PACKET_LENGTH - Self::HEADERS_LENGTH;

    pub fn new(
        flags: AtemPacketFlags,
        session_id: u16,
        acked_packet_id: u16,
        client_packet_id: u16,
        sender_packet_id: u16,
    ) -> Self {
        let mut o = Self {
            flags,
            session_id,
            acked_packet_id,
            unknown: 0,
            client_packet_id,
            sender_packet_id,
            payload: AtemPacketPayload::None,
        };

        o.flags.set_control(false);
        o
    }

    pub fn new_control(
        flags: AtemPacketFlags,
        session_id: u16,
        acked_packet_id: u16,
        client_packet_id: u16,
        sender_packet_id: u16,
        control: AtemControl,
    ) -> Self {
        let mut o = Self {
            flags,
            session_id,
            acked_packet_id,
            unknown: 0,
            client_packet_id,
            sender_packet_id,
            payload: AtemPacketPayload::Control(control),
        };

        o.flags.set_control(true);
        o
    }

    pub fn new_atoms(
        flags: AtemPacketFlags,
        session_id: u16,
        acked_packet_id: u16,
        client_packet_id: u16,
        sender_packet_id: u16,
        atoms: Vec<Atom>,
    ) -> Self {
        let mut o = Self {
            flags,
            session_id,
            acked_packet_id,
            unknown: 0,
            client_packet_id,
            sender_packet_id,
            payload: AtemPacketPayload::Atom(atoms),
        };

        o.flags.set_control(false);
        o
    }

    /// If this packet [requires acknowledgement][AtemPacketFlags::ack], make a
    /// [AtemPacketFlags::response] packet to this packet.
    ///
    /// Otherwise return [None].
    pub fn make_ack(&self) -> Option<Self> {
        if self.flags.ack() {
            let mut p = AtemPacket::new(
                AtemPacketFlags::new().with_response(true),
                self.session_id,
                self.sender_packet_id,
                0,
                0,
            );

            // FIXME: We need to shove our transfer packet ACKs in the
            // normal regular acks, but our controller model doesn't work
            // properly.
            if let Some(commands) = self.atoms() {
                for cmd in commands {
                    let Payload::TransferChunk(chunk) = &cmd.payload else {
                        continue;
                    };

                    let cmd = Atom::new(TransferAck { id: chunk.id }.into());
                    let _ = p.push_atom(cmd);
                    p.flags.set_ack(true);
                }
            }

            Some(p)
        } else {
            None
        }
    }

    /// Gets the [`Atom`][]s for this packet, if this is an Atom packet.
    ///
    /// This will return [`Some`] even if the `Vec<Atom>` is empty.
    ///
    /// See also: [`AtemPacket::has_atoms()`]
    pub fn atoms(&self) -> Option<&Vec<Atom>> {
        match &self.payload {
            AtemPacketPayload::Atom(atoms) => Some(atoms),
            _ => None,
        }
    }

    /// Returns `true` if the payload type is Atoms, and contains at least one atom.
    ///
    /// See also: [`AtemPacket::atoms()`]
    pub fn has_atoms(&self) -> bool {
        self.atoms().is_some_and(|atoms| !atoms.is_empty())
    }

    /// Appends an [`Atom`] to this [`AtemPacket`]'s payload.
    ///
    /// If the [`payload`][Atom::payload] is [`AtemPacketPayload::None`],
    /// it will be changed to [`AtemPacketPayload::Atom`].
    ///
    /// If the [`payload`][Atom::payload] is
    /// [`AtemPacketPayload::Control`], this will return
    /// [`Error::UnexpectedState`].
    fn push_atom(&mut self, cmd: Atom) -> Result<()> {
        match &mut self.payload {
            AtemPacketPayload::Atom(cmds) => {
                cmds.push(cmd);
            }
            AtemPacketPayload::None => {
                // Convert to Commands type
                self.flags.set_control(false);
                let cmds = vec![cmd];
                self.payload = AtemPacketPayload::Atom(cmds);
            }
            AtemPacketPayload::Control(_) => {
                error!("cannot append Atoms to Control payload");
                return Err(Error::UnexpectedState);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_flags() -> Result<()> {
        let mut c = Cursor::new([0; 2]);
        let f = AtemPacketFlagsLength::new().with_length(1);
        f.write(&mut c)?;
        assert_eq!([0, 1], c.into_inner());

        let mut c = Cursor::new([0; 2]);
        let f = AtemPacketFlags::new().with_control(true);
        let f = AtemPacketFlagsLength::new().with_flags(f);
        f.write(&mut c)?;
        assert_eq!([0x10, 0], c.into_inner());

        Ok(())
    }

    #[test]
    fn control_init() -> Result<()> {
        let expected = AtemPacket::new_control(
            AtemPacketFlags::new().with_control(true),
            0x2970,
            0,
            0xb1,
            0,
            AtemControl::Connect,
        );
        let cmd = hex::decode("101429700000000000b100000100000000000000")?;
        let pkt = AtemPacket::read(&mut Cursor::new(&cmd)).unwrap();

        assert_eq!(expected, pkt);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        expected.write(&mut out).unwrap();
        assert_eq!(out.into_inner(), cmd);

        Ok(())
    }

    #[test]
    fn control_init_ack() -> Result<()> {
        let expected = AtemPacket::new_control(
            AtemPacketFlags::new().with_control(true),
            0x2970,
            0,
            0,
            0,
            AtemControl::ConnectAck { session_id: 0x0002 },
        );
        let cmd = hex::decode("1014297000000000000000000200000200000000").unwrap();
        let pkt = AtemPacket::read(&mut Cursor::new(&cmd)).unwrap();

        assert_eq!(expected, pkt);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        pkt.write(&mut out).unwrap();
        assert_eq!(out.into_inner(), cmd);
        Ok(())
    }

    #[test]
    fn rfip() {
        // single RFIP
        let cmd = hex::decode(
            "08288001000000000000003f001c0000524649500001420901000000ffffffffffff01000004cb01",
        )
        .unwrap();
        let pkt = AtemPacket::read(&mut Cursor::new(&cmd)).unwrap();

        // println!("{pkt:?}");

        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        pkt.write(&mut out).unwrap();
        assert_eq!(out.into_inner(), cmd);

        // multiple RFIP/RFLP
        // This dump is a modified to remove uninitialised memory from commands
        let cmd = hex::decode(concat!(
            "08dc80010000000000000040",
            "001c0000524649500002420901000000ffffffffffff01000004ca01",
            "001c0000524649500003420901000000ffffffffffff01000004cb01",
            "001c0000524649500004420901000000ffffffffffff01000004cb01",
            "000c000052464c5002ce0004",
            "001c0000524649500515420901000000ffffffffffffff000004c801",
            "001c000052464950051514c8877f0000ffffffffffffff01000414c8",
            "001c0000524649500516420901000000ffffffffffffff000004cd01",
            "001c0000524649500516a50d887f0000ffffffffffffff010004a50d",
        ))
        .unwrap();

        let pkt = AtemPacket::read(&mut Cursor::new(&cmd)).unwrap();

        // println!("{pkt:?}");
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        pkt.write(&mut out).unwrap();
        assert_eq!(out.into_inner(), cmd);
    }
}
