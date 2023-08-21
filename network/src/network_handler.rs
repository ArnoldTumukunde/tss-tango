use crate::get_bootnodes;
use crate::utils::behaviour::{ComposedEvent, LocalNetworkBehaviour};
use crate::utils::behaviour_protocols::{
    build_gossip, build_identify, build_kademlia, build_mdns, build_ping,
};
use crate::utils::transport::build_transport;

use libp2p::identity::Keypair;
use libp2p::kad::store::MemoryStore;
use libp2p::kad::{Kademlia, KademliaEvent, QueryResult};
use libp2p::mdns::MdnsEvent;
use libp2p::multiaddr::Protocol;
use libp2p::{
    futures::StreamExt,
    gossipsub::IdentTopic as Topic,
    swarm::{SwarmBuilder, SwarmEvent},
    Multiaddr,
};
use libp2p::{identify, identity, ping, PeerId, Swarm};
use message::gossip_message_handler::MessageHandler;
use std::error::Error;
use tokio::sync::mpsc::Receiver;

/// Runs a new instance of tesseract node
/// topic: context on which you want to publish events to other nodes
/// recv: mpsc receiver to tell node to publish events
/// handler: external struct which implements MessageHandler to handle message
/// explicit_peer: explicit address of a peer to add into gossip network
/// seed_node: address of a node to dial explicitiy from swarm
/// listening_port: if provided node will try to start on this specific port
pub async fn run(
    topic: &Topic,
    mut recv: Receiver<Vec<u8>>,
    handler: &dyn MessageHandler,
    explicit_peer: Option<&str>,
    seed_node: Option<&str>,
    listening_port: &str,
    node_identity: (PeerId, Keypair),
) -> Result<(), Box<dyn Error>> {
    //Checking if port number is valid
    assert!(
        listening_port.parse::<i32>().unwrap() <= 65535,
        "Invalid port provided"
    );

    //node keypair
    let peer_id = node_identity.0;
    let id_keys = node_identity.1;

    //building trasport layer
    let transport = build_transport(id_keys.clone())?;

    //swarm manager integration
    let mut swarm = {
        let behaviour = LocalNetworkBehaviour {
            gossipsub: build_gossip(id_keys.clone())?,
            mdns: build_mdns().await,
            kademlia: build_kademlia(peer_id.clone()),
            identify: build_identify(id_keys.public().clone()),
            ping: build_ping(),
            is_bootstrapped: false,
        };

        //composing transport, protocol and peer id into swam manager and running in tokio env
        SwarmBuilder::new(transport, behaviour, peer_id)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build()
    };

    //subscribing an event
    swarm.behaviour_mut().gossipsub.subscribe(&topic).unwrap();

    // add an explicit peer if one was provided
    if let Some(explicit) = explicit_peer {
        let explicit = explicit.clone();
        match explicit.parse() {
            Ok(id) => swarm.behaviour_mut().gossipsub.add_explicit_peer(&id),
            Err(err) => log::warn!("Failed to parse explicit peer id: {:?}", err),
        }
    }

    //assigning a port if argument passed
    let listen_multiaddr_template = "/ip4/0.0.0.0/tcp/";
    let listen_multiaddr = format!("{}{}", listen_multiaddr_template, listening_port);

    // if port is 0 then listen on all interfaces and whatever port the OS assigns
    log::info!("listen_multiaddr is: {}", listen_multiaddr);
    swarm.listen_on(listen_multiaddr.parse().unwrap()).unwrap();

    // Reach out to another node if specified
    if let Some(to_dial) = seed_node {
        let address: Multiaddr = to_dial.parse().expect("User to provide valid address.");
        match swarm.dial(address.clone()) {
            Ok(_) => log::info!("Dialed {:?}", address),
            Err(e) => log::warn!("Dial {:?} failed: {:?}", address, e),
        };
    }

    //Bootstraping kademlia
    for bootstrap in get_bootnodes() {
        log::info!("Adding {} as bootstrap", bootstrap);
        let mut addr = bootstrap.to_owned();
        let peer_id = match addr.pop() {
            Some(Protocol::P2p(hash)) => match PeerId::from_multihash(hash) {
                Ok(id) => id,
                Err(_) => {
                    log::warn!("PeerId could not be formed from multihash");
                    continue;
                }
            },
            _ => {
                log::warn!("Invalid peer id");
                continue;
            }
        };
        swarm
            .behaviour_mut()
            .kademlia
            .add_address(&peer_id, addr.clone());

        if let Err(e) = swarm.dial(addr.clone()) {
            log::warn!(
                "Dial in failed for peer_id {} at address {} with error :{}",
                peer_id,
                addr,
                e
            );
        } else {
            log::info!("Swarm Dial successful");
        }
    }

    // Bootstrap
    if let Err(e) = swarm.behaviour_mut().kademlia.bootstrap() {
        log::warn!("Failed to bootstrap node with error {}", e);
    } else {
        swarm.behaviour_mut().is_bootstrapped = true;
        log::info!("Bootstrap Done");
    }

    loop {
        tokio::select! {
            //received a message from inner program to execute message
            r = recv.recv() => {
                if let Some(data) = r {
                    if let Err(e) = swarm
                    .behaviour_mut()
                    .gossipsub
                    // message format will be topic:message
                    .publish((topic).clone(), data)
                    {
                        log::info!("Publish error: {:?}", e);
                    }
                }
            },

            // receive message from network
            event = swarm.select_next_some() => match event {

                //listening on new address
                SwarmEvent::NewListenAddr { address, .. } => {
                    log::info!("Swarm: Listening on {:?}", address);
                },

                //Handling gossip behaviour => received msg from network
                SwarmEvent::Behaviour(ComposedEvent::Gossipsub(message)) => {
                    let _ = handler.handle_message(message).await;
                },

                SwarmEvent::OutgoingConnectionError{peer_id, ..} => {
                    if let Some(p_id) = peer_id {
                        log::error!("Peer id {:?} is down!", p_id);
                    }
                },

                //handling mdns behaviour => mdns node discovery and disconnection
                SwarmEvent::Behaviour(ComposedEvent::Mdns(event)) => {
                    match event {
                        MdnsEvent::Discovered(nodes)=>{
                            for (peer_id, multiaddrr) in nodes {
                                swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddrr);
                                swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                            }
                            log::info!("MDNS: inserted node into kademlia: {}", peer_id);

                            //bootstrapping if not already done
                            if !swarm.behaviour().is_bootstrapped {
                                if swarm.behaviour_mut().kademlia.bootstrap().is_ok(){
                                    swarm.behaviour_mut().is_bootstrapped = true;
                                    log::info!("MDNS: Kademlia bootstrapped successfully");
                                }
                            }
                        },
                        MdnsEvent::Expired(nodes)=>{
                            for (peer_id, multiaddr) in nodes{
                                swarm.behaviour_mut().kademlia.remove_address(&peer_id, &multiaddr);
                            }
                            log::info!("MDNS: removed node from kademlia: {}", peer_id);
                        }
                    }
                },

                //adding detailed got by identify protocol in kademlia dht
                SwarmEvent::Behaviour(ComposedEvent::Identify(event)) => {
                    match event{
                        identify::IdentifyEvent::Received { peer_id, info } => {
                            for addr in info.listen_addrs{
                                swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                            }
                            log::info!("Identify: received peer id: {}", peer_id);
                        },
                        _ => {}
                    }
                },

                //handling kademlia events
                SwarmEvent::Behaviour(ComposedEvent::Kademlia(event)) => {
                    kad_event_handler(event, &mut swarm);
                },

                //ping events
                SwarmEvent::Behaviour(ComposedEvent::Ping(event)) => {
                     match event {
                        ping::PingEvent {
                            peer,
                            result: Result::Err(ping::PingFailure::Timeout),
                        } => {
                            log::warn!("ping: timeout to {}", peer);
                            swarm.behaviour_mut().kademlia.remove_peer(&peer);
                        }
                        _ => {}
                    }
                },
                _ => {}

            }

        }
    }
}

pub fn kad_event_handler(event: KademliaEvent, swarm: &mut Swarm<LocalNetworkBehaviour>) {
    match event {
        KademliaEvent::OutboundQueryCompleted { result, .. } => match result {
            QueryResult::GetProviders(Ok(_ok)) => {
                log::info!("Kademlia: GetProviders successful");
            }
            QueryResult::GetProviders(Err(e)) => {
                log::info!("Kademlia: GetProviders successful error {}", e);
            }
            QueryResult::StartProviding(Ok(_ok)) => {
                log::info!("Kademlia: StartProviding successful");
            }
            QueryResult::StartProviding(Err(e)) => {
                log::info!("Kademlia: StartProviding error {} ", e);
            }
            QueryResult::GetRecord(Ok(_ok)) => {
                log::info!("Kademlia: GetRecord successful");
            }
            QueryResult::GetRecord(Err(e)) => {
                log::info!("Kademlia: GetRecord error {:?} ", e);
            }
            QueryResult::PutRecord(Ok(_ok)) => {
                log::info!("Kademlia: record successfully inserted");
            }
            QueryResult::PutRecord(Err(e)) => {
                log::info!("Kademlia: record insertion error with id with {}", e);
            }
            QueryResult::GetClosestPeers(Ok(ok)) => {
                log::info!("Kademlia: get closest peer response {:?}", ok);
            }
            QueryResult::GetClosestPeers(Err(e)) => {
                log::info!("Kademlia: get closest peer error {:?}", e);
            }
            QueryResult::Bootstrap(Ok(ok)) => {
                log::info!("Kademlia: Bootstrap reports ok {:?}", ok);
                if ok.num_remaining == 0 {
                    log::info!("Kademlia: Bootstrap Completed");
                    list_peers(swarm);
                    look_for_random_peer(swarm);
                }
            }
            QueryResult::Bootstrap(Err(e)) => {
                log::info!("Kademlia: Bootstrap reports error {:?}", e);
            }
            _ => {}
        },
        KademliaEvent::RoutingUpdated {
            peer, addresses, ..
        } => {
            for addr in addresses.iter() {
                log::info!("[kad] discovered: {} @ {}", peer, addr);
            }
        }
        KademliaEvent::UnroutablePeer { peer } => {
            log::info!("[kad] unroutable peer found {}", peer);
        }
        KademliaEvent::RoutablePeer { peer, .. } => {
            log::info!("[kad] Routable peer found {}", peer);
        }
        _ => {}
    }
}

///list_peers for logging purpose
pub fn list_peers(swarm: &mut Swarm<LocalNetworkBehaviour>) -> i8 {
    let kademlia: &mut Kademlia<MemoryStore> = &mut swarm.behaviour_mut().kademlia;
    let mut count: i8 = 0;
    log::info!("================Kad Bucket=================");
    for bucket in kademlia.kbuckets() {
        if bucket.num_entries() > 0 {
            for item in bucket.iter() {
                log::info!("Peer ID: {:?}", item.node.key.preimage());
                log::info!("Peer value: {:?}", item.node.value);
                count += 1;
            }
        }
    }
    log::info!("==========================================");
    count
}

//for logging purposes
fn look_for_random_peer(swarm: &mut Swarm<LocalNetworkBehaviour>) {
    let id_keys_temp = identity::Keypair::generate_ed25519();
    let peer_id_temp = PeerId::from(id_keys_temp.public());

    swarm
        .behaviour_mut()
        .kademlia
        .get_closest_peers(peer_id_temp);
}
