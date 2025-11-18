use crate::lnk;
use crate::post::PostMeta;
use crate::sitemap::SiteMap;
use crate::templates::base::base;
use crate::templates::partials::navbar::Sections;
use crate::util::shorten;
use crate::{SiteData, metadata::Metadata};
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use hauchiwa::Context;
use hauchiwa::RuntimeError;
use maud::{Markup, PreEscaped, html};
use std::collections::HashMap;

pub fn news_posts(
    sack: &Context<SiteData>,
    site_map: &SiteMap,
    name_map: &HashMap<String, String>,
) -> Result<Markup, RuntimeError> {
    // TODO: pagination. this will get long! yell at peng if we get >100!

    let inner = html! {
        section #hero {
            .container {
                h2 { "ニュース" }
                p { "東京大学ボカロP同好会のニュース目録です。" }
            }
        }

        section #list {
            .listcontainer {
                h3 {
                    "公式ポスト"
                }
                @for post_meta in &site_map.official_posts {
                    (post_card(post_meta, name_map)?)
                }
            }

            .listcontainer {
                h3 {
                    "メンバーポスト"
                }
                @for post_meta in &site_map.posts {
                    (post_card(post_meta, name_map)?)
                }
            }
        }
    };

    let metadata = Metadata {
        page_title: "ニュース".to_string(),
        page_image: None,
        canonical_link: lnk("/news.html"),
        section: Sections::News,
        description: Some("東京大学ボカロP同好会のニュース".to_string()),
        author: None,
        date: None,
    };

    base(sack, &metadata, Some(&[]), inner)
}

pub fn post_card(
    post_meta: &PostMeta,
    name_map: &HashMap<String, String>,
) -> Result<Markup, RuntimeError> {
    let image = match &post_meta.header_image {
        None => "images/gray.jpg".to_string(),
        Some(i) => i.clone(),
    };

    let author_name = name_map.get(&post_meta.author).ok_or(RuntimeError::msg("Could not find author. Does the member page exist? Did you remember to type in the ascii name? Did you mistype it? Yell at peg for more info".to_string()))?;

    let post_short = post_meta.short.as_ref();

    Ok(html! {
        .item-card {
            .item-image {
                img class="img-placeholder" href=(image) {}
            }
            .item-title {
                h3 {
                    a href=(lnk(format!("/news/{}.html", post_reference(post_meta)))) {
                        (post_meta.title)
                    }
                }
                p .member-role {
                    (post_meta.date)
                }
                p .member-department {
                    (author_name)
                }
                p {
                    @if post_meta.official {
                        "⭐ 公式"
                    }
                }
                p {
                    @if let Some(short) = post_short {
                        (short)
                    } @else {
                        em {
                            "筋が提供されませんでした。"
                        }
                    }
                }
            }
        }
    })
}

pub fn post_detail(
    sack: &Context<SiteData>,
    post_meta: &PostMeta,
    content: &str,
    name_map: &HashMap<String, String>,
) -> Result<Markup, RuntimeError> {
    let author_name = name_map.get(&post_meta.author).ok_or(RuntimeError::msg("Could not find author. Does the member page exist? Did you remember to type in the ascii name? Did you mistype it? Yell at peg for more info".to_string()))?;

    let inner = html! {
        section #post-detail {
            .member-detail-container {
                h2 { (post_meta.title) }
                p { (post_meta.date) }
                @if post_meta.official {
                    p {
                        "⭐: 東大ボカロP同好会の公式ポスト"
                    }
                }
                a href=(lnk(format!("/members/{}.html", post_meta.author))) { p { (author_name) } }
                .member-profile {
                    @if let Some(image) = &post_meta.header_image {
                        .member-profile-image {
                            img href=(image) alt="header image" { }
                        }
                    }
                }
            }
        }

        section .container{
            .about-content {
                (PreEscaped(content))
            }
        }

        .back-button{
            a href="../news.html" {
                "ニュース目録一覧に戻る"
            }
        }
    };

    let metadata = Metadata {
        page_title: post_meta.title.clone(),
        page_image: post_meta.header_image.clone(),
        canonical_link: format!("news/{}.html", post_reference(post_meta)),
        section: Sections::NewsPost,
        description: Some(shorten(content)),
        author: Some(post_meta.author.clone()),
        date: Some(post_meta.date.to_string()),
    };

    base(sack, &metadata, Some(&[]), inner)
}

pub fn post_reference(meta: &PostMeta) -> String {
    let authorhash = seahash::hash(meta.author.as_bytes()) as u128;
    let titlehash = seahash::hash(meta.title.as_bytes()) as u128;
    let combined = (authorhash << 64) + titlehash;
    BASE64_URL_SAFE_NO_PAD.encode(combined.to_le_bytes())
}
