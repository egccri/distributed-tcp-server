use crate::router::{RouterId, RouterStorage, Value};
use async_trait::async_trait;
use openraft::storage::Adaptor;
use openraft::{BasicNode, Config, Entry};
use std::collections::BTreeMap;
use std::io::Cursor;
use std::sync::Arc;

use crate::server::channel::ChannelId;
use crate::storage::raft::client::RaftClient;
use crate::storage::raft::network::NetworkManager;
use crate::storage::raft::network_api::start_raft_api_server;
use crate::storage::raft::storage::Store;
use crate::storage::RaftStorageError;
use storage::Request;
use storage::Response;

pub mod client;
mod network;
mod network_api;
mod raft_client_service;
mod raft_service;
mod storage;

/// The unique id of the raft node.
pub type NodeId = u64;
/// Node is custom node data that can used by raft core.
pub type Node = BasicNode;

pub type LogStore = Adaptor<TypeConfig, Arc<Store>>;
pub type StateMachineStore = Adaptor<TypeConfig, Arc<Store>>;
pub type RaftCore = openraft::Raft<TypeConfig, NetworkManager, LogStore, StateMachineStore>;

pub mod error {
    use crate::storage::raft::{Node, NodeId};

    pub type RPCError<E> = openraft::error::RPCError<NodeId, Node, E>;
    pub type RaftError<E = openraft::error::Infallible> = openraft::error::RaftError<NodeId, E>;
    pub type NetworkError = openraft::error::NetworkError;

    pub type StorageError = openraft::StorageError<NodeId>;
    pub type StorageIOError = openraft::StorageIOError<NodeId>;
    pub type ForwardToLeader = openraft::error::ForwardToLeader<NodeId, Node>;
    pub type Fatal = openraft::error::Fatal<NodeId>;
    pub type ChangeMembershipError = openraft::error::ChangeMembershipError<NodeId>;
    pub type ClientWriteError = openraft::error::ClientWriteError<NodeId, Node>;
    pub type InitializeError = openraft::error::InitializeError<NodeId, Node>;
}

openraft::declare_raft_types!(
    /// Declare the type configuration for example K/V store.
    pub TypeConfig: D = Request, R = Response, NodeId = NodeId, Node = Node,
    Entry = Entry<TypeConfig>, SnapshotData = Cursor<Vec<u8>>
);

// Close is cheap, because of Raft clone is cheap.
#[derive(Clone)]
pub struct RaftServer {
    raft: Option<RaftCore>,
    server_addr: String,
    node_id: u64,
    // FIXME add more config here
}

// When main server start and before accept tcp connections, start the RaftStore.
// Include start a raft server, check snapshot data to the router etc.
impl RaftServer {
    pub fn new(node_id: u64, server_addr: String) -> RaftServer {
        RaftServer {
            raft: None,
            server_addr,
            node_id,
        }
    }

    // init nodes config in config file
    pub async fn start(&mut self) -> Result<RaftClient, RaftStorageError> {
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

        let raft = openraft::Raft::new(
            self.node_id,
            config.clone(),
            network,
            log_store,
            state_machine,
        )
        .await
        .map_err(|err| RaftStorageError::RaftError(serde_json::to_string(&err).unwrap()))?;

        self.raft = Some(raft.clone());

        self.init().await?;

        let raft_client =
            RaftClient::new(raft.clone(), store.clone(), 1, Node::new("0.0.0.0:9091"));
        let raft_client_clone = raft_client.clone();
        let raft_server_addr = self.server_addr.clone();
        tokio::spawn(async move {
            // FIXME error handle
            start_raft_api_server(raft_server_addr.as_str(), raft, raft_client_clone)
                .await
                .unwrap();
        });

        Ok(raft_client)
    }

    pub async fn init(&self) -> Result<(), RaftStorageError> {
        let mut nodes = BTreeMap::new();
        nodes.insert(1, Node::new("0.0.0.0:9091"));
        nodes.insert(2, Node::new("0.0.0.0:9092"));
        nodes.insert(3, Node::new("0.0.0.0:9093"));

        Ok(self
            .raft
            .clone()
            .ok_or_else(|| RaftStorageError::RaftServerRaftCoreIsNone)?
            .initialize(nodes)
            .await
            .map_err(|err| RaftStorageError::RaftError(err.to_string()))?)
    }
}

// Hold a raft client there, read or write to the raft.
#[derive(Clone)]
pub struct RaftStorage {
    raft_client: RaftClient,
}

impl RaftStorage {
    pub fn new(raft_client: RaftClient) -> RaftStorage {
        RaftStorage { raft_client }
    }
}

// Impl router operations here.
#[async_trait]
impl RouterStorage for RaftStorage {
    async fn get_channel_router(&self, channel_id: ChannelId) -> Value {
        todo!()
    }

    async fn update_or_insert_channel_node(&self, value: Value) -> Value {
        todo!()
    }

    async fn router_lease(&self, router: RouterId) -> Option<RouterId> {
        todo!()
    }
}
