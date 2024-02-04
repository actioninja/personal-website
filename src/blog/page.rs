use maud::{html, Markup};

use crate::blog::Page;

pub fn blog_page(page: &Page) -> Markup {
    let transformed = crate::djot::transform(&page.content);
    html! {
        header {
            h1 {
                (page.frontmatter.title)
            }
            em {
                (page.frontmatter.subheading)
            }
            div {
                "Originally Posted: "
                (page.frontmatter.date)
            }
            @if let Some(last_mod) = page.frontmatter.last_mod {
                div {
                    "Last Revised: "
                    (last_mod)
                }
            }
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
