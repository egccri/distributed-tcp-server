use crate::protocol::packets::Packet;
use crate::router::RouterId;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tokio::sync::RwLock;
use tonic::transport::{Channel, Endpoint};

pub type RouterClients = Arc<HashMap<RouterId, (IpAddr, RouterGrpcClient)>>;

pub struct RouterGrpcClient {
    grpc_client: Arc<RwLock<Option<Channel>>>,
    endpoint: Endpoint,
    tx: Sender<Packet>,
}
