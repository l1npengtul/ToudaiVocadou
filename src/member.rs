use crate::RENDER_SITE;
use crate::metadata::Metadata;
use crate::templates::partials::navbar::Sections;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemberFeaturedWork {
    pub title: String,
    pub description: Option<String>,
    pub link: String,
    #[allow(non_snake_case)]
    #[serde(skip_serializing, default)]
    pub __DO_NOT_USE_kuwasiku: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemberMeta {
    pub name: String,       // 活動名
    pub ascii_name: String, // 英語のみあり活動名（活動名発音方法） - 注意: これからアイコンのファイルを探します. ケース・インセンシティブ!!!!!

    pub department: Option<String>, // 学部
    pub position: Option<String>,   // 役職
    pub entry_year: Option<i32>,    // 入年
    pub short: String,              // 自己紹介（短い）

    pub links: HashSet<String>, // SNSリンク

    pub featured_works: Vec<MemberFeaturedWork>,
}

impl From<MemberMeta> for Metadata {
    fn from(value: MemberMeta) -> Self {
        Metadata {
            page_title: format!("{}({}) - 東京大学ボカロP同好会", value.name, &value.short),
            page_image: Some(format!("{}.jpg", value.short)),
            canonical_link: format!("{}/members/{}", RENDER_SITE, value.short),
            section: Sections::MemberProfile,
            author: Some(value.short),
            date: None,
        }
    }
}
