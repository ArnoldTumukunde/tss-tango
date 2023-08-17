use ethers::types::{Bytes, H256, Address, U64, U256};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventsModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub address: Address,
    pub topic: Vec<H256>,
    pub data: Option<Bytes>,
    pub block_hash: Option<H256>,
    pub block_number: Option<U64>,
    pub transaction_hash: Option<H256>,
    pub transaction_index: Option<U64>,
    pub log_index: Option<U256>,
    pub transaction_log_index: Option<U256>,
    pub log_type: Option<String>,
    pub removed: Option<bool>,
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
    pub fn new(
        address: Address,
        topic: Vec<H256>,
        data: Option<Bytes>,
        block_hash: Option<H256>,
        block_number: Option<U64>,
        transaction_hash: Option<H256>,
        transaction_index: Option<U64>,
        log_index: Option<U256>,
        transaction_log_index: Option<U256>,
        log_type: Option<String>,
        removed: Option<bool>,
    ) -> Self {
        EventsModel {
            address,
            topic,
            data,
            block_number,
            transaction_hash,
            transaction_index,
            log_index,
            transaction_log_index,
            log_type,
            removed,
            id: None,
            block_hash,
        }
    }
}
