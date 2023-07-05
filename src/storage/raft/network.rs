use super::TypeConfig;
use super::{Node, NodeId};
use crate::storage::raft::raft_service::raft_service_client::RaftServiceClient;
use crate::storage::raft::raft_service::RaftRequest;
use openraft::async_trait::async_trait;
use openraft::error::{InstallSnapshotError, RPCError, RaftError};
use openraft::raft::{
    AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest, InstallSnapshotResponse,
    VoteRequest, VoteResponse,
};
use openraft::{RaftNetwork, RaftNetworkFactory, RaftTypeConfig};

// 无状态？
#[derive(Debug, Clone)]
pub struct NetworkManager {
    // FIXME Grpc client channel with pool
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
        NetworkManager {}
    }

    pub async fn send_append_entries(
        &self,
        json: String,
        target: NodeId,
        target_node: Node,
    ) -> Result<AppendEntriesResponse<NodeId>, RPCError<NodeId, Node, RaftError<NodeId>>> {
        // FIXME map error to the RPCError.
        let mut client = RaftServiceClient::connect(target_node.addr).await.unwrap();
        let request = RaftRequest { data: json };
        let result = client
            .append_entries(tonic::Request::new(request))
            .await
            .unwrap();
        let result = serde_json::from_str(result.into_inner().data.as_str()).unwrap();
        Ok(result)
    }

    pub async fn send_install_snapshot(
        &self,
        json: String,
        target: NodeId,
        target_node: Node,
    ) -> Result<
        InstallSnapshotResponse<NodeId>,
        RPCError<NodeId, Node, RaftError<NodeId, InstallSnapshotError>>,
    > {
        // FIXME map error to the RPCError.
        let mut client = RaftServiceClient::connect(target_node.addr).await.unwrap();
        let request = RaftRequest { data: json };
        let result = client
            .install_snapshot(tonic::Request::new(request))
            .await
            .unwrap();
        let result = serde_json::from_str(result.into_inner().data.as_str()).unwrap();
        Ok(result)
    }

    pub async fn send_vote(
        &self,
        json: String,
        target: NodeId,
        target_node: Node,
    ) -> Result<VoteResponse<NodeId>, RPCError<NodeId, Node, RaftError<NodeId>>> {
        // FIXME map error to the RPCError.
        let mut client = RaftServiceClient::connect(target_node.addr).await.unwrap();
        let request = RaftRequest { data: json };
        let result = client.vote(tonic::Request::new(request)).await.unwrap();
        let result = serde_json::from_str(result.into_inner().data.as_str()).unwrap();
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
        let json = serde_json::to_string(&rpc).unwrap();
        self.manager
            .send_install_snapshot(json, self.target, self.target_node.clone())
            .await
    }

    async fn send_vote(
        &mut self,
        rpc: VoteRequest<NodeId>,
    ) -> Result<VoteResponse<NodeId>, RPCError<NodeId, Node, RaftError<NodeId>>> {
        let json = serde_json::to_string(&rpc).unwrap();
        self.manager
            .send_vote(json, self.target, self.target_node.clone())
            .await
    }
}

impl NetworkManager {}