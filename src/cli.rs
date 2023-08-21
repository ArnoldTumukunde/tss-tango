use clap::Parser;

/// Tessaract Node
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Topic
    #[clap(short, long, default_value = "event_parcel")]
    pub p2p_topic: String,

    /// Seed node url
    #[clap(short, long, default_value = "/ip4/127.0.0.1/tcp/12345")]
    pub seed_node: String,

    /// Explicit peer
    #[clap(short = 'x', long, default_value = "")]
    pub explicit_peer: String,

    /// Node Port
    #[clap(short, long, default_value = "0")]
    pub node_port: String,

    // Run new node i.e. with new Peer ID (p2p)
    #[clap(short = 'N', long, parse(try_from_str), default_value = "false")]
    pub new_node: bool,

    /// db url
    #[clap(short, long, default_value = "mongodb://localhost:27017/admin")]
    pub db_url: String,

    //account password
    #[clap(short = 'P', long, default_value = "accountpassword")]
    pub password: String,

    // total number of tss participants
    #[clap(long, default_value_t = 0)]
    pub tss_nodes: u32,

    // threshold for tss
    #[clap(long, default_value_t = 0)]
    pub tss_threshold: u32,

    #[clap(short, long, default_value = "http://localhost:3000")]
    pub origin: String,

    /// Http server binding ip address
    #[clap(short, long, default_value = "127.0.0.1")]
    pub ip: String,

    /// key type for keystore
    #[clap(short, long, default_value = "tngo")]
    pub key_type: String,

    /// Http server port, default is 8080
    #[clap(long, default_value = "8080")]
    pub port: u16,

    /// How many workers for http server
    #[clap(short, long, default_value = "1")]
    pub workers: usize,

    // block chain
    #[clap(short, long, default_value = "polkadot")]
    pub blockchain: String,
}
