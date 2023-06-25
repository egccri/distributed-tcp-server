use crate::storage::raft::raft_service::raft_service_server::{RaftService, RaftServiceServer};
use crate::storage::raft::raft_service::{RaftReply, RaftRequest};
use crate::storage::raft::RaftCore;
use crate::storage::RaftStorageError;
use openraft::Raft;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub struct Cluster {
    raft: RaftCore,
}

// raft network api impl with grpc server
pub async fn start_raft_api_server(addr: &str) -> Result<(), RaftStorageError> {
    let socket_addr = addr.parse()?;
    let raft_service = RaftSvc::default();
    Server::builder()
        .add_service(RaftServiceServer::new(raft_service))
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
