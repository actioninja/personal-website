use std::fs;

use chrono::Utc;
use rss::{ChannelBuilder, Item};

use crate::blog::BLOG_PAGES;
use crate::djot::transform_feed;

pub fn generate_rss_xml() {
    let current_time = Utc::now();
    let items: Vec<Item> = BLOG_PAGES
        .iter()
        .filter(|page| !page.frontmatter.draft)
        .take(15)
        .map(|page| {
            Item {
                title: Some(page.frontmatter.title.clone()),
                description: Some(page.frontmatter.subheading.clone()),
                link: Some(format!("https://criticalaction.net/blog/{}", page.slug)),
                pub_date: Some(page.frontmatter.date.to_rfc2822()),
                content: Some(
                    transform_feed(&page.content, &page.slug, &page.frontmatter).into_string(),
                ),
                ..Default::default()
            }
        })
        .collect();
    let channel = ChannelBuilder::default()
        .title("Critical Action")
        .link("https://criticalaction.net/blog")
        .description("A tech, electronics, music, whatever blog.")
        .language(Some("en-us".to_string()))
        .last_build_date(Some(current_time.to_rfc2822()))
        .items(items)
        .build();


    let result = channel.to_string();
    fs::write("public/blog/blog.rss", result).unwrap();
}
