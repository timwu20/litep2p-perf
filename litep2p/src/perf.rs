use futures::StreamExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use litep2p::{
    codec::ProtocolCodec,
    protocol::{TransportEvent, TransportService, UserProtocol},
    substream::Substream,
    ProtocolName,
};

const PROTOCOL_NAME: &str = "/litep2p-perf/1.0.0";
const LOG_TARGET: &str = "litep2p-perf";

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PerfMode {
    Server,
    Client {
        upload_bytes: u64,
        download_bytes: u64,
    },
    ClientSubstream {
        substreams: usize,
    },
}

pub struct Perf {
    mode: PerfMode,
}

impl Perf {
    pub fn new(mode: PerfMode) -> Self {
        Self { mode }
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

    async fn recv_bytes(substream: &mut Substream, to_recv: u64) -> litep2p::Result<()> {
        let mut buf = vec![0u8; 1024];
        let mut total = 0;
        while total < to_recv {
            let n = substream.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            total += n as u64;
        }
        Ok(())
    }

    async fn send_bytes(substream: &mut Substream, to_send: u64) -> litep2p::Result<()> {
        let buf = vec![0u8; 1024];
        let mut total = 0;
        while total < to_send {
            substream.write_all(&buf).await?;
            total += buf.len() as u64;
        }
        Ok(())
    }

    async fn server_mode(mut substream: Substream) -> litep2p::Result<()> {
        // Step 1. Read the download bytes.
        let to_recv = Self::read_u64(&mut substream).await?;
        // Step 2. Receive the download bytes.
        Self::recv_bytes(&mut substream, to_recv).await?;

        // Step 3. Read the upload bytes.
        let to_send = Self::read_u64(&mut substream).await?;
        // Step 4. Send the upload bytes.
        Self::send_bytes(&mut substream, to_send).await?;

        Ok(())
    }

    async fn client_mode(
        mut substream: Substream,
        upload_bytes: u64,
        download_bytes: u64,
    ) -> litep2p::Result<()> {
        // Step 1. Send the upload bytes.
        Self::write_u64(&mut substream, upload_bytes).await?;
        // Step 2. Send the upload bytes.
        let now = std::time::Instant::now();
        Self::send_bytes(&mut substream, upload_bytes).await?;
        let elapsed = now.elapsed();
        tracing::info!(
            target: LOG_TARGET,
            "Uploaded {} bytes in {:.4}s bandwidth {}",
            utils::format_bytes(upload_bytes as usize),
            elapsed.as_secs_f64(),
            utils::format_bandwidth(elapsed, upload_bytes as usize)
        );

        // Step 3. Send the download bytes.
        Self::write_u64(&mut substream, download_bytes).await?;
        // Step 4. Receive the download bytes.
        let now = std::time::Instant::now();
        Self::recv_bytes(&mut substream, download_bytes).await?;
        let elapsed = now.elapsed();
        tracing::info!(
            target: LOG_TARGET,
            "Downloaded {} bytes in {:.4}s bandwidth {}",
            utils::format_bytes(download_bytes as usize),
            elapsed.as_secs_f64(),
            utils::format_bandwidth(elapsed, download_bytes as usize)
        );

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
        let mut time_to_open = std::time::Instant::now();
        let mut times = Vec::with_capacity(1024);

        let num_substreams = match self.mode {
            PerfMode::Server => 0,
            PerfMode::Client { .. } => 1,
            PerfMode::ClientSubstream { substreams, .. } => substreams,
        };

        loop {
            tokio::select! {
                event = service.next() => match event {
                    Some(TransportEvent::ConnectionEstablished { peer, .. }) => {

                        for i in 0..num_substreams {
                            match service.open_substream(peer) {
                                Ok(_) => {},
                                Err(e) => {
                                    tracing::error!(target: LOG_TARGET, "open substream error: {:?} while opening iter={i}", e);
                                    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                                }
                            }
                        }
                        time_to_open = std::time::Instant::now();
                    }
                    Some(TransportEvent::ConnectionClosed { peer }) => {
                        tracing::info!(target: LOG_TARGET, ?peer, "connection closed");
                    }
                    Some(TransportEvent::SubstreamOpened {
                        substream,
                        direction,
                        ..
                    }) => {
                        if let PerfMode::ClientSubstream {..}  = self.mode {
                            match direction {
                                litep2p::protocol::Direction::Inbound => {}
                                litep2p::protocol::Direction::Outbound(..) => {
                                    tracing::info!("Substream opened in {:?}", time_to_open.elapsed());

                                    times.push(time_to_open.elapsed());
                                    if times.len() == num_substreams {
                                        let total = times.iter().sum::<std::time::Duration>();
                                        let avg = total / num_substreams as u32;
                                        tracing::info!("Average time to open substreams n={num_substreams}, avg={:?}", avg);
                                        times.clear();
                                    }
                                }
                            }
                        } else {
                            let mode = self.mode.clone();
                            tokio::spawn(async move {
                                match mode {
                                    PerfMode::Server => {
                                        if let Err(e) = Self::server_mode(substream).await {
                                            tracing::error!(target: LOG_TARGET, "server mode error: {:?}", e);
                                        }
                                    }
                                    PerfMode::Client { upload_bytes, download_bytes } => {
                                        if let Err(e) = Self::client_mode(substream, upload_bytes, download_bytes).await {
                                            tracing::error!(target: LOG_TARGET, "client mode error: {:?}", e);
                                        }
                                    }
                                    _ => {},
                                }
                            });
                        }
                    }
                    _ => {},
                },
            }
        }
    }
}
