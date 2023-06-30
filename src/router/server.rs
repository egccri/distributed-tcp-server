// A grpc server that used for transfer income operation that need send packet to the remote.

use crate::router::router_service::router_service_server::{RouterService, RouterServiceServer};
use crate::router::router_service::{RouterReply, RouterRequest};
use crate::router::RouterError;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub async fn start_raft_api_server(addr: &str) -> Result<(), RouterError> {
    let socket_addr = addr.parse()?;
    let router_service = RouterSvc::default();
    Server::builder()
        .add_service(RouterServiceServer::new(router_service))
        .serve(socket_addr)
        .await?;
    Ok(())
}

#[derive(Debug, Default)]
pub struct RouterSvc {}

#[tonic::async_trait]
impl RouterService for RouterSvc {
    async fn send_packet(
        &self,
        request: Request<RouterRequest>,
    ) -> Result<Response<RouterReply>, Status> {
        todo!()
    }
}
