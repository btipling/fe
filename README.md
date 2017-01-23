# fe
fe is a fuzzy file finder in rust. fe's speeds are comparable to find, and often times faster. It respects .gitignores and
is more ergonomic than using find when you just want to find a file:

```shell
~/p/sphela (master) $ time fe gulp
gulpfile.js
        0.00 real         0.00 user         0.00 sys
~/p/sphela (master) $ time find . -name "gulpfile.js"
./gulpfile.js
./node_modules/escope/gulpfile.js
./node_modules/esrecurse/node_modules/estraverse/gulpfile.js
./node_modules/estraverse/gulpfile.js
./node_modules/gulp-stylus/examples/gulpfile.js
./node_modules/macaddress/gulpfile.js
        0.55 real         0.02 user         0.28 sys
```

## Installation

```shell
git clone https://github.com/btipling/fe.git
cargo install
```

Homebrew and debian/aptitude packages are planned.


## Usage:

```shell
$ fe --help
fe 1.0
Helps you find files with a fuzzy search.

USAGE:
    fe [FLAGS] <input>

FLAGS:
    -h, --help            Prints help information
    -i, --insensitive     Sets the filename pattern to be case insensitive
    -n, --name            Search name and extension only.
    -V, --version         Prints version information
    -v, --verbose         Print additional information during search.
    -v, --very_verbose    Print debug information during search.

ARGS:
    <input>    Sets the pattern to search for
```

fe finds files by unicode alpha-numeric characters. It works much like IntelliJ's fuzzy file opener.
Searches start matching at word start, and on match failure stop matching until the next word. Words are separated by non-alphanumeric characters.

This finds main.rs because `m` matches the first word of `main` and `rs` matches the extension from the start.
```shell
~/p/r/fe (master) $ fe mrs
./src/main.rs
```

This matches `src` and `main`.
```shell
~/p/r/fe (master) $ fe srcmain
./src/main.rs
```

Here `Ca` matches the beginning of `Cargo` and `tom` matches the beginning of the `toml` extension.
```shell
~/p/r/fe (master) $ fe Catom
./Cargo.toml
```

This finds non-rust files because `r` matches the first character of `rebase` and the first character of `sample`.
```shell
~/p/r/fe (master) $ fe rs
./.git/hooks/pre-rebase.sample
./.git/hooks/pre-receive.sample
./src/find.rs
./src/main.rs
```

This finds all the files that match `src`.
```shell
~/p/r/fe (master) $ fe src
./src/cli.yaml
./src/find.rs
./src/main.rs
./src
```

This is a pretty specific match:
```shell
~/p/r/fe (master) $ fe workspace
./.idea/workspace.xml
```