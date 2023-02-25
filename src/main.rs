use crate::{
    notifications::notifications::{create_messages, send_messages},
    subito::subito::{get_all_new_listings},
};

mod notifications;
mod subito;
mod config;

#[tokio::main]
async fn main() {
    let listings = get_all_new_listings().await;
    println!("{:#?}", listings);
    let messages = create_messages(listings);
    send_messages(messages).await;
}
