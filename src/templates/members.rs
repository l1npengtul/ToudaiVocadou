use crate::SiteData;
use crate::member::MemberMeta;
use crate::metadata::Metadata;
use crate::sitemap::SiteMap;
use crate::templates::base::base;
use crate::templates::functions::embed::embed;
use crate::templates::functions::sns::sns_icon;
use crate::templates::partials::navbar::Sections;
use crate::templates::works::{thumbnail_link, work_reference};
use crate::util::image;
use crate::work::WorkMeta;
use hauchiwa::Context;
use hauchiwa::RuntimeError;
use maud::{Markup, PreEscaped, html};
use std::cmp::Ordering;

pub fn members(sack: &Context<SiteData>, site_map: &SiteMap) -> Result<Markup, RuntimeError> {
    let mut site_members = site_map.members.clone();
    site_members.sort_by(|a, b| {
        let a_str = a.position.clone().unwrap_or_default();
        let b_str = b.position.clone().unwrap_or_default();

        if a_str == "代表" {
            return Ordering::Less;
        } else if b_str == "代表" {
            return Ordering::Greater;
        }

        if a_str == "副代表" {
            return Ordering::Less;
        } else if b_str == "副代表" {
            return Ordering::Greater;
        }

        if a_str == "広報" {
            return Ordering::Less;
        } else if b_str == "広報" {
            return Ordering::Greater;
        }

        if a.position.is_some() && b.position.is_none() {
            return Ordering::Less;
        } else if a.position.is_none() && b.position.is_some() {
            return Ordering::Greater;
        } else if a.position == b.position {
            return a.name.cmp(&b.name);
        }

        a.name.cmp(&b.name)
    });

    let inner = html! {
        section #members-hero {
            .container {
                h2 { "メンバー紹介" }
                p { "東京大学ボカロP同好会で活動する個性豊かなメンバーたちをご紹介します。" }
            }
        }

        section #staff-members {
            .zcontainer {
                .member-grid {
                    @for member in &site_members {
                        (member_card(sack, member)?)
                    }
                }
            }
        }
    };

    let metadata = Metadata {
        page_title: "メンバー紹介 - 東京大学ボカロP同好会".to_string(),
        page_image: None,
        canonical_link: "members.html".to_string(),
        section: Sections::Members,
        description: Some("東京大学ボカロP同好会のメンバー紹介".to_string()),
        author: None,
        date: None,
    };

    base(sack, &metadata, None, inner)
}

pub fn member_card(sack: &Context<SiteData>, member: &MemberMeta) -> Result<Markup, RuntimeError> {
    let member_links_len = member.links.len();
    Ok(html! {
        .member-item {
            a .member-link href=(format!("/members/{}.html", member.ascii_name)) {
                .member-card {
                    .member-image .img-placeholder {
                        img .member-image .img-placeholder src=(image(sack, format!("images/icon/{}.jpg", member.ascii_name))?) alt=(member.name); // FIXME
                    }
                    .member-info #(member.ascii_name) {
                        h3 { (member.name) }
                        @if let Some(role) = &member.position {
                            p .member-role { (role) }
                        }
                        @if let Some(department) = &member.department {
                            p .member-department { (department) }
                        }
                        p .member-description { (&member.short) }
                        .member-links {
                            // dummy div to fill out the size in case the user has no icons
                            @if member_links_len == 0 {
                                .sns-icon-size {}
                            }
                            @for link in &member.links {
                                (sns_icon(link)?)
                            }
                        }
                    }
                }
            }
        }
    })
}

// TODO: add "worked on albums" and "posts".
pub fn member_detail(
    sack: &Context<SiteData>,
    member: &MemberMeta,
    featured_works: &[WorkMeta],
    content: &str,
) -> Result<Markup, RuntimeError> {
    let this_featured_work = featured_works
        .iter()
        .filter(|featured| featured.author == member.ascii_name)
        .collect::<Vec<&WorkMeta>>();

    let inner = html! {
        section #member-detail {
            .member-detail-container {
                .member-profile {
                    .member-profile-image {
                        img .img-placeholder src=(image(sack, format!("images/icon/{}.jpg", member.ascii_name))?) alt=(member.name);
                    }
                    .member-profile-info {
                        h2 { (member.name) }
                        @if let Some(role) = &member.position {
                            p .member-role { (role) }
                        }
                        .member-bio {
                            (PreEscaped(content))
                        }
                        .member-links {
                            @for link in &member.links {
                                (sns_icon(link)?)
                            }
                        }
                    }
                }
            }

            .member-featured-works {
                h3 { "代表作品" }
                .container {
                    @for featured in &this_featured_work {
                        (featured_work_item_detail(sack, featured)?)
                    }
                }
            }

            .member-featured-works {
                h3 { "最近のポスト" }
                .container {

                }
            }

            .back-button{
                a href="../members.html" {
                    "メンバー一覧に戻る"
                }
            }
        }
    };

    let metadata = MemberMeta::to_metadata(member.clone());

    base(sack, &metadata, None, inner)
}

pub fn featured_work_item_detail(
    sack: &Context<SiteData>,
    item: &WorkMeta,
) -> Result<Markup, RuntimeError> {
    let work_ref = work_reference(&item.title, &item.author);

    Ok(html! {
        .work-item-detail id=(work_ref) {
            h4 { (item.title) }
            .work-youtube-container {
                (thumbnail_link(sack, item)?)
            }

            .work-description {
                @if let Some(desc) = &item.short {
                    p { (desc) }
                }
                @else {
                    p {}
                }
            }

            .back-button{
                a href=(format!("/works/releases/{}.html", work_ref)) {
                    "詳しく見る"
                }
            }
        }
    })
}

pub fn big_display_for_item(
    id: &str,
    detailed: Option<&str>,
    title: &str,
    link: &str,
    short: Option<&str>,
) -> Result<Markup, RuntimeError> {
    Ok(html! {
        .work-item-detail id=(id) {
            h4 { (title) }
            .work-youtube-container {
                (embed(link)?)
            }

            .work-description {
                @if let Some(desc) = short {
                    p { (desc) }
                }
                @else {
                    p {}
                }
            }

            @if let Some(detail) = detailed {
                .back-button{
                    a href=(detail) {
                        "詳しく見る"
                    }
                }
            }
        }
    })
}
