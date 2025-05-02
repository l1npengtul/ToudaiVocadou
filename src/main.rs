#![feature(inherent_str_constructors)]

use std::fs::File;
use std::io::Read;
use camino::Utf8PathBuf;
use hauchiwa::{Collection, Processor, Website};
use minijinja::Environment;
use log::info;
use crate::member::MemberMeta;
use crate::post::PostMeta;
use crate::read::{format_html_with_meta, parse_front_matter_and_fetch_contents, parse_markdown};
use crate::sitemap::SiteMap;
use crate::templates::functions::embed::{jinja_embed};
use crate::templates::functions::member::jinja_member;
use crate::templates::functions::sns::{jinja_sns_icon};
use crate::templates::index::index;
use crate::work::WorkMeta;

pub mod templates;
mod read;
mod optimize;
mod member;
mod post;
mod featured_work;
mod die_linky;
mod metadata;
mod sitemap;
mod work;

pub const FRONT_MATTER_SPLIT: &'static str = "===";
pub const RENDER_SITE: &'static str = "toudaivocadou.org";

fn main() {
    let website = Website::configure().add_collections(
        vec![
            Collection::glob_with("members", "members/*", ["md"], parse_front_matter_and_fetch_contents::<MemberMeta>),
            Collection::glob_with("posts", "posts/*", ["md"], parse_front_matter_and_fetch_contents::<PostMeta>),
            Collection::glob_with("works", "works/*", ["md"], parse_front_matter_and_fetch_contents::<WorkMeta>)
        ]
    ).add_processors(vec![
            Processor::process_images(["png", "jpg", "jpeg", "gif", "avif"]),
        ])
        .add_styles([Utf8PathBuf::from("css/style.css")])
        .add_scripts([("js", "js/script.js")])
        .set_opts_sitemap("sitemap.xml")
        .add_task(|sack| {
            info!("hi");
            // build up our sitemap and metadatas
            let members = sack.query_content::<MemberMeta>("members/*").unwrap();
            let member_name_list = members.iter().map(|x| x.meta.ascii_name.as_str()).collect::<Vec<&str>>();

            let works = sack.query_content::<WorkMeta>("works/*").unwrap();
            if !works.iter().all(|work| {
                member_name_list.contains(&work.meta.author.as_str())
            }) {
                panic!("work contains bad author.")
            }

            let news_posts = sack.query_content::<PostMeta>("posts/*").unwrap();
            if !news_posts.iter().all(|posts| {
                member_name_list.contains(&posts.meta.author.as_str())
            }) {
                panic!("post contains bad author.")
            }

            let sitemap = SiteMap {
                members: members.iter().map(|x| x.meta).cloned().collect(),
                posts: news_posts.iter().map(|x| x.meta).cloned().collect(),
                works: works.iter().map(|x| x.meta).cloned().collect(),
            };

            let sitemap_json = serde_json::to_string(&sitemap)?;

            let mut robots = String::new();
            File::open("robots.txt").unwrap().read_to_string(&mut robots).unwrap();

            // do out set pages (index and members for now)
            let mut set_pages = vec![
                ("/".to_string(), index().into_string()),
                ("/members".to_string(), templates::members::members(&sitemap).into_string()),
                ("/sitemap.json".to_string(), sitemap_json),
                ("/robots.txt".to_string(), robots),
                // ("/works",
            ];

            // environment
            let mut jinja_environment = Environment::new();
            jinja_environment.add_function("sns_link", jinja_sns_icon);
            jinja_environment.add_function("sns_embed", jinja_embed);
            jinja_environment.add_function("member", jinja_member);
            jinja_environment.add_global("SITE", RENDER_SITE);

            // render members

            let member_detail = members.into_iter().map(|member_page| {
                let html = parse_markdown(member_page.content);
                let path = format!("/members/{}", member_page.meta.ascii_name);
                format_html_with_meta(&jinja_environment, &html, member_page.meta).map(|page| (path, page))
            }).collect::<Result<Vec<(String, String)>, anyhow::Error>>()?;

            set_pages.extend(member_detail);

            // query posts
            // query works

            Ok(set_pages.into_iter().map(|(path, page)| {
                (Utf8PathBuf::from(path), page)
            }).collect())
        })
        .finish();

    struct Data;

    website.build(Data {}).unwrap()
}
