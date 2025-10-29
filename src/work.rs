use serde::{Deserialize, Serialize};
use toml::value::Date;

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct WorkMeta {
    pub title: String,
    pub author: String,
    #[serde(default)]
    pub collaborators: Vec<String>,
    pub date: Date,
    #[serde(default)]
    pub short: Option<String>,
    #[serde(default)]
    pub cover_image: Option<String>,
    pub link: String,
    pub remix_original_work: Option<String>, // The link to the original work if it is a remix.
    #[serde(default)]
    pub featured: bool,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct DisplayWorkMeta {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub on_site_link: String,
    pub author_displayname: String,
    pub collaborators: Vec<String>,
    pub remix_original_work: Option<String>,
    pub author_link: String,
    pub embed_html: String,
}
