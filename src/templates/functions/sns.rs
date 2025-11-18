use crate::{die_linky::SocialLinkType, lnk};
use hauchiwa::RuntimeError;
use maud::{Markup, html};
use minijinja::Error as JinjaError;
use minijinja::ErrorKind;

pub fn sns_icon(link: &str) -> Result<Markup, RuntimeError> {
    let temp = link.parse::<SocialLinkType>().unwrap();
    let sns_url_icon = temp.to_svg_icon();
    let special_style = match temp {
        // horrible, horrible hack but we roll with it ig
        SocialLinkType::Bluesky => "width: 100%;",
        _ => "",
    };
    Ok(html! {
        a .social-icon .sns-icon-size href=(link) {
            img alt=(link) src=(lnk(format!("assets/social_icons/{}", sns_url_icon))) style=(special_style);
        }
    })
}

pub fn jinja_sns_icon(link: &str) -> Result<String, JinjaError> {
    Ok(sns_icon(link)
        .map_err(|why| JinjaError::new(ErrorKind::InvalidOperation, why.to_string()))?
        .into_string())
}
