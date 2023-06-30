use crate::server::channel::{ChannelId, ChannelStatus};
use serde::{Deserialize, Serialize};
use std::net::AddrParseError;
use tonic::codegen::http::uri::InvalidUri;
use tonic::transport::Error;
use tonic::Status;

mod remote;
mod router_service;
mod server;
mod storage;

use crate::protocol::PacketError;
pub use storage::Storage as RouterStorage;

#[derive(thiserror::Error, Debug)]
pub enum RouterError {
    #[error("Send packet error cause by {0}.")]
    SendMessageError(#[from] PacketError),

    #[error("Error cause by channel connect.")]
    ChannelConnectError,

    #[error(transparent)]
    InvalidUri(#[from] InvalidUri),

    #[error("Remote received a error status: {0}")]
    ReplyErrorStatus(Status),

    #[error("Socket addr parse error, cause by {0}.")]
    SocketAddrParseError(#[from] AddrParseError),

    #[error("Raft server api start error: cause by {0}")]
    TonicServerError(#[from] Error),
}

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
