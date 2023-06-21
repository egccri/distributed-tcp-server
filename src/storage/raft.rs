use crate::router::{RouterStorage, Value};
use async_trait::async_trait;
use openraft::storage::Adaptor;
use openraft::{BasicNode, Config, Entry};
use std::io::Cursor;
use std::sync::Arc;

use crate::server::channel::ChannelId;
use crate::storage::raft::network::NetworkManager;
use crate::storage::raft::storage::Store;
use storage::Request;
use storage::Response;

mod network;
mod network_api;
mod storage;

/// The unique id of the raft node.
pub type NodeId = u64;
/// Node is custom node data that can used by raft core.
pub type Node = BasicNode;

openraft::declare_raft_types!(
    /// Declare the type configuration for example K/V store.
    pub TypeConfig: D = Request, R = Response, NodeId = NodeId, Node = Node,
    Entry = Entry<TypeConfig>, SnapshotData = Cursor<Vec<u8>>
);

// When main server start and before accept tcp connections, start the RaftStore.
// Include start a raft server, check snapshot data to the router etc.
pub async fn start(node_id: NodeId, addr: &str) {
    let config = Config {
        heartbeat_interval: 500,
        election_timeout_min: 1500,
        election_timeout_max: 3000,
        ..Default::default()
    };

    let config = Arc::new(config.validate().unwrap());

    let store = Arc::new(Store::new());

    let (log_store, state_machine) = Adaptor::new(store.clone());

    let network = NetworkManager::new();

    let raft = openraft::Raft::new(node_id, config.clone(), network, log_store, state_machine)
        .await
        .unwrap();
}

// Hold a raft client there, read or write to the raft.
pub struct RaftStorage {}

impl RaftStorage {}

// Impl router operations here.
#[async_trait]
impl RouterStorage for RaftStorage {
    async fn get_channel_router(channel_id: ChannelId) -> Value {
        todo!()
    }

    async fn update_or_insert_channel_node(value: Value) -> Value {
        todo!()
    }
}
