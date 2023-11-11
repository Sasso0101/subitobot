use crate::data::data::{Listing, Image};

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    chat_id: String,
    text: String,
    parse_mode: String,
    disable_web_page_preview: bool
}

async fn send_message(m: &Message, bot_token: &str) {
    let url = format!(
        "https://api.telegram.org/{}/sendMessage",
        bot_token
    );
    let client = reqwest::Client::new();
    let text = serde_json::to_string(&m).unwrap();
    client
        .post(url)
        .header("Content-Type", "application/json")
        .body(text)
        .send()
        .await
        .unwrap();
}

pub async fn send_messages(messages: &Vec<Message>, bot_token: &str) {
    for message in messages.iter().rev() {
        send_message(message, bot_token).await;
    }
}

pub fn create_messages(listings: &Vec<Listing>, chat_id: &str) -> Vec<Message> {
    let mut messages = Vec::new();
    for listing in listings {
        let price;
        match listing.features.iter().find(|&l| l.uri == "/price") {
            None => price = String::from("non specificato"),
            Some(l) => price = l.values.first().unwrap().value.to_owned(),
        }
        messages.push(Message {
                chat_id: chat_id.to_string(),
                text: format!(
                    "<a href=\"{}?rule=images-auto\">📣</a> <b>{}</b>\n📍 {} ({}) 🕓 {}\n💵 Prezzo: {}\n\n{}\n\n<a href=\"{}\">➡️ Vedi l\'inserzione ⬅️</a>",
                    listing.images.first().get_or_insert(&Image {cdn_base_url:String::new()}).cdn_base_url,
                    listing.subject,
                    listing.geo.town.value,
                    listing.geo.city.short_name,
                    listing.dates.display.format("%R %e/%m/%Y"),
                    price,
                    listing.body,
                    listing.urls.default,
                ),
                parse_mode: String::from("HTML"),
                disable_web_page_preview: false
            });
    }
    messages
}
