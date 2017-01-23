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

    // Set up state for searching: the ignore rules and directory queue. We store ignore rules in a vector
    // and reference them by rule_index so we don't have to store references to rules in subsequent
    // directories we find. The rule index is associated with a directory and attached to the directory search queue.
    // This is done because we merge .gitignore rules in root and subsequent ignore files found later in
    // subdirectories.
    let mut rule_sets = vec![ignore::RuleSet::new_default()];
    let dir = Dir {
        path: path::PathBuf::from("./"),
        rule_index: 0,
    };
    let mut dirs = vec![dir];

    'search_loop: loop {
        // Get next entry or finish.
        let current_path = match dirs.pop() {
            Some(p) => { p },
            None => return,
        };
        let current_path_str = match current_path.path.to_str() {
            Some(s) => s,
            None => continue,
        };

        // Check if there's an ignore for the current directory.
        let mut rule_index = current_path.rule_index;
        let ignore_path_str = &format!("{}/.gitignore", current_path_str);
        let ignore_path = path::Path::new(ignore_path_str);
        match ignore::RuleSet::extend(&rule_sets[rule_index], &ignore_path, options) {
            Ok(rule_set) => {
                if options.verbose { println!("Found a .gitignore: {}", current_path_str); }
                rule_sets.push(rule_set);
                rule_index = rule_sets.len() - 1;
            },
            _ => (),
        };

        let dir_entries = match current_path.path.read_dir() {
            Ok(e) => e,
            Err(e) => {
                if options.verbose { println!("Failed to read directory entries for {} because {}", current_path_str, e); }
                continue 'search_loop;
            }
        };

        // Iterate through directory entries.
        for dir_entry in dir_entries {
            match search_dir_entry(search, dir_entry, &mut rule_sets[rule_index], options) {
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

fn search_dir_entry(search: &str, dir_entry: Result<fs::DirEntry, io::Error>, rule_set: &mut ignore::RuleSet, options: &super::Options) -> Option<fs::DirEntry> {
    let dir_entry: fs::DirEntry = match dir_entry {
        Ok(entity) => entity,
        _ => return None,
    };

    // Get and finesse entry path.
    let path = dir_entry.path();
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

    let is_dir = path.is_dir();
    if rule_set.is_excluded(path_str, is_dir, options) {
        return None;
    }

    let mut s = path_str;
    if options.search_names_only {
        s = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => {
                if options.verbose { println!("No file name found for {}", path_str); }
                return None;
            }
        }
    }

    let s = make_case_insensitive(s, options);
    if path_matches_search(&s[..], search, options) {
        println!("{}", path_str);
    }

    // If we're looking at a directory return it to be iterated through.
    if is_dir {
        return Some(dir_entry);
    }
    None
}

fn path_matches_search(path_str: &str, input: &str, options: &super::Options) -> bool {
    if path_str.len() == 0 {
        return false;
    }
    if options.very_verbose { println!("Matching {} against {}", path_str, input); }

    // ** Set pre-loop path search state.

    // The `input_chars` variable is the character iterator for the search input. It is reset whenever
    // the end of `path_chars` is reached while `match_in_progress` is true. If `input_chars`
    // iterates to its end the path matches the input and `path_matches_search` returns true.
    let mut input_chars = input.chars();
    // The `path_chars` variable is the character iterator for the path. It is reset to the index
    // stored in `index_matched_at` if path_chars runs out while `match_in_progress` is true. Otherwise
    // it is assumed this patch does not match the search input and `path_matches_search` returns false.
    let mut path_chars = path_str.chars();
    // `input` is guaranteed to be greater than 0 chars long so unwrap is safe here.
    let mut current_input_char = input_chars.next().unwrap();

    // Words are consecutive UTF8 alphanumeric characters in a path. `matching_current_word` is how
    // character matches are tracked one after another inside a word starting from its first
    // character. Once character is found that doesn't match, this is set to false. Matching stops.
    // It start again at the start of the next word in the path. Non-alphanumeric characters
    // are still checked for matches however.
    let mut matching_current_word = true;
    // The `match_in_progress` variable tracks that a character starting at the
    // beginning of a word waws matched, beginning the process of a fuzzy match. This variable
    // is reset to false when it runs out of path characters to check.
    let mut match_in_progress = false;
    // The `index_matched_at` variable tracks the first path character that matched the input
    // whether that was an alphanumeric character at the beginning of a word or a non-alphanumeric
    // character in between words.
    let mut index_matched_at = 0;

    'pathsearch: loop {
        let current_path_char;
        let next_possible_path_char = path_chars.next();

        if next_possible_path_char.is_some() {
            if !match_in_progress {
                index_matched_at +=1;
            }
            current_path_char = next_possible_path_char.unwrap();
        } else {
            if !match_in_progress {
                return false;
            }
            match_in_progress = false;
            path_chars = path_str.chars();
            path_chars.nth(index_matched_at).unwrap();
            input_chars = input.chars();
            current_input_char = input_chars.next().unwrap();
            if options.verbose { println!("Resetting search {} against {}, index {}", path_str, input, index_matched_at); }
            continue 'pathsearch;
        }


        let is_alphanumeric = current_path_char.is_alphanumeric();
        if !is_alphanumeric {
            // Potentially starting a new word.
            matching_current_word = true;
        }

        if !matching_current_word {
            // Current word was not matched, proceed until a non-alphanumeric character is found.
            continue;
        }

        if current_input_char == current_path_char {
            match_in_progress = true;
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
}
