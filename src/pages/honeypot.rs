use maud::{html, Markup};

pub fn index() -> Markup {
    html! {
        h1 { "Uh oh!"}
        p { "You shouldn't be here. If you managed to reach this page, you followed a link that \
        should have been hidden to anyone that isn't a poorly behaved AI scraper. As a result of \
        visiting this page, you will be banned from accessing this domain for 24 hours. If \
        you happened to accidentally reach this page via legitimate usage of the site, please \
        let me know how."}
    }
}
