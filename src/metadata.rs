use crate::templates::partials::navbar::Sections;
use crate::{Data, image, lnk};
use hauchiwa::Sack;
use maud::{Markup, html};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub page_title: String,
    pub page_image: Option<String>,
    pub canonical_link: String,
    pub section: Sections,
    pub description: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
}

pub fn render_metadata(sack: &Sack<Data>, metadata: &Metadata) -> Markup {
    let page_type = match metadata.section {
        Sections::Home => "website",
        Sections::Members => "website",
        Sections::MemberProfile => "profile",
        Sections::Activities => "website",
        Sections::Join => "website",
        Sections::News => "website",
        Sections::NewsPost => "article",
        Sections::Works => "website",
        Sections::WorksPost => "article",
    };

    let others = match page_type {
        "article" => {
            html! {
                meta property="og:article:author" content=[&metadata.author];
                meta property="og:article:published_time" content=[&metadata.date];
            }
        }
        "profile" => {
            html! { meta property="og:profile:username" content=[&metadata.author]; }
        }
        _ => html! {},
    };

    let canonical_link = lnk(&metadata.canonical_link);

    html! {
        title { (&metadata.page_title) }
        meta property="og:title" content=(&metadata.page_title);
        meta property="og:url" content=(canonical_link);
        meta property="og:type" content=(page_type);
        meta property="og:site_name" content="東京大学ボカロP同好会 - University of Tokyo Vocaloid Producer Club"; // production -> producer - ありがとーnekojitalter
        meta property="og:locale" content="ja_JP";
        @if let Some(img) = &metadata.page_image {
            meta property="og:image" content=(image(sack, img));
        }
        @if let Some(desc) = &metadata.description {
            meta property="og:description" content=(desc);
        }
        (others)
    }
}
