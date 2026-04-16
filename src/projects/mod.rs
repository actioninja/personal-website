use maud::{html, Markup};
use once_cell::sync::Lazy;

pub struct Project {
    pub name: &'static str,
    pub link: &'static str,
    pub description: &'static str,
}

pub static PROJECTS: Lazy<Vec<Project>> = Lazy::new(|| {
    vec![
        Project {
            name: "/tg/station 13",
            link: "https://github.com/tgstation/tgstation",
            description: "Multiplayer spessman roleplay sandbox on the ancient BYOND engine. I \
                          was a maintainer for several years, and rewrote many systems and lead \
                          many large projects. My biggest contributions included the rewrite of \
                          the UI system to the current React-based tgui system, and creating the \
                          hypnagogic asset preprocessor.",
        },
        Project {
            name: "hypnagogic",
            link: "https://github.com/actioninja/hypnagogic",
            description: "An asset preprocesor to fill in that byond has no built in asset \
                          preprocessing pipeline. Extremely lame wordplay name, hypnagogic refers \
                          to things relating to the time before sleep. Uses toml based metadata \
                          files to automatically generate assets based on the metadata and a \
                          source asset. Originally made for a large project that fell through the \
                          cracks, but ended up becoming incorporated into the main project and \
                          pipeline just for being so useful. Now has several hundreds of \
                          thousands of downloads, probably from CI runs.",
        },
        Project {
            name: "byond_fn",
            link: "https://github.com/Cyberboss/byond_fn",
            description: "A crate I wrote with the intention of having an easy all in one macro \
                          to define Byond FFI functions in rust. I got mostly done with it, and \
                          planned to support the \"v2\" FFI, but got so frustrated with how \
                          poorly implemented the early v2 FFI was, and decided to stop working on \
                          it and handed it to a friend of mine.",
        },
        Project {
            name: "tgui",
            link: "https://www.npmjs.com/package/tgui-core",
            description: "A react-based GUI library for SS13 user interfaces. I wrote a decent \
                          degree of early internal work on this, and a huge amount of UIs written \
                          in it. The library eventually got pulled from the /tg/station 13 \
                          monorepo into its own standalone npm project, so unfortunately now my \
                          contribution history has been eated, so you're just gonna have to trust \
                          me bro on that I made a big chunk of this. It's been a while, so a lot \
                          of what I did has probably been ship of theseused away.",
        },
        Project {
            name: "refpack-rs",
            link: "https://github.com/actioninja/refpack-rs/",
            description: "A rust crate I wrote that implements a compression format used by many \
                          early 2000s EA Games. Written entirely in safe rust with very high \
                          performance, it also unintentionally serves as a reference \
                          implementation of the format because I very obsessively documented the \
                          format while implementing it. Interesting format, dates all the way \
                          back to Origin Software.",
        },
        Project {
            name: "Tragically abandoned The Sims 2 Modding Tools",
            link: "https://github.com/actioninja/s2",
            description: "More of a skeleton than a finished product, but I wanted to make a new \
                          editor suite for The Sims 2 because the existing SimPE is pretty clunky \
                          and slow. I wrote refpack with the intention of using it for this, but \
                          then got sidetracked by other projects before getting past the \
                          'defining a bunch of data types and parsing' stages.",
        },
        Project {
            name: "speex-sys",
            link: "https://github.com/actioninja/speex-sys",
            description: "A rust binding to the speex library. I wrote this because I was reverse \
                          engineering some The Sims 2 data formats at the time, and the one that \
                          I was working on utilized Speex. This is a very simple crate that just \
                          creates hard bindings.",
        },
        Project {
            name: "speex-rs",
            link: "https://github.com/actioninja/speex-rs",
            description: "A safer wrapper around speex-sys which wraps the speex api in a safe, \
                          way. I was planning to reimplement the speex algorithms in pure safe \
                          rust, but got sidetracked so now this crate exists.",
        },
        Project {
            name: "Critical Action",
            link: "https://github.com/actioninja/personal-website",
            description: "My personal website! I wrote this website from scratch in Rust using \
                          the maud template engine. No javascript, no frameworks, just HTML and \
                          CSS and a static site generator.",
        },
        Project {
            name: "8values-svelte",
            link: "https://github.com/8values-svelte/8values-svelte.github.io",
            description: "a 1:1 whitebox reimplementation of the 8values political quiz using \
                          Svelte. I made this to have a hands-on project to learn Svelte, and \
                          ended up coming away with a more negative view of Svelte than I \
                          expected. I was planning to create a bunch of jokey variations of this \
                          because as part of myreimplementation I designed it from the start to \
                          allow adding arbitrary categories and questions ",
        },
        Project {
            name: "Biomes for Vintage Story",
            link: "https://github.com/spooq/biomes",
            description: "A mod to add bioregions to vintage story for when you have a billion \
                          mods that add plants and animals. I was not the original author of \
                          this, but the current versions I maintain and are entirely written by \
                          me. I entirely rewrote the mod to improve the performance from \
                          extremely heavy to essentially free.",
        },
        Project {
            name: "Handbook Declutterer",
            link: "https://github.com/actioninja/handbookdeclutterer",
            description: "A small Vintage Story mod that cleans up duplicate handbook entries in \
                          vanilla and in many other mods. This is sort of a fork of the original \
                          mod by Dana, but I ended up making allof the patches from scratch \
                          myself because it hadn't been updated in a while and many supported \
                          mods had changed a lot.",
        },
        Project {
            name: "Recentering Gears",
            link: "https://github.com/actioninja/recenteringgears",
            description: "Tiny vintage story mod that changes the functionality of the respawn \
                          gears to instead respawn you in an area around the set spawn point \
                          instead of directly on it.",
        },
        Project {
            name: "FastMathLib",
            link: "github.com/actioninja/FastMathLib",
            description: "A C# high performance linear algebra library. I was hoping to use this \
                          for a later Vintage Story mod, but never ended up finishing it. The \
                          types are fully SIMD accelerated despite being fully written in native \
                          C#. I wasn't super happy with the end design because I ended up having \
                          an absolute crapload of boilerplate code and I felt I could have \
                          reduced it with Rosyln codegen.",
        },
        Project {
            name: "resocraft",
            link: "https://github.com/actioninja/resocraft",
            description: "I was inspired while working on hypnagogic of 'what if a similar tool \
                          existed for minecraft' and started writing that. I didn't get very far \
                          in, but ended up making a few useful utilities in the process like a \
                          crate for Minecraft launcher metadata. I'd like to return to this \
                          eventually but I've been super busy.",
        },
        Project {
            name: "mc-launchermeta",
            link: "https://github.com/actioninja/mc-launchermeta",
            description: "A crate for parsing Minecraft launcher metadata. I wrote this for \
                          resocraft, but ended up making it a standalone crate because it's \
                          useful for other projects.",
        },
        Project {
            name: "Burning Sun",
            link: "https://github.com/actioninja/BurningSun",
            description: "A tiny minecraft mod I wrote almost a decade ago. Makes you light on \
                          fire in the sun like other mobs. Not very impressive, but it holds a \
                          soft spot in my heart.",
        },
        Project {
            name: "Realistic Flashlights Fixed",
            link: "https://github.com/actioninja/factorio-realisticflashlight-fixed",
            description: "A mod for Factorio that makes flashlights nicer and more realistic \
                          looking. I am not the original creator, but the original mod is no \
                          longer maintained. This is a fork that works on modern versions of \
                          Factorio and has a few bugs fixed.",
        },
    ]
});

pub struct Contribution {
    pub name: &'static str,
    pub link: &'static str,
    pub description: &'static str,
}

pub static CONTRIBUTIONS: Lazy<Vec<Contribution>> = Lazy::new(|| {
    vec![
        Contribution {
            name: "bevy_cursor",
            link: "https://github.com/tguichaoua/bevy_cursor",
            description: "A bevy crate for handling position across multiple windows more easily",
        },
        Contribution {
            name: "comtrya",
            link: "https://github.com/comtrya/comtrya",
            description: "A (unfortunately now discontinued) dotfile manager written in Rust",
        },
        Contribution {
            name: "onlyerror",
            link: "https://github.com/parasyte/onlyerror",
            description: "A very lightweight reimplementation of thiserror which doesn't use syn",
        },
        Contribution {
            name: "rust-g",
            link: "https://github.com/tgstation/rust-g",
            description: "A rust crate which acts as an FFI library for BYOND for performance \
                          sensitive stuff.",
        },
        Contribution {
            name: "dmi-rust",
            link: "https://github.com/spacestation13/dmi-rust",
            description: "A rust crate for parsing DMI files, the image format used by the BYOND \
                          engine for sprites and animations.",
        },
        Contribution {
            name: "gitsync",
            link: "https://github.com/rawkode/gitsync",
            description: "A rust crate for syncing git repositories with the remote.",
        },
        Contribution {
            name: "dfhack-scripts",
            link: "https://github.com/DFHack/scripts",
            description: "The official DFHack scripts repository. I helped migrate to lua.",
        },
        Contribution {
            name: "rustrover-ron-remix",
            link: "https://github.com/AmonDeShir/rustrover-ron-remix",
            description: "A RustRover extension for the Ron serialization format.",
        },
    ]
});

pub fn project_cards() -> Markup {
    html! {
        @for project in PROJECTS.iter() {
            div.project {
                a href=(project.link) {
                    h3 {
                        (project.name)
                        " "
                        img.link-svg src="/img/link.svg" alt="A link icon to indicate this title is a followable link" {}
                    }
                }
                p {
                    (project.description)
                }
            }
        }
    }
}

pub fn contribution_list() -> Markup {
    html! {
        ul {
            @for contrib in CONTRIBUTIONS.iter() {
                li {
                    a href=(contrib.link) {
                        (contrib.name)
                    }
                    " - "
                    (contrib.description)
                }
            }
        }
    }
}

pub fn projects() -> Markup {
    html! {
        h2 {
            "Projects"
        }
        section {
            "Various things I've made or significantly contributed to."
        }
        (project_cards())
        h2 {
            "Contributions"
        }
        section {
            "Stuff I've made small contributions to."
        }
        (contribution_list())
    }
}
