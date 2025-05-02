use maud::{html, Markup};
use crate::templates::functions::sns::sns_icon;

pub fn footer() -> Markup {
    html! {
        .container {
            p {
                "&copy; 2025 東京大学ボカロP同好会"
            }
            .social-links {
                (sns_icon("https://x.com/toudaivocadou"))
            }
        }
    }
}
