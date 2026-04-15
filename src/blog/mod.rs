use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use chrono::{DateTime, FixedOffset};
use gray_matter::engine::{TOML, YAML};
use gray_matter::Matter;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::blog::index::{accumulate_categories, accumulate_tags, tag_page};
use crate::blog::rss::generate_rss_xml;
use crate::layout::wrap;

pub mod index;
mod page;
mod rss;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frontmatter {
    pub title: String,
    pub subheading: String,
    pub date: DateTime<FixedOffset>,
    pub last_mod: Option<DateTime<FixedOffset>>,
    pub category: String,
    pub tags: Vec<String>,
    pub draft: bool,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub slug: String,
    pub frontmatter: Frontmatter,
    pub content: String,
}

pub static BLOG_PAGES: Lazy<Vec<Page>> = Lazy::new(load_blog_pages);
pub static TAGS: Lazy<BTreeMap<String, u32>> = Lazy::new(|| {
    let pages: Vec<_> = BLOG_PAGES.iter().filter(|p| !p.frontmatter.draft).collect();
    accumulate_tags(&pages)
});
pub static CATEGORIES: Lazy<BTreeMap<String, u32>> = Lazy::new(|| {
    let pages: Vec<_> = BLOG_PAGES.iter().filter(|p| !p.frontmatter.draft).collect();
    accumulate_categories(&pages)
});

const INVALID_SLUGS: [&str; 3] = ["tag", "category", "rss.xml"];

pub fn load_blog_pages() -> Vec<Page> {
    let path = Path::new("assets/pages/blog");
    let files = path.read_dir().unwrap();
    // use the filenames to create a vector of (filename, file contents) tuples
    let mut pages: Vec<Page> = Vec::new();
    for file in files {
        let file = file.unwrap();
        let filename = file.file_name().into_string().unwrap().replace(".dj", "");
        if INVALID_SLUGS.contains(&filename.as_str()) {
            panic!("You used an invalid name for the blog slug ya goober ({INVALID_SLUGS:?})");
        }
        let contents = fs::read_to_string(file.path()).unwrap();
        let matter = Matter::<YAML>::new();
        let raw_parsed = matter.parse(&contents);
        let frontmatter: Frontmatter = raw_parsed.data.unwrap().deserialize().unwrap();
        dbg!(&frontmatter);
        pages.push(Page {
            slug: filename,
            frontmatter,
            content: raw_parsed.content,
        });
    }
    pages.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
    pages
}

pub fn generate_blog_pages() {
    fs::create_dir_all("public/blog").unwrap();

    for page in BLOG_PAGES.iter() {
        fs::create_dir_all(format!("public/blog/{}", page.slug)).unwrap();
        fs::write(
            format!("public/blog/{}/index.html", page.slug),
            wrap(
                &format!("{} - Critical Action", page.frontmatter.title),
                page::blog_page(page),
            )
            .into_string(),
        )
        .expect("Could not write blog page");
    }
    for (tag, _) in TAGS.iter() {
        let filtered_pages: Vec<_> = BLOG_PAGES
            .iter()
            .filter(|page| page.frontmatter.tags.contains(&tag.to_string()))
            .collect();
        fs::create_dir_all(format!("public/blog/tag/{tag}")).unwrap();
        fs::write(
            format!("public/blog/tag/{tag}/index.html"),
            wrap(
                &format!("{} - Critical Action", tag),
                tag_page(tag, &filtered_pages),
            )
            .into_string(),
        )
        .unwrap()
    }
    generate_rss_xml();
}
