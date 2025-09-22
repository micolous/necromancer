//! Transport layer for ATEM mixer control over UDP ("BURP").
//!
//! This typically operates over port 9910.
//!
//! ## Discovery
//!
//! There are two services advertised with MDNS:
//!
//! * `_switcher_ctrl._udp`: UDP BURP protocol
//! * `_blackmagic._tcp`: TCP config protocol
use crate::{protocol::AtemPacket, Error, Result};
use binrw::{BinRead, BinWrite};
use std::{
    io::Cursor,
    net::{Ipv4Addr, SocketAddrV4},
};
use tokio::net::{ToSocketAddrs, UdpSocket};

pub struct AtemUdpChannel {
    sock: Option<UdpSocket>,
}

impl AtemUdpChannel {
    pub fn new() -> Self {
        Self { sock: None }
    }

    pub async fn connect<A: ToSocketAddrs>(&mut self, addr: A) -> Result {
        let sock = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)).await?;
        sock.connect(addr).await?;
        self.sock = Some(sock);
        Ok(())
    }

    pub async fn send(&self, packet: &AtemPacket) -> Result {
        let sock = self.sock.as_ref().ok_or(Error::ChannelUnavailable)?;
        let mut out = Cursor::new(Vec::new());
        packet.write(&mut out)?;
        let out = out.into_inner();
        sock.send(&out).await?;
        Ok(())
    }

    /// Extracts the inner [std::net::UdpSocket] from this channel.
    ///
    /// This renders the [AtemUdpChannel] unusable.
    ///
    /// This is needed for clean-up tasks, where we might not have an async
    /// runtime available anymore.
    pub fn take_std_socket(&mut self) -> Result<std::net::UdpSocket> {
        let sock = self.sock.take().ok_or(Error::ChannelUnavailable)?;
        Ok(sock.into_std()?)
    }

    pub async fn recv(&self) -> Result<AtemPacket> {
        let sock = self.sock.as_ref().ok_or(Error::ChannelUnavailable)?;
        let mut b = [0u8; AtemPacket::MAX_PACKET_LENGTH as usize];
        let l = sock.recv(&mut b).await?;
        let b = &b[..l];

        Ok(AtemPacket::read(&mut Cursor::new(b))?)
    }
}
