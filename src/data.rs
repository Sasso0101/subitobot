pub mod data {

    use chrono::{DateTime, Utc};
    use serde::{Serialize, Deserialize};

    use super::date_format;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Check {
        pub newads: bool,
        pub count: u8
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SearchResults {
        pub ads: Vec<Listing>
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Listing {
        pub urn: String,
        pub subject: String,
        pub body: String,
        pub r#type: Type,
        pub dates: Dates,
        pub images: Vec<Image>,
        pub advertiser: Advertiser,
        pub geo: Geo,
        pub features: Vec<Feature>,
        pub urls: Urls
    }
    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Type {
        pub key: String,
    }
    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Dates {
        #[serde(with = "date_format")]
        pub display: DateTime<Utc>,
        #[serde(with = "date_format")]
        expiration: DateTime<Utc>,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Image {
        pub cdn_base_url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Feature {
        pub uri: String,
        pub values: Vec<Value>,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Value {
        pub value: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Advertiser {
        name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Geo {
        pub city: City,
        pub town: Town,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Town {
        pub value: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct City {
        value: String,
        pub short_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Urls {
        pub default: String,
    }
}

pub mod date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}