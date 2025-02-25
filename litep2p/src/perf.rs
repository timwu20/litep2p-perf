
use futures::StreamExt;

use litep2p::{
    codec::ProtocolCodec,
    protocol::{Direction, TransportEvent, TransportService, UserProtocol},
    substream::Substream,
    PeerId, ProtocolName,
};

const PROTOCOL_NAME: &str = "/litep2p-perf/1.0.0";
const LOG_TARGET: &str = "litep2p-perf";

pub struct Perf {
    is_server: bool,
    upload_bytes: u64,
    download_bytes: u64,
}

impl Perf {
    pub fn new(is_server: bool, upload_bytes: u64, download_bytes: u64) -> Self {
        Self {
            is_server,
            upload_bytes,
            download_bytes,
        }
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
                        if !self.is_server {
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
