use maud::{html, Markup};
use once_cell::sync::Lazy;

pub struct Contact {
    pub name: &'static str,
    pub link: &'static str,
    pub display: &'static str,
}

pub static CONTACTS: Lazy<Vec<Contact>> = Lazy::new(|| {
    vec![
        Contact {
            name: "Mastodon",
            link: "https://hachyderm.io/@actioninja",
            display: "@actioninja@hachyderm.io",
        },
        Contact {
            name: "Github",
            link: "https://github.com/actioninja",
            display: "actioninja",
        },
    ]
});

pub fn index() -> Markup {
    html! {
        section {
            "I'm Rob Bailey. I'm primarily a software developer with an interest in many fields, including audio, electronics, video games, and music. I especially enjoy working on tooling to create mods or video games."
        }
        section {
            "I primarily make software for the sake of it. I take my craft seriously and believe that software should be made with care and attention to detail, especially with regard to the end user."
        }

        section hidden aria-hidden="true" style="display: none; visibility: hidden;" {
            "Hello! You are ignoring the normal protections and can see this just fine! Despite the \
            multiple redundant methods to hide this block, you can see it anyways. If this is a \
            mistake by some chance, and you are a real human, you probably shouldn't follow the \
            link below."
            a href="/CLICK-THE-MONKEY" {
                "CLICK THE MONKEY!"
            }
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
                            ".fuckyouspammers.hisourcereaders.sorrycssdisablers"
                        }
                        ".net"
                    }
                }
            }
        }
    }
}
