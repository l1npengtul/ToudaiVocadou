use crate::{SiteData, templates::functions::sns::sns_icon};
use hauchiwa::Context;
use maud::{Markup, html};

pub fn footer(context: &Context<SiteData>) -> Markup {
    html! {
        footer {
            .container {
                p {
                    "© 2025 東京大学ボカロP同好会"
                }
                .social-links .sns-footer {
                    (sns_icon(context, "https://x.com/toudaivocadou"))
                }
            }
        }
    }
}
