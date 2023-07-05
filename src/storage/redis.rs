use crate::router::{RouterId, RouterStorage, Value};
use crate::server::channel::ChannelId;
use async_trait::async_trait;

pub struct RedisStorage {}

#[async_trait]
impl RouterStorage for RedisStorage {
    async fn get_channel_router(channel_id: ChannelId) -> Value {
        todo!()
    }

    async fn update_or_insert_channel_node(value: Value) -> Value {
        todo!()
    }

    async fn router_lease(router: RouterId) -> Option<RouterId> {
        todo!()
    }
}
