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
