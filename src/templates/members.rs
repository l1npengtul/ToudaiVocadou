use maud::{html, Markup};
use crate::member::{MemberFeaturedWork, MemberMeta};
use crate::metadata::Metadata;
use crate::sitemap::SiteMap;
use crate::templates::base::base;
use crate::templates::functions::embed::embed;
use crate::templates::functions::sns::sns_icon;
use crate::templates::partials::navbar::Sections;

pub fn members(site_map: &SiteMap) -> Markup {
    let inner = html! {
        section #members-hero {
            .container {
                h2 { "メンバー紹介" }
                p { "東京大学ボカロP同好会で活動する個性豊かなメンバーたちをご紹介します。" }
            }
        }

        section #staff-members {
            .zcontainer {
                @for member in &site_map.members {
                    (member_card(member))
                }
            }
        }
    };

    let metadata = Metadata {
        page_title: "メンバー紹介 - 東京大学ボカロP同好会".to_string(),
        page_image: None,
        canonical_link: "/members".to_string(),
        section: Sections::Members,
        author: None,
        date: None,
    };

    base(&metadata, inner)
}

pub fn member_card(member: &MemberMeta) -> Markup {
    html! {
        .member-item {
            a .member-link href=(format!("/members/{}", member.ascii_name)) {
                .member-card {
                    .member-image .img-placeholder {
                        img .member-image .img-placeholder src=(format!("{}.jpg", member.short)) alt=(member.name); // FIXME
                    }
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

pub fn member_detail(member: &MemberMeta, content: &str) -> Markup {
    let inner = html! {
        section #member-detail {
            .member-detail-container {
                .member-profile {
                    .member-profile-image {
                        img .img-placeholder src=(format!("{}.jpg", member.short)) alt=(member.name); // FIXME
                    }
                    .member-profile-info {
                        h2 { (member.name) }
                        .member-bio {
                            (content)
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
        }
    };

    let metadata: Metadata = member.clone().into();

    base(&metadata, inner)
}

pub fn featured_work_item_detail(item: &MemberFeaturedWork) -> Markup {
    html! {
        .work-item-detail {
            h4 { (item.title) }
            .work-youtube-container {
                (embed(item.link.as_str()))
            }
            @if let Some(desc) = &item.description {
                .work-description {
                    p { (desc) }
                }
            }
        }
    }
}