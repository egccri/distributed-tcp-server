use crate::storage::raft::raft_client_service::raft_client_service_server::{
    RaftClientService, RaftClientServiceServer,
};
use crate::storage::raft::raft_client_service::{RaftClientReply, RaftClientRequest};
use crate::storage::raft::raft_service::raft_service_server::{RaftService, RaftServiceServer};
use crate::storage::raft::raft_service::{RaftReply, RaftRequest};
use crate::storage::raft::RaftCore;
use crate::storage::RaftStorageError;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

// raft network api impl with grpc server
pub async fn start_raft_api_server(addr: &str) -> Result<(), RaftStorageError> {
    let socket_addr = addr.parse()?;
    let raft_service = RaftSvc::default();
    let raft_client_service = RaftClientSvc::default();
    Server::builder()
        .add_service(RaftServiceServer::new(raft_service))
        .add_service(RaftClientServiceServer::new(raft_client_service))
        .serve(socket_addr)
        .await?;
    Ok(())
}

#[derive(Debug, Default)]
pub struct RaftSvc {}

#[tonic::async_trait]
impl RaftService for RaftSvc {
    async fn forward(&self, request: Request<RaftRequest>) -> Result<Response<RaftReply>, Status> {
        todo!()
    }

    async fn append_entries(
        &self,
        request: Request<RaftRequest>,
    ) -> Result<Response<RaftReply>, Status> {
        todo!()
    }

    async fn install_snapshot(
        &self,
        request: Request<RaftRequest>,
    ) -> Result<Response<RaftReply>, Status> {
        todo!()
    }

    async fn vote(&self, request: Request<RaftRequest>) -> Result<Response<RaftReply>, Status> {
        todo!()
    }
}

#[derive(Debug, Default)]
struct RaftClientSvc {}

#[tonic::async_trait]
impl RaftClientService for RaftClientSvc {
    async fn send_message(
        &self,
        request: Request<RaftClientRequest>,
    ) -> Result<Response<RaftClientReply>, Status> {
        todo!()
    }
}
