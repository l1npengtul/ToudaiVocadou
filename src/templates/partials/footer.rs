use crate::templates::functions::sns::sns_icon;
use hauchiwa::RuntimeError;
use maud::{Markup, html};

pub fn footer() -> Result<Markup, RuntimeError> {
    Ok(html! {
        footer {
            .container {
                p {
                    "© 2025 東京大学ボカロP同好会"
                }
                .social-links .sns-footer {
                    (sns_icon("https://x.com/toudaivocadou")?)
                }
            }
        }
    })
}
