use crate::post::PostMeta;
use crate::work::WorkMeta;
use crate::{album::AlbumMeta, member::MemberMeta};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteMap {
    pub members: Vec<MemberMeta>,
    pub official_posts: Vec<PostMeta>,
    pub posts: Vec<PostMeta>,
    pub works: Vec<WorkMeta>,
    pub featured_works: Vec<WorkMeta>,
    pub albums: Vec<AlbumMeta>,
}
