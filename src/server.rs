use crate::config::ServerConfig;
use crate::protocol::packets::Packet;
use crate::protocol::PacketError;
use crate::server::broker::BrokerServer;
use crate::server::session::SharedSession;
use std::io;
use tokio::sync::broadcast::Receiver;
use tokio::sync::mpsc::Sender;
use tokio_util::codec::LinesCodecError;
use tracing::info;

mod broker;
pub mod channel;
mod session;

pub async fn start(
    server_config: ServerConfig,
    server_sender: Sender<Packet>,
    ctrl_c_rx: Receiver<()>,
) -> Result<(), ServerSideError> {
    let session = SharedSession::init().await;
    info!(
        "Server starting with cli config addr: {:?}",
        server_config.bind_address
    );
    let iot_server = BrokerServer::bind(
        server_config.bind_address.as_str(),
        ctrl_c_rx,
        server_sender,
        session.clone(),
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
