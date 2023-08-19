use borsh::BorshDeserialize;
use tokio::sync::mpsc;
use tss::tss_event_model::TSSData;

pub async fn handle_tss_event(gossip_to_tss_sender: mpsc::Sender<TSSData>, data: &Vec<u8>) {
    if let Ok(parsed_data) = TSSData::try_from_slice(data) {
        if let Err(e) = gossip_to_tss_sender.send(parsed_data).await {
            log::error!("error sending gossip to tss: {}", e);
        }
    } else {
        log::error!("Unable to parse tss data");
    }
}
