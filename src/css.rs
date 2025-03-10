use lightningcss::printer::PrinterOptions;
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, StyleSheet};

pub fn optimize_css(in_str: &str) -> String {
    let options = ParserOptions::default();
    let mut stylesheet = StyleSheet::parse(in_str, options).expect("Failed to parse css");

    let minify_options = MinifyOptions::default();
    stylesheet.minify(minify_options).expect("Failed to minify");

    let printer_options = PrinterOptions {
        minify: true,
        ..PrinterOptions::default()
    };
    stylesheet.to_css(printer_options).unwrap().code
}
