use libp2p::core::PublicKey;
use libp2p::gossipsub::{Gossipsub, GossipsubMessage, MessageAuthenticity, MessageId};
use libp2p::identify::{Identify, IdentifyConfig};
use libp2p::kad::store::MemoryStore;
use libp2p::kad::{Kademlia, KademliaConfig, KademliaStoreInserts};
use libp2p::mdns::{Mdns, MdnsConfig};
use libp2p::ping::{Ping, PingConfig};
use libp2p::{gossipsub, identity, PeerId};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{self};
use std::time::Duration;

///builds messaging protocol based on gossip
pub fn build_gossip(local_key: identity::Keypair) -> io::Result<Gossipsub> {
    let message_id_fn = |message: &GossipsubMessage| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        MessageId::from(s.finish().to_string())
    };

    // Set a custom gossipsub
    let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
        .message_id_fn(message_id_fn) // content-address messages. No two messages of the
        // same content will be propagated.
        .build()
        .expect("Valid config");

    let gossipsub: Gossipsub =
        gossipsub::Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config)
            .expect("Correct configuration");

    Ok(gossipsub)
}

///builds mdns behaviour to be use in swarm
pub async fn build_mdns() -> Mdns {
    let mdns = Mdns::new(MdnsConfig::default()).await.unwrap();
    mdns
}

///builds kademlia behaviour to be use in swarm
pub fn build_kademlia(peer_id: PeerId) -> Kademlia<MemoryStore> {
    let store = MemoryStore::new(peer_id);
    let mut kad_config = KademliaConfig::default();
    kad_config.set_protocol_name("/tango/kad/1.0.0".as_bytes());
    kad_config.set_query_timeout(Duration::from_secs(300));
    kad_config.set_record_filtering(KademliaStoreInserts::FilterBoth);
    // set disjoint_query_paths to true. Ref: https://discuss.libp2p.io/t/s-kademlia-lookups-over-disjoint-paths-in-rust-libp2p/571
    kad_config.disjoint_query_paths(true);
    let kademlia = Kademlia::with_config(peer_id, store, kad_config);
    kademlia
}

///builds ping behaviour to be use in swarm
pub fn build_ping() -> Ping {
    let ping_config = PingConfig::new().with_keep_alive(true);
    Ping::new(ping_config)
}

///builds kademlia behaviour to be use in swarm
pub fn build_identify(local_public_key: PublicKey) -> Identify {
    Identify::new(IdentifyConfig::new(
        "/tango/id/1.0.0".into(),
        local_public_key,
    ))
}
