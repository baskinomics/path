# `path`
Enables a user to manage the contents of their `$PATH` environment variable in a standardized fashion, without touching your dotfiles.

```bash
$ path --help
path 0.0.1
Sean Baskin <seanbaskin@gmail.com
Sane $PATH variable management for the 21st century.

USAGE:
    path [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       Adds a new <tag, path> key-value pair, and adds the path to $PATH.
    help      Prints this message or the help of the given subcommand(s)
    list      Lists the <tag, path> key-value pairs in the $PATH environment variable.
    remove    Removes a tag key-value pair, and removes the path from $PATH.
    update    Updates a tag and updates the associated path in $PATH.

```
