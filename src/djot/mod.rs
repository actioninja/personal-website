mod transform_headers;
mod transform_sidenotes;

use jotdown::Render;
use maud::{html, Markup, PreEscaped};

use crate::blog::Frontmatter;
use crate::djot::transform_headers::TransformHeaders;
use crate::djot::transform_sidenotes::TransformSidenotes;

pub fn transform(djot: &str) -> Markup {
    let events = jotdown::Parser::new(djot);
    let transformed = TransformHeaders::new(events);
    let transformed = TransformSidenotes::new(transformed);
    let mut html = String::new();

    jotdown::html::Renderer::default()
        .push(transformed, &mut html)
        .expect("Failed to render");
    PreEscaped(html)
}

pub fn transform_feed(djot: &str, slug: &str, frontmatter: &Frontmatter) -> Markup {
    let feed_notice = html! {
       "Hello! I make a lot of effort to ensure maximum readability with minimum weight on my blog. If you prefer to read via your feed reader, that's fine, but the experience is better "
        a href=(format!("https://criticalaction.net/blog/${slug}")) {
            "on the original site."
        }
    };

    let events = jotdown::Parser::new(djot);
    let transformed = TransformHeaders::new(events);
    let mut html = String::new();

    jotdown::html::Renderer::default()
        .push(transformed, &mut html)
        .expect("Failed to render");

    let result = PreEscaped(html);
    html! {
        h1 {
            (frontmatter.title)
        }
        (feed_notice)
        (result)
    }
}


#[cfg(test)]
mod test {
    use jotdown::Parser;

    use super::*;

    #[test]
    fn test_transform_markdown() {
        let test_str = r#"`<div>test verbatim</div>`{=html}"#;
        let test_parser = Parser::new(test_str);
        for event in test_parser.into_iter() {
            println!("{:?}", event);
        }
    }
}
