use mongodb::Client;
use std::{process::Command, time::Instant};
use tango_database::models::EventsModel;

////////////////////////////////////////
/// Warning this program can kill all your tmux sessions
/// Requirements:
/// have a mongodb running with a database called tango_db and a collection called events
/// have tmux installed on your system for more info read scripts/tmux/readme.md

#[tokio::main]
async fn main() {
    let db_url = "mongodb://localhost:27017".to_string();
    let client = Client::with_uri_str(db_url).await.unwrap();
    let db = client.database("tango_db");
    let collection = db.collection::<EventsModel>("events");

    let mut subscription_start_time = Instant::now();

    let mut avg_time_arr = vec![];

    let total_rounds = 5;
    for i in 0..total_rounds {
        //killing all tmux sessions
        let _ = Command::new("pkill").args(&["-f", "tmux"]).output();

        //starting tmux tss_benchmark session
        let _ = Command::new("tmuxp")
            .args(&["load", "tss_bench_n3t2.yaml"])
            .current_dir("scripts/tmux/")
            .output();

        let mut start_timer = true;

        //fetching initial number of docs in the collection
        let starting_event_num = collection.count_documents(None, None).await.unwrap();
        loop {
            // tokio::time::sleep(time::Duration::from_millis(100)).await;
            let total_events = collection.count_documents(None, None).await.unwrap();
            println!("Total events: {:?} for round {}", total_events, i + 1);

            if start_timer && total_events > starting_event_num + 2 {
                println!("started timer");
                subscription_start_time = Instant::now();
                start_timer = false;
            }
            if total_events >= starting_event_num + 2000 {
                avg_time_arr.push(subscription_start_time.elapsed().as_secs());
                println!("time elapsed: {:?}", subscription_start_time.elapsed());
                break;
            }
        }
    }
    //killing all tmux sessions
    let _ = Command::new("pkill").args(&["-f", "tmux"]).output();
    println!("Time taken by each round: {:?}", avg_time_arr);
    println!(
        "Average time: {:?} secs",
        avg_time_arr.iter().sum::<u64>() / total_rounds
    );
}
