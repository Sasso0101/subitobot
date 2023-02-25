mod data;

pub mod notifications {
    use std::env;

    use crate::subito::data::data::Listing;
    use crate::notifications::data::data::Message;
    
    async fn send_message(m: Message) {
        let url = format!("https://api.telegram.org/{}/sendMessage", env::var("BOT_TOKEN").unwrap());
        let client = reqwest::Client::new();
        let text = serde_json::to_string(&m).unwrap();
        client.post(url)
        .header("Content-Type", "application/json")
        .body(text)
        .send()
        .await.unwrap();
    }

    pub async fn send_messages(messages: Vec<Message>) {
        for message in messages {
            send_message(message).await;
        }
    }

    pub fn create_messages(listings: Vec<Listing>) -> Vec<Message> {
        let mut messages = Vec::new();
        for listing in listings {
            let price;
            match listing.features.iter().find(|&l| l.uri == "/price") {
                None => price = String::from("non specificato"),
                Some(l) => price = l.values.first().unwrap().value.to_owned(),
            }
            messages.push(Message {
                chat_id: env::var("CHAT_ID").unwrap(),
                text: format!(
                    "<a href=\"{}?rule=images-auto\">📣</a> <b>{}</b>\n📍 {} ({}) 🕓 {}\n💵 Prezzo: {}\n\n{}\n\n<a href=\"{}\">➡️ Vedi l\'inserzione ⬅️</a>",
                    listing.images.first().unwrap().cdn_base_url,
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
}