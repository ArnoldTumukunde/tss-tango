use ethers::prelude::*;
use eyre::Result;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct Connector {
    pub infura_endpoint: String,
    pub contract_address: String,
    pub topic: String,
    pub sender: mpsc::Sender<String>,
}

impl Connector {
    /// create a new event instance
    pub fn new(
        infura_endpoint: &str,
        contract_address: &str,
        topic: &str,
        sender: mpsc::Sender<String>,
    ) -> Self {
        Connector {
            infura_endpoint: infura_endpoint.to_string(),
            contract_address: contract_address.to_string(),
            topic: topic.to_string(),
            sender: sender,
        }
    }

    pub async fn event_listener(&self) -> Result<()> {
        let provider = Provider::<Ws>::connect(self.infura_endpoint.as_str()).await;
        if let Err(e) = provider {
            log::error!("Error connecting to provider: {:?}", e);
            return Err(eyre::eyre!("Error connecting to provider"));
        }
        let client = Arc::new(provider.unwrap());

        let current_block = client
            .get_block(BlockNumber::Latest)
            .await?
            .unwrap()
            .number
            .unwrap();
        println!("current_block: {}", current_block);

        // Fetch the last block number stored on chain.
        let mut file = File::open("blockdata.db").unwrap();
        let mut last_blocknumber = String::new();
        file.read_to_string(&mut last_blocknumber).unwrap();

        // Creating filter to fetch the events of smart contract.
        let erc20_transfer_filter = Filter::new()
            .from_block(BlockNumber::Number(U64::from(
                last_blocknumber.parse::<i64>().unwrap(),
            )))
            .address(vec![
                Address::from_str(self.contract_address.as_str()).unwrap()
            ])
            .topic0(vec![H256::from_str(self.topic.as_ref()).unwrap()]);

        let mut stream = client.subscribe_logs(&erc20_transfer_filter).await?;

        // Streaming the events to the TSS channel.
        while let Some(log) = stream.next().await {
            // Updating the block no in the db.
            let mut file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open("blockdata.db")
                .unwrap();
            let blocknumber = i64::try_from(log.block_number.unwrap()).unwrap();
            file.write_all(blocknumber.to_string().as_ref())?;

            let serialized_event = serde_json::to_string(&log).unwrap();

            //Sending the event to TSS channel.
            match self.sender.send(serialized_event).await {
                Ok(()) => log::info!("Connector successfully send event to channel"),
                Err(_) => log::info!("Connector failed to send event to channel"),
            }
        }

        Ok(())
    }
}
