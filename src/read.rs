use crate::work::{CoverOrImage, RawWorkMeta, WorkMeta};
use crate::{FRONT_MATTER_SPLIT, SiteData};
use camino::Utf8PathBuf;
use hauchiwa::Context;
use hauchiwa::loader::Image;
use minijinja::Environment;
use pulldown_cmark::html::push_html;
use pulldown_cmark::{CowStr, Event, Options, Parser, Tag};
use serde::Serialize;
use serde::de::DeserializeOwned;
use url::Url;

pub fn parse_front_matter_and_fetch_contents<Metadata>(
    file: &str,
) -> Result<(Metadata, String), anyhow::Error>
where
    Metadata: DeserializeOwned,
{
    let (front_matter, content) = match file.split_once(FRONT_MATTER_SPLIT) {
        Some(v) => v,
        None => {
            return Err(anyhow::Error::msg(format!(
                "Failed to split front matter! Ensure that the front matter splitter \"{FRONT_MATTER_SPLIT}\" exists! - フロントデータを分離できませんでした。フロントデータ分離マーカー「{FRONT_MATTER_SPLIT}」があるのかを確認してください！"
            )));
        }
    };

    let toml_parsed = toml::from_str::<Metadata>(front_matter)?;

    Ok((toml_parsed, content.to_string()))
}

pub fn parse_work_meta(file: &str) -> Result<(WorkMeta, String), anyhow::Error> {
    let (raw_work, content) = parse_front_matter_and_fetch_contents::<RawWorkMeta>(file)?;

    let coi = match &raw_work.cover_image {
        Some(coverimg) => CoverOrImage::Cover(coverimg.to_string()),
        None => match &raw_work.link {
            Some(wlnl) => CoverOrImage::Link(wlnl.clone()),
            None => match &raw_work.file {
                Some(f) => CoverOrImage::AudioFile(f.to_string()),
                None => {
                    return Err(anyhow::Error::msg(
                        "Could not find a suitable display. Please ensure one of the following is set: `link`, `cover`, `file`.",
                    ))?;
                }
            },
        },
    };

    Ok((
        WorkMeta {
            title: raw_work.title,
            author: raw_work.author,
            collaborators: raw_work.collaborators,
            date: raw_work.date,
            short: raw_work.short,
            display: coi,
            cover_image: raw_work.cover_image,
            link: raw_work.link,
            file: raw_work.file,
            remix_original_work: raw_work.remix_original_work,
            featured: raw_work.featured,
            streaming: raw_work.streaming,
            duration_seconds: raw_work.duration_seconds,
        },
        content,
    ))
}

// pub fn parse_and_format<Metadata>(
//     sack: &Context<SiteData>,
//     context: &Metadata,
//     environment: &Environment,
//     content: &str,
// ) -> Result<String, anyhow::Error>
// where
//     Metadata: Serialize,
// {
//     let rendering = environment.render_str(content, context)?;

//     let markdown_opts = Options::ENABLE_DEFINITION_LIST
//         | Options::ENABLE_FOOTNOTES
//         | Options::ENABLE_SMART_PUNCTUATION
//         | Options::ENABLE_SUBSCRIPT
//         | Options::ENABLE_SUPERSCRIPT
//         | Options::ENABLE_TABLES
//         | Options::ENABLE_TASKLISTS
//         | Options::ENABLE_STRIKETHROUGH;

//     let mut output_str_buf = String::new();

//     let parser = Parser::new_ext(&rendering, markdown_opts);

//     push_html(
//         &mut output_str_buf,
//         parser.map(|event| -> Event {
//             match event {
//                 Event::Start(start) => {
//                     let tag = match start {
//                         Tag::Image {
//                             link_type,
//                             dest_url,
//                             title,
//                             id,
//                         } => {
//                             let url_utf8 = Utf8PathBuf::from(dest_url.as_ref());
//                             if let Ok(picture) = sack.get::<Image>(&url_utf8) {
//                                 Tag::Image {
//                                     link_type,
//                                     dest_url: CowStr::from(picture.path.to_string()),
//                                     title,
//                                     id,
//                                 }
//                             } else {
//                                 Tag::Image {
//                                     link_type,
//                                     dest_url,
//                                     title,
//                                     id,
//                                 }
//                             }
//                         }
//                         other => other,
//                     };
//                     Event::Start(tag)
//                 }
//                 e => e,
//             }
//         }),
//     );

//     Ok(output_str_buf)
// }
