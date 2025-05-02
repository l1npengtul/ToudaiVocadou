use maud::{html, Markup};
use crate::metadata::Metadata;

pub fn html_head(metadata: &Metadata) -> Markup {
    html! {
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            (metadata)
            link rel="stylesheet" href="css/style.css";
        }
    }
}
