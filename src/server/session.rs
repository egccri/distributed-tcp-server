use crate::protocol::packets::Packet;
use crate::server::channel::{Channel, ChannelId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct SharedSession(Arc<RwLock<HashMap<String, Channel>>>);

impl SharedSession {
    pub async fn close(channel_id: ChannelId) -> Option<Channel> {
        None
    }

    pub async fn send(channel_id: ChannelId, packet: Packet) {}

    pub async fn add(channel_id: ChannelId, channel: Channel) {}

    pub async fn find(channel_id: ChannelId) -> Option<Channel> {
        None
    }
}
