pub mod subito {
    use crate::{config::{Config, Search, set_config}, data::data::{Check, SearchResults, Listing}};
    use itertools::Itertools;

    const CHECK_URL:&str = "https://www.subito.it/hades/v1/search/items/check";
    const ITEMS_URL:&str = "https://www.subito.it/hades/v1/search/items";

    async fn get_last_listing(search: &Search) -> Option<Listing> {
        let mut listings = fetch_search_results(search, 1).await;
        if listings.ads.is_empty() {
            None
        } else {
            Some(std::mem::take(&mut listings.ads[0]))
        }
    }

    async fn get_new_listings(search: &mut Search) -> Vec<Listing> {
        // Initialization of last listing
        if search.last_listing.is_none() {
            search.last_listing = match get_last_listing(search).await {
                Some(l) => { Some(l.urn) },
                None => { Some(String::new()) }
            };
            return Vec::new()
        }
        let limit = get_new_listings_count(search).await;
        match limit {
            Some(n) => {
                if n != 0 { fetch_search_results(search, n).await.ads }
                else { Vec::new() }
            },
            None => {
                fetch_search_results(search, 30).await.ads
            }
        }
    }

    async fn fetch_search_results(search: &Search, limit: u8) -> SearchResults {
        let listings = reqwest::get(
            format!("{}?{}&lim={}&order=datedesc&start=0&t=s", 
            ITEMS_URL,
            format_to_query(search),
            limit)
        ).await.unwrap().text().await.unwrap();
        serde_json::from_str(&listings).unwrap()
    }

    fn format_to_query(search: &Search) -> String {
        let mut query = format!("q={}&t=s", search.keyword);
        if let Some(region) = search.region.as_ref() {
            query = format!("{}&r={}", query, region.iter().format(","));
        }
        if let Some(province) = search.province {
            query = format!("{}&ci={}", query, province);
        }
        if let Some(city) = search.city.as_ref() {
            query = format!("{}&to={}", query, city);
        }
        if let Some(category) = search.category {
            query = format!("{}&c={}", query, category);
        }
        if let Some(search_only_title) = search.search_only_title {
            query = format!("{}&qso={}", query,search_only_title);
        }
        query
    }

    async fn get_new_listings_count(search: &Search) -> Option<u8> {
        match search.last_listing.as_ref() {
            Some(last_listing) => {
                let result = reqwest::get(
                    format!(
                        "{}?{}&last={}", 
                        CHECK_URL, 
                        format_to_query(search), 
                        last_listing
                    )
                ).await.unwrap().text().await.unwrap();
                let result: Check = serde_json::from_str(&result).unwrap();
                Some(result.count)
            },
            None => {
                None
            }
        }
    }
    
    pub async fn get_all_new_listings(config: &mut Config) -> Vec<Listing> {
        let mut listings:Vec<Listing> = vec![];
        for search in &mut config.item {
            let mut l = get_new_listings(search).await;
            if let Some(l) = l.first() {
                search.last_listing = Some(l.urn.clone());
            }
            listings.append(&mut l);
        }
        println!(
            "Found {} new listings\n{}", 
            listings.len(), 
            listings.iter().fold(
                "".to_string(), 
                |acc, listing| {
                    format!("{}\nurl: {}, subject: {} ", acc, listing.urls.default, listing.subject)
                }
            )
        );

        // Update config file with latest listing
        set_config(config);
        listings
    }
}