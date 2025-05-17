use crate::Data;
use crate::metadata::Metadata;
use crate::templates::partials::footer::footer;
use crate::templates::partials::head::html_head;
use crate::templates::partials::navbar::navbar;
use hauchiwa::Sack;
use maud::{DOCTYPE, Markup, Render, html};

pub fn base<'a, Meta>(sack: &Sack<Data>, header_metadata: &'a Meta, inner: impl Render) -> Markup
where
    &'a Meta: Into<&'a Metadata>,
{
    let metadata = Into::into(header_metadata);

    html! {
        (DOCTYPE)
        html lang="ja" {
            (html_head(sack, metadata))
            body {
                (navbar(metadata.section))
                .main-content-container {
                    (inner)
                }
                (footer())
            }
        }
    }
}
