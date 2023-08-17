use super::identity_handler::get_node_identity;
use crate::network_handler;
use libp2p::gossipsub::Topic;
use message::gossip_message_handler::GossipEventHandler;
use tokio;
use tokio::sync::mpsc;
use tss::tss_event_model::TSSData;

pub async fn network(new_node: bool, p2p_topic: String, seed_node: String, node_port: &String) {
    let (peer_id, id_keys) = get_node_identity(new_node);
    let (gossip_sender, gossip_receiver) = mpsc::channel::<Vec<u8>>(1000);
    let message_handler_to_gossip_sender = gossip_sender.clone();
    let (message_handler_to_tss_sender, _message_handler_to_tss_receiver) =
        mpsc::channel::<TSSData>(100);

    //Handler struct to handle messages
    let handler_message = GossipEventHandler {
        gossip_sender: message_handler_to_gossip_sender,
        gossip_to_tss_sender: message_handler_to_tss_sender,
    };

    let network_topic = &Topic::new(p2p_topic);
    let _ = network_handler::run(
        network_topic,
        gossip_receiver,
        &handler_message,
        None,
        //seed node from cli if something is passed
        if seed_node.is_empty() {
            None
        } else {
            Some(&seed_node)
        },
        //specific port to run on
        &node_port,
        (peer_id, id_keys),
    )
    .await;
}
