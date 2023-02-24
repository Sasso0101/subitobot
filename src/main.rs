use std::{collections::HashMap, fs::File};

use crate::{subito::subito::{get_all_new_listings, get_listings, update_last_listings}, notifications::notifications::{create_messages, send_messages}};

mod subito;
mod notifications;

#[tokio::main]
async fn main() {
    // If file with last known listings does not exist generate one with current latest listing for every keyword
    if File::open("lastUpdated.txt").is_err() {
        println!("Creating lastUpdated.txt...");
        let mut keywords = HashMap::from([
            (String::from("commodore"), String::new()),
            (String::from("atari"), String::new()),
            (String::from("spectrum"), String::new()),
            (String::from("amstrad"), String::new()),
        ]);
        for (keyword, value) in &mut keywords {
            match get_listings(&keyword.as_str(), 1).await.first() {
                Some(l) => { *value = l.urn.to_owned(); }
                // Listing from 24/02/2023 (any listing published after this date will match)
                None => { *value = String::from("id:ad:487907096:list:442795607"); }
            }
        }
        update_last_listings(keywords);
    }

    let listings = get_all_new_listings().await;
    println!("{:#?}", listings);
    let messages = create_messages(listings);
    send_messages(messages).await;
}