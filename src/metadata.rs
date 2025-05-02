use maud::{html, Markup, Render};
use serde::{Deserialize, Serialize};
use crate::templates::partials::navbar::Sections;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub page_title: String,
    pub page_image: Option<String>,
    pub canonical_link: String,
    pub section: Sections,

    pub author: Option<String>,
    pub date: Option<String>,
}

impl Render for Metadata {
    fn render(&self) -> Markup {
        let page_type = match self.section {
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
                    meta property="og:article:author" content=[&self.author];
                    meta property="og:article:published_time" content=[&self.date];
                }
            }
            "profile" => {
                html! { meta property="og:profile:username" content=[&self.author]; }
            }
            _ => html! { },
        };


        html! {
            title { (&self.page_title) }
            meta property="og:title" content=(&self.page_title);
            meta property="og:url" content=(&self.canonical_link);
            meta property="og:type" content=(page_type);
            meta property="og:site_name" content="東京大学ボカロP同好会 - University of Tokyo Vocaloid Producer Club"; // production -> producer - ありがとーnekojitalter
            meta property="og:locale" content="ja_JP";
            @if let Some(img) = &self.page_image {
                meta property="og:image" content=(img);
            }
            (others)
        }
    }
}
