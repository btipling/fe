#[macro_use]
extern crate clap;
use clap::App;

mod find;

fn main() {
    println!("Hello, world!");

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let insensitive = matches.is_present("insensitive");
    if insensitive {
        println!("Searching with case sensitivity turned on.");
    }
    let pattern = matches.value_of("input").unwrap();
    println!("Search pattern is: {}", pattern);
    find::find(pattern, insensitive);
}
