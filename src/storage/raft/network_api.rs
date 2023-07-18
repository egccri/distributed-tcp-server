use crate::storage::raft::client::RaftClient;
use crate::storage::raft::raft_client_service::raft_client_service_server::{
    RaftClientService, RaftClientServiceServer,
};
use crate::storage::raft::raft_client_service::{RaftClientReply, RaftClientRequest};
use crate::storage::raft::raft_service::raft_service_server::{RaftService, RaftServiceServer};
use crate::storage::raft::raft_service::{RaftReply, RaftRequest};
use crate::storage::raft::{NodeId, RaftCore, TypeConfig};
use crate::storage::RaftStorageError;
use openraft::error::RaftError;
use openraft::raft::ClientWriteResponse;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tracing::info;

// raft network api impl with grpc server
pub async fn start_raft_api_server(
    addr: &str,
    raft_core: RaftCore,
    raft_client: RaftClient,
) -> Result<(), RaftStorageError> {
    let socket_addr = addr.parse()?;
    let raft_service = RaftSvc::new(raft_core.clone());
    let raft_client_service = RaftClientSvc::new(raft_client);
    Server::builder()
        .add_service(RaftServiceServer::new(raft_service))
        .add_service(RaftClientServiceServer::new(raft_client_service))
        .serve(socket_addr)
        .await?;
    Ok(())
}

pub struct RaftSvc {
    raft_core: RaftCore,
}

impl RaftSvc {
    pub fn new(raft_core: RaftCore) -> RaftSvc {
        RaftSvc { raft_core }
    }
}

#[tonic::async_trait]
impl RaftService for RaftSvc {
    // append_entries:call_core: openraft::raft: failure sending tx to RaftCore; message: AppendEntries: vote=4-1:committed, prev_log_id=None, leader_commit=None, entries=[0-0-0:membership: voters:[{1:{BasicNode { addr: "0.0.0.0:9091" }},2:{BasicNode { addr: "0.0.0.0:9092" }},3:{BasicNode { addr: "0.0.0.0:9093" }}}], learners:[],4-1-1:blank] core_result=Err(Panicked)
    async fn append_entries(
        &self,
        request: Request<RaftRequest>,
    ) -> Result<Response<RaftReply>, Status> {
        let request = request.into_inner().data;
        info!("Received append entries call with payload {}", &request);
        let rpc = serde_json::from_str(request.as_str())
            .map_err(|err| Status::internal(err.to_string()))?;
        let res = self
            .raft_core
            .append_entries(rpc)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        let reply = serde_json::to_string(&res).map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(RaftReply { data: reply }))
    }

    async fn install_snapshot(
        &self,
        request: Request<RaftRequest>,
    ) -> Result<Response<RaftReply>, Status> {
        let request = request.into_inner().data;
        info!("Received install snapshot call with payload {}", &request);
        let rpc = serde_json::from_str(request.as_str())
            .map_err(|err| Status::internal(err.to_string()))?;
        let res = self
            .raft_core
            .install_snapshot(rpc)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        let reply = serde_json::to_string(&res).map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(RaftReply { data: reply }))
    }

    async fn vote(&self, request: Request<RaftRequest>) -> Result<Response<RaftReply>, Status> {
        let request = request.into_inner().data;
        info!("Received vote call with payload {}", &request);
        let rpc = serde_json::from_str(request.as_str())
            .map_err(|err| Status::internal(err.to_string()))?;
        let res = self
            .raft_core
            .vote(rpc)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        let reply = serde_json::to_string(&res).map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(RaftReply { data: reply }))
    }
}

struct RaftClientSvc {
    raft_client: RaftClient,
}

impl RaftClientSvc {
    pub fn new(raft_client: RaftClient) -> Self {
        RaftClientSvc { raft_client }
    }
}

#[tonic::async_trait]
impl RaftClientService for RaftClientSvc {
    async fn forward(
        &self,
        request: Request<RaftClientRequest>,
    ) -> Result<Response<RaftClientReply>, Status> {
        let request = request.into_inner().inner;
        info!("Received forward request with payload {}", &request);
        let request = serde_json::from_str(request.as_str()).unwrap();
        let result = self.raft_client._write(request).await;
        match result {
            Ok(response) => {
                let json = serde_json::to_string(&response).unwrap();
                let reply = RaftClientReply {
                    inner: json,
                    error: "".to_string(),
                };
                Ok(Response::new(reply))
            }
            Err(err) => {
                let json = serde_json::to_string(&err).unwrap();
                let reply = RaftClientReply {
                    inner: "".to_string(),
                    error: json,
                };
                Ok(Response::new(reply))
            }
        }
    }
}
