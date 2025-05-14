use crate::{Data, FRONT_MATTER_SPLIT};
use camino::Utf8PathBuf;
use hauchiwa::Sack;
use minijinja::Environment;
use pulldown_cmark::html::push_html;
use pulldown_cmark::{CowStr, Event, Options, Parser, Tag};
use serde::Serialize;
use serde::de::DeserializeOwned;

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

pub fn parse_and_format<Metadata>(
    sack: &Sack<Data>,
    context: &Metadata,
    environment: &Environment,
    content: &str,
) -> Result<String, anyhow::Error>
where
    Metadata: Serialize,
{
    let rendering = environment.render_str(content, context)?;

    let markdown_opts = Options::ENABLE_DEFINITION_LIST
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_SMART_PUNCTUATION
        | Options::ENABLE_SUBSCRIPT
        | Options::ENABLE_SUPERSCRIPT
        | Options::ENABLE_TABLES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_STRIKETHROUGH;

    let mut output_str_buf = String::new();

    let parser = Parser::new_ext(&rendering, markdown_opts);

    push_html(
        &mut output_str_buf,
        parser.map(|event| -> Event {
            match event {
                Event::Start(start) => {
                    let tag = match start {
                        Tag::Image {
                            link_type,
                            dest_url,
                            title,
                            id,
                        } => {
                            let url_utf8 = Utf8PathBuf::from(dest_url.as_ref());
                            if let Ok(picture) = sack.get_picture(&url_utf8) {
                                Tag::Image {
                                    link_type,
                                    dest_url: CowStr::from(picture.to_string()),
                                    title,
                                    id,
                                }
                            } else {
                                Tag::Image {
                                    link_type,
                                    dest_url,
                                    title,
                                    id,
                                }
                            }
                        }
                        other => other,
                    };
                    Event::Start(tag)
                }
                e => e,
            }
        }),
    );

    Ok(output_str_buf)
}
