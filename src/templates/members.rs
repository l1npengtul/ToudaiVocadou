use crate::member::MemberMeta;
use crate::metadata::Metadata;
use crate::sitemap::SiteMap;
use crate::templates::base::base;
use crate::templates::functions::embed::embed;
use crate::templates::functions::sns::sns_icon;
use crate::templates::partials::navbar::Sections;
use crate::templates::works::work_reference;
use crate::work::WorkMeta;
use crate::{Data, image};
use hauchiwa::Sack;
use maud::{Markup, PreEscaped, html};
use std::cmp::Ordering;

pub fn members(sack: &Sack<Data>, site_map: &SiteMap) -> Markup {
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
                        (member_card(sack, member))
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

pub fn member_card(sack: &Sack<Data>, member: &MemberMeta) -> Markup {
    let member_links_len = member.links.len();
    html! {
        .member-item {
            a .member-link href=(format!("/members/{}.html", member.ascii_name)) {
                .member-card {
                    .member-image .img-placeholder {
                        img .member-image .img-placeholder src=(image(sack, format!("/icon/{}.jpg", member.ascii_name))) alt=(member.name); // FIXME
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
                                (sns_icon(link))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn member_detail(
    sack: &Sack<Data>,
    member: &MemberMeta,
    featured_works: &Vec<&WorkMeta>,
    content: &str,
) -> Markup {
    let this_featured_work = featured_works
        .iter()
        .filter(|featured| featured.author == member.ascii_name)
        .collect::<Vec<&&WorkMeta>>();

    let inner = html! {
        section #member-detail {
            .member-detail-container {
                .member-profile {
                    .member-profile-image {
                        img .img-placeholder src=(image(sack, format!("/icon/{}.jpg", member.ascii_name))) alt=(member.name);
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
                                (sns_icon(link))
                            }
                        }
                    }
                }
            }
            .member-featured-works {
                h3 { "代表作品" }
                .container {
                    @for featured in &this_featured_work {
                        (featured_work_item_detail(featured))
                    }
                }
            }

            .back-button{
                a href="../members.html" {
                    "メンバー一覧に戻る"
                }
            }
        }
    };

    let metadata: Metadata = member.clone().into();

    base(sack, &metadata, None, inner)
}

pub fn featured_work_item_detail(item: &WorkMeta) -> Markup {
    let work_ref = work_reference(&item.link);

    html! {
        .work-item-detail id=(work_ref) {
            h4 { (item.title) }
            .work-youtube-container {
                (embed(item.link.as_str()))
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
    }
}
