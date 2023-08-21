use borsh::{BorshDeserialize, BorshSerialize};
use frost_dalek::Parameters;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TSSEventType {
    // PublishPeerIDForIndex,
    ReceivePeerIDForIndex,
    ReceiveParams,
    ReceivePeersWithColParticipant,
    ReceiveParticipant,
    ReceiveSecretShare,
    ReceiveCommitment,
    PartialSignatureGenerateReq,
    PartialSignatureReceived,
    VerifyThresholdSignature,
    ResetTSSState,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TSSData {
    pub peer_id: String,
    pub tss_event_type: TSSEventType,
    pub tss_data: Vec<u8>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PublishPeerIDCall {
    pub peer_id: String,
    pub random: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ResetTSSCall {
    pub reason: String,
    pub random: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ReceiveParamsWithPeerCall {
    pub peer_id: String,
    pub random: String,
    pub params: Parameters,
}