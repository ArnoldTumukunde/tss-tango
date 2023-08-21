use dirs;
use libp2p::identity::{self, ed25519, Keypair};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use zeroize::Zeroizing;

use crate::utils::identity_handler;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Config {
    pub identity: Identity,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Identity {
    #[serde(rename = "PeerID")]
    pub peer_id: String,
    pub priv_key: String,
}

impl Config {
    #[allow(dead_code)]
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
    }

    #[allow(dead_code)]
    pub fn from_key_material(peer_id: PeerId, keypair: &Keypair) -> Result<Self, Box<dyn Error>> {
        let priv_key = base64::encode(keypair.to_protobuf_encoding()?);
        let peer_id = peer_id.to_base58();
        Ok(Self {
            identity: Identity { peer_id, priv_key },
        })
    }
}

impl zeroize::Zeroize for Config {
    fn zeroize(&mut self) {
        self.identity.peer_id.zeroize();
        self.identity.priv_key.zeroize();
    }
}

#[allow(dead_code)]
pub fn get_or_create() -> Result<(PeerId, Keypair), Box<dyn Error>> {
    let keypair_path: String = if let Some(path) = get_keypair_path() {
        path
    } else {
        let curr_path = String::from(env::current_dir().unwrap().to_string_lossy());
        let identity_path = format!("{}/identity.json", curr_path);
        identity_path
    };

    validate_path(&keypair_path)?;

    if Path::new(&keypair_path).is_file() {
        //read from file
        if let Ok((peer_id, keypair)) = read_config_from_file(Path::new(&keypair_path)) {
            return Ok((peer_id, keypair));
        } else {
            return Err("Unable to get identity from file".into());
        }
    } else {
        //create new config and save
        let (local_peer_id, local_keypair) = create_new_identity();

        //save into file
        if let Ok(config) = Config::from_key_material(local_peer_id, &local_keypair) {
            if let Err(_) = write_config_to_file(&keypair_path, &config) {
                return Err("Failed to save config".into());
            }
        } else {
            return Err("Failed make config object".into());
        }

        Ok((local_peer_id, local_keypair))
    }
}

#[allow(dead_code)]
pub fn create_new_identity() -> (PeerId, Keypair) {
    let keypair = identity::Keypair::Ed25519(ed25519::Keypair::generate());
    (keypair.public().into(), keypair)
}

#[allow(dead_code)]
fn read_config_from_file(path: &Path) -> Result<(PeerId, Keypair), Box<dyn Error>> {
    let config = Zeroizing::new(Config::from_file(path)?);
    let keypair = identity::Keypair::from_protobuf_encoding(&Zeroizing::new(base64::decode(
        config.identity.priv_key.as_bytes(),
    )?))?;
    let peer_id: PeerId = keypair.public().into();
    assert_eq!(
        PeerId::from_str(&config.identity.peer_id)?,
        peer_id,
        "Expect peer id derived from private key and peer id retrieved from config to match."
    );

    Ok((peer_id, keypair))
}

#[allow(dead_code)]
fn write_config_to_file(path: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    serde_json::to_writer(&File::create(path)?, config)?;
    Ok(())
}

fn get_keypair_path() -> Option<String> {
    let default_proj_path = dirs::home_dir();
    if let Some(path) = default_proj_path {
        let my_home = path.into_os_string().into_string().unwrap();
        let identity_folder = my_home + "/analog/identity/";
        if let Err(e) = std::fs::create_dir_all(PathBuf::from(identity_folder.clone())) {
            log::error!("Unable to create identity file in home {}", e);
        }
        let identity_path = format!("{}identity.json", identity_folder);
        Some(identity_path)
    } else {
        log::error!("Unable to get home path error");
        None
    }
}

fn validate_path(path: &str) -> Result<(), Box<dyn Error>> {
    log::info!("Identity path is: {}", path);
    if !path.split(".").last().eq(&Some("json")) {
        log::error!("Invalid identity path");
        return Err("Invalid identity path".into());
    }
    Ok(())
}

pub fn get_node_identity(new_node: bool) -> (PeerId, Keypair) {
    if new_node {
        // Create a random PeerId
        let id_keys = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());
        log::info!("Local peer id: {:?}", peer_id);
        (peer_id, id_keys)
    } else {
        //load identity file
        if let Ok((peer_id, id_keys)) = identity_handler::get_or_create() {
            log::info!("Local peer id: {:?}", peer_id);
            (peer_id, id_keys)
        } else {
            //if error getting identity create new
            let id_keys = identity::Keypair::generate_ed25519();
            let peer_id = PeerId::from(id_keys.public());
            log::info!("Local peer id: {:?}", peer_id);
            (peer_id, id_keys)
        }
    }
}
