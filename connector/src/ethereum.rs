use database::models::TokenSwap;
use database::MongoRepo;
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use tokio::sync::mpsc;
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::types::{Address, U256};

#[derive(Clone)]
pub struct SwapToken {
    pub web_socket: web3::Web3<Http>,
    pub connection: MongoRepo,
    pub sender: mpsc::Sender<String>,
}

impl SwapToken {
    /// create a new event instance
    pub fn new(
        web_socket: web3::Web3<Http>,
        connection: MongoRepo,
        sender: mpsc::Sender<String>,
    ) -> Self {
        SwapToken {
            web_socket: web_socket,
            connection: connection,
            sender: sender,
        }
    }

    pub async fn swap_thread_handler(&self, index: i32) {
        let swap_data = MongoRepo::get_swap_data(&self.connection.clone())
            .await
            .unwrap();

        for swap in swap_data {
            let arguments = self.clone();
            tokio::spawn(async move {
                let _ = SwapToken::swap_handler(&arguments, swap, index).await;
            });
        }
    }

    pub async fn swap_handler_old(&self, swap: TokenSwap) -> Result<(), Box<dyn Error>> {
        let websocket = web3::Web3::new(
            Http::new("https://a507940678ae4740a727967aa8566e08.eth.rpc.rivet.cloud").unwrap(),
        );
        let unit_from: u64 = i64::pow(
            10,
            SwapToken::decimals(
                &websocket,
                swap.token_endpoint.as_str(),
                swap.token_address.as_str(),
            )
            .await
            .map_err(|e| Into::<Box<dyn Error>>::into(e))? as u32,
        ) as u64;

        let unit_to: u64 = i64::pow(
            10,
            SwapToken::decimals(
                &websocket,
                swap.token_endpoint.as_str(),
                swap.swap_token_address.as_str(),
            )
            .await
            .map_err(|e| Into::<Box<dyn Error>>::into(e))? as u32,
        ) as u64;
        // log::info!("atomic values ---> {:?}--{:?}", unit_to, unit_from);

        let from_val: f64 = 1.0;
        let query_method = "getAmountsOut";

        let query_parameter = (
            U256::from((from_val * unit_from as f64) as u64),
            [
                Address::from_str(swap.token_address.as_str()).unwrap(),
                Address::from_str(swap.swap_token_address.as_str()).unwrap(),
            ],
            12,
        );

        // log::info!("query_parameter ---> {:?}", query_parameter);
        let swap_result: Vec<i32> = SwapToken::swap_price(
            &self.web_socket,
            swap.exchange_endpoint.as_str(),
            swap.exchange_address.as_str(),
            query_method,
            query_parameter,
        )
        .await
        .map_err(|e| Into::<Box<dyn std::error::Error>>::into(e))?;
        log::info!("swap ----> {:?}", swap_result);
        if swap_result.len() != 2 {
            return Err(From::from("Received Unexpected swap results."));
        }

        // let swap_value = swap_result[1].as_u128() as f64 / unit_to as f64;
        let swap_value = swap_result[1] / unit_to as i32;
        log::info!(
            "swap {} token to {} token:  {:?}",
            swap.token,
            swap.swap_token,
            swap_value
        );

        Ok(())
    }

    pub async fn swap_handler(
        &self,
        swap: TokenSwap,
        index: i32,
    ) -> Result<Vec<i32>, Box<dyn Error>> {
        // let websocket = web3::Web3::new(
        //     Http::new("https://a507940678ae4740a727967aa8566e08.eth.rpc.rivet.cloud").unwrap(),
        // );

        let unit_from: u64 = i64::pow(
            10,
            SwapToken::decimals(
                &self.web_socket,
                swap.token_endpoint.as_str(),
                swap.token_address.as_str(),
            )
            .await
            .map_err(|e| Into::<Box<dyn Error>>::into(e))? as u32,
        ) as u64;
        log::info!("query_parameter ---> {:?}", unit_from);
        let unit_to: u64 = i64::pow(
            10,
            SwapToken::decimals(
                &self.web_socket,
                swap.token_endpoint.as_str(),
                swap.swap_token_address.as_str(),
            )
            .await
            .map_err(|e| Into::<Box<dyn Error>>::into(e))? as u32,
        ) as u64;
        log::info!("query_parameter ---> {:?}", unit_to);
        let from_val: f64 = 1.0;
        let query_method = "getAmountsOut";

        let query_parameter = (
            U256::from((from_val * unit_from as f64) as u64),
            [
                Address::from_str(swap.token_address.as_str()).unwrap(),
                Address::from_str(swap.swap_token_address.as_str()).unwrap(),
            ],
            12,
        );
        // log::info!("query_parameter ---> {:?}", query_parameter);
        let swap_result: Vec<i32> = SwapToken::swap_price(
            &self.web_socket,
            swap.exchange_endpoint.as_str(),
            swap.exchange_address.as_str(),
            query_method,
            query_parameter,
        )
        .await
        .map_err(|e| Into::<Box<dyn std::error::Error>>::into(e))?;
        log::info!("swap ----> {:?}", swap_result);
        if swap_result.len() != 2 {
            return Err(From::from("Received Unexpected swap results."));
        }

        // let swap_value = swap_result[1].as_u128() as f64 / unit_to as f64;
        let swap_value = swap_result[0];
        log::info!(
            "swap {} token to {} token:  {:?}",
            swap.token,
            swap.swap_token,
            swap_value
        );

        let swapevent_json = json!({
            "chain": swap.chain.clone(),
            "exchange": swap.exchange.clone(),
            "swap_from": swap.token_address.clone(),
            "swap_to": swap.swap_token_address.clone(),
            "swap_price": swap_value.clone().to_string(),
            "swap_index": index,
            "event_type": "swap"
        });
        let serialized_swapevent = serde_json::to_string(&swapevent_json).unwrap();
        //Sending the event to TSS channel.
        match self.sender.send(serialized_swapevent).await {
            Ok(()) => log::info!("Connector successfully send swap event to channel"),
            Err(e) => log::info!("Connector failed to send swap event to channel: {:?}", e),
        }

        Ok(swap_result)
    }

    /// This function is to fetch the swap price of a token.
    pub async fn swap_price<P: web3::contract::tokens::Tokenize>(
        web_socket: &web3::Web3<Http>,
        abi_url: &str,
        exchange_address: &str,
        query_method: &str,
        _query_parameter: P,
    ) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
        let exchange = Address::from_str(exchange_address).unwrap();
        let mut res = String::new();
        if abi_url.contains("http") {
            res = reqwest::blocking::get(abi_url).unwrap().text().unwrap();
        } else {
            let mut abi_file = File::open(abi_url).unwrap();
            abi_file.read_to_string(&mut res).unwrap();
        }

        // log::info!("token_contract ---> node start up {}",res);
        let json: serde_json::Value =
            serde_json::from_str(&res.to_owned()).expect("JSON was not well-formatted");
        // log::info!("token_contract ---> node start up {}",json);

        let abi_date = match serde_json::to_string(&json["abi"]) {
            Ok(response_str) => response_str,
            Err(_) => "data Error".to_string(),
        };
        // log::info!("abi_date ---> node start up {}",abi_date);

        // let abi: String = match &json["abi"] {
        //     serde_json::Value::String(v) => v.clone(),
        //     _ => return Err(From::from("abi not found")),
        // };
        // log::info!("abi ---> node start up {}",abi);
        // let abi = match serde_json::to_string(&json["abi"]) {
        //     Ok(response_str) => response_str,
        //     Err(_) => "data Error".to_string(),
        // };
        // log::info!("json ---> node start up {}",json);
        // Accessing existing contract of exchange.
        let token_contract =
            match Contract::from_json(web_socket.eth(), exchange, abi_date.as_bytes()) {
                Ok(contract) => contract,
                Err(error) => return Err(From::from(error)),
            };
        // log::info!("json ---> node start up {:?}",token_contract);
        let query_response: Vec<i32> = match token_contract
            .query(
                query_method,
                (exchange.clone(), exchange.clone(), 25),
                None,
                Options::default(),
                None,
            )
            .await
        {
            Ok(query_response) => query_response,
            Err(error) => return Err(From::from(error)),
        };
        // log::info!("json ---> node start up {:?}", query_response);
        Ok(query_response)
    }

    /// This function is to fetch the decimals digit of a token.
    pub async fn decimals(
        web_socket: &web3::Web3<Http>,
        token_abi_url: &str,
        token_address: &str,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let exchange = match Address::from_str(token_address) {
            Ok(address) => address,
            Err(error) => return Err(From::from(error)),
        };
        // log::info!("atomic values ----3");
        let mut abi_file = File::open(token_abi_url).unwrap();
        let mut res = String::new();
        abi_file.read_to_string(&mut res).unwrap();

        // log::info!("token_contract ---> node start up {}",res);
        let json: serde_json::Value =
            serde_json::from_str(&res.to_owned()).expect("JSON was not well-formatted");
        // log::info!("token_contract ---> node start up {}",json);

        let abi_date = match serde_json::to_string(&json["abi"]) {
            Ok(response_str) => response_str,
            Err(_) => "data Error".to_string(),
        };
        // log::info!("atomic values ----44");
        // let abi: String = match &json["abi"] {
        //     serde_json::Value::String(v) => v.clone(),
        //     _ => String::from(""),
        // };

        // Accessing existing contract of exchange.
        let token_contract =
            match Contract::from_json(web_socket.eth(), exchange, abi_date.as_bytes()) {
                Ok(contract) => contract,
                Err(error) => return Err(From::from(error)),
            };
        // fetching the decimal of a particular token.
        // log::info!("atomic values ----5");
        token_contract
            .query("decimals", (), None, Options::default(), None)
            .await
            .map_err(Into::into)
    }
    pub async fn decimals_old(
        web_socket: &web3::Web3<Http>,
        token_abi_url: &str,
        token_address: &str,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let exchange = match Address::from_str(token_address) {
            Ok(address) => address,
            Err(error) => return Err(From::from(error)),
        };
        // todo file or url
        let res = match reqwest::blocking::get(token_abi_url) {
            Ok(url) => match url.text() {
                Ok(url) => url,
                Err(error) => return Err(From::from(error)),
            },
            Err(error) => return Err(From::from(error)),
        };

        let json: serde_json::Value =
            serde_json::from_str(&res.to_owned()).expect("JSON was not well-formatted");
        log::info!("atomic values ----4");
        let abi: String = match &json["result"] {
            serde_json::Value::String(v) => v.clone(),
            _ => String::from(""),
        };

        // Accessing existing contract of exchange.
        let token_contract = match Contract::from_json(web_socket.eth(), exchange, abi.as_bytes()) {
            Ok(contract) => contract,
            Err(error) => return Err(From::from(error)),
        };
        // fetching the decimal of a particular token.

        token_contract
            .query("decimals", (), None, Options::default(), None)
            .await
            .map_err(Into::into)
    }
}
