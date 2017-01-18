use std::fs;
use std::path;

pub fn find (input: &str, insensitive: bool) {
    println!("Looking for: {}, insensitive: {}", input, insensitive);
    let listings = fs::read_dir("./").unwrap();

    let mut found_files: Vec<path::PathBuf> = vec![];

    for listing in listings {
        let entity: fs::DirEntry = listing.unwrap();
        let path = entity.path();
        let path_str = path.to_str().unwrap();
        println!("as path string: {:?}", path_str);
        if path_str == input {
            found_files.push(entity.path());
        }
    }

    println!("Found files:");

    for file in found_files {
        println!("Found file: {:?}", file);
    }

}