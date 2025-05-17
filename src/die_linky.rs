use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use url::Url;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocialLinkType {
    Twitter,
    Xitter,
    Bluesky,
    Youtube,
    NicoDouga,
    Soundcloud,
    Github,
    LinkTree,
    Spotify,
    TikTok,
    Instagram,
    OtherUnknown(String),
}

impl SocialLinkType {
    pub fn to_svg_icon(&self) -> &str {
        match self {
            SocialLinkType::Twitter => "twitter.svg",
            SocialLinkType::Xitter => "twitter.svg", // FIXME
            SocialLinkType::Bluesky => "bluesky.svg",
            SocialLinkType::Youtube => "youtube.svg",
            SocialLinkType::NicoDouga => "niconico.svg",
            SocialLinkType::Soundcloud => "soundcloud.svg",
            SocialLinkType::Github => "github.svg",
            SocialLinkType::LinkTree => "linktree.svg",
            SocialLinkType::Spotify => "spotify.svg",
            SocialLinkType::TikTok => "tiktok.svg",
            SocialLinkType::Instagram => "instagram.svg",
            SocialLinkType::OtherUnknown(_) => "link.svg",
        }
    }
}

impl FromStr for SocialLinkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parse into a URL
        let url = Url::parse(s)?;
        let domain = url.domain().ok_or(Error::msg("Bad URL"))?;
        let url_type = match domain {
            "twitter.com" => SocialLinkType::Twitter,
            "x.com" => SocialLinkType::Xitter,
            "bsky.app" => SocialLinkType::Bluesky,
            "youtube.com" | "www.youtube.com" => SocialLinkType::Youtube,
            "soundcloud.com" => SocialLinkType::Youtube,
            "nicovideo.jp" | "www.nicovideo.jp" => SocialLinkType::NicoDouga,
            "github.com" => SocialLinkType::Github,
            "linktree.com" | "linktr.ee" => SocialLinkType::LinkTree,
            "spotify.com" => SocialLinkType::Spotify,
            "tiktok.com" => SocialLinkType::TikTok,
            "instagram.com" => SocialLinkType::Instagram,
            other => SocialLinkType::OtherUnknown(other.to_string()),
        };

        Ok(url_type)
    }
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileOrPost {
    Profile(String),
    Post(String),
}

impl Display for ProfileOrPost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProfileOrPost::Profile(p) | ProfileOrPost::Post(p) => p,
            }
        )
    }
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct SocialLink {
    pub social_link_type: SocialLinkType,
    pub profile_or_post: ProfileOrPost,
}

impl Display for SocialLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let path = self.profile_or_post.to_string();

        let url = match &self.social_link_type {
            SocialLinkType::Twitter => "https://twitter.com",
            SocialLinkType::Xitter => "https://x.com",
            SocialLinkType::Bluesky => "https://bsky.app",
            SocialLinkType::Youtube => "https://www.youtube.com",
            SocialLinkType::NicoDouga => "https://nicovideo.jp",
            SocialLinkType::Soundcloud => "https://soundcloud.com",
            SocialLinkType::Github => "https://github.com",
            SocialLinkType::LinkTree => "https://linktr.ee",
            SocialLinkType::OtherUnknown(other) => other,
            _ => "",
        };

        write!(f, "{url}/{path}")
    }
}
