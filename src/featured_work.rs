use crate::die_linky::SocialLink;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeaturedWorkMeta {
    pub title: String,
    pub description: String,
    pub author: String,
    pub additional_publish_links: HashSet<SocialLink>,

    pub video: FeatureUpload,
}

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum FeatureUpload {
    AudioFile(String),
    VideoFile(String),
    YouTube(String),
}
