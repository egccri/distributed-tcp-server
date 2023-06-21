use crate::router::Value;
use crate::server::channel::ChannelId;
use async_trait::async_trait;

/// Define all state that need
#[async_trait]
pub trait Storage {
    // fetch channel in which node.
    async fn get_channel_router(channel_id: ChannelId) -> Value;

    async fn update_or_insert_channel_node(value: Value) -> Value;
}
