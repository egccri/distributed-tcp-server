use crate::protocol::PacketError;
use std::io;
use tokio_util::codec::LinesCodecError;

mod broker;
mod channel;
mod session;

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

#[derive(thiserror::Error, Debug)]
pub enum ClientSideError {}
