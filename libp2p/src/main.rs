use clap::Parser as ClapParser;
use futures::StreamExt;
use libp2p_swarm::SwarmEvent;

use utils::Command;

mod client;
mod server;

mod perf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let command = Command::parse();
    match command {
        Command::Server(server_opts) => {
            let mut bytes = server_opts.node_key.as_bytes().to_vec();
            if bytes.len() > 32 {
                bytes.truncate(32);
            } else if bytes.len() < 32 {
                bytes.resize(32, 0);
            }

            let secret_key = libp2p::identity::ed25519::SecretKey::try_from_bytes(bytes)?;
            let local_key = libp2p::identity::ed25519::Keypair::from(secret_key);
            let local_key: libp2p::identity::Keypair = local_key.into();

            let tcp_config = libp2p::tcp::Config::new().nodelay(true);
            let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
                .with_tokio()
                .with_tcp(
                    tcp_config,
                    libp2p_noise::Config::new,
                    libp2p_yamux::Config::default,
                )?
                .with_dns()?
                .with_behaviour(|_key| crate::server::behaviour::Behaviour::new())?
                .with_swarm_config(|cfg| {
                    cfg.with_idle_connection_timeout(std::time::Duration::from_secs(60))
                })
                .build();

            swarm.listen_on(server_opts.listen_address.parse()?)?;

            loop {
                let event = swarm.next().await;
                tracing::info!("Event: {:?}", event);
            }
        }
        Command::Client(client_opts) => {
            let local_key = libp2p::identity::Keypair::generate_ed25519();

            let mut swarm = match client_opts.transport_layer {
                utils::TransportLayer::Tcp => {
                    let tcp_config = libp2p::tcp::Config::new().nodelay(true);
                    libp2p::SwarmBuilder::with_existing_identity(local_key)
                        .with_tokio()
                        .with_tcp(
                            tcp_config,
                            libp2p_noise::Config::new,
                            libp2p_yamux::Config::default,
                        )?
                        .with_dns()?
                        .with_behaviour(|_key| crate::client::behaviour::Behaviour::new())?
                        .with_swarm_config(|cfg| {
                            cfg.with_idle_connection_timeout(std::time::Duration::from_secs(60))
                        })
                        .build()
                }
                utils::TransportLayer::WebSocket => {
                    unimplemented!("WebSocket transport layer not implemented yet");
                }
                utils::TransportLayer::WebRTC => {
                    unimplemented!("WebRTC transport layer not implemented yet");
                }
            };

            let addr: libp2p::Multiaddr = client_opts.server_address.parse()?;
            swarm.dial(addr)?;

            let server_peer_id = match swarm.next().await.unwrap() {
                SwarmEvent::ConnectionEstablished { peer_id, .. } => peer_id,
                e => panic!("{e:?}"),
            };

            swarm.behaviour_mut().perf(
                server_peer_id,
                client_opts.upload_bytes as u64,
                client_opts.download_bytes as u64,
            )?;

            loop {
                let event = swarm.next().await;
                tracing::info!("Even: {:?}", event);

                match event {
                    Some(SwarmEvent::Behaviour(..)) => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
        _ => panic!("Command unimplemented"),
    };
}
