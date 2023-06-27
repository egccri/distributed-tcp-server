use crate::server::channel::{ChannelId, ChannelStatus};
use serde::{Deserialize, Serialize};

mod remote;
mod router_service;
mod server;
mod storage;

pub use storage::Storage as RouterStorage;

/// router saved the connection and channel map state
/// any channel status changed is send to the raft leader
///
/// router is store the device_id with channel
/// channel split local channel and remote channel.
/// router shared between raft cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Router {
    router: RouterId,
    local_address: String,
    remote_addr: String,
}

pub type RouterId = u64;

/// Key is the link between connection and channel
pub type Key = ChannelId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    channel_id: ChannelId,
    router: Router,
    channel_status: ChannelStatus,
}

impl Router {
    // Split local and remote message here.
    // Process local to the local broker session.
}

// raft shared rpc caller

// raft shared channel status

// if device sign_in in another node, it will update router and notice old
// node clear resources.

// heartbeat timer task update channel status and disconnection remove channel,
// should there acquire a distributed lock.
