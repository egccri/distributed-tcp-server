mod codec;
pub(crate) mod packets;

#[derive(thiserror::Error, Debug)]
pub enum PacketError {
    #[error("无法从Frame解析为Packet，原始Frame：{packet}")]
    ParsePacketError { packet: String },

    #[error("解析PacketHeader错误，原始Packet：{packet}")]
    ParsePacketHeaderError { packet: String },

    #[error("未定义的Packet，原始Packet：{packet}")]
    UnKnowPacketError { packet: String },
}
