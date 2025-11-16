use crate::album::AlbumMeta;
use crate::member::MemberMeta;
use crate::post::PostMeta;
use crate::read::{parse_and_format, parse_front_matter_and_fetch_contents, parse_work_meta};
use crate::sitemap::SiteMap;
use crate::templates::error::notfound;
use crate::templates::functions::embed::{embed, jinja_embed};
use crate::templates::functions::member::jinja_member;
use crate::templates::functions::sns::jinja_sns_icon;
use crate::templates::index::index;
use crate::templates::members::member_detail;
use crate::templates::news::post_reference;
use crate::templates::works::{thumbnail, work_reference};
use crate::util::SvgData;
use crate::work::{DisplayWorkMeta, RawWorkMeta, WorkMeta};
use anyhow::Error;
use camino::Utf8PathBuf;
use clap::{Parser, ValueEnum};
use hauchiwa::RuntimeError;
use hauchiwa::loader::Image;
use hauchiwa::{Collection, Context, Page, Processor, Sack, Website, loader};
use log::{error, info, warn};
use maud::{Markup, Render};
use minijinja::Environment;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::sync::OnceLock;
use url::Url;

mod album;
mod die_linky;
mod featured_work;
mod member;
mod metadata;
mod optimize;
mod post;
mod read;
mod sitemap;
pub mod templates;
mod util;
mod work;

pub const FRONT_MATTER_SPLIT: &str = "===";

pub fn lnk(url: impl AsRef<str>) -> String {
    static SITEROOT: OnceLock<String> = OnceLock::new();
    let root = SITEROOT
        .get_or_init(|| std::env::var("SITEROOT").unwrap_or("toudaivocadou.org".to_string()));

    slash_guard(root, url.as_ref())
}

pub fn lnk_s3(url: impl AsRef<str>) -> String {
    static EXTERNALROOT: OnceLock<String> = OnceLock::new();

    let url = url.as_ref();

    if let Ok(u) = Url::parse(url) {
        println!("Warning: Passed link to lnk that is already a url");
        return u.to_string();
    }

    let root = EXTERNALROOT.get_or_init(|| {
        std::env::var("EXTERNALROOT").unwrap_or("miku.toudaivocadou.org".to_string())
    });

    slash_guard(root, url)
}

fn slash_guard(root: &str, thing: &str) -> String {
    if thing.starts_with("/") {
        format!("{root}{}", thing)
    } else {
        format!("{root}/{}", thing)
    }
}

fn dataroot() -> &'static str {
    static DATAROOT: OnceLock<String> = OnceLock::new();

    DATAROOT.get_or_init(|| std::env::var("DATAROOT").unwrap_or(".".to_string()))
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuildData {
    pub name_map: HashMap<String, String>,
}

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

#[derive(Copy, Clone, Debug)]
pub struct SiteData {
    // pub artist_name_map: HashMap<String, String>,
    pub build_id: u64,
}

pub fn build_site(build_id: u64) {
    let site_data = SiteData { build_id };
    let website = Website::config()
        .add_loaders([
            // load site content
            loader::glob_content(
                dataroot(),
                "members/[!_]*.md",
                parse_front_matter_and_fetch_contents::<MemberMeta>,
            ),
            loader::glob_content(
                dataroot(),
                "posts/[!_]*.md",
                parse_front_matter_and_fetch_contents::<PostMeta>,
            ),
            loader::glob_content(dataroot(), "works/[!_]*.md", parse_work_meta),
            loader::glob_content(
                dataroot(),
                "album/[!_]*.md",
                parse_front_matter_and_fetch_contents::<AlbumMeta>,
            ),
            // load CSS
            loader::glob_styles(dataroot(), "styles/[!_]*.css"),
            // load JS
            loader::glob_scripts(dataroot(), "js/[!_]*.js"),
            // load images
            loader::glob_images(dataroot(), "images/**/*.jpg"),
            loader::glob_images(dataroot(), "images/**/*.png"),
            loader::glob_images(dataroot(), "images/**/*.gif"),
            loader::glob_images(dataroot(), "images/**/*.avif"),
            // SVG assets require special treatment, we dont want processing
            loader::glob_assets(dataroot(), "assets/*.svg", |rt, data| {
                let path = rt.store(&data, "svg")?;
                Ok(SvgData { path, data })
            }),
            loader::glob_assets(dataroot(), "audio/**/*.ogg", |rt, data| {})
        ])
        .add_task("STATIC: build robots", |ctx| {
            let robots = robots_txt()?;
            Ok(vec![robots])
        })
        .add_task("STATIC: build index", |ctx| {
            let index = markup_to_page("index.html", index(&ctx)?);
            Ok(vec![index])
        })
        .add_task("STATIC: build 404", |ctx| {
            let notfound = markup_to_page("404.html", notfound(&ctx)?);
            Ok(vec![notfound])
        })
        .add_task("DYNAMIC: build all dynamic content", |ctx| {
            info!(
                "BUILD-{}: Starting dynamic content build",
                ctx.get_globals().data.build_id
            );
            let members = ctx.glob_with_file::<MemberMeta>("*")?;
            // construct name map
            let member_ascii_to_name = members
                .iter()
                .map(|member_with_file| member_with_file.data)
                .map(|member_meta| (member_meta.ascii_name.clone(), member_meta.name.clone()))
                .collect::<HashMap<String, String>>();

            let works = ctx.glob_with_file::<WorkMeta>("*")?;
            info!(
                "BUILD-{}: Ensuring all names exist in works.",
                ctx.get_globals().data.build_id
            );
            for work in works.iter() {
                let file_path = work.file.file;
                let work_meta = work.data;
                if !member_ascii_to_name.contains_key(&work_meta.author) {
                    let error_str = format!("BUILD-{}: ファイル {}の内, メタデータフィルド`author`でエーラ発生: {} はメンバー中見つかりませんでした。 英語ネーム使うかどうか確認してください。", ctx.get_globals().data.build_id, file_path, &work_meta.author);
                    error!("{}", &error_str);
                    return Err(RuntimeError::msg(error_str))
                }
                for collaborator in &work_meta.collaborators {
                    if !member_ascii_to_name.contains_key(collaborator) {
                        let error_str = format!("BUILD-{}: ファイル {}の内, メタデータフィルド`collaborators`でエーラ発生: {} はメンバー中見つかりませんでした。 英語ネーム使うかどうか確認してください。投稿者が東大ボカロP同好会のメンバーじゃないければ、`extra_collaborators`で入れてください。", ctx.get_globals().data.build_id, file_path, &collaborator);
                    error!("{}", &error_str);
                        return Err(RuntimeError::msg(error_str))
                    }
                }
            }

            let albums = ctx.glob_with_file::<AlbumMeta>("*")?;
            for album in &albums {
                let file_path = album.file.file;
                let album_meta = album.data;
                for contributor in &album_meta.contributors {
                    if !member_ascii_to_name.contains_key(contributor) {
                        let error_str = format!("BUILD-{}: ファイル {}の内, メタデータフィルド`contributors`でエーラ発生: {} はメンバー中見つかりませんでした。 英語ネーム使うかどうか確認してください。投稿者が東大ボカロP同好会のメンバーじゃないければ、`extra_contributors`で入れてください。", ctx.get_globals().data.build_id, file_path, &contributor);
                    error!("{}", &error_str);
                        return Err(RuntimeError::msg(error_str))
                    }
                }

                // ensure work exists
                

            }

            let news = ctx.glob_with_file::<PostMeta>("*")?;
            for post in &news {
                let file_path = post.file.file;
                let post_meta = post.data;

                if !member_ascii_to_name.contains_key(&post_meta.author) {
                        let error_str = format!("BUILD-{}: ファイル {}の内, メタデータフィルド`author`でエーラ発生: {} はメンバー中見つかりませんでした。 英語ネーム使うかどうか確認してください。", ctx.get_globals().data.build_id, file_path, &post_meta.author);
                    error!("{}", &error_str);
                        return Err(RuntimeError::msg(error_str))
                }
            }

            let 

            info!(
                "BUILD-{}: Ensuring all works exist in naes.",
                ctx.get_globals().data.build_id
            );

            Ok(vec![])
        });
}

fn main() {
    let args = Args::parse();
    let script = format!("{}/js/script.js", dataroot());

    let website = Website::config()
        .add_collections(vec![
            Collection::glob_with(dataroot(), "members/[!_]*", ["md"]),
            Collection::glob_with(
                dataroot(),
                "posts/[!_]*",
                ["md"],
                parse_front_matter_and_fetch_contents::<PostMeta>,
            ),
            Collection::glob_with(
                dataroot(),
                "works/[!_]*",
                ["md"],
                parse_front_matter_and_fetch_contents::<WorkMeta>,
            ),
            Collection::glob_with(
                dataroot(),
                "albums/[!_]*",
                ["md"],
                parse_front_matter_and_fetch_contents::<AlbumMeta>,
            ),
        ])
        .add_processors(vec![Processor::process_images([
            "png", "jpg", "jpeg", "gif", "avif",
        ])])
        .add_styles([Utf8PathBuf::from(&format!("{}/css", dataroot()))])
        .add_scripts([("script", script.as_str())])
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
            if !works.iter().all(|work| {
                member_name_list.contains(&work.meta.author.as_str())
                    && work
                        .meta
                        .collaborators
                        .iter()
                        .all(|collaborator| member_name_list.contains(&collaborator.as_str()))
            }) {
                panic!("work contains bad author.")
            }

            let albums = sack.query_content::<AlbumMeta>("*").unwrap();
            if !albums.iter().all(|album| {
                album
                    .meta
                    .contributors
                    .iter()
                    .all(|collaborator| member_name_list.contains(&collaborator.as_str()))
            }) {
                panic!("album contains bad contributor.")
            }

            // let works_urls = works
            //     .iter()
            //     .filter(|work| Option::is_some)
            //     .map(|work| work.meta.link.clone())
            //     .collect::<HashSet<String>>();

            let featured_works = works
                .iter()
                .filter(|work| work.meta.featured == true)
                .map(|work| work.meta)
                .collect::<Vec<&WorkMeta>>();

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
                albums: albums.iter().map(|x| x.meta).cloned().collect(),
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
            jinja_environment.add_global("SITE", lnk(""));

            // render members

            let mut works_list = vec![];

            let member_detail = members
                .into_iter()
                .map(|member_page| -> Result<(String, String), anyhow::Error> {
                    let content_html = parse_and_format(
                        &sack,
                        member_page.meta,
                        &jinja_environment,
                        member_page.content,
                    )?;
                    let rendered_page =
                        member_detail(&sack, member_page.meta, &featured_works, &content_html)
                            .into_string();
                    let path = lnk(format!("members/{}.html", &member_page.meta.ascii_name));
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
                    let path = format!(
                        "works/{}.html",
                        work_reference(&works_page.meta.title, &works_page.meta.author)
                    );
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
                    let path = lnk(format!("news/{}.html", post_reference(news_page.meta)));
                    Ok((path, rendered_page))
                })
                .collect::<Result<Vec<(String, String)>, anyhow::Error>>()?;

            set_pages.extend(member_detail);
            set_pages.extend(works_detail);
            set_pages.extend(posts_detail);

            // generate the work list

            let display_works = works_list
                .into_iter()
                .enumerate()
                .map(|(idx, works)| DisplayWorkMeta {
                    id: idx as i32,
                    on_site_link: work_reference(&works.title, &works.author),
                    embed_html: embed(&thumbnail(&sack, &works)).render().into_string(),
                    title: works.title,
                    description: works.short,
                    author_link: ascii_name_to_author.get(&works.author).unwrap().clone(),
                    author_displayname: works.author,
                    collaborators: works.collaborators,
                    remix_original_work: works.remix_original_work,
                })
                .collect::<Vec<DisplayWorkMeta>>();

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
        Mode::Build => website.build(SiteData {}).unwrap(),
        Mode::Watch => website.watch(SiteData {}).unwrap(),
    }
}

pub fn image(sack: &Context<SiteData>, path: impl AsRef<str>) -> Result<String, RuntimeError> {
    let path = path.as_ref();
    if path.starts_with("miku:") {
        return Ok(lnk_s3(path));
    }

    let picture_path = Utf8PathBuf::from(path);
    let image = sack.get::<Image>(&picture_path)?;
    Ok(lnk(image.path.to_string()))
}

pub struct AudioFile {}

pub fn audio(sack: &Context<SiteData>, path: impl AsRef<str>) -> Result<String, RuntimeError> {}

fn robots_txt() -> Result<Page, RuntimeError> {
    let mut robots = String::new();
    File::open("robots.txt")
        .unwrap()
        .read_to_string(&mut robots)
        .unwrap();
    Ok(Page::text(
        camino::Utf8PathBuf::from_str("robots.txt")?,
        robots,
    ))
}

fn markup_to_page(path: impl AsRef<str>, markup: Markup) -> Page {
    Page::html(lnk(path), &markup.0)
}
