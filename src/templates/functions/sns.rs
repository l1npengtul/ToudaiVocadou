use crate::die_linky::SocialLinkType;
use maud::{Markup, html};

pub fn sns_icon(link: &str) -> Markup {
    let temp = link.parse::<SocialLinkType>().unwrap();
    let sns_url_icon = temp.to_svg_icon();
    let special_style = match temp {
        // horrible, horrible hack but we roll with it ig
        SocialLinkType::Bluesky => "width: 100%;",
        _ => "",
    };
    html! {
        a .social-icon .sns-icon-size href=(link) {
            img alt=(link) src=(format!("/icon/social_icons/{}", sns_url_icon)) style=(special_style);
        }
    }
}

pub fn jinja_sns_icon(link: &str) -> String {
    sns_icon(link).into_string()
}
