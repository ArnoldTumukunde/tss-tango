use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    results::DeleteResult,
};
pub mod models;

use crate::models::{ContractJson, EventsModel, SwapEvent, Token, TokenSwap};
use mongodb::{
    bson::extjson::de::Error,
    results::{InsertManyResult, InsertOneResult},
    Client, Collection,
};

use futures::stream::StreamExt;
// use futures::stream::{StreamExt, TryStreamExt};

#[derive(Clone)]
pub struct MongoRepo {
    events: Collection<EventsModel>,
    swap_event: Collection<SwapEvent>,
    token_swap: Collection<TokenSwap>,
    contracts: Collection<ContractJson>,
    tokens: Collection<Token>,
}

impl MongoRepo {
    pub async fn init(uri: &str, database: &str, collection: Vec<&str>) -> Self {
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database(&database);
        let events: Collection<EventsModel> = db.collection::<EventsModel>(&collection[0]);
        let contracts: Collection<ContractJson> = db.collection::<ContractJson>(&collection[1]);
        let token_swap: Collection<TokenSwap> = db.collection::<TokenSwap>(&collection[2]);
        let swap_event: Collection<SwapEvent> = db.collection::<SwapEvent>(&collection[3]);
        let tokens: Collection<Token> = db.collection::<Token>(&collection[4]);
        MongoRepo {
            events,
            contracts,
            token_swap,
            swap_event,
            tokens,
        }
    }

    pub async fn insert_event(
        collection: &Self,
        event: EventsModel,
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        collection.events.insert_one(event, None).await
    }

    pub async fn get_event_data(collection: &Self) -> Result<Vec<EventsModel>, Error> {
        let mut cursor = collection
            .events
            .find(None, None)
            .await
            .ok()
            .expect("Error getting events");

        let mut data: Vec<EventsModel> = Vec::new();

        while let Some(doc) = cursor.next().await {
            data.push(doc.unwrap());
        }

        Ok(data)
    }

    pub async fn get_contract_event(
        collection: &Self,
        contract: String,
    ) -> Result<Vec<EventsModel>, Error> {
        let option = FindOptions::builder().limit(25).build();
        let mut cursor = collection
            .events
            .find(doc! {"address":contract}, option)
            .await
            .ok()
            .expect("Error getting events");

        let mut data: Vec<EventsModel> = Vec::new();

        while let Some(doc) = cursor.next().await {
            data.push(doc.unwrap());
        }

        Ok(data)
    }
    pub async fn get_swap_data(collection: &Self) -> Result<Vec<TokenSwap>, Error> {
        let cursor = collection.token_swap.find(None, None).await;
        match cursor {
            Ok(mut cursor) => {
                let mut data: Vec<TokenSwap> = Vec::new();
                while let Some(doc) = cursor.next().await {
                    data.push(doc.unwrap());
                }

                Ok(data)
            }
            Err(_msg) => Ok(vec![]),
        }
    }
    pub async fn if_exist_swap_data(
        collection: &Self,
        token_swap: TokenSwap,
    ) -> Result<bool, mongodb::error::Error> {
        let result = collection
            .token_swap
            .find_one(doc! {"token_address":token_swap.token_address, "swap_token_address":token_swap.swap_token_address}, None)
            .await
            .ok()
            .expect("Error finding Token swap pair");
        let value = result.ok_or(false);

        let val = match value {
            Ok(_res) => true,
            Err(err) => err,
        };

        Ok(val)
    }
    pub async fn insert_single_swap_data(
        collection: &Self,
        token_swap: TokenSwap,
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        let result = collection
            .token_swap
            .insert_one(token_swap, None)
            .await
            .ok()
            .expect("Error creating Token swap");

        Ok(result)
    }
    pub async fn insert_swap_data(
        collection: &Self,
        token_swap: Vec<TokenSwap>,
    ) -> Result<InsertManyResult, mongodb::error::Error> {
        let result = collection
            .token_swap
            .insert_many(token_swap, None)
            .await
            .ok()
            .expect("Error creating Token swap");

        Ok(result)
    }
    pub async fn insert_contract_json(
        collection: &Self,
        contract_json: Vec<ContractJson>,
    ) -> Result<InsertManyResult, mongodb::error::Error> {
        let result = collection
            .contracts
            .insert_many(contract_json, None)
            .await
            .ok()
            .expect("Error creating Contracts");

        Ok(result)
    }

    pub async fn delete_contract(
        collection: &Self,
        id: String,
    ) -> Result<DeleteResult, mongodb::error::Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let result = collection
            .contracts
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error creating Contracts");

        Ok(result)
    }
    pub async fn get_contract_json(collection: &Self) -> Result<Vec<ContractJson>, Error> {
        let mut cursor = collection
            .contracts
            .find(None, None)
            .await
            .ok()
            .expect("Error getting contracts");

        let mut data: Vec<ContractJson> = Vec::new();

        while let Some(doc) = cursor.next().await {
            data.push(doc.unwrap());
        }

        Ok(data)
    }

    pub async fn get_swap_events(collection: &Self) -> Result<Vec<SwapEvent>, Error> {
        let mut cursor = collection
            .swap_event
            .find(None, None)
            .await
            .ok()
            .expect("Error getting token swap");

        let mut data: Vec<SwapEvent> = Vec::new();

        while let Some(doc) = cursor.next().await {
            data.push(doc.unwrap());
        }

        Ok(data)
    }

    pub async fn get_swap_pair_price(
        swap_from: String,
        swap_to: String,
        collection: &Self,
    ) -> Result<Vec<EventsModel>, Error> {
        let mut cursor = collection
            .events
            .find(
                doc! {"data.swap_from":swap_from, "data.swap_to":swap_to},
                None,
            )
            .await
            .ok()
            .expect("Error getting token swap");

        let mut data: Vec<EventsModel> = Vec::new();

        while let Some(doc) = cursor.next().await {
            data.push(doc.unwrap());
        }

        Ok(data)
    }

    pub async fn insert_swap_event(
        collection: &Self,
        token_swap: Vec<SwapEvent>,
    ) -> Result<InsertManyResult, mongodb::error::Error> {
        // log::info!("created at val --> {}", token_swap[0].created_at);
        let result = collection
            .swap_event
            .insert_many(token_swap, None)
            .await
            .ok()
            .expect("Error creating Token swap");

        Ok(result)
    }

    pub async fn get_tokens(collection: &Self) -> Result<Vec<Token>, Error> {
        let mut cursor = collection
            .tokens
            .find(None, None)
            .await
            .ok()
            .expect("Error getting token swap");

        let mut data: Vec<Token> = Vec::new();

        while let Some(doc) = cursor.next().await {
            data.push(doc.unwrap());
        }

        Ok(data)
    }
    pub async fn insert_tokens(
        collection: &Self,
        tokens: Token,
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        let result = collection
            .tokens
            .insert_one(tokens, None)
            .await
            .ok()
            .expect("Error creating Token swap");

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{ContractJson, EventsModel},
        MongoRepo,
    };

    async fn get_connection() -> MongoRepo {
        let db_url = "mongodb://localhost:27017/admin";
        let mut collections = Vec::new();
        collections.push("events");
        collections.push("contracts");
        collections.push("token_swap");
        collections.push("swap_events");
        collections.push("tokens");
        let connector = MongoRepo::init(&db_url, "tango_db", collections).await;
        connector
    }

    #[tokio::test]

    async fn connecting_test() {
        let _connector = get_connection().await;
    }

    #[tokio::test]
    async fn retrive_contracts() {
        let _connector = get_connection().await;
        let connector_json = MongoRepo::get_contract_json(&_connector).await.unwrap();

        assert!(!connector_json.is_empty(), "No records found");
    }

    #[tokio::test]
    async fn retrive_events() {
        let _connector = get_connection().await;
        let _result = MongoRepo::get_event_data(&_connector).await.unwrap();
    }
    #[tokio::test]
    async fn insert_contracts() {
        let mut data: Vec<ContractJson> = Vec::new();
        let input = ContractJson::new(
            "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984".to_string(),
            "wss://mainnet.infura.io/ws/v3/81e31919d7dd4150a26402a0374fe923".to_string(),
            "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string(),
        );
        data.push(input);
        let _connector = get_connection().await;
        let _result = MongoRepo::insert_contract_json(&_connector, data)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn insert_events() {
        let msg = r#"{"address":"0x0000000000000000000000000000000000000000","topics":["0x0000000000000000000000000000000000000000000000000000000000000000"],"data":"0x0000000000000000000000000000000000000000000000000000000000000000","block_hash":null,"block_number":null,"transaction_hash":null,"transaction_index":null,"log_index":null,"transaction_log_index":null,"log_type":null,"removed":null}"#;
        let data = serde_json::to_value(msg).unwrap();
        let input = EventsModel::new(data);

        let _connector = get_connection().await;
        let _result = MongoRepo::insert_event(&_connector, input).await.unwrap();
    }
}
