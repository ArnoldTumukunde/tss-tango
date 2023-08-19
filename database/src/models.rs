use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::Value;

extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventsModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub data: Value,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContractJson {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub chain_endpoint: String,
    pub contract_address: String,
    pub event_type: String,
}

impl ContractJson {
    pub fn new(contract_address: String, event_type: String, chain_endpoint: String) -> Self {
        ContractJson {
            chain_endpoint,
            contract_address,
            event_type,
            id: None,
        }
    }
}

impl EventsModel {
    pub fn new(data: Value) -> Self {
        EventsModel { data, id: None }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenSwap {
    pub id: Option<ObjectId>,
    pub chain: String,
    pub chain_endpoint: String,
    pub exchange: String,
    pub exchange_address: String,
    pub exchange_endpoint: String,
    pub token: String,
    pub token_address: String,
    pub token_endpoint: String,
    pub swap_token: String,
    pub swap_token_address: String,
    pub swap_token_endpoint: String,
}

impl TokenSwap {
    pub fn new(
        chain: String,
        chain_endpoint: String,
        exchange: String,
        exchange_address: String,
        exchange_endpoint: String,
        token: String,
        token_address: String,
        token_endpoint: String,
        swap_token: String,
        swap_token_address: String,
        swap_token_endpoint: String,
    ) -> Self {
        TokenSwap {
            chain,
            chain_endpoint,
            exchange,
            exchange_address,
            exchange_endpoint,
            token,
            token_address,
            token_endpoint,
            swap_token,
            swap_token_address,
            swap_token_endpoint,
            id: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapEvent {
    pub id: Option<ObjectId>,
    pub chain: String,
    pub exchange: String,
    pub swap_from: String,
    pub swap_to: String,
    pub swap_price: String,
    pub created_at: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SwapEvent {
//     pub id: Option<ObjectId>,
//     pub data: Value,
// }

impl SwapEvent {
    pub fn new(
        chain: String,
        exchange: String,
        swap_from: String,
        swap_to: String,
        swap_price: String,
    ) -> Self {
        let created_at = Utc::now().timestamp_millis().to_string();
        SwapEvent {
            chain,
            exchange,
            swap_from,
            swap_to,
            swap_price,
            id: None,
            created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
    pub token_address: String,
    pub token_endpoint: String,
}

impl Token {
    pub fn new(token: String, token_address: String, token_endpoint: String) -> Self {
        Token {
            token,
            token_address,
            token_endpoint,
        }
    }
}
