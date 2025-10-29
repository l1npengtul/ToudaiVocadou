use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use toml::value::Date;

use crate::{metadata::Metadata, templates::partials::navbar::Sections};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlbumMeta {
    pub title: String,
    pub subtitle: Option<String>,
    pub release_date: Date,
    pub short: String,
    pub album_type: AlbumType,
    //
    pub contributors: Vec<String>,
    pub extra_contributors: Vec<String>,

    pub crossfade_demonstration: Option<String>,

    pub front_cover: String,
    pub other_covers: HashMap<String, String>,
}

impl AlbumMeta {
    pub fn contributors_str(&self) -> String {
        let mut all_contributors = HashSet::new();
        all_contributors.extend(&self.contributors);
        all_contributors.extend(&self.extra_contributors);

        all_contributors
            .into_iter()
            .map(String::as_str)
            .collect::<Vec<&str>>()
            .join(", ")
    }
}

impl From<AlbumMeta> for Metadata {
    fn from(value: AlbumMeta) -> Self {
        let authors = value.contributors_str();
        Metadata {
            canonical_link: format!("/works/albums/{}", &value.title),
            page_title: value.title,
            page_image: Some(value.front_cover),
            section: Sections::Works,
            description: Some(value.short),
            author: Some(authors),
            date: Some(value.release_date.to_string()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum AlbumType {
    Solo,
    GroupExternal,
    ToudaiVocadou,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Track {
    pub number: i32,
    pub data: TrackData,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TrackData {
    Registered {
        link: String,
    },
    Unregistered {
        title: String,
        author: String,
        link: Option<String>,
    },
}
