use maud::{html, Markup};

#[derive(Debug, Clone, Copy)]
struct CoolThing {
    pub name: &'static str,
    pub description: &'static str,
    pub link: &'static str,
}

const COOL_STUFF: [CoolThing; 12] = [
    CoolThing {
        name: "Warp Door",
        description: "A blog where various strange and small games from around the internet are \
                      showcased daily. Extremely fun showcase of the weird side of game dev, \
                      makes an excellent substitute for Glorious Trainwrecks nowadays.",
        link: "https://warpdoor.com/",
    },
    CoolThing {
        name: "Glorious Trainwrecks",
        description: "Used to be a community and forum of amateur, chaotic, fast paced game \
                      development with a strong punk ethos of \"who cares if it's any good, at \
                      least you made it.\" Nowadays has unfortunately declined in popularity a \
                      lot and the forums are mostly dead.",
        link: "https://glorioustrainwrecks.com/",
    },
    CoolThing {
        name: "Lix",
        description: "Hard fork of cppnix, the actual runtime part of Nix. Formed due to the many \
                      political and ideological issues with the original project, but now \
                      starting to make serious and significant efforts to actually fix serious \
                      technical problems with Nix.",
        link: "https://lix.systems/",
    },
    CoolThing {
        name: "Auxolotl",
        description: "An attempt to \"start over\" on nixpkgs and create an alternative ecosystem \
                      which is designed better from the start instead of the more organic, \
                      directionless, and messy way that nixpkgs formed. Slow going but very \
                      promising.",
        link: "https://auxolotl.org/",
    },
    CoolThing {
        name: "lobsters",
        description: "Tech-focused link aggregation website. Good alternative to vile orange site.",
        link: "https://lobste.rs/",
    },
    CoolThing {
        name: "Without boats, dreams dry up",
        description: "Quality blog about a lot of technical stuff related to Rust, primarily \
                      about async/await.",
        link: "https://without.boats/",
    },
    CoolThing {
        name: "Ted Kaminski's Blog",
        description: "Easily some of the strongest resources on software design I've ever seen. I \
                      highly recommend reading everything on this blog, even if his project to \
                      eventually write a book about it seems to have stalled out.",
        link: "https://www.tedinski.com/",
    },
    CoolThing {
        name: "Gemini Protocol",
        description: "A new internet protocol that is a spiritual successor to Gopher. A \
                      significantly more minimal alternative to HTTP focused around minimally \
                      formatted text.",
        link: "https://geminiprotocol.net/",
    },
    CoolThing {
        name: "The Rust Programming Language",
        description: "Niche underground programming language that cool edgy people use. Ignore \
                      all those people who \"work\" at \"Microsoft\" or \"Mozilla\" that use it, \
                      they're a myth.",
        link: "https://www.rust-lang.org/",
    },
    CoolThing {
        name: "moth.fans",
        description: "Cool and epic website that is very cool and epic for cool and epic moth \
                      fans.",
        link: "https://moth.fans/",
    },
    CoolThing {
        name: "maia arson crimew",
        description: "That one hacktivist that leaked the no fly list. Does a lot of neat \
                      blogging about tech and weird shit going in with the endless hole that is \
                      the internet. Actually a for real adorable kitten irl.",
        link: "https://maia.crimew.gay/",
    },
    CoolThing {
        name: "4d2.org",
        description: "Free, public hosted instances of various federated and decentralized \
                      services",
        link: "https://4d2.org/",
    },
];

fn cool_stuff_entry(thing: &CoolThing) -> Markup {
    html! {
        li {
            a href=(thing.link) {
                (thing.name)
            }
            " - "
            span {
                (thing.description)
            }
        }
    }
}

pub fn cool_stuff() -> Markup {
    html! {
        h1 {
            "Cool Stuff"
        }
        p {
            "This is a page for cool stuff I like around the internet. Not Particularly organized in any way."
        }
        ul {
            @for thing in COOL_STUFF.iter() {
                (cool_stuff_entry(thing))
            }
        }
    }
}
