use crate::config::ServerConfig;
use crate::protocol::packets::Packet;
use crate::protocol::PacketError;
use crate::router::server::RouterServer;
use crate::server::broker::BrokerServer;
use crate::server::session::SharedSession;
use crate::storage::raft::RaftServer;
use std::io;
use std::time::Duration;
use tokio::sync::broadcast::Receiver;
use tokio::sync::mpsc::Sender;
use tokio_util::codec::LinesCodecError;
use tracing::info;

mod broker;
pub mod channel;
pub mod session;

pub struct Cluster {
    broker: BrokerServer,
    router: RouterServer,
    #[cfg(feature = "raft-store")]
    raft: RaftServer,
}

pub async fn start(
    server_config: ServerConfig,
    server_sender: Sender<Packet>,
    ctrl_c_rx: Receiver<()>,
) -> Result<(), ServerSideError> {
    let session = SharedSession::init().await;

    #[cfg(feature = "raft-store")]
    if server_config.raft.is_none() {
        panic!("Miss raft config when raft-store feature is turn on.")
    }
    let raft_config = server_config.raft.unwrap();
    let raft_node_id = raft_config.node_id;
    let raft_network_addr = raft_config.raft_network_addr.clone();
    let mut raft_storage = RaftServer::new(raft_node_id, raft_network_addr.clone());
    let mut raft_storage_clone = raft_storage.clone();
    info!(
        "Raft Storage Server {} starting with cli config addr: {}",
        raft_node_id, raft_network_addr
    );
    tokio::spawn(async move {
        // FIXME error handle
        let _ = raft_storage_clone.start().await;
    });

    tokio::time::sleep(Duration::from_secs(2)).await;

    let router_id = server_config.router.router_id;
    let router_server_addr = server_config.router.router_server_addr.clone();
    let router = RouterServer::new(router_id, router_server_addr.clone());
    info!(
        "Router {} starting with cli config addr: {}",
        router_id, router_server_addr
    );
    let session_router = session.clone();
    tokio::spawn(async move {
        // FIXME error handle
        let _ = router.start_router_server(session_router).await;
    });

    info!(
        "Server starting with cli config addr: {:?}",
        server_config.bind_address
    );
    let iot_server = BrokerServer::bind(
        server_config.bind_address.as_str(),
        ctrl_c_rx,
        server_sender,
        session,
    )
    .await?;
    iot_server.start().await;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("channel send error with I/O : {0}")]
    ChannelSendError(#[from] io::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ServerSideError {
    #[error("Server broker start accept cause a error, child: {0}")]
    ServerAcceptError(#[from] io::Error),

    #[error("Server codec error, cause by: {0}")]
    ServerCodecError(#[from] LinesCodecError),

    #[error("Channel create fault with error: ...")]
    ChannelCreateError,

    #[error(transparent)]
    PacketError(#[from] PacketError),

    #[error("New stream first packet is not 'sign_in': {0}")]
    FirstPacketError(String),
}

// FIXME split read and write packet, read should bu ClientSideError
#[derive(thiserror::Error, Debug)]
pub enum ClientSideError {
    #[error(transparent)]
    PacketError(#[from] PacketError),
}
