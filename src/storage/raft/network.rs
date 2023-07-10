use super::TypeConfig;
use super::{Node, NodeId};
use crate::storage::raft::raft_service::raft_service_client::RaftServiceClient;
use crate::storage::raft::raft_service::RaftRequest;
use openraft::async_trait::async_trait;
use openraft::error::{InstallSnapshotError, NetworkError, RPCError, RaftError};
use openraft::raft::{
    AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest, InstallSnapshotResponse,
    VoteRequest, VoteResponse,
};
use openraft::MessageSummary;
use openraft::{RaftNetwork, RaftNetworkFactory, RaftTypeConfig};
use pool::MutexPool;
use tonic::transport::{Channel, Error};
use tracing::info;

#[derive(Debug, Clone)]
struct ChannelBuilder;

#[async_trait::async_trait]
impl pool::PoolItemBuilder for ChannelBuilder {
    type Token = String;
    type Item = Channel;
    type Error = Error;

    async fn build(&self, addr: &Self::Token) -> Result<Self::Item, Self::Error> {
        info!("Building channel for addr: {}", &addr);
        tonic::transport::Endpoint::new(addr.clone())?
            .connect()
            .await
    }
}

#[derive(Debug, Clone)]
pub struct NetworkManager {
    channel_pool: MutexPool<ChannelBuilder>,
}

#[derive(Debug)]
pub struct Network {
    manager: NetworkManager,
    target: NodeId,
    target_node: Node,
}

#[async_trait]
impl RaftNetworkFactory<TypeConfig> for NetworkManager {
    type Network = Network;

    async fn new_client(
        &mut self,
        target: <TypeConfig as RaftTypeConfig>::NodeId,
        target_node: &<TypeConfig as RaftTypeConfig>::Node,
    ) -> Self::Network {
        Network {
            manager: self.clone(),
            target,
            target_node: target_node.clone(),
        }
    }
}

impl NetworkManager {
    pub fn new() -> NetworkManager {
        let channel_builder = ChannelBuilder;
        let channel_pool = MutexPool::new(channel_builder, None);
        NetworkManager { channel_pool }
    }

    pub async fn make_client(&self, target_node: Node) -> Result<Channel, Error> {
        let addr = format!("http://{}", target_node.addr);
        self.channel_pool.get(&addr).await
    }

    pub async fn send_append_entries(
        &self,
        payload: String,
        target: NodeId,
        target_node: Node,
    ) -> Result<AppendEntriesResponse<NodeId>, RPCError<NodeId, Node, RaftError<NodeId>>> {
        info!(
            "Send append entries call to [target: {}, node: {}], with payload {}",
            target, &target_node, &payload
        );
        let channel = self
            .make_client(target_node)
            .await
            .map_err(|err| RPCError::Network(NetworkError::new(&err)))?;
        let mut client = RaftServiceClient::new(channel);

        let request = RaftRequest { data: payload };
        let result = client
            .append_entries(tonic::Request::new(request))
            .await
            .map_err(|e| RPCError::Network(NetworkError::new(&e)))?;
        let result: AppendEntriesResponse<NodeId> =
            serde_json::from_str(result.into_inner().data.as_str())
                .map_err(|e| RPCError::Network(NetworkError::new(&e)))?;
        Ok(result)
    }

    pub async fn send_install_snapshot(
        &self,
        payload: String,
        target: NodeId,
        target_node: Node,
    ) -> Result<
        InstallSnapshotResponse<NodeId>,
        RPCError<NodeId, Node, RaftError<NodeId, InstallSnapshotError>>,
    > {
        info!(
            "Send install snapshot call to [target: {}, node: {}], with payload {}",
            target, &target_node, &payload
        );
        let channel = self
            .make_client(target_node)
            .await
            .map_err(|err| RPCError::Network(NetworkError::new(&err)))?;
        let mut client = RaftServiceClient::new(channel);

        let request = RaftRequest { data: payload };
        let result = client
            .install_snapshot(tonic::Request::new(request))
            .await
            .map_err(|e| RPCError::Network(NetworkError::new(&e)))?;
        let result: InstallSnapshotResponse<NodeId> =
            serde_json::from_str(result.into_inner().data.as_str())
                .map_err(|e| RPCError::Network(NetworkError::new(&e)))?;
        Ok(result)
    }

    pub async fn send_vote(
        &self,
        payload: String,
        target: NodeId,
        target_node: Node,
    ) -> Result<VoteResponse<NodeId>, RPCError<NodeId, Node, RaftError<NodeId>>> {
        info!(
            "Send vote call to [target: {}, node: {}], with payload {}",
            target, &target_node, &payload
        );
        let mut client = RaftServiceClient::connect(format!("http://{}", target_node.addr))
            .await
            .map_err(|e| RPCError::Network(NetworkError::new(&e)))?;
        let request = RaftRequest { data: payload };
        let result = client
            .vote(tonic::Request::new(request))
            .await
            .map_err(|e| RPCError::Network(NetworkError::new(&e)))?;
        let result: VoteResponse<NodeId> = serde_json::from_str(result.into_inner().data.as_str())
            .map_err(|e| RPCError::Network(NetworkError::new(&e)))?;
        Ok(result)
    }
}

#[async_trait]
impl RaftNetwork<TypeConfig> for Network {
    // how to ensure the log's seq in the raft core, there's multi rpc calls.
    // log index and term
    async fn send_append_entries(
        &mut self,
        rpc: AppendEntriesRequest<TypeConfig>,
    ) -> Result<AppendEntriesResponse<NodeId>, RPCError<NodeId, Node, RaftError<NodeId>>> {
        info!(
            "send_append_entries target: {}, rpc: {}",
            self.target,
            rpc.summary()
        );
        let json = serde_json::to_string(&rpc).unwrap();
        self.manager
            .send_append_entries(json, self.target, self.target_node.clone())
            .await
    }

    async fn send_install_snapshot(
        &mut self,
        rpc: InstallSnapshotRequest<TypeConfig>,
    ) -> Result<
        InstallSnapshotResponse<NodeId>,
        RPCError<NodeId, Node, RaftError<NodeId, InstallSnapshotError>>,
    > {
        info!(
            "send_install_snapshot target: {}, rpc: {}",
            self.target,
            rpc.summary()
        );
        let json = serde_json::to_string(&rpc).unwrap();
        self.manager
            .send_install_snapshot(json, self.target, self.target_node.clone())
            .await
    }

    async fn send_vote(
        &mut self,
        rpc: VoteRequest<NodeId>,
    ) -> Result<VoteResponse<NodeId>, RPCError<NodeId, Node, RaftError<NodeId>>> {
        info!("send_vote: target: {} rpc: {}", self.target, rpc.summary());

        let json = serde_json::to_string(&rpc).unwrap();
        self.manager
            .send_vote(json, self.target, self.target_node.clone())
            .await
    }
}

impl NetworkManager {}
