use crate::server::channel::{ChannelId, ChannelStatus};
use serde::{Deserialize, Serialize};
use std::net::AddrParseError;
use tonic::codegen::http::uri::InvalidUri;
use tonic::transport::Error;
use tonic::Status;

mod remote;
mod router_service;
pub mod server;
mod storage;

use crate::protocol::packets::RawPacket;
use crate::protocol::PacketError;
use crate::router::remote::Remotes;
use crate::server::session::SharedSession;
pub use storage::RouterStorage;

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub channel_id: ChannelId,
    pub router: Router,
    pub channel_status: ChannelStatus,
}

#[derive(Debug, Clone)]
pub struct RouterClient<Storage> {
    router_id: RouterId,
    local: SharedSession,
    remotes: Remotes,
    storage: Storage,
}

impl<Storage> RouterClient<Storage>
where
    Storage: RouterStorage + Clone,
{
    pub async fn new(
        router_id: RouterId,
        session: SharedSession,
        storage: Storage,
    ) -> RouterClient<Storage> {
        RouterClient {
            router_id,
            local: session,
            remotes: Remotes::new().await,
            storage,
        }
    }

    // Split local and remote message here.
    // Process local to the local broker session.
    pub async fn send(&self, raw_packet: RawPacket) -> Result<(), RouterError> {
        let channel_id = ChannelId::from(raw_packet.header().client_id());
        let value: Value = self.storage.get_channel_router(channel_id.clone()).await;
        if self.router_id == value.router.router {
            self.local.send(&channel_id, raw_packet.packet()).await;
        } else {
            let _ = self.remotes.send(value, raw_packet.packet()).await;
        }
        Ok(())
    }
}

impl Value {
    pub fn channel_id(&self) -> ChannelId {
        self.channel_id.clone()
    }
}

// raft shared rpc caller

// raft shared channel status

// if device sign_in in another node, it will update router and notice old
// node clear resources.

// heartbeat timer task update channel status and disconnection remove channel,
// should there acquire a distributed lock.
