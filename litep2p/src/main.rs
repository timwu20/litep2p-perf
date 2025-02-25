use clap::Parser as ClapParser;

mod perf;

/// Command for interacting with the CLI.
#[derive(Debug, ClapParser)]
pub enum Command {
    /// Start the performance in server mode.
    Server(ServerOpts),

    /// Start the performance in client mode.
    Client(ClientOpts),
}

/// The server options.
#[derive(Debug, ClapParser)]
pub struct ServerOpts {
    /// The address on which the server listens on.
    #[clap(long, short)]
    listen_address: String,

    /// The node key used to derive the server peer ID.
    #[clap(long, short)]
    node_key: String,
}

/// The client options.
#[derive(Debug, ClapParser)]
pub struct ClientOpts {
    /// The address on which the server listens on.
    #[clap(long, short)]
    server_address: String,

    /// The uploaded bytes.
    #[clap(long)]
    upload_bytes: usize,

    /// The downloaded bytes.
    #[clap(long)]
    download_bytes: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let command = Command::parse();

    let mut litep2p = match command {
        Command::Server(server_opts) => {
            let perf = Box::new(perf::Perf::new(perf::PerfMode::Server));

            let litep2p_config = litep2p::config::ConfigBuilder::new()
                .with_tcp(litep2p::transport::tcp::config::Config {
                    listen_addresses: vec![server_opts
                        .listen_address
                        .parse()
                        .expect("Valid listen address")],
                    reuse_port: true,
                    nodelay: true,
                    ..Default::default()
                })
                .with_user_protocol(perf)
                .build();

            let litep2p = litep2p::Litep2p::new(litep2p_config)?;

            let listen_addr: Vec<_> = litep2p.listen_addresses().collect();
            tracing::info!("Server listening on address: {listen_addr:?}");

            litep2p
        }
        Command::Client(client_opts) => {
            let perf = Box::new(perf::Perf::new(perf::PerfMode::Client {
                upload_bytes: client_opts.upload_bytes as u64,
                download_bytes: client_opts.download_bytes as u64,
            }));

            let litep2p_config = litep2p::config::ConfigBuilder::new()
                .with_tcp(litep2p::transport::tcp::config::Config {
                    reuse_port: true,
                    nodelay: true,
                    ..Default::default()
                })
                .with_user_protocol(perf)
                .build();

            let mut litep2p = litep2p::Litep2p::new(litep2p_config)?;

            litep2p
                .dial_address(client_opts.server_address.parse()?)
                .await?;

            litep2p
        }
    };

    while let Some(event) = litep2p.next_event().await {
        tracing::info!("Event: {event:?}");
    }

    Ok(())
}
