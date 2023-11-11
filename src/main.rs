mod notifications;
mod subito;
mod data;
mod config;

use crate::{config::{Config, get_config}, subito::subito::get_all_new_listings, notifications::{create_messages, send_messages}};

#[tokio::main]
async fn main() {
    let mut config:Config = get_config();
    let listings = get_all_new_listings(&mut config).await;
    let messages = create_messages(&listings, &config.chat_id);
    send_messages(&messages, &config.bot_token).await;
}
