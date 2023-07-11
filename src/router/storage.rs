use crate::router::{Key, RouterId, Value};
use async_trait::async_trait;

/// Define all state that need
#[async_trait]
pub trait RouterStorage: Clone {
    // fetch channel in which router.
    async fn get_channel_router(&self, key: Key) -> Value;

    async fn update_or_insert_channel_node(&self, value: Value) -> Value;

    // registry router
    async fn router_lease(&self, router: RouterId) -> Option<RouterId>;
}
