use serde::{Deserialize, Serialize};
use toml::value::Date;

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct WorkMeta {
    pub title: String,
    pub author: String,
    pub date: Date,
    pub short: Option<String>,
    pub link: String,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct DisplayWorkMeta {
    pub title: String,
    pub description: Option<String>,
    pub on_site_link: String,
    pub author_displayname: String,
    pub author_link: String,
    pub embed_html: String,
}
