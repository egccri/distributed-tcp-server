use std::io;

mod broker;
mod channel;
mod session;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("channel send error with I/O : {0}")]
    ChannelSendError(#[from] io::Error),
}
