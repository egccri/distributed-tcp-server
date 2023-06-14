use std::collections::HashMap;

mod storage;

pub use storage::Storage as RouterStorage;

pub type Link = (String, String);

/// router saved the connection and channel map state
/// any channel status changed is send to the raft leader
///
/// router is store the device_id with channel
/// channel split local channel and remote channel.
/// router shared between raft cluster
pub struct Router {
    /// store device_id and node_id,
    pub links: HashMap<String, Link>,
}

// raft shared rpc caller

// raft shared channel status

// if device sign_in in another node, it will update router and notice old
// node clear resources.

// heartbeat timer task update channel status and disconnection remove channel,
// should there acquire a distributed lock.
