pub mod data;

pub mod subito {
    use std::{collections::HashMap, fs::{self, File}, io::Write};

    use super::data::data::{Search, Listing, Check};

    pub async fn get_listings(keyword: &str, lim: i32) -> Vec<Listing> {
        const URL:&str = "https://www.subito.it/hades/v1/search/items";
        const PARAMS:&str = "&order=datedesc&start=0&r=7&t=s";
        let search = reqwest::get(format!("{}?q={}&lim={}{}", URL, keyword, lim, PARAMS)).await.unwrap().text().await.unwrap();
        let search: Search = serde_json::from_str(&search).unwrap();
        search.ads
    }

    async fn check_new_listings(keyword: &str, last_listing: &str) -> i32 {
        const URL:&str = "https://www.subito.it/hades/v1/search/items/check";
        const PARAMS:&str = "&r=7&t=s";
        let result = reqwest::get(format!("{}?q={}&last={}{}", URL, keyword, last_listing, PARAMS)).await.unwrap().text().await.unwrap();
        let result: Check = serde_json::from_str(&result).unwrap();
        match result.newads {
            // For some reason the subito.it api multiplies the count by 3 and starts from 0
            true => (result.count+1)/3,
            false => 0
        }
    }
    
    fn get_all_last_listings() -> HashMap<String, String> {
        let file = fs::read_to_string("data/last_updated.txt");
        let mut listings = HashMap::new();
        for line in file.unwrap().lines() {
            let mut line = line.split(" ").map(|f| String::from(f));
            listings.insert(line.next().unwrap(), line.next().unwrap());
        }
        listings
    }

    pub fn update_last_listings(all_last_listings: HashMap<String, String>) {
        let mut file = File::create("data/last_updated.txt").unwrap();
        let mut text = String::new();
        for (keyword, last_listing) in all_last_listings {
            text = format!("{}{} {}\n", text.as_str(), keyword, last_listing);
        }
        file.write_all(text.as_bytes()).unwrap();
    }

    pub async fn get_all_new_listings() -> Vec<Listing> {
        let mut all_new_listings = Vec::new();
        let mut all_last_listings = get_all_last_listings();
        for (keyword, last_listing) in &mut all_last_listings {
            let new_listings = check_new_listings(&keyword, &last_listing).await;
            if new_listings == 0 {
                continue;
            } else {
                let new_listings = &mut get_listings(&keyword, new_listings).await;
                *last_listing = new_listings.first().unwrap().urn.to_owned();
                all_new_listings.append(new_listings);
            }
        }
        update_last_listings(all_last_listings);
        all_new_listings
    }
}