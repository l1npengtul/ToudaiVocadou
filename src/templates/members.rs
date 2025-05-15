use crate::member::{MemberFeaturedWork, MemberMeta};
use crate::metadata::Metadata;
use crate::sitemap::SiteMap;
use crate::templates::base::base;
use crate::templates::functions::embed::embed;
use crate::templates::functions::sns::sns_icon;
use crate::templates::partials::navbar::Sections;
use crate::{Data, image};
use base64::Engine;
use base64::prelude::BASE64_STANDARD_NO_PAD;
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
        canonical_link: "/members.html".to_string(),
        section: Sections::Members,
        author: None,
        date: None,
    };

    base(sack, &metadata, inner)
}

pub fn member_card(sack: &Sack<Data>, member: &MemberMeta) -> Markup {
    html! {
        .member-item {
            a .member-link href=(format!("/members/{}.html", member.ascii_name)) {
                .member-card {
                    .member-image .img-placeholder {
                        img .member-image .img-placeholder src=(image(sack, format!("/icon/{}.jpg", member.ascii_name))) alt=(member.name); // FIXME
                    }
                    .member-info {
                        h3 { (member.name) }
                        @if let Some(role) = &member.position {
                            p .member-role { (role) }
                        }
                        @if let Some(department) = &member.department {
                            p .member-department { (department) }
                        }
                        p .member-description { (&member.short) }
                        .member-links {
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

pub fn member_detail(sack: &Sack<Data>, member: &MemberMeta, content: &str) -> Markup {
    let inner = html! {
        section #member-detail {
            .member-detail-container {
                .member-profile {
                    .member-profile-image {
                        img .img-placeholder src=(image(sack, format!("/icon/{}.jpg", member.ascii_name))) alt=(member.name); // FIXME
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
                @for featured in &member.featured_works {
                    (featured_work_item_detail(featured))
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

    base(sack, &metadata, inner)
}

pub fn featured_work_item_detail(item: &MemberFeaturedWork) -> Markup {
    let link_hash = seahash::hash(item.link.as_bytes()).to_le_bytes();
    let base64 = BASE64_STANDARD_NO_PAD.encode(link_hash);

    html! {
        .work-item-detail id=(base64) {
            h4 { (item.title) }
            .work-youtube-container {
                (embed(item.link.as_str()))
            }
            @if let Some(desc) = &item.description {
                .work-description {
                    p { (desc) }
                }
            }
            @if item.__do_not_use_kuwasiku {
                .back-button{
                    a href=(format!("/works/{}.html", base64)) {
                        "詳しく見る"
                    }
                }
            }
        }
    }
}
