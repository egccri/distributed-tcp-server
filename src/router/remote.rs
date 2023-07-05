use crate::protocol::packets::Packet;
use crate::router::router_service::router_service_client::RouterServiceClient;
use crate::router::router_service::RouterRequest;
use crate::router::{RouterError, RouterId};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::{Channel, Endpoint};

// if need add lock here, or use channel
pub type RouterClients = Arc<RwLock<HashMap<RouterId, (IpAddr, RouterGrpcClient)>>>;

#[derive(Debug, Clone)]
pub struct Remotes {
    inner: RouterClients,
}

/// A remote call with flow steps:
/// local: rpc call -> make packet -> fetch router value -> make inner rpc call with value's addr and packet
/// remote: receive inner call -> send packet to the local session -> return reply packet
#[derive(Debug, Clone)]
pub struct RouterGrpcClient {
    grpc_client: Option<Channel>,
    endpoint: Endpoint,
}

impl RouterGrpcClient {
    pub async fn new(addr: IpAddr) -> Result<RouterGrpcClient, RouterError> {
        let endpoint = Channel::from_shared(format!("http://{}", addr))?;
        Ok(Self {
            grpc_client: endpoint.connect().await.ok(),
            endpoint,
        })
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) {
        self.endpoint = endpoint
    }

    pub async fn connect(&self) -> Option<Channel> {
        self.endpoint.connect().await.ok()
    }

    async fn send_packet(&self, packet: Packet) -> Result<Packet, RouterError> {
        let raw = packet.write()?;

        let reply = {
            let channel = self
                .grpc_client
                .clone()
                .ok_or_else(|| RouterError::ChannelConnectError)?;
            let mut client = RouterServiceClient::new(channel);
            let message = RouterRequest { packet: raw };
            let reply = client
                .send_packet(tonic::Request::new(message))
                .await
                .map_err(|status| RouterError::ReplyErrorStatus(status))?;
            reply
        };

        Ok(Packet::read(reply.into_inner().packet)?)
    }
}

impl Remotes {
    pub async fn new() -> Remotes {
        Remotes {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // init with config routers
    pub async fn init(&mut self, routers: Vec<(RouterId, IpAddr)>) -> Result<(), RouterError> {
        for (router_id, ip_addr) in routers.iter() {
            let client = {
                self.create_client(router_id.clone(), ip_addr.clone())
                    .await?
            };
            self.inner
                .write()
                .await
                .insert(router_id.clone(), (ip_addr.clone(), client));
        }
        Ok(())
    }

    pub async fn create_client(
        &mut self,
        router_id: RouterId,
        router_addr: IpAddr,
    ) -> Result<RouterGrpcClient, RouterError> {
        let client = RouterGrpcClient::new(router_addr).await?;
        let client_clone = client.clone();
        let _ = self
            .inner
            .write()
            .await
            .insert(router_id, (router_addr, client_clone));
        Ok(client)
    }

    pub async fn send_packet(&mut self, packet: Packet) -> Result<Packet, RouterError> {
        // TODO find packet owns node in the storage
        let router_id: RouterId = 1;
        let router_addr: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
        // find RouterGrpcClient
        let reply = self
            .find_router_and_send(router_id, router_addr, packet)
            .await?;
        Ok(reply)
    }

    pub async fn find_router_and_send(
        &mut self,
        router_id: RouterId,
        router_addr: IpAddr,
        packet: Packet,
    ) -> Result<Packet, RouterError> {
        if let Some((a, b)) = self.inner.read().await.get(&router_id) {
            if *a == router_addr {
                return Ok(b.send_packet(packet).await?);
            }
        };
        let reply = self
            .create_client(router_id, router_addr)
            .await?
            .send_packet(packet.clone())
            .await?;
        Ok(reply)
    }
}
