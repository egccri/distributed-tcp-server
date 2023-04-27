use crate::protocol::packets::Packet;
use crate::server::channel::Channel;
use crate::server::session::SharedSession;
use crate::server::ServerSideError;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc};
use tokio::{io, select};
use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};
use tracing::{debug, error, info};

#[derive(Debug)]
pub struct BrokerServer {
    listener: TcpListener,
    codec: LinesCodec,
    ctrl_c_rx: broadcast::Receiver<()>,
    server_sender: mpsc::Sender<Packet>,
    session: SharedSession,
}

impl BrokerServer {
    pub async fn bind(
        addr: &str,
        ctrl_c_rx: broadcast::Receiver<()>,
        server_sender: mpsc::Sender<Packet>,
        session: SharedSession,
    ) -> io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(BrokerServer {
            listener,
            codec: LinesCodec::new(),
            ctrl_c_rx,
            server_sender,
            session,
        })
    }

    pub async fn start(mut self) {
        loop {
            select! {
                _ = self.ctrl_c_rx.recv() => {
                    info!("Stopping server broker listener at {}", chrono::Local::now());
                    break;
                }
                _ = Self::accept(self.codec.clone(), &self.listener, self.server_sender.clone(), self.session.clone()) => {}
            }
        }
        info!("Server broker has stopped!");
    }

    async fn accept(
        codec: LinesCodec,
        listener: &TcpListener,
        server_sender: mpsc::Sender<Packet>,
        session: SharedSession,
    ) {
        let (socket, remote) = match listener.accept().await {
            Ok((tcp_stream, socket_address)) => (tcp_stream, socket_address),
            Err(err) => {
                error!("{}", ServerSideError::ServerAcceptError(err));
                return;
            }
        };

        let (framed_writer, mut framed_reader) = Framed::new(socket, codec).split();

        // Some protocol maybe use sign packet message to create connections, like username, password etc.
        // There only log sign in packet.
        let first_packet = match Self::first_packet(&mut framed_reader).await {
            Ok(first_packet) => first_packet,
            Err(err) => {
                error!("{}", err);
                return;
            }
        };
        info!("A new client sign in: {:?}", first_packet);

        //todo! Channel should hold a heartbeat timer, used to update channel status.
        //todo! And session need a background task to clear closed channel, it's
        let (client_sender, client_receiver) = broadcast::channel::<Packet>(10);
        let channel = match Self::create_channel(remote, client_sender) {
            Ok(channel) => channel,
            Err(err) => {
                error!("{}", err);
                return;
            }
        };
        session.add(channel).await;

        tokio::spawn(async move {
            Self::handle_writeable(framed_writer, client_receiver).await;
        });
    }

    async fn first_packet(
        framed_reader: &mut SplitStream<Framed<TcpStream, LinesCodec>>,
    ) -> Result<Packet, ServerSideError> {
        let Some(frame) = framed_reader.next().await
            else { return Err(ServerSideError::FirstPacketError("None".to_string())) };
        let raw = match frame {
            Ok(raw) => raw,
            Err(err) => {
                return Err(ServerSideError::ServerCodecError(err));
            }
        };
        let is_first_packet = Packet::check_sign_in_packet(raw.as_str())?;
        if is_first_packet {
            Ok(Packet::read(raw)?)
        } else {
            Err(ServerSideError::FirstPacketError(raw))
        }
    }

    fn create_channel(
        remote_address: SocketAddr,
        client_sender: broadcast::Sender<Packet>,
    ) -> Result<Channel, ServerSideError> {
        Ok(Channel::new(remote_address, client_sender))
    }

    async fn handle_writeable(
        mut framed_writer: SplitSink<Framed<TcpStream, LinesCodec>, String>,
        mut receiver: broadcast::Receiver<Packet>,
    ) {
        while let Ok(packet) = receiver.recv().await {
            debug!("Channel try send packet: {}", &packet);
            let raw = match Packet::write(packet) {
                Ok(raw) => raw,
                Err(err) => {
                    error!("Packet write into raw cause a error: {}", err);
                    continue;
                }
            };
            match framed_writer.send(raw).await {
                Ok(_) => {
                    let _ = framed_writer.flush().await;
                }
                Err(err) => {
                    error!("Channel send packet with error: {err}");
                    match err {
                        LinesCodecError::MaxLineLengthExceeded => {
                            continue;
                        }
                        LinesCodecError::Io(_) => {
                            // todo! Signal readable task or add a flag to channel, there's only change channel status.
                            let _ = framed_writer.close().await;
                        }
                    }
                }
            }
        }
    }
}
