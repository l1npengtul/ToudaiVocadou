use crate::member::{MemberFeaturedWork, MemberMeta};
use crate::post::PostMeta;
use crate::read::{parse_and_format, parse_front_matter_and_fetch_contents};
use crate::sitemap::SiteMap;
use crate::templates::error::notfound;
use crate::templates::functions::embed::{embed, jinja_embed};
use crate::templates::functions::member::jinja_member;
use crate::templates::functions::sns::jinja_sns_icon;
use crate::templates::index::index;
use crate::templates::members::member_detail;
use crate::templates::news::post_reference;
use crate::templates::works::work_reference;
use crate::work::{DisplayWorkMeta, WorkMeta};
use camino::Utf8PathBuf;
use clap::{Parser, ValueEnum};
use hauchiwa::{Collection, Processor, Sack, Website};
use maud::Render;
use minijinja::Environment;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

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
mod util;

pub const FRONT_MATTER_SPLIT: &str = "===";
pub const RENDER_SITE: &str = "toudaivocadou.org";

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
                "[!_]*",
                ["md"],
                parse_front_matter_and_fetch_contents::<MemberMeta>,
            ),
            Collection::glob_with(
                "posts",
                "[!_]*",
                ["md"],
                parse_front_matter_and_fetch_contents::<PostMeta>,
            ),
            Collection::glob_with(
                "works",
                "[!_]*",
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

            // this is a bunch of fancy logic to make the "random work" button work.
            // what we do is that we
            // 1. see what featured works members have
            // 2. parse all works in the works/ folder
            // 3. if the featured work also has a dedicated md file... create a 詳しく見る link to the work page on the featured work.
            // 4. if the featured work does not exist ... create a "fake" card on the work. these dont have dates so they will always be set as
            // published on some random date (we are NOT reaching out to the API to get the proper date - some future poor sap can do that)
            // 5. gather all of this and put it in a JSON, which then the client side JS can consume to power the random work button.

            let works = sack.query_content::<WorkMeta>("*").unwrap();
            if !works
                .iter()
                .all(|work| member_name_list.contains(&work.meta.author.as_str()))
            {
                panic!("work contains bad author.")
            }

            let works_urls = works
                .iter()
                .map(|work| work.meta.link.clone())
                .collect::<HashSet<String>>();

            let news_posts = sack.query_content::<PostMeta>("*").unwrap();
            if !news_posts
                .iter()
                .all(|posts| member_name_list.contains(&posts.meta.author.as_str()))
            {
                panic!("post contains bad author.")
            }

            let sitemap = SiteMap {
                members: members.iter().map(|x| x.meta).cloned().collect(),
                posts: news_posts
                    .iter()
                    .map(|x| (x.meta.clone(), x.content.chars().take(100).collect()))
                    .collect(),
                works: works.iter().map(|x| x.meta).cloned().collect(),
            };

            let ascii_name_to_author = members
                .iter()
                .map(|auth_meta| {
                    (
                        auth_meta.meta.ascii_name.clone(),
                        auth_meta.meta.name.clone(),
                    )
                })
                .collect::<HashMap<String, String>>();

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
                (
                    "works.html".to_string(),
                    templates::works::works(&sack, &sitemap, &ascii_name_to_author).into_string(),
                ),
                (
                    "news.html".to_string(),
                    templates::news::news_posts(&sack, &sitemap, &ascii_name_to_author)
                        .into_string(),
                ),
                ("404.html".to_string(), notfound(&sack).into_string()),
                ("robots.txt".to_string(), robots),
            ];

            // environment
            let mut jinja_environment = Environment::new();
            jinja_environment.add_function("sns_link", jinja_sns_icon);
            jinja_environment.add_function("sns_embed", jinja_embed);
            jinja_environment.add_function("member", jinja_member);
            jinja_environment.add_global("SITE", RENDER_SITE);

            // render members

            let mut featured_works_leftovers = vec![];
            let mut works_list = vec![];

            let member_detail = members
                .into_iter()
                .map(|member_page| -> Result<(String, String), anyhow::Error> {
                    let mut member_page_meta = member_page.meta.clone();

                    member_page_meta.featured_works = member_page
                        .meta
                        .featured_works
                        .iter()
                        .map(|featured| match works_urls.get(&featured.link) {
                            Some(_) => MemberFeaturedWork {
                                title: featured.title.clone(),
                                description: featured.description.clone(),
                                link: featured.link.clone(),
                                __do_not_use_kuwasiku: true,
                            },
                            None => {
                                featured_works_leftovers
                                    .push((featured.clone(), member_page_meta.ascii_name.clone()));
                                featured.clone()
                            }
                        })
                        .collect();

                    let content_html = parse_and_format(
                        &sack,
                        &member_page_meta,
                        &jinja_environment,
                        member_page.content,
                    )?;
                    let rendered_page =
                        member_detail(&sack, &member_page_meta, &content_html).into_string();
                    let path = format!("members/{}.html", &member_page_meta.ascii_name);
                    Ok((path, rendered_page))
                })
                .collect::<Result<Vec<(String, String)>, anyhow::Error>>()?;

            let works_detail = works
                .into_iter()
                .map(|works_page| {
                    works_list.push(works_page.meta.clone());

                    let content_html = parse_and_format(
                        &sack,
                        works_page.meta,
                        &jinja_environment,
                        works_page.content,
                    )?;
                    let rendered_page = crate::templates::works::work_detail(
                        &sack,
                        works_page.meta,
                        &ascii_name_to_author,
                        &content_html,
                    )
                    .into_string();
                    let path = format!("works/{}.html", work_reference(&works_page.meta.link));
                    Ok((path, rendered_page))
                })
                .collect::<Result<Vec<(String, String)>, anyhow::Error>>()?;

            let posts_detail = news_posts
                .into_iter()
                .map(|news_page| {
                    let content_html = parse_and_format(
                        &sack,
                        news_page.meta,
                        &jinja_environment,
                        news_page.content,
                    )?;
                    let rendered_page = crate::templates::news::post_detail(
                        &sack,
                        news_page.meta,
                        &content_html,
                        &ascii_name_to_author,
                    )
                    .into_string();
                    let path = format!("news/{}.html", post_reference(news_page.meta));
                    Ok((path, rendered_page))
                })
                .collect::<Result<Vec<(String, String)>, anyhow::Error>>()?;

            set_pages.extend(member_detail);
            set_pages.extend(works_detail);
            set_pages.extend(posts_detail);

            // generate the work list

            let mut display_works = works_list
                .into_iter()
                .map(|w| DisplayWorkMeta {
                    title: w.title,
                    description: w.short,
                    on_site_link: work_reference(&w.link),
                    embed_html: embed(&w.link).render().into_string(),
                    author_link: ascii_name_to_author.get(&w.author).unwrap().clone(),
                    author_displayname: w.author,
                })
                .collect::<Vec<DisplayWorkMeta>>();

            display_works.extend(
                featured_works_leftovers
                    .into_iter()
                    .map(|(featured, author)| DisplayWorkMeta {
                        title: featured.title,
                        description: featured.description,
                        on_site_link: work_reference(&featured.link),
                        author_link: ascii_name_to_author.get(&author).unwrap().clone(),
                        author_displayname: author,
                        embed_html: embed(&featured.link).render().into_string(),
                    }),
            );

            let works_json =
                serde_json::to_string(&display_works).expect("Failed to serialize works list");

            set_pages.push(("works_list.json".to_string(), works_json));

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
