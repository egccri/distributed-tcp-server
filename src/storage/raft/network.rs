use super::TypeConfig;
use super::{Node, NodeId};
use openraft::async_trait::async_trait;
use openraft::error::{InstallSnapshotError, RPCError, RaftError};
use openraft::raft::{
    AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest, InstallSnapshotResponse,
    VoteRequest, VoteResponse,
};
use openraft::{RaftNetwork, RaftNetworkFactory, RaftTypeConfig};
use std::sync::Arc;
use tonic::transport::channel::Channel;

pub struct NetworkManager {
    // FIXME Grpc client channel with pool
    connections: Arc<Channel>,
}

pub struct Network {
    manager: NetworkManager,
}

#[async_trait]
impl RaftNetworkFactory<TypeConfig> for NetworkManager {
    type Network = Network;

    async fn new_client(
        &mut self,
        target: <TypeConfig as RaftTypeConfig>::NodeId,
        node: &<TypeConfig as RaftTypeConfig>::Node,
    ) -> Self::Network {
        todo!()
    }
}

impl NetworkManager {
    pub fn new() -> NetworkManager {
        todo!()
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
