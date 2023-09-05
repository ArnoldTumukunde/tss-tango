use serde_json::json;
use subxt::{OnlineClient, PolkadotConfig};
use tokio::sync::mpsc;

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod substrate {}

#[derive(Clone)]
pub struct Polkadot {
    pub sender: mpsc::Sender<String>,
}

impl Polkadot {
    /// create a new event instance
    pub fn new(sender: mpsc::Sender<String>) -> Self {
        Polkadot { sender: sender }
    }

    /// Fetch the accounts data from the substrate chain.
    pub async fn get_accounts(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create a client to fetch the onchain data from polkadot.
        let api = OnlineClient::<PolkadotConfig>::new().await?;

        let address = substrate::storage().system().account_iter();

        let mut iter =  api.storage().at_latest().await?.iter(address).await?;

        while let Some(Ok((key, account))) = iter.next().await {
            log::info!("{}: {}", hex::encode(key.clone()), account.data.free);

            let json_data = json!({
                "account": hex::encode(key),
                "balance": account.data.free.to_string()
            });

            let serialized_json_data = serde_json::to_string(&json_data)?;

            //Sending the event to TSS channel.
            match self.sender.send(serialized_json_data).await {
                Ok(()) => log::info!("Connector successfully send swap event to channel"),
                Err(e) => log::info!("Connector failed to send swap event to channel: {:?}", e),
            }
        }

        Ok(())
    }
}