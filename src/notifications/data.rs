pub mod data {
    use serde::{Serialize, Deserialize};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message {
        pub chat_id: String,
        pub text: String,
        pub parse_mode: String,
        pub disable_web_page_preview: bool
    }
}