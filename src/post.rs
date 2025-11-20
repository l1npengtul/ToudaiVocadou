use serde::{Deserialize, Serialize};
use toml::value::Date;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub author: String,
    #[serde(default)]
    pub header_image: Option<String>,
    pub date: Date,

    #[serde(default)]
    pub short: String,

    #[serde(default)]
    pub official: bool,

    #[serde(default)]
    pub social_links: Vec<Url>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawPostMeta {
    pub title: String,
    pub author: String,
    #[serde(default)]
    pub header_image: Option<String>,
    pub date: Date,

    #[serde(default)]
    pub short: Option<String>,

    #[serde(default)]
    pub official: bool,

    #[serde(default)]
    pub social_links: Vec<Url>,
}
