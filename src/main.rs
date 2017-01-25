#[macro_use]
extern crate clap;
extern crate glob;
extern crate regex;
use clap::App;

mod find;
mod ignore;

#[derive(Debug)]
pub struct Options {
    verbose: bool,
    very_verbose: bool,
    insensitive: bool,
    search_names_only: bool,
    regex: bool,
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // Unwrap in input is safe, clap guarantees it.
    let pattern = matches.value_of("input").unwrap();

    let options = Options {
        verbose: matches.is_present("verbose"),
        very_verbose: matches.occurrences_of("verbose") > 1,
        insensitive: matches.is_present("insensitive"),
        search_names_only: matches.is_present("name"),
        regex: matches.is_present("regex"),
    };
    if options.verbose { println!("Search pattern is: {}, options: {:?}", pattern, options); }

    find::find(pattern, &options);
}
