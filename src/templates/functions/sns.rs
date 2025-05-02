use maud::{html, Markup};
use crate::die_linky::SocialLinkType;

pub fn sns_icon(link: &str) -> Markup {
    let temp = link.parse::<SocialLinkType>().unwrap();
    let sns_url_icon = temp.to_svg_icon();
    html! {
        a .social-icon href=(link) {
            img alt=(link) src=(sns_url_icon);
        }
    }
}

pub fn jinja_sns_icon(link: &str) -> String {
    sns_icon(link).into_string()
}
