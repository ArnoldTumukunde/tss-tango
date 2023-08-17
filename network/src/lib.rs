use libp2p::Multiaddr;
use std::str::FromStr;

pub mod network_handler;
pub mod utils;

///////////
/// Bootnodes are hosted on a public IP and with open port so other users can connect with it.
/// Right now there is only 1 bootnode which is hosted on aws server with 52200 port open.
/// we can read below line as we have an ipv4 as 100.24.209.156 who is listening on 52200 tcp port
/// for p2p connection having its peer id as 12D3KooWFEXXy8iJfWk3ZG5883GiebFpg1QbNCivXKSE5VZ9YkHF.
pub fn get_bootnodes() -> Vec<Multiaddr> {
    ["/ip4/100.24.209.156/tcp/52200/p2p/12D3KooWFEXXy8iJfWk3ZG5883GiebFpg1QbNCivXKSE5VZ9YkHF"]
        .iter()
        .filter_map(|s| Multiaddr::from_str(s).ok())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::utils::identity_handler::get_node_identity;

    use super::*;
    use async_trait::async_trait;
    use libp2p::gossipsub::{GossipsubEvent, Topic};
    use message::gossip_message_handler::MessageHandler;
    use once_cell::sync::Lazy;
    use std::sync::Mutex;
    use std::{thread, time::Duration};
    use tokio::sync::mpsc;

    static ARRAY_SEED: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![]));
    static ARRAY_MDNS: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![]));

    /// Runs test to crate and run 2 nodes and then verify if both nodes exchanged data
    /// run with cargo test -- tests::communication_with_seed --exact --nocapture to see detailed output
    #[tokio::test]
    async fn communication_with_seed() {
        fn increment_static_vec() {
            ARRAY_SEED.lock().unwrap().push(1);
        }

        pub struct HandleMessage {
            compare_val: Vec<u8>,
        }

        #[async_trait]
        impl MessageHandler for HandleMessage {
            async fn handle_message(&self, event: GossipsubEvent) {
                match event {
                    GossipsubEvent::Message {
                        propagation_source: _peer_id,
                        message_id: _id,
                        message,
                    } => {
                        // let msg = String::from_utf8_lossy(&message.data);
                        println!("================================================");
                        println!(
                            "comparing {:?} with {:?}",
                            String::from_utf8(self.compare_val.clone()).unwrap(),
                            String::from_utf8(message.data.clone()).unwrap()
                        );
                        assert_eq!(self.compare_val, message.data);
                        println!("received test passed successfully");
                        //updating length of passed test in static vector
                        //so can fail test in main thread of any thing failed
                        increment_static_vec();
                    }
                    _ => log::info!("handler was not able to handle the event: {:?}", event),
                }
            }
        }

        //first node sending value
        let first_sending_val = "hello".as_bytes().to_vec();

        //second node sending value
        let second_sending_val = "world".as_bytes().to_vec();

        let (first_n_event_sender, first_n_event_rec) = mpsc::channel::<Vec<u8>>(100);
        let topic = Topic::new("test_case");

        let first_node_handler = HandleMessage {
            compare_val: second_sending_val.clone(),
        };

        let topic_clone1 = topic.clone();
        let topic_clone2 = topic.clone();

        //node 1 identity
        let (peer_id_1, id_keys_1) = get_node_identity(true);

        thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            let _ = runtime.block_on(runtime.spawn(async move {
                let _ = network_handler::run(
                    &topic_clone1,
                    first_n_event_rec,
                    &first_node_handler,
                    None,
                    None,
                    "39900",
                    (peer_id_1, id_keys_1),
                )
                .await;
            }));
        });

        let (second_n_event_sender, second_n_event_rec) = mpsc::channel::<Vec<u8>>(100);

        let second_node_handler = HandleMessage {
            compare_val: first_sending_val.clone(),
        };

        tokio::time::sleep(Duration::from_millis(2000)).await;

        let (peer_id_2, id_keys_2) = get_node_identity(true);
        thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            let _ = runtime.block_on(runtime.spawn(async move {
                let _ = network_handler::run(
                    &topic_clone2,
                    second_n_event_rec,
                    &second_node_handler,
                    None,
                    Some("/ip4/127.0.0.1/tcp/39900"),
                    "39911",
                    (peer_id_2, id_keys_2),
                )
                .await;
            }));
        });

        tokio::time::sleep(Duration::from_millis(5000)).await;

        if let Err(e) = first_n_event_sender.send(first_sending_val).await {
            println!("error while sending value to node {}", e);
        }

        if let Err(e) = second_n_event_sender.send(second_sending_val).await {
            println!("error while sending value to node B {}", e);
        }

        tokio::time::sleep(Duration::from_millis(5000)).await;

        //verifying number of nodes data verified should be 2
        assert_eq!(ARRAY_SEED.lock().unwrap().len(), 2);
    }

    // Runs test to crate and run 2 nodes and then verify if both nodes exchanged data
    // run with cargo test -- tests::communication_with_mdns --exact --nocapture to see detailed output
    #[tokio::test]
    async fn communication_with_mdns() {
        fn increment_static_vec() {
            ARRAY_MDNS.lock().unwrap().push(1);
        }

        pub struct HandleMessageMDNS {
            compare_val: Vec<u8>,
        }

        #[async_trait]
        impl MessageHandler for HandleMessageMDNS {
            async fn handle_message(&self, event: GossipsubEvent) {
                match event {
                    GossipsubEvent::Message {
                        propagation_source: _peer_id,
                        message_id: _id,
                        message,
                    } => {
                        println!("================================================");
                        println!(
                            "comparing {:?} with {:?}",
                            String::from_utf8(self.compare_val.clone()).unwrap(),
                            String::from_utf8(message.data.clone()).unwrap()
                        );
                        assert_eq!(self.compare_val, message.data);
                        println!("received test passed successfully");
                        //updating length of passed test in static vector
                        //so can fail test in main thread of any thing failed
                        increment_static_vec();
                    }
                    _ => log::info!("handler was not able to handle the event: {:?}", event),
                }
            }
        }

        //first node sending value
        let first_sending_val = "hello mdns".as_bytes().to_vec();

        //second node sending value
        let second_sending_val = "world mdns".as_bytes().to_vec();

        let (first_n_event_sender, first_n_event_rec) = mpsc::channel::<Vec<u8>>(32);
        let topic = Topic::new("test_mdns");

        let first_node_handler_mdns = HandleMessageMDNS {
            compare_val: second_sending_val.clone(),
        };

        let topic_clone1 = topic.clone();
        let topic_clone2 = topic.clone();

        let (peer_id_1, id_keys_1) = get_node_identity(true);

        thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            let _ = runtime.block_on(runtime.spawn(async move {
                let _ = network_handler::run(
                    &topic_clone1,
                    first_n_event_rec,
                    &first_node_handler_mdns,
                    None,
                    None,
                    "0",
                    (peer_id_1, id_keys_1),
                )
                .await;
            }));
        });

        let (second_n_event_sender, second_n_event_rec) = mpsc::channel::<Vec<u8>>(32);

        let second_node_handler_mdns = HandleMessageMDNS {
            compare_val: first_sending_val.clone(),
        };

        tokio::time::sleep(Duration::from_millis(2000)).await;

        let (peer_id_2, id_keys_2) = get_node_identity(true);

        thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            let _ = runtime.block_on(runtime.spawn(async move {
                let _ = network_handler::run(
                    &topic_clone2,
                    second_n_event_rec,
                    &second_node_handler_mdns,
                    None,
                    None,
                    "0",
                    (peer_id_2, id_keys_2),
                )
                .await;
            }));
        });

        tokio::time::sleep(Duration::from_millis(5000)).await;

        if let Err(e) = first_n_event_sender.send(first_sending_val).await {
            println!("error while sending value to node {}", e);
        };

        if let Err(e) = second_n_event_sender.send(second_sending_val).await {
            println!("error while sending value to node B {}", e);
        };

        tokio::time::sleep(Duration::from_millis(5000)).await;

        //verifying number of nodes data verified should be 2
        assert_eq!(ARRAY_MDNS.lock().unwrap().len(), 2);
    }
}
