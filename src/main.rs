/// See: https://en.wikipedia.org/wiki/PATH_(variable)
extern crate clap;
use clap::App;
use std::env;
use std::path::PathBuf;

/// Represents an abstraction of an element contained in the set of
/// directory paths in the `$PATH` environment variable.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Tag {
    name: String,
    path: PathBuf,
}

impl Tag {
    pub fn new(name: String, path: PathBuf) -> Self {
        Tag { name, path }
    }
}

/// Contains the business logic for the `path` CLI tool.
fn main() {
    let matches = App::new("path")
        .version("0.0.1")
        .author("Sean Baskin <seanbaskin@gmail.com")
        .about("Sane $PATH variable management for the 21st century.")
        .subcommand(
            App::new("list")
                .about("Lists the <tag, path> key-value pairs in the $PATH environment variable."),
        )
        .subcommand(
            App::new("add")
                .about("Adds a new <tag, path> key-value pair, and adds the path to $PATH."),
        )
        .subcommand(
            App::new("remove")
                .about("Removes a tag key-value pair, and removes the path from $PATH."),
        )
        .subcommand(
            App::new("update").about("Updates a tag and updates the associated path in $PATH."),
        )
        .get_matches();

    if let Some(ref _matches) = matches.subcommand_matches("list") {
        let tags: Vec<Tag> = build_tag_set();
        list_tags(&tags);
    }
}

/// Prints the given collection `tags`.
fn list_tags(tags: &Vec<Tag>) {
    println!("Current $PATH environment variable:");
    for tag in tags {
        println!(
            "  tag[value: '{}', path: '{}']",
            tag.name,
            tag.path.display()
        );
    }
}

/// Returns a collection of `Tag` instances derived from the $PATH environment
/// variable.
fn build_tag_set() -> Vec<Tag> {
    // Create an empty vector to hold values of type `Tag`.
    let mut tags: Vec<Tag> = Vec::new();

    // Build collection of `Tag` instances.
    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let tag = Tag::new("Tag".to_string(), path);
                tags.push(tag);
            }
        }
        None => println!("{} is not defined in the environment.", key),
    }

    // Sort the collection of `Tag` instances.
    tags.sort();
    return tags;
}
