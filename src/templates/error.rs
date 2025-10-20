use crate::Data;
use crate::metadata::Metadata;
use crate::templates::base::base;
use crate::templates::partials::navbar::Sections;
use hauchiwa::Sack;
use maud::{Markup, html};

pub fn notfound(sack: &Sack<Data>) -> Markup {
    let inner = html! {
        section #hero {
            h2 { "このページは見つかりませんでした。" }
        }

        section #content {
            .container {
                a href="/index.html" .back-button {
                    "メインページに一覧に戻る"
                }
            }
        }
    };

    let meta = Metadata {
        page_title: "404 - このページを見つかりませんでした。".to_string(),
        page_image: None,
        canonical_link: "/404.html".to_string(),
        section: Sections::Home,
        description: None,
        author: None,
        date: None,
    };

    base(sack, &meta, Some(&[]), inner)
}
