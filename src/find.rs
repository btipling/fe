use std::fs;
use std::path;

fn make_case_insensitive(input: &str, insensitive: bool) -> String {
    if !insensitive {
        return String::from(input)
    }
    input.to_lowercase()
}

pub fn find (input: &str, insensitive: bool, verbose: bool) {
    if verbose { println!("Looking for: {}, insensitive: {}", input, insensitive); }

    if input.len() == 0 {
        println!("No valid input given.");
        return;
    }

    let s = make_case_insensitive(input, insensitive);
    let search = &s[..];

    let mut dirs = vec![path::PathBuf::from("./")];
    loop {
        let current_path = match dirs.pop() {
            Some(p) => { p },
            None => { return; }
        };

        let listings = current_path.read_dir().unwrap();
        for listing in listings {
            let entity: fs::DirEntry = listing.unwrap();
            let path_buffer = entity.path();
            let path = path_buffer;
            {
                let path_str = path.to_str().unwrap();
                let s = make_case_insensitive(path_str, insensitive);
                if path_matches_search(&s[..], search, verbose) {
                    println!("{}", path_str);
                }
            }
            if path.is_dir() {
                dirs.push(path);
            }
        }
    }

}

fn path_matches_search(path_str: &str, input: &str, verbose: bool) -> bool {

    if verbose { println!("Matching {} against {}", path_str, input); }

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
