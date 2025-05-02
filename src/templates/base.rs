use maud::{html, Markup, Render, DOCTYPE};
use crate::metadata::Metadata;
use crate::templates::partials::footer::footer;
use crate::templates::partials::head::html_head;
use crate::templates::partials::navbar::navbar;

pub fn base<'a, Meta>(header_metadata: &'a Meta, inner: impl Render) -> Markup where
    &'a Meta: Into<&'a Metadata>,
{
    let metadata = Into::into(header_metadata);

    html! {
        (DOCTYPE)
        html lang="ja" {
            (html_head(metadata))
            body {
                (navbar(metadata.section))
                (inner)
                (footer())
            }
        }
    }
}