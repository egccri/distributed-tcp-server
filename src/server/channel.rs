use crate::protocol::packets::Packet;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use tokio::sync::broadcast::Sender;

// RxPacket 用泛型灵活，但心智负担中，协议相关适合用枚举，因为协议是确定有限的
#[derive(Debug)]
pub struct Channel {
    channel_id: ChannelId,
    remote_address: SocketAddr,
    #[allow(dead_code)]
    rx: Sender<Packet>,
    channel_status: ChannelStatus,
}

impl Display for Channel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Channel(id: {}, status: {}) connect with foreign address: {}.",
            self.channel_id, self.channel_status, self.remote_address
        )
    }
}

//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelId {
    id: String,
}

impl Display for ChannelId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl ChannelId {
    pub fn generate() -> Self {
        ChannelId { id: "".to_string() }
    }
}

impl From<String> for ChannelId {
    fn from(value: String) -> Self {
        Self { id: value }
    }
}

impl Into<String> for ChannelId {
    fn into(self) -> String {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelStatus {
    Established,
    Closing,
    Closed,
}

impl Display for ChannelStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelStatus::Established => {
                write!(f, "Established")
            }
            ChannelStatus::Closing => {
                write!(f, "Closing")
            }
            ChannelStatus::Closed => {
                write!(f, "Closed")
            }
        }
    }
}

impl Channel {
    pub fn new(remote_address: SocketAddr, sender: Sender<Packet>) -> Self {
        Channel {
            channel_id: ChannelId::generate(),
            remote_address,
            rx: sender,
            channel_status: ChannelStatus::Established,
        }
    }

    pub fn channel_id(&self) -> &ChannelId {
        &self.channel_id
    }
}
