use maud::{html, Markup};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Sections {
    Home,
    Members,
    MemberProfile,
    Activities,
    Join,
    News,
    NewsPost,
    Works,
    WorksPost,
}

pub fn navbar(current_section: Sections) -> Markup {
    html! {
        header {
            div .container {
                h1 {
                    "東京大学ボカロP同好会"
                }
                nav {
                    ul {
                        (navbar_item("/", current_section == Sections::Home, "ホーム"))
                        (navbar_item("/members", current_section == Sections::Members || current_section == Sections::MemberProfile, "メンバー紹介"))
                        (navbar_item("/#activities", current_section == Sections::Activities, "活動内容"))
                        (navbar_item("/#join", current_section == Sections::Join, "入会案内"))
                        (navbar_item("/news", current_section == Sections::News || current_section == Sections::NewsPost, "ニュース・ブログ"))
                        (navbar_item("/works", current_section == Sections::Works || current_section == Sections::WorksPost, "作品"))
                    }
                }
            }
        }
    }
}

fn navbar_item(link: &str, active: bool, inner: &str) -> Markup {
    html! {
        li {
            a .active[active] href=(link) {
                (inner)
            }
        }
    }
}
