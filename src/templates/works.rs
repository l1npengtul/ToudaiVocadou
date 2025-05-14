use crate::Data;
use crate::die_linky::SocialLinkType;
use crate::metadata::Metadata;
use crate::sitemap::SiteMap;
use crate::templates::base::base;
use crate::templates::functions::embed::embed;
use crate::templates::partials::navbar::Sections;
use crate::work::WorkMeta;
use base64::Engine;
use base64::prelude::BASE64_STANDARD_NO_PAD;
use hauchiwa::Sack;
use maud::{Markup, PreEscaped, html};
use std::collections::HashMap;
use std::str::FromStr;
use url::Url;

pub fn works(sack: &Sack<Data>, site_map: &SiteMap, name_map: &HashMap<String, String>) -> Markup {
    // TODO: pagination. this will get ungodly long. yell at peng if we get >100!

    let inner = html! {
        section #hero {
            .container {
                h2 { "作品" }
                p { "東京大学ボカロP同好会のメンバーの作品目録です。" }
            }
        }

        section #list {
            .listcontainer {
                @for work in &site_map.works {
                    (work_card(work, name_map))
                }
            }
        }
    };

    let metadata = Metadata {
        page_title: "作品集合".to_string(),
        page_image: None,
        canonical_link: "/works.html".to_string(),
        section: Sections::Works,
        author: None,
        date: None,
    };

    base(sack, &metadata, inner)
}

pub fn work_card(work_meta: &WorkMeta, name_map: &HashMap<String, String>) -> Markup {
    let author_name = name_map.get(&work_meta.author).expect("Could not find author. Does the member page exist? Did you remember to type in the ascii name? Did you mistype it? Yell at peg for more info");

    html! {
        .item-card {
            .item-image {
                img .work-item-thumb src=(get_thumbnail(&work_meta.link)) alt=(work_meta.title) {}
            }
            .item-title {
                h3 {
                    a href=(format!("/works/{}.html", work_reference(&work_meta.link))){
                        (work_meta.title)
                    }
                }
                p .member-role {
                    (work_meta.date)
                }
                p .member-department {
                    (author_name)
                }
                p {
                    (work_meta.short.clone().unwrap_or_default())
                }
            }
        }

    }
}

pub fn work_reference(work_link: &str) -> String {
    BASE64_STANDARD_NO_PAD.encode(seahash::hash(work_link.as_bytes()).to_le_bytes())

}

pub fn work_detail(
    sack: &Sack<Data>,
    work_meta: &WorkMeta,
    name_map: &HashMap<String, String>,
    content: &str,
) -> Markup {
    let author_name = name_map.get(&work_meta.author).expect("Could not find author. Does the member page exist? Did you remember to type in the ascii name? Did you mistype it? Yell at peg for more info");

    let inner = html! {
        section #work-detail {
            .member-detail-container {
                .member-profile {
                    .member-profile-image {
                        (embed(&work_meta.link))
                    }
                    .member-profile-info {
                        h2 { (work_meta.title) }
                        a href=(format!("/members/{}.html", work_meta.author)) { p { (author_name) } }
                        .member-bio {
                            (PreEscaped(content))
                        }
                    }
                }
            }
        }

        .back-button{
            a href="../works.html" {
                "作品集合一覧に戻る"
            }
        }
    };

    let metadata = Metadata {
        page_title: work_meta.title.clone(),
        page_image: Some(get_thumbnail(&work_meta.link)),
        canonical_link: work_meta.link.to_string(),
        section: Sections::WorksPost,
        author: Some(work_meta.author.clone()),
        date: Some(work_meta.date.to_string()),
    };
    base(sack, &metadata, inner)
}

pub fn get_thumbnail(link: &str) -> String {
    let url_type = SocialLinkType::from_str(link).unwrap();
    let url_parse = Url::parse(link).unwrap();

    match url_type {
        SocialLinkType::Youtube => {
            let youtube_video_id = url_parse
                .query_pairs()
                .find(|(key, _)| key == "v")
                .unwrap()
                .1;
            format!(
                "https://img.youtube.com/vi/{}/maxresdefault.jpg",
                youtube_video_id
            )
        }
        SocialLinkType::NicoDouga => {
            // FIXME: NND is fucking cringe and you need some sort of key to download their thumbs.
            // TODO: use request and fetch the thumbnails and host them locally.
            // Until then, lol.
            "/images/gray.jpg".to_string()
        }
        _ => "/images/gray.jpg".to_string(),
    }
}
