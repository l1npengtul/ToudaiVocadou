use serde::{Deserialize, Serialize};
use toml::value::Date;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub author: String,
    pub header_image: Option<String>,
    pub date: Date,
}
