use crate::protocol::packets::Packet;
use crate::server::channel::{Channel, ChannelId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct SharedSession(Arc<RwLock<HashMap<String, Channel>>>);

impl SharedSession {
    pub async fn close(&self, channel_id: &ChannelId) -> Option<Channel> {
        None
    }

    pub async fn send(&self, channel_id: &ChannelId, packet: Packet) {}

    pub async fn add(&self, channel: Channel) {}

    pub async fn find(&self, channel_id: &ChannelId) -> Option<Channel> {
        None
    }

    pub async fn clear_closed_channel(&self) {}
}
