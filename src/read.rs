use std::fs::File;
use std::path::{Path, PathBuf};
use memmap2::{Mmap, MmapOptions};
use serde::{Deserialize, Serialize};
use eyre::{Report, Result};
use minijinja::{Environment, Template};
use pulldown_cmark::{Options, Parser};
use pulldown_cmark::html::push_html;
use crate::FRONT_MATTER_SPLIT;

pub fn read_file_into_memmap(path: &Path) -> Result<Mmap> {
    let mut file = File::open(path)?;
    let mapped = unsafe {
        MmapOptions::default()
            .map_copy_read_only(&file)?
    };

    Ok(mapped)
}

pub fn parse_front_matter_and_fetch_contents<Metadata>(file: &str) -> Result<(Metadata, &str)>
where Metadata: Deserialize {
    let (front_matter, content) = match file.split_once(FRONT_MATTER_SPLIT) {
        Some(v) => v,
        None => {
            return Err(Report::msg(format!("Failed to split front matter! Ensure that the front matter splitter \"{FRONT_MATTER_SPLIT}\" exists! - フロントデータを分離できませんでした。フロントデータ分離マーカー「{FRONT_MATTER_SPLIT}」があるのかを確認してください！")));
        }
    };

    let toml_parsed = toml::from_str::<Metadata>(front_matter)?;

    Ok((toml_parsed, content))
}

pub fn parse_markdown(content: &str) -> String {
    const MARKDOWN_PARSER_OPTS: Options = Options::ENABLE_DEFINITION_LIST | Options::ENABLE_FOOTNOTES | Options::ENABLE_SMART_PUNCTUATION | Options::ENABLE_SUBSCRIPT | Options::ENABLE_SUPERSCRIPT | Options::ENABLE_TABLES | Options::ENABLE_TASKLISTS | Options::ENABLE_STRIKETHROUGH;

    let mut output_str_buf = String::new();
    push_html(&mut output_str_buf, Parser::new_ext(content, MARKDOWN_PARSER_OPTS));

    output_str_buf
}

pub fn format_html_with_meta<Metadata>(environment: &mut Environment,html: String, context: &Metadata) -> Result<String> where Metadata: Serialize {
    let template_name = format!("md_{}", seahash::hash(html.as_bytes()));
    environment.add_template(&template_name, &html)?;

    let rendering = environment.get_template("this")?.render(context)?;
    Ok(rendering)
}

pub fn load_template(environment: &mut Environment,path: &Path) -> Result<()> {
    let name = path.file_name().ok_or(Report::msg(format!("Failed to load template {}", path)))?.to_string_lossy();
    environment.add_template()

}
