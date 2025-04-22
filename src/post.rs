use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub category: Category,
    pub author: String,
    pub header_image: Option<String>,
    pub date: String,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Category {
    News,
    Blog
}