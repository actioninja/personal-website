use maud::{html, Markup};

use crate::blog::Page;

pub fn blog_page(page: &Page) -> Markup {
    let transformed = crate::djot::transform(&page.content);
    html! {
        h1 {
            (page.frontmatter.title)
        }
        p {
            /*(page.frontmatter.date)*/
        }
        p {
            @for tag in page.frontmatter.tags.iter() {
                (tag)
                " "
            }
        }
        article {
            (transformed)
        }
    }
}
