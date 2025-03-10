mod transform_headers;

use jotdown::Render;
use maud::{Markup, PreEscaped};

use crate::djot::transform_headers::TransformHeaders;

pub fn transform(djot: &str) -> Markup {
    let events = jotdown::Parser::new(djot);
    let transformed = TransformHeaders::new(events);
    let mut html = String::new();

    jotdown::html::Renderer::default()
        .push(transformed, &mut html)
        .expect("Failed to render");
    PreEscaped(html)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transform_markdown() {}
}
