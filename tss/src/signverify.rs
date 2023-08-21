use accounts::Account;
use keystore::commands::KeyTypeId;
use keystore::error;
use serde_json::Value;
use sp_core::sr25519::Signature;
use sp_core::{Pair, Public};
use sp_keystore::SyncCryptoStore;
use std::collections::HashMap;
use std::error::Error;
use std::{convert::TryFrom, sync::Arc};
use tango_database::models::EventsModel;
use tango_database::MongoRepo;

pub async fn sign_data(
    acc: Account,
    connector: MongoRepo,
    msg: String,
    key_type: KeyTypeId,
    keystore: Arc<dyn SyncCryptoStore>,
) -> Result<Signature, Box<dyn Error>> {
    let sig_data = match SyncCryptoStore::sign_with(
        &*keystore,
        key_type,
        &acc.accounts.to_public_crypto_pair(),
        &msg.clone().as_bytes(),
    ) {
        Ok(sig) => match sig {
            Some(sig) => sig,
            None => return Err(Box::new(error::Error::from("Key doesn't exist"))),
        },
        Err(e) => {
            log::error!("Error signing data: {:?}", e);
            return Err(Box::new(e));
        }
    };

    //create signature
    let signature = match <sp_core::sr25519::Pair as Pair>::Signature::try_from(sig_data.as_slice())
        .map_err(|_| error::Error::SignatureFormatInvalid)
    {
        Ok(sig) => sig,
        Err(e) => {
            log::error!("Error creating signature: {:?}", e);
            return Err(Box::new(e));
        }
    };
    
    // let connector = get_connection("mongodb://localhost:27017".to_string()).await;
    // deserialize message(event_data) to Log
    let sig_value = match serde_json::to_value(signature.clone()) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error creating signature value: {:?}", e);
            return Err(Box::new(e));
        }
    };

    let pubkey_value = match serde_json::to_value(acc.accounts) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error creating pubkey value: {:?}", e);
            return Err(Box::new(e));
        }
    };

    let mut data = match serde_json::from_str::<HashMap<String, Value>>(&msg) {
        Ok(data) => data,
        Err(e) => return Err(e.into()),
    };

    data.insert("signature".to_string(), sig_value);
    data.insert("signer".to_string(), pubkey_value);

    match store_data(data, connector).await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error storing data value: {:?}", e);
            return Err(e.into());
        }
    }
    //return signature
    Ok(signature)
}

pub async fn store_data(
    data: HashMap<String, Value>,
    connector: MongoRepo,
) -> Result<(), Box<dyn Error>> {
    //store event data in db
    let data = serde_json::to_value(data).unwrap();
    let _connector_json = match MongoRepo::insert_event(
        &connector,
        EventsModel { id: None, data },
    )
    .await
    {
        Ok(data) => data,
        Err(e) => return Err(e.into()),
    };
    Ok(())
}

pub async fn verify_data(
    sig: Signature,
    msg: String,
    pubkey: sp_core::sr25519::Public,
) -> Result<(), Box<dyn Error>> {
    //check if the signature message and public key are valid
    if <sp_core::sr25519::Pair as Pair>::verify(&sig, &msg, &pubkey) {
        log::info!("Signature verifies correctly.");
        Ok(())
    } else {
        log::error!("Signature invalid.");
        return Err("Signature invalid/incorrect".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use keystore::params::keystore_params::KeystoreParams;
    use sc_keystore::LocalKeystore;
    use sc_service::config::KeystoreConfig;
    use sp_keystore::SyncCryptoStorePtr;
    use std::env;

    async fn get_connection(db_url: String) -> MongoRepo {
        let mut collections = Vec::new();
        collections.push("events");
        collections.push("contracts");
        let connector = MongoRepo::init(&db_url, "tango_db", collections).await;

        connector
    }

    #[tokio::test]
    async fn test_sign_event_data() {
        let keystore_params = KeystoreParams::default();

        let config_dir = env::current_dir().unwrap();
        let keystore = match keystore_params.keystore_config(&config_dir).unwrap() {
            (_, KeystoreConfig::Path { path, password }) => {
                // let public = with_crypto_scheme!(self.scheme, to_vec(&suri, password.clone()))?;
                let keystore: SyncCryptoStorePtr =
                    Arc::new(LocalKeystore::open(path, password).unwrap());
                keystore
            }
            _ => unreachable!("keystore_config always returns path and password; qed"),
        };
        let key_type_str = "tngo";
        let key_type = KeyTypeId::try_from(key_type_str).unwrap();

        let acc = match Account::new("tango", key_type, keystore.clone()){
            Ok(acc) => acc,
            Err(e) => {
                log::error!("Error creating account {}",e);
                return;
            }
        };
        let connector = get_connection("mongodb://localhost:27017".to_string()).await;
        let msg = r#"{"address":"0x0000000000000000000000000000000000000000","topics":["0x0000000000000000000000000000000000000000000000000000000000000000"],"data":"0x0000000000000000000000000000000000000000000000000000000000000000","block_hash":null,"block_number":null,"transaction_hash":null,"transaction_index":null,"log_index":null,"transaction_log_index":null,"log_type":null,"removed":null}"#;
        let sig = sign_data(acc.clone(), connector, msg.to_string(), key_type, keystore)
            .await
            .unwrap();
        match verify_data(sig, msg.to_string(), acc.accounts).await {
            Ok(_d) => assert!(true),
            Err(_e) => assert!(false),
        };
    }
}
