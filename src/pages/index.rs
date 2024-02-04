use maud::{html, Markup};
use once_cell::sync::Lazy;

pub struct Contact {
    pub name: &'static str,
    pub link: &'static str,
    pub display: &'static str,
}

pub static CONTACTS: Lazy<Vec<Contact>> = Lazy::new(|| {
    vec![Contact {
        name: "Mastodon",
        link: "https://hachyderm.io/@actioninja",
        display: "@actioninja@hachyderm.io",
    }]
});

pub fn index() -> Markup {
    html! {
        section {

            "I primarily make software for the sake of it. I take my craft seriously and believe that software should be made with care and attention to detail."
        }

        h2 {
            "Contacts"
        }
        section {
            "Here are some ways you can get in touch with me. They are (roughly) in order of how likely I will respond to them."
            ul {
                @for contact in CONTACTS.iter() {
                    li {
                        (contact.name)
                        " - "
                        a rel="me" href=(contact.link) {
                            (contact.display)
                        }
                    }
                }
                li {
                    .email-tricky {
                        "Or you can just email me at actioninja@criticalaction"
                        b {
                            ".fuckyouspammers"
                        }
                        ".net"
                    }
                }
            }
        }
    }
}
