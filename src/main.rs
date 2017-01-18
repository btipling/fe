#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    println!("Hello, world!");

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    if matches.is_present("insensitive") {
        println!("Searching with case sensitivity turned on.");
    }
    println!("Search pattern is: {}", matches.value_of("input").unwrap());
}
