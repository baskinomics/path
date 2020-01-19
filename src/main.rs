use std::env;
use std::path::PathBuf;

/// Represents an abstraction of an element contained in the set of 
/// directory paths in the `$PATH` environment variable.
struct Tag {
    id: u32,
    value: String,
    path: PathBuf,
}

/// `path list` -> pretty-print
/// `path add -t <tag> -p <path>`
/// `path remove -t <tag>`
/// `path update -t <tag> -p <path>`
fn main() {
    let key = "PATH";
    switch env::vars_os(key) {
        Some(paths) => {

        }
    }
    for (key, value) in env::vars_os() {
        println!("{:?}: {:?}", key, value);
    }
}


fn get_path() {

}
