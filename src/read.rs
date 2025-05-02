use serde::{Serialize};
use minijinja::{Environment};
use pulldown_cmark::{Options, Parser};
use pulldown_cmark::html::push_html;
use serde::de::DeserializeOwned;
use crate::FRONT_MATTER_SPLIT;

pub fn parse_front_matter_and_fetch_contents<Metadata>(file: &str) -> Result<(Metadata, String), anyhow::Error>
where Metadata: DeserializeOwned {
    let (front_matter, content) = match file.split_once(FRONT_MATTER_SPLIT) {
        Some(v) => v,
        None => {
            return Err(anyhow::Error::msg(format!("Failed to split front matter! Ensure that the front matter splitter \"{FRONT_MATTER_SPLIT}\" exists! - フロントデータを分離できませんでした。フロントデータ分離マーカー「{FRONT_MATTER_SPLIT}」があるのかを確認してください！")));
        }
    };

    let toml_parsed = toml::from_str::<Metadata>(front_matter)?;

    Ok((toml_parsed, content.to_string()))
}

pub fn parse_markdown(content: &str) -> String {
    let markdown_opts = Options::ENABLE_DEFINITION_LIST | Options::ENABLE_FOOTNOTES | Options::ENABLE_SMART_PUNCTUATION | Options::ENABLE_SUBSCRIPT | Options::ENABLE_SUPERSCRIPT | Options::ENABLE_TABLES | Options::ENABLE_TASKLISTS | Options::ENABLE_STRIKETHROUGH;

    let mut output_str_buf = String::new();
    push_html(&mut output_str_buf, Parser::new_ext(content, markdown_opts));

    output_str_buf
}

pub fn format_html_with_meta<Metadata>(environment: &Environment, html: &str, context: &Metadata) -> Result<String, anyhow::Error> where Metadata: Serialize {
    let rendering = environment.render_str(html, context)?;
    Ok(rendering)
}

