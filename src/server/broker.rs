use crate::protocol::packets::Packet;
use crate::server::session::SharedSession;
use tokio::io;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::mpsc::Sender;
use tokio_util::codec::LinesCodec;

#[derive(Debug)]
pub struct BrokerServer {
    listener: TcpListener,
    codec: LinesCodec,
    ctrl_c_rx: broadcast::Receiver<()>,
    channel_sender: Sender<Packet>,
    session: SharedSession,
}

impl BrokerServer {
    pub async fn bind(
        addr: &str,
        ctrl_c_rx: broadcast::Receiver<()>,
        channel_sender: Sender<Packet>,
        session: SharedSession,
    ) -> io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(BrokerServer {
            listener,
            codec: LinesCodec::new(),
            ctrl_c_rx,
            channel_sender,
            session,
        })
    }
}
