use crate::protocol::packets::Recv;
use crate::protocol::PacketError;
use crate::protocol::PacketError::ParsePacketError;

#[derive(Debug, Clone, PartialEq)]
pub struct HeartbeatRecv {}

impl TryFrom<String> for HeartbeatRecv {
    type Error = PacketError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(ParsePacketError { raw: value })
        } else {
            Ok(HeartbeatRecv {})
        }
    }
}

impl Recv for HeartbeatRecv {}
