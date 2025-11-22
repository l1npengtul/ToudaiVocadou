use crate::news::NewsMeta;
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
                @for post_meta in &site_map.news {
                    (post_card(sack, post_meta, name_map)?)
                }
                @if site_map.news.is_empty() {
                    p .work-description style="text-align: center;" {
                        em {
                            "ニュースがありません。"
                        }
                    }
                }
            }
        }
    };

    let metadata = Metadata {
        page_title: "ニュース".to_string(),
        page_image: None,
        canonical_link: "/news.html".to_string(),
        section: Sections::News,
        description: Some("東京大学ボカロP同好会のニュース".to_string()),
        author: None,
        date: None,
    };

    base(sack, &metadata, Some(&[]), inner)
}

pub fn post_card(
    context: &Context<SiteData>,
    post_meta: &NewsMeta,
    name_map: &HashMap<String, String>,
) -> Result<Markup, RuntimeError> {
    let author_name = name_map.get(&post_meta.author).ok_or(RuntimeError::msg("Could not find author. Does the member page exist? Did you remember to type in the ascii name? Did you mistype it? Yell at peg for more info".to_string()))?;

    Ok(html! {
        .item-card {
            .item-image {
                img class="img-placeholder" href=(post_thumbnail(context, post_meta)?) {}
            }
            .item-title {
                h3 {
                    a href=(format!("/news/{}.html", post_reference(post_meta))) {
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
                    (post_meta.short)
                }
            }
        }
    })
}

pub fn post_detail(
    sack: &Context<SiteData>,
    post_meta: &NewsMeta,
    content: &str,
    name_map: &HashMap<String, String>,
) -> Result<Markup, RuntimeError> {
    let author_name = name_map.get(&post_meta.author).ok_or(RuntimeError::msg("Could not find author. Does the member page exist? Did you remember to type in the ascii name? Did you mistype it? Yell at peg for more info".to_string()))?;

    let inner = html! {
        section #post-detail {
            .member-detail-container {
                h2 { (post_meta.title) }
                p { (post_meta.date) }
                a href=(format!("/members/{}.html", post_meta.author)) { p { (author_name) } }
                .member-profile {
                    .member-profile-image {
                        img href=(post_thumbnail(sack, post_meta)?) alt="header image" { }
                    }

                }
            }
        }

        section #post-content .container{
            .description {
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
        page_image: Some(post_thumbnail(sack, post_meta)?),
        canonical_link: format!("/news/{}.html", post_reference(post_meta)),
        section: Sections::NewsPost,
        description: Some(shorten(content)),
        author: Some(post_meta.author.clone()),
        date: Some(post_meta.date.to_string()),
    };

    base(sack, &metadata, Some(&[]), inner)
}

pub fn post_thumbnail(_sack: &Context<SiteData>, item: &NewsMeta) -> Result<String, RuntimeError> {
    match &item.header_image {
        Some(header) => Ok(format!("images/{}", header)),
        None => Ok("images/gray.jpg".to_string()),
    }

    // TODO: Get thumbnail from SNS post.
}

pub fn post_reference(meta: &NewsMeta) -> String {
    let authorhash = seahash::hash(meta.author.as_bytes()) as u128;
    let titlehash = seahash::hash(meta.title.as_bytes()) as u128;
    let combined = (authorhash << 64) + titlehash;
    BASE64_URL_SAFE_NO_PAD.encode(combined.to_le_bytes())
}
