use serde::{Deserialize, Serialize};
use toml::value::Date;

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct WorkMeta {
    pub title: String,
    pub author: String,
    pub date: Date,
    pub short: Option<String>,
    pub link: String,
    pub featured: bool,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct IntermediaryDisplayWorkMeta {
    pub title: String,
    pub description: Option<String>,
    pub on_site_link: String,
    pub author_displayname: String,
    pub author_link: String,
    pub embed_html: String,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct DisplayWorkMeta {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub on_site_link: String,
    pub author_displayname: String,
    pub author_link: String,
    pub embed_html: String,
}
