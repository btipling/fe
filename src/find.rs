use std::fs;
use std::path;

pub fn find (input: &str, insensitive: bool, verbose: bool) {
    if verbose { println!("Looking for: {}, insensitive: {}", input, insensitive); }

    if input.len() == 0 {
        println!("No valid input given.");
        return;
    }

    let entry_path = path::Path::new("./");
    let mut found_files: Vec<path::PathBuf> = vec![];
    traverse(input, &entry_path, &mut found_files);

    if verbose { println!("Found files:"); }
    for file in found_files {
        println!("{}", file.to_str().unwrap());
    }

}

pub fn traverse(input: &str, current_path: &path::Path, found_files: &mut Vec<path::PathBuf>) {

    if !current_path.is_dir() {
        return;
    }
    let listings = current_path.read_dir().unwrap();

    for listing in listings {
        let entity: fs::DirEntry = listing.unwrap();
        let path_buffer = entity.path();
        let path = path_buffer.as_path();

        if path.is_dir() {
            traverse(input, &path, found_files);
        }

        let path_str = path.to_str().unwrap();
        if path_matches_search(&path_str,/**/ input) {
            found_files.push(entity.path());
        }
    }
}

fn path_matches_search(path_str: &str, input: &str) -> bool {
    if input.len() > path_str.len() {
        return false;
    }

    let mut input_chars = input.chars();
    // `input` is guaranteed to be greater than 0 chars long.
    let mut current_input_char = input_chars.next().unwrap();
    let mut matching_current_word = true;

    for current_path_char in path_str.chars() {
        if !current_path_char.is_alphanumeric() {
            // Potentially starting a new word.
            matching_current_word = true;
            // We're not matching non-alphanumeric so continue.
            continue;
        }

        if !matching_current_word {
            // Current word was not matched, proceed until we get to a non-alphanumeric character.
            continue;
        }

        if current_input_char == current_path_char {
            match input_chars.next() {
                Some(c) => {
                    current_input_char = c;
                },
                None => {
                    return true;
                }
            }
        } else {
            matching_current_word = false;
        }
    }

    false
}
