use comrak::nodes::{AstNode, NodeValue};
use comrak::{
    format_html,
    markdown_to_html,
    Arena,
    ExtensionOptions,
    Options,
    ParseOptionsBuilder,
};
use maud::{html, Markup, PreEscaped};

pub fn transform(djot: &str) -> Markup {
    html! {
        "Penis!"
    }
}

pub fn transform_markdown(markdown: &str) -> Markup {
    let mut options = Options::default();
    options.parse.smart = true;
    options.extension.table = true;
    options.extension.autolink = false;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.footnotes = true;
    options.render.unsafe_ = true;


    let html = markdown_to_html(markdown, &options);
    PreEscaped(html)
}

// fn transform_node(node: &AstNode) -> Markup {
// match node.data.borrow().value {
// NodeValue::BlockQuote => html! {
// blockquote {
// @for child in node.children() {
// (transform_node(child))
// }
// }
// },
//
// NodeValue::Document => html! {
// @for child in node.children() {
// (transform_node(child))
// }
// },
// NodeValue::FrontMatter(_) => panic!("Frontmatter should have been stripped
// out by now"), NodeValue::List(_) => {}
// NodeValue::Item(_) => {}
// NodeValue::DescriptionList => {}
// NodeValue::DescriptionItem(_) => {}
// NodeValue::DescriptionTerm => {}
// NodeValue::DescriptionDetails => {}
// NodeValue::CodeBlock(_) => {}
// NodeValue::HtmlBlock(_) => {}
// NodeValue::Paragraph => {}
// NodeValue::Heading(_) => {}
// NodeValue::ThematicBreak => {}
// NodeValue::FootnoteDefinition(_) => {}
// NodeValue::Table(_) => {}
// NodeValue::TableRow(_) => {}
// NodeValue::TableCell => {}
// NodeValue::Text(_) => {}
// NodeValue::TaskItem(_) => {}
// NodeValue::SoftBreak => {}
// NodeValue::LineBreak => {}
// NodeValue::Code(_) => {}
// NodeValue::HtmlInline(_) => {}
// NodeValue::Emph => {}
// NodeValue::Strong => {}
// NodeValue::Strikethrough => {}
// NodeValue::Superscript => {}
// NodeValue::Link(_) => {}
// NodeValue::Image(_) => {}
// NodeValue::FootnoteReference(_) => {}
// NodeValue::ShortCode(_) => {}
// }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transform_markdown() {}
}
