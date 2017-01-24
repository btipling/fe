#[macro_use]
extern crate clap;
extern crate glob;
use clap::App;

mod find;
mod ignore;

#[derive(Debug)]
pub struct Options {
    verbose: bool,
    very_verbose: bool,
    insensitive: bool,
    search_names_only: bool,
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
    };
    if options.verbose { println!("Search pattern is: {}, options: {:?}", pattern, options); }

    find::find(pattern, &options);
}
