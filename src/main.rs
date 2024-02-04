use std::fs::read_to_string;
use std::path::Path;
use std::{fs, io};

use crate::css::optimize_css;
use crate::layout::wrap;

mod blog;
mod css;
pub mod djot;
pub mod layout;
pub mod pages;
mod projects;

const ASCII_NONSENSE: &str = r#"___________ _____  __________ ______________________ 
\_   _____//  _  \ \______   \\__    ___/\__    ___/ 
 |    __) /  /_\  \ |       _/  |    |     |    |    
 |     \ /    |    \|    |   \  |    |     |    |    
 \___  / \____|__  /|____|_  /  |____|     |____|    
     \/          \/        \/                        "#;


// Stackoverflow Special: https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    println!("{ASCII_NONSENSE}");
    println!("Ferris Approved Really Terrific Templates");
    println!("\"They really suck!\" - Me");

    // doesn't matter if it fails, we'll just overwrite it
    let _ = fs::remove_dir("public");
    fs::create_dir("public").expect("Could not create public directory");
    println!("Dumping assets into the public dir...");
    // dump all the assets into the public directory
    copy_dir_all("assets/public", "public").expect("Could not copy assets to public directory");
    println!("Minifying and writing css...");
    let css = read_to_string("assets/style.css").expect("Failed to read style.css");
    let css = optimize_css(&css);
    fs::write("public/style.css", css).expect("Failed to write css");
    println!("Generating pages...");
    println!("Generating index.html...");
    // generate static pages
    fs::write(
        "public/index.html",
        wrap("Critical Action", pages::index::index()).into_string(),
    )
    .expect("Could not write index.html");

    println!("Generating blog...");
    blog::generate_blog_pages();
    fs::write(
        "public/blog/index.html",
        wrap("Blog - Critical Action", blog::index::blog_index()).into_string(),
    )
    .expect("Could not write blog index");

    println!("Generating cool-stuff.html...");
    fs::create_dir_all("public/cool-stuff").unwrap();
    fs::write(
        "public/cool-stuff/index.html",
        wrap(
            "Cool Stuff - Critical Action",
            pages::cool_stuff::cool_stuff(),
        )
        .into_string(),
    )
    .expect("Could not write cool-stuff.html");

    println!("Generating projects page...");
    fs::create_dir_all("public/projects").unwrap();
    fs::write(
        "public/projects/index.html",
        wrap("Projects - Critical Action", projects::projects()).into_string(),
    )
    .expect("Could not write the projects page");
}
