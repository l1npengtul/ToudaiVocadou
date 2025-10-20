use serde::{Deserialize, Serialize};
use toml::value::Date;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AlbumMeta {
    pub title: String,
    pub subtitle: Option<String>,
    pub release_date: Date,
    pub short: String,
    pub album_type: AlbumType,
    //
    pub contributors: Vec<String>,
    pub extra_contributors: Vec<String>,
    pub illustrators: Vec<String>,
    pub extra_illustrators: Vec<String>,
    pub engineers: Vec<String>,
    pub extra_engineers: Vec<String>,
    pub track_list: Vec<Track>,

    pub crossfade_demonstration: Option<String>,
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
