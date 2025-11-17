use crate::AudioFile;
use crate::SiteData;
use crate::album::AlbumMeta;
use crate::die_linky::SocialLinkType;
use crate::image;
use crate::lnk;
use crate::lnk_s3;
use crate::metadata::Metadata;
use crate::sitemap::SiteMap;
use crate::templates::base::base;
use crate::templates::members::big_display_for_item;
use crate::templates::partials::navbar::Sections;
use crate::util::shorten;
use crate::work::WorkMeta;
use base64::Engine;
use base64::prelude::BASE64_STANDARD_NO_PAD;
use hauchiwa::{Context, RuntimeError};
use maud::{Markup, PreEscaped, html};
use std::collections::HashMap;
use std::str::FromStr;
use url::Url;

pub fn works(
    sack: &Context<SiteData>,
    site_map: &SiteMap,
    name_map: &HashMap<String, String>,
) -> Result<Markup, RuntimeError> {
    // TODO: pagination. this will get ungodly long. yell at peng if we get >100!

    let inner = html! {
        section #hero {
            .container {
                h2 { "リリース" }
                p { "東京大学ボカロP同好会のメンバーの作品目録です。" }
            }
        }

        section .filters {
            a .filter-link href="#songs" {
                "リリース"
            }
            a .filter-link href="#albums" {
                "アルバム"
            }
        }

        section #songs .list {
            h3 {
                "リリース"
            }
            .listcontainer {
                @for work in &site_map.works {
                    (work_card(sack, work, name_map)?)
                }
            }
        }

        section #albums .list {
            h3 {
                "アルバム"
            }
            .listcontainer {
                @for album in &site_map.albums {
                    (album_card(sack, album, name_map)?)
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

pub fn work_card(
    sack: &Context<SiteData>,
    work_meta: &WorkMeta,
    name_map: &HashMap<String, String>,
) -> Result<Markup, RuntimeError> {
    let author_name = name_map.get(&work_meta.author).ok_or(RuntimeError::msg("Could not find author. Does the member page exist? Did you remember to type in the ascii name? Did you mistype it?".to_string()))?;

    Ok(html! {
        .item-card {
            .item-type {
                p {
                    @if work_meta.remix_original_work.is_some() {
                        "リミックス"
                    } @else {
                        "リリース曲"
                    }
                }
            }
            .item-image {
                img .work-item-thumb src=(thumbnail(sack, work_meta)?) alt=(work_meta.title) {}
            }
            .item-title {
                h3 {
                    a href=(lnk(sack, format!("/works/releases/{}.html", work_reference(&work_meta.title, &work_meta.author)))){
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
    })
}

pub fn work_reference(title: &str, author_ascii: &str) -> String {
    let titlehash = seahash::hash(title.as_bytes()) as u128;
    let authorhash = seahash::hash(author_ascii.as_bytes()) as u128;
    let combined = titlehash + authorhash;
    BASE64_STANDARD_NO_PAD.encode(combined.to_le_bytes())
}

pub fn album_reference(title: &str, cover: &str) -> String {
    let titlehash = seahash::hash(title.as_bytes()) as u128;
    let coverhash = seahash::hash(cover.as_bytes()) as u128;
    let combined = titlehash + coverhash;
    BASE64_STANDARD_NO_PAD.encode(combined.to_le_bytes())
}

pub fn album_card(
    sack: &Context<SiteData>,
    album_meta: &AlbumMeta,
    name_map: &HashMap<String, String>,
) -> Result<Markup, RuntimeError> {
    Ok(html! {
        .item-card {
            .item-type {
                p {
                    "アルバム"
                }
            }
            .item-image {
                img .work-item-thumb src=(image(sack, &album_meta.front_cover)?) alt=(&album_meta.title) {}
            }
            .item-title {
                h3 {
                    a href=(
                        lnk(sack, format!("works/albums/{}.html", album_reference(&album_meta.title, &album_meta.front_cover)))
                    ) {
                        (album_meta.title)
                    }
                }
                p .member-role {
                    (album_meta.release_date)
                }
                p .member-department {
                    (album_meta.contributors_str(name_map))
                }
                p {
                    (album_meta.short)
                }
            }
        }
    })
}

pub fn album_detail(
    sack: &Context<SiteData>,
    album_meta: &AlbumMeta,
    name_map: &HashMap<String, String>,
    content: &str,
) -> Result<Markup, RuntimeError> {
    let contributors = album_meta.contributors.iter().map(|contributor| {
        let ascii_name = name_map.get(contributor).unwrap();
        html! {
            a href=(lnk(sack, format!("members/{}.html", ascii_name))) {
                (contributor)
            }
        }
    });
    let extra_contributors = album_meta.extra_contributors.iter();

    let has_additional_illusts = album_meta.other_covers.len() > 0;

    let inner = html! {
        section #work-section {
            .work-detail-container {
                .work-detail {
                    .work-image {
                        (image(sack, &album_meta.front_cover)?)
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
                        @if has_additional_illusts {
                            .album-images {
                                h4 {  }
                                .work-item-detail {

                                }
                            }
                        }
                        @ if let Some(crossfade_demonstration) = &album_meta.crossfade_demonstration {
                            (big_display_for_item(
                                "試聴動画",
                                None,
                                "試聴動画 - Cross Fade Demonstration",
                                crossfade_demonstration,
                                None,
                            )?)
                        }
                    }
                }
            }
            .work-description {
                (PreEscaped(content))
            }
        }

        .back-button{
            a href="../works.html" {
                "リリース集合一覧に戻る"
            }
        }
    };

    let metadata = Metadata {
        page_title: album_meta.title.clone(),
        page_image: Some(image(sack, &album_meta.front_cover)?),
        canonical_link: format!(
            "works/albums/{}.html",
            album_reference(&album_meta.title, &album_meta.front_cover)
        ),
        section: Sections::WorksPost,
        description: Some(album_meta.short.clone()),
        author: Some(album_meta.contributors_str(name_map)),
        date: Some(album_meta.release_date.to_string()),
    };
    base(sack, &metadata, Some(&[]), inner)
}

pub fn front_image_lnk_for_work(
    link: Option<&str>,
    cover: Option<&str>,
) -> Result<String, RuntimeError> {
    match cover {
        Some(cover_lnk) => Ok(lnk_s3(cover_lnk)),
        None => match link {
            Some(video) => get_link_image_thumb(video),
            None => panic!("fuck! fuck! fuck! no linky :c :c"),
        },
    }
}

pub fn work_detail(
    sack: &Context<SiteData>,
    work_meta: &WorkMeta,
    name_map: &HashMap<String, String>,
    content: &str,
) -> Result<Markup, RuntimeError> {
    let author_name = name_map.get(&work_meta.author).expect("Could not find author. Does the member page exist? Did you remember to type in the ascii name? Did you mistype it? Yell at peg for more info");

    let inner = html! {
        section #work-section {
            .work-detail-container {
                .work-detail {
                    .work-image {
                        (thumbnail(sack, work_meta)?)
                    }
                    .work-info {
                        h2 { (work_meta.title) }
                        .work-featured-work {
                            @if work_meta.featured {
                                h4 { "このリリースはメンバーページでフィーチャーされています。" }
                            }
                        }
                        a href=(lnk(sack, format!("members/{}.html", work_meta.author))) { p { (author_name) } }
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
            a href="../works.html" {
                "リリース集合一覧に戻る"
            }
        }
    };

    let metadata = Metadata {
        page_title: work_meta.title.clone(),
        page_image: Some(thumbnail(sack, work_meta)?),
        canonical_link: format!("works/releases/{}.html", work_meta.title),
        section: Sections::WorksPost,
        description: Some(work_meta.short.clone().unwrap_or(shorten(content))),
        author: Some(work_meta.author.clone()),
        date: Some(work_meta.date.to_string()),
    };
    base(sack, &metadata, Some(&[]), inner)
}

pub fn thumbnail(sack: &Context<SiteData>, meta: &WorkMeta) -> Result<String, RuntimeError> {
    match &meta.display {
        crate::work::CoverOrImage::Cover(cover) => image(sack, cover),
        crate::work::CoverOrImage::Link(url) => get_link_image_thumb(url.as_str()),
        crate::work::CoverOrImage::AudioFile(audio_file) => {
            match sack.glob_one_with_file::<AudioFile>(audio_file) {
                Ok(audio) => Ok(audio.file.file.to_string()),
                Err(why) => Err(RuntimeError::from(why)),
            }
        }
    }
}

pub fn get_link_image_thumb(link: &str) -> Result<String, RuntimeError> {
    let url_type =
        SocialLinkType::from_str(link).map_err(|why| RuntimeError::msg(why.to_string()))?;
    let url_parse = Url::parse(link).map_err(|why| RuntimeError::msg(why.to_string()))?;

    match url_type {
        SocialLinkType::Youtube => {
            let youtube_video_id = url_parse
                .query_pairs()
                .find(|(key, _)| key == "v")
                .ok_or(RuntimeError::msg("Invalid youtube id"))?
                .1;
            Ok(format!(
                "https://img.youtube.com/vi/{}/maxresdefault.jpg",
                youtube_video_id
            ))
        }
        SocialLinkType::NicoDouga => {
            // FIXME: NND is fucking cringe and you need some sort of key to download their thumbs.
            // use request and fetch the thumbnails and host them locally.
            // Until then, lol.
            Ok("/images/gray.jpg".to_string())
        }
        _ => Ok("/images/gray.jpg".to_string()),
    }
}
