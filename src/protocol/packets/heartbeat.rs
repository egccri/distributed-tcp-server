use crate::protocol::packets::Recv;
use crate::protocol::PacketError;
use crate::protocol::PacketError::ParsePacketError;

#[derive(Debug, Clone, PartialEq)]
pub struct HeartbeatRecv {
    pub seq: u32,
}

impl TryFrom<String> for HeartbeatRecv {
    type Error = PacketError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let attrs = value.split(',').collect::<Vec<&str>>();
        if attrs.len() != 2 {
            Err(ParsePacketError { raw: value })
        } else {
            Ok(HeartbeatRecv {
                seq: attrs[1].parse::<u32>().unwrap_or_default(),
            })
        }
    }
}

impl Recv for HeartbeatRecv {}
