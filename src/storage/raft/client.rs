use crate::storage::raft::raft_client_service::raft_client_service_client::RaftClientServiceClient;
use crate::storage::raft::raft_client_service::RaftClientRequest;
use crate::storage::raft::storage::{Request, Response};
use crate::storage::raft::NodeId;
use crate::storage::RaftStorageError;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct RaftClient {
    // client rpc should all send to leader
    leader: Arc<Mutex<(NodeId, String)>>, // grpc client pool
}

impl RaftClient {
    pub fn connect() -> anyhow::Result<()> {
        todo!()
    }

    pub fn dis_connect() -> anyhow::Result<()> {
        todo!()
    }

    pub fn get_client_node_id(channel_id: String) -> anyhow::Result<NodeId> {
        todo!()
    }

    pub fn broker_shutdown() -> anyhow::Result<()> {
        todo!()
    }

    async fn send_rpc(&self, request: Request) -> Result<Response, RaftStorageError> {
        let (_, mut addr) = self.leader.lock().await.clone();
        let mut client = RaftClientServiceClient::connect(addr).await?;
        let request = serde_json::to_string(&request).unwrap();
        let request = tonic::Request::new(RaftClientRequest { inner: request });
        // FIXME catch rpc error and retry and update leader addr
        let result = client.send_message(request).await.unwrap();
        let response: Response = serde_json::from_str(result.into_inner().inner.as_str()).unwrap();
        Ok(response)
    }
}
