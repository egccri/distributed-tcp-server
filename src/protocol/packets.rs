use crate::protocol::packets::heartbeat::HeartbeatRecv;
use crate::protocol::packets::sign_in::{SignInFire, SignInRecv};
use crate::protocol::PacketError;

mod heartbeat;
mod sign_in;

pub trait Fire: Into<String> {}

pub trait Recv: TryFrom<String> {}

// 如果你的协议区分req和resp，可以使用一个packet，如果你的协议不区分，则需要分开SendPacket和ReceivePacket
pub enum Packet {
    SignIn(SignInRecv),
    SignInAck(SignInFire),
    HeartBeat(HeartbeatRecv),
}

pub const SIGN_IN: u8 = 1;
pub const SIGN_IN_ACK: u8 = 2;
pub const HEARTBEAT: u8 = 3;

impl Packet {
    pub fn read(raw: String) -> Result<Self, PacketError> {
        let header = PacketHeader::new(raw.as_str())?;
        match header.packet_type {
            SIGN_IN => Ok(Packet::SignIn(SignInRecv::try_from(raw)?)),
            u8::MAX | _ => Err(PacketError::UnKnowPacketError { packet: raw }),
        }
    }
}

pub struct PacketHeader {
    packet_type: u8,
}

impl PacketHeader {
    pub fn new(raw: &str) -> Result<Self, PacketError> {
        let raw_vec = raw.split(",").collect::<Vec<&str>>();
        if raw_vec.len() > 1 {
            Ok(PacketHeader {
                packet_type: raw_vec[0].parse::<u8>().unwrap_or(u8::MAX),
            })
        } else {
            Err(PacketError::ParsePacketHeaderError {
                packet: raw.to_string(),
            })
        }
    }

    pub fn packet_type(&self) -> u8 {
        self.packet_type
    }
}
