use std::fs;
use std::path;
use std::io;
use ignore;

fn make_case_insensitive(input: &str, options: &super::Options) -> String {
    if !options.insensitive {
        return String::from(input)
    }
    input.to_lowercase()
}

struct Dir {
    path: path::PathBuf,
    rule_index: usize,
}

pub fn find (input: &str, options: &super::Options) {
    if options.verbose { println!("Looking for: {}, insensitive: {}", input, options.insensitive); }

    if input.len() == 0 {
        println!("No valid input given.");
        return;
    }

    let s = make_case_insensitive(input, options);
    let search = &s[..];

    let mut rule_sets = vec![ignore::RuleSet::new_default()];
    let dir = Dir {
        path: path::PathBuf::from("./"),
        rule_index: 0,
    };
    let mut dirs = vec![dir];
    loop {
        let current_path = match dirs.pop() {
            Some(p) => { p },
            None => { return; }
        };
        let mut rule_index = current_path.rule_index;
        let ignore_path_str = &format!("{}/.gitignore", current_path.path.to_str().unwrap());
        let ignore_path = path::Path::new(ignore_path_str);
        match ignore::RuleSet::extend(&rule_sets[rule_index], &ignore_path, options) {
            Ok(rule_set) => {
                if options.verbose { println!("Found a .gitignore: {}", current_path.path.to_str().unwrap()); }
                rule_sets.push(rule_set);
                rule_index = rule_sets.len() - 1;
            },
            _ => (),
        };

        let listings = current_path.path.read_dir().unwrap();
        for listing in listings {
            match search_listing(search, listing, &mut rule_sets[rule_index], options) {
                Some(path) => {
                    dirs.push(Dir {
                        path: path.path(),
                        rule_index: rule_index,
                    });
                },
                None => continue,
            }
        }
    }
}

fn search_listing(search: &str, listing: Result<fs::DirEntry, io::Error>, rule_set: &mut ignore::RuleSet, options: &super::Options) -> Option<fs::DirEntry> {
    let entity: fs::DirEntry = listing.unwrap();
    let path = entity.path();
    let is_dir = path.is_dir();
    {
        let path_str = match path.to_str() {
            Some(mut s) => {
                if s.starts_with("./") {
                    s = &s[2..];
                }
                s
            },
            _ => {
                if options.verbose { println!("Found invalid path string.") }
                return None;
            },
        };

        if rule_set.is_excluded(path_str, is_dir, options) {
            return None;
        }

        let s = make_case_insensitive(path_str, options);
        if path_matches_search(&s[..], search, options.verbose) {
            println!("{}", path_str);
        }
    }
    if is_dir {
        return Some(entity);
    }
    None
}

fn path_matches_search(path_str: &str, input: &str, verbose: bool) -> bool {

    if verbose { println!("Matching {} against {}", path_str, input); }

    let mut input_chars = input.chars();
    // `input` is guaranteed to be greater than 0 chars long.
    let mut current_input_char = input_chars.next().unwrap();
    let mut matching_current_word = true;

    for current_path_char in path_str.chars() {
        let is_alphanumeric = current_path_char.is_alphanumeric();
        if !is_alphanumeric {
            // Potentially starting a new word.
            matching_current_word = true;
            // We're not matching non-alphanumeric so continue.
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
                None => return true
            }
        } else if is_alphanumeric {
            matching_current_word = false;
        }
    }
    false
}
