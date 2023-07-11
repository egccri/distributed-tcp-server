use crate::protocol::packets::Packet;
use crate::router::router_service::router_service_client::RouterServiceClient;
use crate::router::router_service::RouterRequest;
use crate::router::{RouterError, RouterId, Value};
use crate::server::channel::ChannelId;
use pool::MutexPool;
use std::net::IpAddr;
use tonic::transport::{Channel, Endpoint};
use tracing::info;

#[derive(Debug, Clone)]
struct ChannelBuilder;

#[async_trait::async_trait]
impl pool::PoolItemBuilder for ChannelBuilder {
    type Token = String;
    type Item = Channel;
    type Error = tonic::transport::Error;

    async fn build(&self, addr: &Self::Token) -> Result<Self::Item, Self::Error> {
        info!("Building channel for addr: {}", &addr);
        Endpoint::new(addr.clone())?.connect().await
    }
}

/// A remote call with flow steps:
/// local: rpc call -> make packet -> fetch router value -> make inner rpc call with value's addr and packet
/// remote: receive inner call -> send packet to the local session -> return reply packet
#[derive(Debug, Clone)]
pub struct Remotes {
    inner: MutexPool<ChannelBuilder>,
}

impl Remotes {
    pub async fn new() -> Remotes {
        let channel_builder = ChannelBuilder;
        let channel_pool = MutexPool::new(channel_builder, None);
        Remotes {
            inner: channel_pool,
        }
    }

    pub async fn send(&self, value: Value, packet: Packet) -> Result<Packet, RouterError> {
        let channel_id = value.channel_id;
        let router_id: RouterId = value.router.router;
        let router_addr: String = value.router.remote_addr;

        let channel = self
            .inner
            .get(&router_addr)
            .await
            .map_err(|err| RouterError::ChannelConnectError)?;
        // find RouterGrpcClient
        let reply = self.send_packet(channel, channel_id, packet).await?;
        Ok(reply)
    }

    async fn send_packet(
        &self,
        channel: Channel,
        channel_id: ChannelId,
        packet: Packet,
    ) -> Result<Packet, RouterError> {
        let raw = packet.write()?;

        let reply = {
            let mut client = RouterServiceClient::new(channel);
            let message = RouterRequest {
                channel_id: channel_id.into(),
                packet: raw,
            };
            let reply = client
                .send_packet(tonic::Request::new(message))
                .await
                .map_err(|status| RouterError::ReplyErrorStatus(status))?;
            reply
        };

        Ok(Packet::read(reply.into_inner().packet)?)
    }

    // init with config routers, maybe not use
    #[warn(dead_code)]
    pub async fn init(&mut self, routers: Vec<(RouterId, IpAddr)>) -> Result<(), RouterError> {
        for (router_id, ip_addr) in routers.iter() {
            let _ = self.inner.get(&ip_addr.to_string()).await;
        }
        Ok(())
    }
}
