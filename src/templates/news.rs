use crate::Data;
use crate::metadata::Metadata;
use crate::post::PostMeta;
use crate::sitemap::SiteMap;
use crate::templates::base::base;
use crate::templates::partials::navbar::Sections;
use crate::util::shorten;
use base64::Engine;
use base64::prelude::BASE64_STANDARD_NO_PAD;
use hauchiwa::Sack;
use maud::{Markup, PreEscaped, html};
use std::collections::HashMap;

pub fn news_posts(
    sack: &Sack<Data>,
    site_map: &SiteMap,
    name_map: &HashMap<String, String>,
) -> Markup {
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
                @for (post_meta, short) in &site_map.posts {
                    (post_card(post_meta, short, name_map))
                }
            }
        }
    };

    let metadata = Metadata {
        page_title: "作品集合".to_string(),
        page_image: None,
        canonical_link: "/works.html".to_string(),
        section: Sections::News,
        description: Some("東京大学ボカロP同好会のニュース".to_string()),
        author: None,
        date: None,
    };

    base(sack, &metadata, Some(&[]), inner)
}

pub fn post_card(post_meta: &PostMeta, short: &str, name_map: &HashMap<String, String>) -> Markup {
    let image = match &post_meta.header_image {
        None => "/images/gray.jpg".to_string(),
        Some(i) => i.clone(),
    };

    let author_name = name_map.get(&post_meta.author).expect("Could not find author. Does the member page exist? Did you remember to type in the ascii name? Did you mistype it? Yell at peg for more info");

    html! {
        .item-card {
            .item-image {
                img class="img-placeholder" href=(image) {}
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
                    (short)
                }
            }
        }
    }
}

pub fn post_detail(
    sack: &Sack<Data>,
    post_meta: &PostMeta,
    content: &str,
    name_map: &HashMap<String, String>,
) -> Markup {
    let author_name = name_map.get(&post_meta.author).expect("Could not find author. Does the member page exist? Did you remember to type in the ascii name? Did you mistype it? Yell at peg for more info");

    let inner = html! {
        section #post-detail {
            .member-detail-container {
                h2 { (post_meta.title) }
                p { (post_meta.date) }
                a href=(format!("/members/{}.html", post_meta.author)) { p { (author_name) } }
                .member-profile {
                    @if let Some(image) = &post_meta.header_image {
                        .member-profile-image {
                            img href=(image) alt="header" { }
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
    let title_chars = meta.title.chars().take(10).collect::<String>();
    let date_str = meta.date.to_string();
    let hash_base64 = BASE64_STANDARD_NO_PAD
        .encode(seahash::hash((date_str + &meta.title + &meta.author).as_bytes()).to_le_bytes());
    format!("{}-{}", title_chars, hash_base64)
}
