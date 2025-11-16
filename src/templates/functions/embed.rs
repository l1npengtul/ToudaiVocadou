use crate::{die_linky::SocialLinkType, lnk_s3};
use anyhow::Error;
use maud::{Render, html};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;
use urlencoding::encode;

pub fn embed(link: &str) -> Result<impl Render, Error> {
    println!("processing: {}", link);

    if link.ends_with(".png")
        || link.ends_with(".jpeg")
        || link.ends_with(".jpg")
        || link.ends_with(".gif")
    {
        return Ok(html! {
            img href=(lnk_s3(link)) {}
        });
    }

    if link.ends_with(".mp3") || link.ends_with(".ogg") || link.ends_with(".wav") {
        return Ok(html! {
            figure {
                audio controls src=(lnk_s3(link));
                a href=(lnk_s3(link)) {
                    "ファイルをダウンロードする"
                }
            }
        });
    }

    let url_type = SocialLinkType::from_str(link).unwrap();
    let url_parse = Url::parse(link).unwrap();

    println!("link type: {:?}", url_type);

    match url_type {
        SocialLinkType::Twitter | SocialLinkType::Xitter => Ok(html! {
            blockquote .twitter-tweet {
                script async src="https://platform.twitter.com/widgets.js" charset="utf-8";
                a href=(link);
            }
        }),
        SocialLinkType::Bluesky => {
            let link_encoded = encode(link);
            let bluesky_oembed = reqwest::blocking::get(format!(
                "https://embed.bsky.app/oembed?url={}",
                link_encoded
            ))?;

            if bluesky_oembed.status() != StatusCode::OK {
                panic!("failed to get bluesky oembed - try building again or fixing this url!")
            }

            let embed_html = bluesky_oembed.json::<OEmbed>()?;

            if let Some(html) = embed_html.html {
                return Ok(html! { (html) });
            } else if let Some(image) = embed_html.url {
                return Ok(html! {
                    a href=(link) {
                        img src=(image) alt=(link);
                    }
                });
            }

            panic!("returned oembed did not match any known items.")
        }
        SocialLinkType::Youtube => {
            let youtube_video_id = url_parse
                .query_pairs()
                .find(|(key, _)| key == "v")
                .ok_or(Error::msg("invalid youtube link"))?
                .1;
            let embed_link = format!("https://www.youtube.com/embed/{youtube_video_id}");

            Ok(html! {
                .youtube-embed-container {
                    iframe src=(embed_link) title="Youtube Video Player" height="360" width="640" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen style="width: 100%;"{}
                }
            })
        }
        SocialLinkType::NicoDouga => {
            let nnd_video_id = url_parse
                .path_segments()
                .ok_or(Error::msg("invalid nnd link"))?
                .find(|segment| segment.starts_with("sm"))
                .ok_or(Error::msg("no id in nnd link"))?;
            let nnd_video_link = format!("https://embed.nicovideo.jp/watch/{nnd_video_id}");
            Ok(html! {
                .youtube-embed-container {
                    iframe src=(nnd_video_link) title="Nicovideo Video Player" height="360" width="640" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen {}
                }
            })
        }
        _ => panic!("unsupported embed type."),
    }

    // soundcloud embed
    // twitter embed
    // youtube embed
    // nicovideo embed
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OEmbed {
    pub version: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub provider_name: Option<String>,
    pub provider_url: Option<String>,
    pub html: Option<String>,
}

pub fn jinja_embed(link: &str) -> Result<String, anyhow::Error> {
    Ok(embed(link)?.render().into_string())
}
