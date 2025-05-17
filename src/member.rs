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
    pub __do_not_use_kuwasiku: bool,
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
        let page_title = if value.name == value.ascii_name {
            value.name.clone()
        } else {
            format!("{}({})", value.name, value.ascii_name)
        };
        
        Metadata {
            page_title: format!("{page_title} - 東京大学ボカロP同好会"),
            page_image: Some(format!("/icon/{}.jpg", value.ascii_name)),
            canonical_link: format!("members/{}.html", value.ascii_name),
            section: Sections::MemberProfile,
            description: Some(value.short),
            author: Some(value.name),
            date: None,
        }
    }
}
