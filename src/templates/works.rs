use crate::Data;
use crate::album::AlbumMeta;
use crate::die_linky::SocialLinkType;
use crate::image;
use crate::lnk;
use crate::metadata::Metadata;
use crate::sitemap::SiteMap;
use crate::templates::base::base;
use crate::templates::functions::embed::embed;
use crate::templates::partials::navbar::Sections;
use crate::util::shorten;
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
                h2 { "リリース" }
                p { "東京大学ボカロP同好会のメンバーの作品目録です。" }
            }
        }

        section #filters {

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
        page_title: "リリース".to_string(),
        page_image: None,
        canonical_link: "works.html".to_string(),
        section: Sections::Works,
        description: Some("東京大学ボカロP同好会のメンバーの作品展示館".to_string()),
        author: None,
        date: None,
    };

    base(sack, &metadata, Some(&[]), inner)
}

pub fn work_card(work_meta: &WorkMeta, name_map: &HashMap<String, String>) -> Markup {
    let author_name = name_map.get(&work_meta.author).expect("Could not find author. Does the member page exist? Did you remember to type in the ascii name? Did you mistype it?");

    html! {
        .item-card {
            .item-type {
                p {
                    "リリース曲"
                }
            }
            .item-image {
                img .work-item-thumb src=(get_thumbnail(&work_meta.link)) alt=(work_meta.title) {}
            }
            .item-title {
                h3 {
                    a href=(lnk(format!("/works/releases/{}.html", work_reference(&work_meta.link)))){
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

pub fn album_card(album_meta: &AlbumMeta) -> Markup {
    html! {
        .item-card {
            .item-type {
                p {
                    "アルバム"
                }
            }
            .item-image {
                img .work-item-thumb src=(&album_meta.front_cover) alt=(&album_meta.title) {}
            }
            .item-title {
                h3 {
                    a href=(
                        lnk(format!("works/albums/{}.html", album_meta.title))
                    ) {
                        (album_meta.title)
                    }
                }
                p .member-role {
                    (album_meta.release_date)
                }
                p .member-department {
                    (album_meta.contributors_str())
                }
                p {
                    (album_meta.short)
                }
            }
        }
    }
}

pub fn album_detail(
    sack: &Sack<Data>,
    album_meta: &AlbumMeta,
    name_map: &HashMap<String, String>,
    content: &str,
) -> Markup {
    let contributors = album_meta.contributors.iter().map(|contributor| {
        let ascii_name = name_map
            .get(contributor)
            .expect(format!("Error! Failed to find user {contributor} in the name map.").as_str());
        html! {
            a href=(lnk(format!("members/{}.html", ascii_name))) {
                (contributor)
            }
        }
    });
    let extra_contributors = album_meta.extra_contributors.iter();

    let inner = html! {
        section #work-section {
            .work-detail-container {
                .work-detail {
                    .work-image {
                        (image(sack, &album_meta.front_cover))
                    }
                    .work-info {
                        h2 { (album_meta.title) }
                        .work-contributors {
                            h4 { "投稿者" }
                            p {
                                @for contrib in contributors {
                                    (contrib) " "
                                }
                                @for extrac in extra_contributors {
                                    (extrac) " "
                                }
                            }
                        }
                        hr {}
                        p {
                            (album_meta.short)
                        }
                    }
                }
            }
            .work-description {
                (PreEscaped(content))
            }
        }

        .back-button{
            a href=(lnk("works.html")) {
                "リリース集合一覧に戻る"
            }
        }
    };

    let metadata = Metadata {
        page_title: album_meta.title.clone(),
        page_image: Some(image(sack, &album_meta.front_cover)),
        canonical_link: format!("works/albums/{}.html", album_meta.title),
        section: Sections::WorksPost,
        description: Some(album_meta.short.clone()),
        author: Some(album_meta.contributors_str()),
        date: Some(album_meta.release_date.to_string()),
    };
    base(sack, &metadata, Some(&[]), inner)
}

pub fn work_detail(
    sack: &Sack<Data>,
    work_meta: &WorkMeta,
    name_map: &HashMap<String, String>,
    content: &str,
) -> Markup {
    let author_name = name_map.get(&work_meta.author).expect("Could not find author. Does the member page exist? Did you remember to type in the ascii name? Did you mistype it? Yell at peg for more info");

    let inner = html! {
        section #work-section {
            .work-detail-container {
                .work-detail {
                    .work-image {
                        (embed(&work_meta.link))
                    }
                    .work-info {
                        h2 { (work_meta.title) }
                        .work-featured-work {
                            @if work_meta.featured {
                                h4 { "このリリースはメンバーページでフィーチャーされています。" }
                            }
                        }
                        a href=(lnk(format!("members/{}.html", work_meta.author))) { p { (author_name) } }
                        hr {}
                        @if let Some(short) = &work_meta.short {
                            p { (short) }
                        }
                    }
                }
            }
            .work-description {
                (PreEscaped(content))
            }
        }

        .back-button{
            a href=(lnk("works.html")) {
                "リリース集合一覧に戻る"
            }
        }
    };

    let metadata = Metadata {
        page_title: work_meta.title.clone(),
        page_image: Some(get_thumbnail(&work_meta.link)),
        canonical_link: format!("works/releases/{}.html", work_meta.title),
        section: Sections::WorksPost,
        description: Some(work_meta.short.clone().unwrap_or(shorten(content))),
        author: Some(work_meta.author.clone()),
        date: Some(work_meta.date.to_string()),
    };
    base(sack, &metadata, Some(&[]), inner)
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
            // use request and fetch the thumbnails and host them locally.
            // Until then, lol.
            "/images/gray.jpg".to_string()
        }
        _ => "/images/gray.jpg".to_string(),
    }
}
