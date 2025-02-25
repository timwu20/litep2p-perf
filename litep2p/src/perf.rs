use futures::StreamExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use litep2p::{
    codec::ProtocolCodec,
    protocol::{Direction, TransportEvent, TransportService, UserProtocol},
    substream::Substream,
    PeerId, ProtocolName,
};

const PROTOCOL_NAME: &str = "/litep2p-perf/1.0.0";
const LOG_TARGET: &str = "litep2p-perf";

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PerfMode {
    Server,
    Client,
}

pub struct Perf {
    mode: PerfMode,
    upload_bytes: u64,
    download_bytes: u64,
}

impl Perf {
    pub fn new(mode: PerfMode, upload_bytes: u64, download_bytes: u64) -> Self {
        Self {
            mode,
            upload_bytes,
            download_bytes,
        }
    }

    async fn read_u64(substream: &mut Substream) -> litep2p::Result<u64> {
        let mut buf = [0u8; 8];
        substream.read_exact(&mut buf).await?;
        Ok(u64::from_be_bytes(buf))
    }

    async fn write_u64(substream: &mut Substream, value: u64) -> litep2p::Result<()> {
        substream.write_all(&value.to_be_bytes()).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl UserProtocol for Perf {
    fn protocol(&self) -> ProtocolName {
        PROTOCOL_NAME.into()
    }

    fn codec(&self) -> ProtocolCodec {
        ProtocolCodec::Unspecified
    }

    async fn run(self: Box<Self>, mut service: TransportService) -> litep2p::Result<()> {
        loop {
            tokio::select! {
                event = service.next() => match event {
                    Some(TransportEvent::ConnectionEstablished { peer, .. }) => {
                        if let PerfMode::Client = self.mode {
                            service.open_substream(peer).unwrap();
                        }
                    }
                    Some(TransportEvent::ConnectionClosed { peer }) => {
                        tracing::debug!(target: LOG_TARGET, "connection closed: peer={}", peer);
                    }
                    Some(TransportEvent::SubstreamOpened {
                        substream,
                        direction,
                        ..
                    }) => match direction {
                        Direction::Inbound => {
                        }
                        Direction::Outbound(_substream_id) => {
                        }
                    },
                    _ => {},
                },
            }
        }
    }
}
