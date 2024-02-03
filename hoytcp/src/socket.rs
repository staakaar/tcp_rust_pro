use crate::packet::TCPPacket;
use crate::tcpflags;
use anyhow::{Context, Result};
use pnet::packet::{ip::IpNextHeaderProtocols, packet};
use pnet::transport::{self, TransportChannelType, TransportProtocol, TransportSender};
use pnet::util;
use std::collections::VecDeque;
use std::fmt::{self, Display};
use std::net::{IpAddr, Ipv4Addr};
use std::time::SystemTime;

const SOCKET_BUFFER_SIZE = 4380;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct SockID(pub Ipv4Addr, pub Ipv4Addr, pub u16, pub u16);

pub struct Socket {
    pub local_addr: Ipv4Addr,
    pub remote_addr: Ipv4Addr,
    pub local_port: u16,
    pub remote_port: u16,
    pub sender: TransportSender,
    pub send_param: SendParam,
    pub recv_param: RecvParam,
    pub status: TcpStatus,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TcpStatus {
    Listen,
    SynSent,
    SynRcvd,
    Established,
    FinWait1,
    FinWait2,
    TimeWait,
    CloseWait,
    LastAck,
}

impl Display for TacpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TcpStatus::Listen => write!(f, "LISTEN"),
            TCPStatus::SynSent => write!(f, "SYNSENT"),
            TCPStatus::SynRcvd => write!(f, "SYNRCVD"),
            TCPStatus::Established => write!(f, "ESTABLISHED"),
            TCPStatus::FinWait1 => write!(f, "FINWAIT1"),
            TCPStatus::FinWait2 => write!(f, "FINWAIT2"),
            TCPStatus::TimeWait => write!(f, "TIMEWAIT"),
            TcpStatus::CloseWait => write!(f, "CLOSEWAIT"),
            TCPStatus::LastAck => write!(f, "LASTACK"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SendParam {
    pub unacked_seq: u32,
    pub next: u32,
    pub window: u16,
    pub initial_seq: u32,
}

#[derive(Clone, Debug)]
pub struct RecvParam {
    pub next: u32,
    pub window: u16,
    pub initial_seq: u32,
    pub tail: u32,
}

impl Socket {
    pub fn new(local_addr: Ipv4Addr, remote_addr: Ipv4Addr, local_port: u16, remote_port: u16, status: TcpStatus) -> Result<Self> {
        let (sender, _) = transport::transport_channel(65535, TransportChannelType::Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::TCP)),)?;
        Ok(Self {
            local_addr,
            remote_addr,
            local_port,
            remote_port,
            sender,
            send_param: SendParam {
                unacked_seq: 0,
                initial_seq: 0,
                next: 0,
                window: SOCKET_BUFFER_SIZE as u16,
            },
            recv_param: RecvParam {
                initial_seq: 0,
                next: 0,
                window: SOCKET_BUFFER_SIZE as u16,
                tail: 0
            },
            status,
        })
    }

    pub fn send_tcp_packet(&mut self, flag: u8, payload: &[u8]) -> Result<usize> {
        let mut tcp_packet = TCPPacket::new(payload.len());
        tcp_packet.set_src(self.local_port);
        tcp_packet.set_dest(self.remote_port);
        tcp_packet.set_flag(flag);
        tcp_packet.set_seq(seq);
        tcp_packet.set_ack(ack);
        tcp_packet.set_data_offset(5);
        tcp_packet.set_window_size(self.recv_param.window);
        tcp_paket.set_payload(payload);
        tcp_packet.set_checksum(util::ipv4_checksum(
            &tcp_packet.packet(),
            8,
            &[],
            &self.local_addr,
            &self.remote_addr,
            IpNextHeaderProtocols::Tcp,
        ));
        let sent_size = self.sender.send_to(tcp_packet.clone(), Ipv4Addr::V4(self.remote_addr)).unwrap();.context(format!("failed to send: \n {:?}", tcp_packet))?;
        dbg!("senf", &tcp_packet);
        Ok(sent_size)
    }

    pub gn get_sock_id(&self) -> SockID {
        self.local_addr,
        self.remote_addr,
        self.local_port,
        self.remote_port,
    }
}
