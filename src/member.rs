use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::die_linky::SocialLinkType;
use crate::featured_work::FeatureUpload;

#[derive(Clone, Debug, Serialize ,Deserialize)]
pub struct MemberMeta {
    pub name: String, // 活動名
    pub ascii_name: String, // 英語のみあり活動名（活動名発音方法） - 注意: これからアイコンのファイルを探します

    pub department: Option<String>, // 学部
    pub position: Option<String>, // 役職
    pub entry_year: Option<i32>, // 入年
    pub short: String, // 自己紹介（短い）

    pub link: HashSet<SocialLinkType>, // SNSリンク

    pub featured_works: HashSet<FeatureUpload>,
}
