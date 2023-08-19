use crate::tss_event_sender::handle_tss_event;
use async_trait::async_trait;
use borsh::BorshDeserialize;
use events::events::{Event, EventType};
use libp2p::gossipsub::GossipsubEvent;
use tokio::sync::mpsc;
use tss::tss_event_model::TSSData;

#[async_trait]
pub trait MessageHandler: Send + Sync {
    async fn handle_message(&self, event: GossipsubEvent);
}

pub struct GossipEventHandler {
    pub gossip_sender: mpsc::Sender<Vec<u8>>,
    pub gossip_to_tss_sender: mpsc::Sender<TSSData>,
}

#[async_trait]
impl MessageHandler for GossipEventHandler {
    async fn handle_message(&self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message {
                propagation_source: _peer_id,
                message_id: _id,
                message,
            } => {
                if let Ok(event) = Event::try_from_slice(&message.data) {
                    match event.event_type {
                        EventType::TSSEvent => {
                            //send event to tss event parser
                            handle_tss_event(self.gossip_to_tss_sender.clone(), &event.data).await;
                        }
                    }
                } else {
                    log::error!("Unable to parse event data");
                }
            }
            _ => {}
        }
    }
}

/////dummy handle message just prints the message
/// Just for testing
pub struct DummyHandleMessage;

#[async_trait]
impl MessageHandler for DummyHandleMessage {
    async fn handle_message(&self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message {
                propagation_source: peer_id,
                message_id: _id,
                message,
            } => {
                let msg = String::from_utf8_lossy(&message.data);
                log::info!("==========================================");
                log::info!(
                    "received data in message handler {:?} from {}",
                    msg,
                    peer_id
                );
                log::info!("==========================================");
            }
            _ => {}
        }
    }
}
