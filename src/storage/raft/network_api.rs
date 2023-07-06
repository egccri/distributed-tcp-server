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
pub async fn start_raft_api_server(
    addr: &str,
    raft_core: RaftCore,
) -> Result<(), RaftStorageError> {
    let socket_addr = addr.parse()?;
    let raft_service = RaftSvc::new(raft_core.clone());
    let raft_client_service = RaftClientSvc::new(raft_core);
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
    async fn forward(&self, request: Request<RaftRequest>) -> Result<Response<RaftReply>, Status> {
        todo!()
    }

    async fn append_entries(
        &self,
        request: Request<RaftRequest>,
    ) -> Result<Response<RaftReply>, Status> {
        let request = request.into_inner().data;
        let rpc = serde_json::from_str(request.as_str()).unwrap();
        let res = self.raft_core.append_entries(rpc).await.unwrap();
        let reply = serde_json::to_string(&res).unwrap();
        Ok(Response::new(RaftReply {
            data: reply,
            error: "".to_string(),
        }))
    }

    async fn install_snapshot(
        &self,
        request: Request<RaftRequest>,
    ) -> Result<Response<RaftReply>, Status> {
        let request = request.into_inner().data;
        let rpc = serde_json::from_str(request.as_str()).unwrap();
        let res = self.raft_core.install_snapshot(rpc).await.unwrap();
        let reply = serde_json::to_string(&res).unwrap();
        Ok(Response::new(RaftReply {
            data: reply,
            error: "".to_string(),
        }))
    }

    async fn vote(&self, request: Request<RaftRequest>) -> Result<Response<RaftReply>, Status> {
        let request = request.into_inner().data;
        let rpc = serde_json::from_str(request.as_str()).unwrap();
        let res = self.raft_core.vote(rpc).await.unwrap();
        let reply = serde_json::to_string(&res).unwrap();
        Ok(Response::new(RaftReply {
            data: reply,
            error: "".to_string(),
        }))
    }
}

struct RaftClientSvc {
    raft_core: RaftCore,
}

impl RaftClientSvc {
    pub fn new(raft_core: RaftCore) -> Self {
        RaftClientSvc { raft_core }
    }
}

#[tonic::async_trait]
impl RaftClientService for RaftClientSvc {
    async fn send_message(
        &self,
        request: Request<RaftClientRequest>,
    ) -> Result<Response<RaftClientReply>, Status> {
        todo!()
    }
}
