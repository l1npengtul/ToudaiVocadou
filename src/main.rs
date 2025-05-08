use crate::member::MemberMeta;
use crate::post::PostMeta;
use crate::read::{format_html_with_meta, parse_front_matter_and_fetch_contents, parse_markdown};
use crate::sitemap::SiteMap;
use crate::templates::functions::embed::jinja_embed;
use crate::templates::functions::member::jinja_member;
use crate::templates::functions::sns::jinja_sns_icon;
use crate::templates::index::index;
use crate::work::WorkMeta;
use camino::Utf8PathBuf;
use clap::{Parser, ValueEnum};
use hauchiwa::{Collection, HauchiwaError, Processor, Sack, Website};
use minijinja::Environment;
use std::fs::File;
use std::io::Read;
use crate::templates::members::member_detail;

mod die_linky;
mod featured_work;
mod member;
mod metadata;
mod optimize;
mod post;
mod read;
mod sitemap;
pub mod templates;
mod work;

pub const FRONT_MATTER_SPLIT: &'static str = "===";
pub const RENDER_SITE: &'static str = "toudaivocadou.org";

pub struct Data;

#[derive(Parser, Debug, Clone)]
struct Args {
    #[clap(value_enum, index = 1, default_value = "build")]
    mode: Mode,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum Mode {
    Build,
    Watch,
}

fn main() {
    let args = Args::parse();

    let website = Website::configure()
        .add_collections(vec![
            Collection::glob_with(
                "members",
                "*",
                ["md"],
                parse_front_matter_and_fetch_contents::<MemberMeta>,
            ),
            Collection::glob_with(
                "posts",
                "*",
                ["md"],
                parse_front_matter_and_fetch_contents::<PostMeta>,
            ),
            Collection::glob_with(
                "works",
                "*",
                ["md"],
                parse_front_matter_and_fetch_contents::<WorkMeta>,
            ),
        ])
        .add_processors(vec![Processor::process_images([
            "png", "jpg", "jpeg", "gif", "avif",
        ])])
        .add_styles([Utf8PathBuf::from("css")])
        .add_scripts([("script", "js/script.js")])
        .set_opts_sitemap("sitemap.xml")
        .add_task(|sack| {
            // build up our sitemap and metadatas
            let members = sack.query_content::<MemberMeta>("*").unwrap();
            let member_name_list = members
                .iter()
                .map(|x| x.meta.ascii_name.as_str())
                .collect::<Vec<&str>>();

            let works = sack.query_content::<WorkMeta>("works/*").unwrap();
            if !works
                .iter()
                .all(|work| member_name_list.contains(&work.meta.author.as_str()))
            {
                panic!("work contains bad author.")
            }

            let news_posts = sack.query_content::<PostMeta>("posts/*").unwrap();
            if !news_posts
                .iter()
                .all(|posts| member_name_list.contains(&posts.meta.author.as_str()))
            {
                panic!("post contains bad author.")
            }

            let sitemap = SiteMap {
                members: members.iter().map(|x| x.meta).cloned().collect(),
                posts: news_posts.iter().map(|x| x.meta).cloned().collect(),
                works: works.iter().map(|x| x.meta).cloned().collect(),
            };

            let sitemap_json = serde_json::to_string(&sitemap)?;

            let mut robots = String::new();
            File::open("robots.txt")
                .unwrap()
                .read_to_string(&mut robots)
                .unwrap();

            // do out set pages (index and members for now)
            let mut set_pages = vec![
                ("index.html".to_string(), index(&sack).into_string()),
                (
                    "members.html".to_string(),
                    templates::members::members(&sack, &sitemap).into_string(),
                ),
                ("sitemap.json".to_string(), sitemap_json),
                ("robots.txt".to_string(), robots),
                // ("/works",
            ];

            // environment
            let mut jinja_environment = Environment::new();
            jinja_environment.add_function("sns_link", jinja_sns_icon);
            jinja_environment.add_function("sns_embed", jinja_embed);
            jinja_environment.add_function("member", jinja_member);
            jinja_environment.add_global("SITE", RENDER_SITE);

            // render members

            let member_detail = members
                .into_iter()
                .map(|member_page| -> Result<(String, String), anyhow::Error> {
                    let temp_html_render = parse_markdown(&sack, member_page.content);
                    let content_html = format_html_with_meta(&jinja_environment, &temp_html_render, member_page.meta)?;
                    let rendered_page = member_detail(&sack, member_page.meta, &content_html).into_string();
                    let path = format!("members/{}.html", member_page.meta.ascii_name);
                    Ok((path, rendered_page))
                })
                .collect::<Result<Vec<(String, String)>, anyhow::Error>>()?;

            set_pages.extend(member_detail);

            // query posts
            // query works

            Ok(set_pages
                .into_iter()
                .map(|(path, page)| (Utf8PathBuf::from(path), page))
                .collect())
        })
        .finish();

    match args.mode {
        Mode::Build => website.build(Data {}).unwrap(),
        Mode::Watch => website.watch(Data {}).unwrap(),
    }
}

pub fn image(sack: &Sack<Data>, path: impl AsRef<str>) -> String {
    let picture_path = Utf8PathBuf::from(path.as_ref());
    match sack.get_picture(&picture_path) {
        Ok(p) => p.into_string(),
        Err(_) => path.as_ref().to_string(),
    }
}
