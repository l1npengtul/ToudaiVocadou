use crate::member::{MemberFeaturedWork, MemberMeta};
use crate::metadata::Metadata;
use crate::sitemap::SiteMap;
use crate::templates::base::base;
use crate::templates::functions::embed::embed;
use crate::templates::functions::sns::sns_icon;
use crate::templates::partials::navbar::Sections;
use crate::{Data, image};
use hauchiwa::Sack;
use maud::{Markup, html, PreEscaped};

pub fn members(sack: &Sack<Data>, site_map: &SiteMap) -> Markup {
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
                    @for member in &site_map.members {
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

            a href="../members.html" .back-button {
                "メンバー一覧に戻る"
            }
        }
    };

    let metadata: Metadata = member.clone().into();

    base(sack, &metadata, inner)
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
