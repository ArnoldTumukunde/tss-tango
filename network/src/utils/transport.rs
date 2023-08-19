use libp2p::core::muxing::StreamMuxerBox;
use libp2p::core::transport::upgrade::Version;
use libp2p::core::transport::Boxed;
use libp2p::core::upgrade::SelectUpgrade;
use libp2p::identity;
use libp2p::mplex::MplexConfig;
use libp2p::noise::{self, NoiseConfig};
use libp2p::yamux::YamuxConfig;
use libp2p::{PeerId, Transport};
use std::io::{self, Error, ErrorKind};
use std::time::Duration;

use libp2p::dns::GenDnsConfig;
use libp2p::tcp;

//https://github.com/rs-ipfs/rust-ipfs/blob/master/src/p2p/transport.rs

type TTransport = Boxed<(PeerId, StreamMuxerBox)>;

/// Builds the transport that serves as a common ground for all connections.
/// Set up an encrypted TCP transport over the Mplex protocol.
pub fn build_transport(keypair: identity::Keypair) -> io::Result<TTransport> {
    let xx_keypair = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&keypair)
        .unwrap();
    let noise_config = NoiseConfig::xx(xx_keypair).into_authenticated();

    Ok(tcp::tokio::Transport::default()
        .upgrade(Version::V1)
        .authenticate(noise_config)
        .multiplex(SelectUpgrade::new(
            YamuxConfig::default(),
            MplexConfig::new(),
        ))
        .timeout(Duration::from_secs(20))
        .map(|(peer_id, muxer), _| (peer_id, StreamMuxerBox::new(muxer)))
        .map_err(|err| Error::new(ErrorKind::Other, err))
        .boxed())
}
