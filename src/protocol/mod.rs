use crate::protocol::packets::Packet;

mod codec;
pub(crate) mod packets;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum PacketError {
    #[error("Can't parse raw to packet, raw: {raw}")]
    ParsePacketError { raw: String },

    #[error("Can't parse packet header from raw, raw: {raw}")]
    ParsePacketHeaderError { raw: String },

    #[error("UnKnow recv packet, raw: {raw}")]
    UnKnowRecvPacketError { raw: String },

    #[error("Packet is not support for fire, packet: {packet}")]
    UnSupportFirePacketError { packet: Packet },
}
