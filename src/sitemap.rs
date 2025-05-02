use serde::{Deserialize, Serialize};
use crate::member::MemberMeta;
use crate::post::PostMeta;
use crate::work::WorkMeta;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteMap {
    pub members: Vec<MemberMeta>,
    pub posts: Vec<PostMeta>,
    pub works: Vec<WorkMeta>,
}
