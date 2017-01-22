#[macro_use]
extern crate clap;
extern crate glob;
use clap::App;

mod find;
mod ignore;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let insensitive = matches.is_present("insensitive");
    let verbose = matches.is_present("verbose");
    if insensitive {
    } else {
        if verbose { println!("Searching with case sensitivity turned off."); }
    }
    let pattern = matches.value_of("input").unwrap();
    if verbose { println!("Search pattern is: {}", pattern); }
    find::find(pattern, insensitive, verbose);
}
