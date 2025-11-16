use maud::{Markup, html};

use crate::lnk;

pub fn member(to_link: &str) -> Markup {
    html! {
        a href=(lnk(format!("members/{to_link}")))  {
            (to_link)
        }
    }
}

pub fn jinja_member(to_link: &str) -> String {
    member(to_link).into_string()
}
