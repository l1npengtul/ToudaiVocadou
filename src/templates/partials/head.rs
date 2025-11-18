use crate::SiteData;
use crate::metadata::{Metadata, render_metadata};
use camino::Utf8PathBuf;
use hauchiwa::loader::Style;
use hauchiwa::{Context, RuntimeError};
use maud::{Markup, html};

pub fn html_head(
    sack: &Context<SiteData>,
    metadata: &Metadata,
    scripts: &[&str],
) -> Result<Markup, RuntimeError> {
    let style_path = Utf8PathBuf::from("styles/style.css");
    let style = sack.get::<Style>(&style_path)?.path.as_str();

    Ok(html! {
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            (render_metadata(sack, metadata)?)
            link rel="stylesheet" href=(style);
            link rel="icon" type="image/x-icon" href="/favicon.ico";
            @for script_url in scripts {
                script src=(script_url) {}
            }
        }
    })
}
