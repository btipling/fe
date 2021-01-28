#[macro_use]
extern crate clap;
extern crate glob;
extern crate regex;
extern crate term_painter;
use clap::App;

mod find;
mod ignore;
mod display;
mod fileinfo;
mod log;

use log::*;

#[derive(Debug)]
pub enum SearchType {
    Fuzzy,
    Regex,
    Exact,
}

#[derive(Debug)]
pub struct Options {
    verbose: bool,
    very_verbose: bool,
    insensitive: bool,
    search_names_only: bool,
    no_colors: bool,
    search_type: SearchType,
    search_dir: String,
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut search_type = SearchType::Fuzzy;
    if matches.is_present("regex") {
        search_type = SearchType::Regex;
    } else if matches.is_present("exact") {
        search_type = SearchType::Exact;
    }

    let options = Options {
        verbose: matches.is_present("verbose"),
        very_verbose: matches.occurrences_of("verbose") > 1,
        insensitive: matches.is_present("insensitive"),
        search_names_only: matches.is_present("name"),
        no_colors: matches.is_present("plain"),
        search_type: search_type,
        search_dir: matches.value_of("search_dir").unwrap_or("./").to_string(),
    };

    // Unwrap in pattern is safe, clap guarantees it.
    let pattern = match matches.value_of("pattern") {
        Some(p) => p,
        _ => {
            v(format!("Listing files with options: {:?}", options), &options);
            find::list(".", &options);
            return;
        },
    };

    if pattern.ends_with('/') {
        find::list(pattern, &options);
        return;
    }
    v(format!("Search pattern is: {}, options: {:?}", pattern, options), &options);
    find::find(pattern, &options);
}
