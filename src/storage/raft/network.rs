use super::TypeConfig;
use super::{Node, NodeId};
use crate::storage::raft::raft_service::raft_service_client::RaftServiceClient;
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

    // Rpc call here.
    pub fn call() {}
}

#[async_trait]
impl RaftNetwork<TypeConfig> for Network {
    // how to ensure the log's seq in the raft core, there's multi rpc calls.
    // log index and term
    async fn send_append_entries(
        &mut self,
        rpc: AppendEntriesRequest<TypeConfig>,
    ) -> Result<AppendEntriesResponse<NodeId>, RPCError<NodeId, Node, RaftError<NodeId>>> {
        todo!()
    }

    async fn send_install_snapshot(
        &mut self,
        rpc: InstallSnapshotRequest<TypeConfig>,
    ) -> Result<
        InstallSnapshotResponse<NodeId>,
        RPCError<NodeId, Node, RaftError<NodeId, InstallSnapshotError>>,
    > {
        todo!()
    }

    async fn send_vote(
        &mut self,
        rpc: VoteRequest<NodeId>,
    ) -> Result<VoteResponse<NodeId>, RPCError<NodeId, Node, RaftError<NodeId>>> {
        todo!()
    }
}

impl NetworkManager {
    pub async fn send_append_entries(
        node: Node,
    ) -> Result<(), RPCError<NodeId, Node, RaftError<NodeId>>> {
        // FIXME map unwrap to the RPCError.
        let client = RaftServiceClient::connect(node.addr).await.unwrap();
        Ok(())
    }
}
