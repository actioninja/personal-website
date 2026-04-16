use std::collections::{BTreeMap, HashMap};

use chrono::Datelike;
use maud::{html, Markup};

use crate::blog::Page;

pub fn accumulate_tags(pages: &[&Page]) -> BTreeMap<String, u32> {
    let mut tags: BTreeMap<String, u32> = BTreeMap::new();
    for page in pages {
        for tag in page.frontmatter.tags.iter() {
            let tag = tag.to_lowercase();
            let count = tags.entry(tag.to_string()).or_insert(0);
            *count += 1;
        }
    }
    tags
}

pub fn accumulate_categories(pages: &[&Page]) -> BTreeMap<String, u32> {
    let mut categories: BTreeMap<String, u32> = BTreeMap::new();
    for page in pages {
        let category = page.frontmatter.category.to_lowercase();
        let count = categories.entry(category).or_insert(0);
        *count += 1;
    }
    categories
}

// this is nasty and I hate it
fn generate_tags(tags: &BTreeMap<String, u32>, path: &'static str) -> Markup {
    let mut out = vec![];
    for (key, count) in tags.iter() {
        out.push(html! {
            a href=(format!("/blog/{path}/{}", key)) {
                (key)
                " ("
                (count)
                ")"
            }
        });
        out.push(html! {
            ", "
        });
    }
    out.pop();
    html! {
        @for markup in out {
            (markup)
        }
    }
}


fn entries(pages: &[&Page]) -> Markup {
    html! {
        ul {
            @for page in pages {
                li {
                    (page.frontmatter.date.date_naive().format("%Y-%m-%d"))
                    " - "
                    b {
                        (page.frontmatter.category)
                    }
                    " - "
                    a href=(format!("/blog/{}", page.slug)) {
                        (page.frontmatter.title)
                    }
                }
            }
        }
    }
}

pub fn blog_index() -> Markup {
    let mut pages: Vec<_> = crate::blog::BLOG_PAGES
        .iter()
        .filter(|p| !p.frontmatter.draft)
        .collect();
    pages.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
    let tags = accumulate_tags(&pages);
    let categories = accumulate_categories(&pages);
    html! {
        h1 {
            "Blog"
        }
        "Here's where I share information and opinions. The information useful, the opinions well informed, hopefully."
        br {}
        "You can find an RSS Feed "
        a href="/blog/blog.rss" {
            "here."
        }
        h2 {
            "Tags"
        }
        (generate_tags(&tags, "tag"))
        h2 {
            "Entries"
        }
        (entries(&pages))
    }
}

pub fn category_page() -> Markup {
    html! {
        p {

        }
        h2 {
            "Entries"
        }
    }
}

pub fn tag_page(tag: &str, pages: &[&Page]) -> Markup {
    let filtered_pages: Vec<_> = pages
        .iter()
        .filter(|page| page.frontmatter.tags.contains(&tag.to_string()))
        .copied()
        .collect();
    html! {
        "Here's all entries on my blog using the \""
        (tag)
        "\" tag."
        p {
            a href="/blog" {
                "Go back"
            }
        }
        h2 {
            "Entries"
        }
        (entries(&filtered_pages))
    }
}
