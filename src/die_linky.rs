use std::fmt::{Display, Formatter};
use url::Url;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq)]
pub enum SocialLinkType {
    Twitter,
    Xitter,
    Bluesky,
    Youtube,
    NicoDouga,
    Soundcloud,
    SelfHostedSite(Url),
    Github,
    LinkTree,
    OtherUnknown(String),
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq)]
pub enum ProfileOrPost {
    Profile(String),
    Post(String),
}

impl Display for ProfileOrPost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ProfileOrPost::Profile(p) | ProfileOrPost::Post(p) => p,
        })
    }
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq)]
pub struct SocialLink {
    pub social_link_type: SocialLinkType,
    pub profile_or_post: ProfileOrPost,
}

impl Display for SocialLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let path = self.profile_or_post.to_string();

        let url = match &self.social_link_type {
            SocialLinkType::Twitter => {
                "https://twitter.com"
            }
            SocialLinkType::Xitter => {
                "https://x.com"
            }
            SocialLinkType::Bluesky => {
                "https://bsky.app"
            }
            SocialLinkType::Youtube => {
                "https://www.youtube.com"
            }
            SocialLinkType::NicoDouga => {
                "https://nicovideo.jp"
            }
            SocialLinkType::Soundcloud => {
                "https://soundcloud.com"
            }
            SocialLinkType::SelfHostedSite(site) => {
                site.as_str()
            }
            SocialLinkType::Github => {
                "https://github.com"
            }
            SocialLinkType::LinkTree => {
                "https://linktr.ee"
            }
            SocialLinkType::OtherUnknown(other) => {
                &other
            }
        };

        write!(f, "{url}/{path}")
    }
}
