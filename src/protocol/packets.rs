use crate::protocol::packets::heartbeat::HeartbeatRecv;
use crate::protocol::packets::sign_in::{SignInFire, SignInRecv};
use crate::protocol::PacketError;
use std::fmt::{Display, Formatter};

mod heartbeat;
mod sign_in;

pub trait Fire: Into<String> {}

pub trait Recv: TryFrom<String> {}

// If your protocol can split req or resp itself, you can take there all in one packet enum.
#[derive(Debug, Clone, PartialEq)]
pub struct RawPacket {
    header: PacketHeader,
    packet: Packet,
}

impl RawPacket {
    pub fn read(raw: String) -> Result<Self, PacketError> {
        let header = PacketHeader::new(raw.as_str())?;
        Ok(RawPacket {
            header,
            packet: Packet::read(raw)?,
        })
    }

    pub fn write(self) -> Result<String, PacketError> {
        Packet::write(self.packet)
    }

    pub fn header(&self) -> &PacketHeader {
        &self.header
    }

    pub fn packet(&self) -> Packet {
        self.packet.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Packet {
    SignIn(SignInRecv),
    SignInAck(SignInFire),
    HeartBeat(HeartbeatRecv),
    Close(()),
}

pub const SIGN_IN: u8 = 1;
pub const SIGN_IN_ACK: u8 = 2;
pub const HEARTBEAT: u8 = 3;

impl Packet {
    pub fn read(raw: String) -> Result<Self, PacketError> {
        let header = PacketHeader::new(raw.as_str())?;
        match header.packet_type {
            SIGN_IN => Ok(Packet::SignIn(SignInRecv::try_from(raw)?)),
            HEARTBEAT => Ok(Packet::HeartBeat(HeartbeatRecv::try_from(raw)?)),
            u8::MAX | _ => Err(PacketError::UnKnowRecvPacketError { raw }),
        }
    }

    pub fn write(self) -> Result<String, PacketError> {
        match self {
            Packet::SignInAck(sign_in_ack) => Ok(format!(
                "{},{}",
                SIGN_IN_ACK,
                <SignInFire as Into<String>>::into(sign_in_ack)
            )),
            _ => Err(PacketError::UnSupportFirePacketError { packet: self }),
        }
    }

    pub fn check_sign_in_packet(raw: &str) -> Result<bool, PacketError> {
        let header = PacketHeader::new(raw)?;
        Ok(header.packet_type() == SIGN_IN)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PacketHeader {
    packet_type: u8,
    client_id: String,
}

impl PacketHeader {
    pub fn new(raw: &str) -> Result<Self, PacketError> {
        let raw_vec = raw.split(",").collect::<Vec<&str>>();
        if raw_vec.len() > 1 {
            Ok(PacketHeader {
                packet_type: raw_vec[0].parse::<u8>().unwrap_or(u8::MAX),
                client_id: raw_vec[1]
                    .parse::<String>()
                    .unwrap_or("unknown".to_string()),
            })
        } else {
            Err(PacketError::ParsePacketHeaderError {
                raw: raw.to_string(),
            })
        }
    }

    pub fn packet_type(&self) -> u8 {
        self.packet_type
    }

    pub fn client_id(&self) -> String {
        self.client_id.clone()
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::SignIn(sign_in) => {
                write!(f, "SignIn:{:?}", sign_in)
            }
            Packet::SignInAck(sign_in_ack) => {
                write!(f, "SignInAck:{:?}", sign_in_ack)
            }
            Packet::HeartBeat(heartbeat) => {
                write!(f, "Heartbeat:{:?}", heartbeat)
            }
            Packet::Close(_) => {
                write!(f, "Close")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_sign_in_packet() {
        let raw_packet = "1,client_id,username,password".to_string();
        let expected_packet = Packet::SignIn(SignInRecv {
            client_id: "client_id".to_string(),
            username: "username".to_string(),
            password: "password".to_string(),
        });
        assert_eq!(Packet::read(raw_packet), Ok(expected_packet));
    }

    #[test]
    fn test_read_sign_in_ack_packet() {
        let raw_packet = "2, 100".to_string();
        let expected_error = PacketError::UnKnowRecvPacketError {
            raw: raw_packet.clone(),
        };
        assert_eq!(Packet::read(raw_packet), Err(expected_error));
    }

    #[test]
    fn test_read_heartbeat_packet() {
        let raw_packet = "3,12345".to_string();
        let expected_packet = Packet::HeartBeat(HeartbeatRecv { seq: 12345 });
        assert_eq!(Packet::read(raw_packet), Ok(expected_packet));
    }

    #[test]
    fn test_read_unknown_packet() {
        let raw_packet = "4,data".to_string();
        let expected_error = PacketError::UnKnowRecvPacketError {
            raw: raw_packet.clone(),
        };
        assert_eq!(Packet::read(raw_packet), Err(expected_error));
    }

    #[test]
    fn test_write_sign_in_ack_packet() {
        let packet = Packet::SignInAck(SignInFire {
            code: "100".to_string(),
        });
        let expected_raw_packet = "2,100".to_string();
        assert_eq!(packet.write(), Ok(expected_raw_packet));
    }

    #[test]
    fn test_write_unsupported_packet() {
        let packet = Packet::HeartBeat(HeartbeatRecv { seq: 12345 });
        let expected_error = PacketError::UnSupportFirePacketError {
            packet: packet.clone(),
        };
        assert_eq!(packet.write(), Err(expected_error));
    }

    #[test]
    fn test_check_sign_in_packet_valid() {
        let raw_packet = "1,username,password";
        assert_eq!(Packet::check_sign_in_packet(raw_packet), Ok(true));
    }

    #[test]
    fn test_check_sign_in_packet_invalid() {
        let raw_packet = "2,client_id";
        assert_eq!(Packet::check_sign_in_packet(raw_packet), Ok(false));
    }
}
