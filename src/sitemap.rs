use crate::member::MemberMeta;
use crate::post::PostMeta;
use crate::work::WorkMeta;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteMap {
    pub members: Vec<MemberMeta>,
    pub posts: Vec<(PostMeta, String)>,
    pub works: Vec<WorkMeta>,
}
