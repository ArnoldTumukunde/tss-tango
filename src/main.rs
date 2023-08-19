extern crate network;
use accounts::Account;
use clap::Parser;
use connector::ethereum::SwapToken;
use connector::polkadot;
use env_logger::Env;
use keystore::params::keystore_params::KeystoreParams;
use network::utils::identity_handler::get_node_identity;
use sc_keystore::LocalKeystore;
use sc_service::config::KeystoreConfig;
use sp_core::crypto::KeyTypeId;
use sp_keyring::AccountKeyring;
use sp_keystore::SyncCryptoStorePtr;
use std::convert::TryFrom;
use std::env;
use std::{sync::Arc, time::Duration};
use subxt::{tx::PairSigner, OnlineClient, PolkadotConfig};
use tango_database::MongoRepo;
use tango_node::cli::Args;
use tokio;
use tokio::sync::{mpsc, Mutex};
use tss::tss_event_model::TSSData;
use tss::tss_service::TssService;
use web3::transports::Http;
#[derive(Debug)]
pub struct BLOCKCHAIN {
    pub polkadot: String,
    pub ethereum: String,
}
impl BLOCKCHAIN {
    fn new(polka: &str, eth: &str) -> Self {
        BLOCKCHAIN {
            ethereum: String::from(eth),
            polkadot: String::from(polka),
        }
    }
}
async fn get_connection(db_url: String) -> MongoRepo {
    let mut collections = Vec::new();
    collections.push("events");
    collections.push("contracts");
    collections.push("token_swap");
    collections.push("swap_events");
    collections.push("tokens");
    let connector = MongoRepo::init(&db_url, "tango_db", collections).await;

    connector
}
use messages::gossip_message_handler::GossipEventHandler;

#[tokio::main]
async fn main() {
    // log
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let config_dir = env::current_dir().unwrap();
    log::info!("tango node start up ");
    //Declare channels and variables
    //Channels
    let (gossip_sender, _gossip_receiver) = mpsc::channel::<Vec<u8>>(1000);
    let message_handler_to_gossip_sender = gossip_sender.clone();
    let tss_to_gossip_sender = gossip_sender.clone();

    let (message_handler_to_tss_sender, message_handler_to_tss_receiver) =
        mpsc::channel::<TSSData>(100);
    let (event_sender, event_receiver) = mpsc::channel::<String>(1000);

    //Keystore
    let keystore_params = KeystoreParams::default();

    let keystore = match keystore_params.keystore_config(&config_dir).unwrap() {
        (_, KeystoreConfig::Path { path, password }) => {
            // let public = with_crypto_scheme!(self.scheme, to_vec(&suri, password.clone()))?;
            let keystore: SyncCryptoStorePtr =
                Arc::new(LocalKeystore::open(path, password).unwrap());
            keystore
        }
        _ => unreachable!("keystore_config always returns path and password; qed"),
    };

    // get the cli arguments
    let args = Args::parse();


    // assign tss keytype and keystore
    let key_type_option = Some(KeyTypeId::try_from(args.key_type.as_str()).unwrap());
    let keystore_option = Some(keystore.clone());

    let key_type = match key_type_option {
        Some(key_type) => key_type,
        None => {
            log::error!("Key type not set");
            return;
        }
    };

    let key_store = match keystore_option.clone() {
        Some(keystore) => keystore,
        None => {
            log::error!("Keystore not set");
            return;
        }
    };

    // create account
    let acc = Account::new(&args.password.clone(), key_type, key_store);
    let account = AccountKeyring::Alice.to_account_id();

    // start the db instance
    // start the actix server mongo instance
    let db_url = args.db_url.clone();
    tokio::spawn(async move {
        let connector = get_connection(db_url).await;
        let ip = args.ip;
        let port = args.port;
        let origin = args.origin;

        server::start_server(
            Arc::new(Mutex::new(connector)),
            ip,
            port,
            origin,
            args.workers,
        )
        .await
        .unwrap()
    });
    let selected_chain = BLOCKCHAIN::new("polkadot", "ethereum");
    let blockchain = args.blockchain.clone();

    // start connector
    let conn_db_url = args.db_url.clone();
    //tss should take the event_receiver
    let connector = get_connection(conn_db_url.clone()).await;
    // get polkadot on-chain accounts data.
    if blockchain == selected_chain.polkadot {
        log::info!("Polkadot chain connected.");
        let polkadot_event_sender = event_sender.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(10)).await;
            let arguments = polkadot::Polkadot {
                sender: polkadot_event_sender,
            };

            let _ = polkadot::Polkadot::get_accounts(&arguments).await;
        });
    } else if blockchain == selected_chain.ethereum {
        log::info!("Etherum chain connected.");
        // let connector_json = MongoRepo::get_contract_json(&connector).await.unwrap();
        log::info!("Data fetched successfully from the contract database.",);
        /////////////////////// event data fetch  removed
        //
        // Start the swap token data thread.
        let connection = connector.clone();
        let event_sender_cloned = event_sender.clone();
        tokio::spawn(async move {
            let mut swap_index = 0;
            loop {
                tokio::time::sleep(Duration::from_millis(10000)).await;
                swap_index += 1;
                let swap_data = MongoRepo::get_swap_data(&connection.clone()).await;
                match swap_data {
                    Ok(swap_data) => {
                        if swap_data.len() > 0 {
                            let end_point = Http::new(&swap_data[0].chain_endpoint);
                            match end_point {
                                Ok(end_point) => {
                                    let websocket = web3::Web3::new(end_point);
                                    let arguments = SwapToken {
                                        web_socket: websocket,
                                        connection: connection.clone(),
                                        sender: event_sender_cloned.clone(),
                                    };
                                    let _ = SwapToken::swap_thread_handler(&arguments, swap_index)
                                        .await;
                                }
                                Err(msg) => {
                                    println!("Endpoint not valid {}", msg);
                                }
                            }
                        }
                    }
                    Err(msg) => {
                        println!("Database connection Error {}", msg);
                    }
                }
            }
        });
        ///////////////////////
    }

    //Network creating or getting identity
    let (peer_id, _id_keys) = get_node_identity(args.new_node);

    //////////////////////////
    //Handler struct to handle messages
    let _handler_message = GossipEventHandler {
        gossip_sender: message_handler_to_gossip_sender,
        gossip_to_tss_sender: message_handler_to_tss_sender,
    };

    ////////////////////////// TSS Operations //////////////////////////
    let mut tss_service = TssService::new(
        message_handler_to_tss_receiver,
        tss_to_gossip_sender,
        event_receiver,
        acc,
        connector,
        !args.new_node,
        peer_id.to_string(),
        (args.tss_nodes, args.tss_threshold),
        key_type_option,
        keystore_option,
    )
    .await;

    tokio::spawn(async move {
        tss_service.run().await;
    });

    ////////////////////////// Network Operations //////////////////////////

    // start the network
    network::utils::network::network(
        args.new_node,
        args.p2p_topic,
        args.seed_node,
        &args.node_port,
    )
    .await;
}
