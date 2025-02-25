use clap::Parser as ClapParser;

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
    pub listen_address: String,

    /// The node key used to derive the server peer ID.
    #[clap(long, short)]
    pub node_key: String,
}

/// The client options.
#[derive(Debug, ClapParser)]
pub struct ClientOpts {
    /// The address on which the server listens on.
    #[clap(long, short)]
    pub server_address: String,

    /// The uploaded bytes.
    #[clap(long)]
    pub upload_bytes: usize,

    /// The downloaded bytes.
    #[clap(long)]
    pub download_bytes: usize,
}
