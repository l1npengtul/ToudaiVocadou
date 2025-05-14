use crate::Data;
use crate::metadata::{Metadata, render_metadata};
use camino::Utf8PathBuf;
use hauchiwa::Sack;
use maud::{Markup, html};

pub fn html_head(sack: &Sack<Data>, metadata: &Metadata) -> Markup {
    let style_path = Utf8PathBuf::from("css/style.css");
    let style = sack.get_styles(&style_path).unwrap().into_string();
    let script = sack.get_script("script.js").unwrap().into_string();

    html! {
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            (render_metadata(sack, metadata))
            link rel="stylesheet" href=(style);
            link rel="icon" type="image/x-icon" href="/favicon.ico";
            script src=(script) {}
        }
    }
}
