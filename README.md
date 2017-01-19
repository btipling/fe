# fe
Fuzzy file finder in rust


## Usage:

fe finds files by unicode alpha-numeric characters. It works much like IntelliJ's fuzzy file opener.
Searches start matching at word start, stop matching until the next word. Words are separated by non-alphanumeric characters.

This finds main.rs because `m` matches the first word of `main` and `rs` matches the extension from the start.
```sh
~/p/r/fe (master) $ fe mrs
./src/main.rs
```

This matches `src` and `main`.
```sh
~/p/r/fe (master) $ fe srcmain
./src/main.rs
```

Here `Ca` matches the beginning of `Cargo` and `tom` matches the beginning of the `toml` extension.
```sh
~/p/r/fe (master) $ fe Catom
./Cargo.toml
```

Non-alphanumeric characters in the search string will never match.
```sh
~/p/r/fe (master) $ fe .rs
```

This finds non-rust files because `r` matches the first character of `rebase` and the first character of `sample`.
```sh
~/p/r/fe (master) $ fe rs
./.git/hooks/pre-rebase.sample
./.git/hooks/pre-receive.sample
./src/find.rs
./src/main.rs
```

This finds all the files that match `src`.
```sh
~/p/r/fe (master) $ fe src
./src/cli.yaml
./src/find.rs
./src/main.rs
./src
```

This is a pretty specific match:
```sh
~/p/r/fe (master) $ fe workspace
./.idea/workspace.xml
```