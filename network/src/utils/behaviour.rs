use libp2p::{
    gossipsub::{Gossipsub, GossipsubEvent},
    identify::{Identify, IdentifyEvent},
    kad::{store::MemoryStore, Kademlia, KademliaEvent},
    mdns::{self, MdnsEvent},
    ping::{Ping, PingEvent},
    swarm::NetworkBehaviour,
};

#[derive(Debug)]
///Enum to map seperate events for different behaviours
pub enum ComposedEvent {
    Gossipsub(GossipsubEvent),
    Mdns(MdnsEvent),
    Kademlia(KademliaEvent),
    Identify(IdentifyEvent),
    Ping(PingEvent),
}

///Implementation of Network Behaviour that will be used in swarm to listen for events
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "ComposedEvent")]
pub struct LocalNetworkBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns: mdns::tokio::Behaviour,
    pub kademlia: Kademlia<MemoryStore>,
    pub identify: Identify,
    pub ping: Ping,
}

impl From<GossipsubEvent> for ComposedEvent {
    fn from(event: GossipsubEvent) -> Self {
        ComposedEvent::Gossipsub(event)
    }
}

impl From<MdnsEvent> for ComposedEvent {
    fn from(event: MdnsEvent) -> Self {
        ComposedEvent::Mdns(event)
    }
}

impl From<KademliaEvent> for ComposedEvent {
    fn from(event: KademliaEvent) -> Self {
        ComposedEvent::Kademlia(event)
    }
}

impl From<IdentifyEvent> for ComposedEvent {
    fn from(event: IdentifyEvent) -> Self {
        ComposedEvent::Identify(event)
    }
}

impl From<PingEvent> for ComposedEvent {
    fn from(event: PingEvent) -> Self {
        ComposedEvent::Ping(event)
    }
}
