use maud::{html, Markup, DOCTYPE};

pub fn header_info(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            meta name="description" content="This";
            meta name="author" content="Rob Bailey (actioninja)";
            meta name="generator" content="FARTT";
            meta name="robots" content="index, follow";
            link rel="stylesheet" href="/style.css";
            title { (page_title) }
        }
    }
}

pub fn navbar() -> Markup {
    html! {
        nav {
            ul {
                li {
                    a href="/" {
                        "Critical Action"
                    }
                }
                li {
                    a href="/blog" {
                        "Blog"
                    }
                }
                li {
                    a href="/projects" {
                        "Projects"
                    }
                }
                li {
                    a href="/cool-stuff" {
                        "Cool Stuff"
                    }
                }
            }
        }
    }
}

pub fn footer() -> Markup {
    html! {
        footer {
            section {
                "Copyright 2025 Rob Bailey. All source code for this website is licensed under the terms of "
                a href="https://www.mozilla.org/en-US/MPL/2.0/" {
                    "MPL 2.0"
                }
                ", or the applicable license for used libraries. All writing and content on this site is the sole ownership of Rob Bailey, unless otherwise indicated, and may be used licensed under "
                a href="https://creativecommons.org/licenses/by-sa/4.0/" {
                    "CC-BY-SA 4.0"
                }
                ". Any and all opinions are my own and not representative of my employers; future, past, or present."
            }
        }
    }
}

pub fn wrap(page_title: &str, content: Markup) -> Markup {
    html! {
        (header_info(page_title))
        body {
            (navbar())
            main {
                (content)
            }
            (footer())
        }
    }
}
