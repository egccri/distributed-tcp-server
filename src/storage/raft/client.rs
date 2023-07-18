use crate::storage::raft::error::ForwardToLeader;
use crate::storage::raft::raft_client_service::raft_client_service_client::RaftClientServiceClient;
use crate::storage::raft::raft_client_service::RaftClientRequest;
use crate::storage::raft::storage::{Request, Response, Store};
use crate::storage::raft::{error, Node, NodeId, RaftCore, TypeConfig};
use crate::storage::RaftStorageError;
use openraft::error::{ClientWriteError, RaftError};
use openraft::raft::ClientWriteResponse;
use pool::MutexPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::{Channel, Error};
use tonic::Status;
use tracing::info;

#[derive(Debug, Clone)]
struct ForwardChannelBuilder;

#[async_trait::async_trait]
impl pool::PoolItemBuilder for ForwardChannelBuilder {
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

#[derive(Clone)]
pub struct RaftClient {
    inner: RaftCore,
    storage: Arc<Store>,
    // client rpc should all send to leader
    leader: Arc<Mutex<(NodeId, Node)>>,
    // grpc client pool
    channel_pool: MutexPool<ForwardChannelBuilder>,
}

impl RaftClient {
    pub fn new(
        raft: RaftCore,
        store: Arc<Store>,
        leader_node_id: NodeId,
        leader_node: Node,
    ) -> RaftClient {
        let channel_builder = ForwardChannelBuilder;
        let channel_pool = MutexPool::new(channel_builder, None);
        RaftClient {
            inner: raft,
            storage: store,
            leader: Arc::new(Mutex::new((leader_node_id, leader_node))),
            channel_pool,
        }
    }

    async fn write(
        &self,
        req: Request,
    ) -> Result<ClientWriteResponse<TypeConfig>, RaftError<NodeId, ClientWriteError<NodeId, Node>>>
    {
        let mut n_retry = 3;
        loop {
            let leader = self.leader.lock().await;
            let forward_channel = self.channel_pool.get(&leader.1.addr).await.unwrap();
            let result = self.send_rpc_to_leader(req.clone(), forward_channel).await;

            let rpc_err = match result {
                Ok(x) => return Ok(x),
                Err(rpc_err) => rpc_err,
            };

            if let Some(ForwardToLeader {
                leader_id: Some(leader_id),
                leader_node: Some(leader_node),
            }) = rpc_err.forward_to_leader()
            {
                // Update target to the new leader.
                {
                    let mut t = self.leader.lock().await;
                    *t = (*leader_id, leader_node.clone());
                }

                n_retry -= 1;
                if n_retry > 0 {
                    continue;
                }
            }
        }
    }

    pub async fn _write(
        &self,
        req: Request,
    ) -> Result<ClientWriteResponse<TypeConfig>, RaftError<NodeId, ClientWriteError<NodeId, Node>>>
    {
        self.inner.client_write(req).await
    }

    pub async fn read(&self, key: String) -> Result<String, RaftStorageError> {
        let sm = self.storage.state_machine.read().await;
        let a = sm
            .data_tree
            .get(&key)
            .ok_or_else(|| RaftStorageError::ClientKeyNotFoundError)?;
        Ok(a.clone())
    }

    async fn send_rpc_to_leader(
        &self,
        request: Request,
        forward_channel: Channel,
    ) -> Result<ClientWriteResponse<TypeConfig>, RaftError<NodeId, ClientWriteError<NodeId, Node>>>
    {
        let mut client = RaftClientServiceClient::new(forward_channel);
        let request = serde_json::to_string(&request).unwrap();
        let request = tonic::Request::new(RaftClientRequest { inner: request });
        let result = client.forward(request).await.unwrap().into_inner();

        if !result.inner.is_empty() {
            let reply: ClientWriteResponse<TypeConfig> =
                serde_json::from_str(result.inner.as_str()).unwrap();
            Ok(reply)
        } else {
            let err: RaftError<NodeId, ClientWriteError<NodeId, Node>> =
                serde_json::from_str(result.error.as_str()).unwrap();
            Err(err)
        }
    }
}
