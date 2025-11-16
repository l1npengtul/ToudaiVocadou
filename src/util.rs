use crate::{lnk, lnk_s3};

pub fn shorten(content: &str) -> String {
    content.chars().take(150).collect::<String>()
}

#[derive(Clone, Debug, PartialEq)]
pub struct SvgData {
    pub path: camino::Utf8PathBuf,
    pub data: Vec<u8>,
}
