use std::net::AddrParseError;
use tonic::transport::Error;

pub mod raft;
pub mod redis;

#[derive(Debug, thiserror::Error)]
pub enum RaftStorageError {
    #[error("Socket addr parse error, cause by {0}.")]
    SocketAddrParseError(#[from] AddrParseError),

    #[error("Raft server api start error: cause by {0}")]
    TonicServerError(#[from] Error),
}
