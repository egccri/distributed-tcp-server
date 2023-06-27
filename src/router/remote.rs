use crate::protocol::packets::Packet;
use crate::router::RouterId;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tokio::sync::RwLock;
use tonic::transport::{Channel, Endpoint};

pub type RouterClients = Arc<HashMap<RouterId, (IpAddr, RouterGrpcClient)>>;

/// A remote call with flow steps:
/// local: rpc call -> make packet -> fetch router value -> make inner rpc call with value's addr and packet
/// remote: receive inner call -> send packet to the local session -> return reply packet
pub struct RouterGrpcClient {
    grpc_client: Arc<RwLock<Option<Channel>>>,
    endpoint: Endpoint,
    tx: Sender<Packet>,
}
