use std::path::PathBuf;
// A grpc server that used for transfer income operation that need send packet to the remote.
use crate::router::remote::Remotes;
use crate::router::router_service::router_service_server::{RouterService, RouterServiceServer};
use crate::router::router_service::{RouterReply, RouterRequest};
use crate::router::{RouterError, RouterId};
use crate::server::session::SharedSession;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub struct RouterServer {
    id: RouterId,
    addr: String,
    remotes: Option<Remotes>,
}

impl RouterServer {
    pub fn new(id: RouterId, addr: String) -> RouterServer {
        RouterServer {
            id,
            addr,
            remotes: None,
        }
    }

    pub async fn start_router_server(
        &self,
        local_session: SharedSession,
    ) -> Result<(), RouterError> {
        let socket_addr = self.addr.as_str().parse()?;
        let router_service = RouterSvc::new(local_session);
        Server::builder()
            .add_service(RouterServiceServer::new(router_service))
            .serve(socket_addr)
            .await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct RouterSvc {
    local_session: SharedSession,
}

impl RouterSvc {
    pub fn new(local_session: SharedSession) -> RouterSvc {
        RouterSvc { local_session }
    }
}

#[tonic::async_trait]
impl RouterService for RouterSvc {
    async fn send_packet(
        &self,
        request: Request<RouterRequest>,
    ) -> Result<Response<RouterReply>, Status> {
        todo!()
    }
}
