pub mod config {
    use std::{collections::HashMap, fs::{File, self}, io::Write, env};

    use crate::subito::subito::get_listings;

    async fn import_from_env() -> HashMap<String, String> {
        let keywords = env::var("KEYWORDS").unwrap();
        let mut listings = HashMap::new();
        for keyword in keywords.split(",") {
            match get_listings(keyword, 1).await.first() {
                Some(l) => { listings.insert(keyword.to_string(), l.urn.to_owned()); }
                // Use as default a listing from 24/02/2023 (any listing published after this date will be notified)
                None => { listings.insert(keyword.to_string(), String::from("id:ad:487907096:list:442795607")); }
            }
        }
        listings
    }

    pub fn update_last_listings(all_last_listings: &HashMap<String, String>) {
        let mut file = File::create("data/last_updated.txt").unwrap();
        let mut text = String::new();
        for (keyword, last_listing) in all_last_listings {
            text = format!("{}{} {}\n", text.as_str(), keyword, last_listing);
        }
        file.write_all(text.as_bytes()).unwrap();
    }

    pub async fn get_all_last_listings() -> HashMap<String, String> {
        let mut listings = import_from_env().await;
        match fs::read_to_string("data/last_updated.txt") {
            Ok(f) => {
                for line in f.lines() {
                    let mut line = line.split(" ").map(|f| String::from(f));
                    // If keyword in file matches one present in env set correct last listing
                    match listings.get_mut(&line.next().unwrap()) {
                        Some(l) => {*l = line.next().unwrap()}
                        None => {}
                    }
                }
            }
            Err(_) => {}
        }
        update_last_listings(&listings);
        listings
    }
}