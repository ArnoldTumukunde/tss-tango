use borsh::{BorshDeserialize, BorshSerialize};
use frost_dalek::{
    keygen::{Coefficients, RoundOne, RoundTwo, SecretShare},
    precomputation::{PublicCommitmentShareList, SecretCommitmentShareList},
    signature::{PartialThresholdSignature, SecretKey, Signer, ThresholdSignature},
    DistributedKeyGeneration, GroupKey, IndividualPublicKey, Parameters, Participant,
};
use keystore::commands::KeyTypeId;
use sp_keystore::SyncCryptoStore;
use std::sync::Arc;
use std::{collections::HashMap, fmt};

use crate::{DEFUALT_TSS_THRESHOLD, DEFUALT_TSS_TOTAL_NODES};

#[derive(Debug, BorshSerialize, BorshDeserialize, Clone, PartialEq)]
pub struct OthersCommitmentShares {
    pub public_key: IndividualPublicKey,
    pub public_commitment_share_list: PublicCommitmentShareList,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Clone)]
pub struct PartialMessageSign {
    pub msg_hash: [u8; 64],
    pub signers: Vec<Signer>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Clone)]
pub struct ReceivePartialSignatureReq {
    pub msg_hash: [u8; 64],
    pub partial_sign: PartialThresholdSignature,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct VerifyThresholdSignatureReq {
    pub msg_hash: [u8; 64],
    pub threshold_sign: ThresholdSignature,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct FilterAndPublishParticipant {
    pub total_peer_list: Vec<String>,
    pub col_participant: Participant,
}


pub struct TSSCliParams {
    pub total_nodes: u8,
    pub threshold: u8,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
pub enum TSSLocalStateType {
    NotParticipating,
    Empty,
    ReceivedPeers,
    ReceivedParams,
    DkgGeneratedR1,
    DkgGeneratedR2,
    StateFinished,
    CommitmentsReceived,
}

// #[derive(Debug)]
pub struct TSSLocalStateData {
    pub is_node_collector: bool,
    pub is_node_aggregator: bool,
    pub context: [u8; 25],
    pub tss_process_state: TSSLocalStateType,
    pub tss_params: Parameters,
    pub local_peer_id: Option<String>,
    pub others_peer_id: Vec<String>,
    pub local_index: Option<u32>,
    pub key_type: Option<KeyTypeId>,
    pub keystore: Option<Arc<dyn SyncCryptoStore>>,
    pub local_participant: Option<(Participant, Coefficients)>,
    pub others_participants: Vec<Participant>,
    pub local_dkg_r1_state: Option<DistributedKeyGeneration<RoundOne>>,
    pub others_my_secret_share: Vec<SecretShare>,
    pub local_dkg_r2_state: Option<DistributedKeyGeneration<RoundTwo>>,
    pub local_finished_state: Option<(GroupKey, SecretKey)>,
    pub local_public_key: Option<IndividualPublicKey>,
    pub local_commitment_share: Option<(PublicCommitmentShareList, SecretCommitmentShareList)>,
    pub others_commitment_share: Vec<OthersCommitmentShares>,
    pub others_partial_signature: HashMap<[u8; 64], Vec<PartialThresholdSignature>>,
    pub msg_pool: HashMap<[u8; 64], Vec<u8>>,
    pub msgs_signature_pending: HashMap<[u8; 64], Vec<Signer>>,
}

impl fmt::Debug for TSSLocalStateData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TSSLocalStateData")
            .field("is_node_collector", &self.is_node_collector)
            .field("is_node_aggregator", &self.is_node_aggregator)
            .field("tss_process_state", &self.tss_process_state)
            .field("tss_params", &self.tss_params)
            .field("key_type", &self.key_type)
            .field("local_peer_id", &self.local_peer_id)
            .field("others_peer_id", &self.others_peer_id)
            .field("local_index", &self.local_index)
            .field("local_participant", &self.local_participant.is_some())
            .field("others_participants", &self.others_participants.len())
            .field("local_dkg_r1_state", &self.local_dkg_r1_state.is_some())
            .field("others_my_secret_share", &self.others_my_secret_share.len())
            .field("local_dkg_r2_state", &self.local_dkg_r2_state)
            .field("local_finished_state", &self.local_finished_state)
            .field("local_public_key", &self.local_public_key)
            .field("local_commitment_share", &self.local_commitment_share)
            .field("others_commitment_share", &self.others_commitment_share)
            .field("others_partial_signature", &self.others_partial_signature)
            .finish()
    }
}

impl TSSLocalStateData {
    pub fn new() -> TSSLocalStateData {
        TSSLocalStateData {
            is_node_collector: false,
            is_node_aggregator: false,
            context: *b"TANGOS-EVENT-DATA-SIGNING",
            tss_process_state: TSSLocalStateType::Empty,
            tss_params: Parameters {
                n: DEFUALT_TSS_TOTAL_NODES,
                t: DEFUALT_TSS_THRESHOLD,
            },
            key_type: None,
            keystore: None,
            local_peer_id: None,
            others_peer_id: vec![],
            local_index: None,
            local_participant: None,
            others_participants: vec![],
            local_dkg_r1_state: None,
            others_my_secret_share: vec![],
            local_dkg_r2_state: None,
            local_finished_state: None,
            local_public_key: None,
            local_commitment_share: None,
            others_commitment_share: vec![],
            others_partial_signature: HashMap::new(),
            msg_pool: HashMap::new(),
            msgs_signature_pending: HashMap::new(),
        }
    }

    pub fn reset(self: &mut Self) {
        self.is_node_collector = false;
        self.is_node_aggregator = false;
        self.tss_process_state = TSSLocalStateType::Empty;
        self.tss_params = Parameters {
            n: DEFUALT_TSS_TOTAL_NODES,
            t: DEFUALT_TSS_THRESHOLD,
        };
        self.others_peer_id = vec![];
        self.local_index = None;
        self.local_participant = None;
        self.others_participants = vec![];
        self.local_dkg_r1_state = None;
        self.others_my_secret_share = vec![];
        self.local_dkg_r2_state = None;
        self.local_finished_state = None;
        self.local_public_key = None;
        self.local_commitment_share = None;
        self.others_commitment_share = vec![];
        self.others_partial_signature = HashMap::new();
        self.msg_pool = HashMap::new();
        self.msgs_signature_pending = HashMap::new();
    }
}
