use regex;
use std::fs;
use std::path;
use std::io;
use ignore;
use display;

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

struct SearchContext<'a> {
    search: &'a str,
    regex: Option<&'a regex::Regex>,
    options: &'a super::Options,
}

pub fn list (path: &str, options: &super::Options) {
    //! Just iterates through the current directory.
    let current_path = path::PathBuf::from(path);
    let current_path_str = current_path.to_str().unwrap_or("");

    if !current_path.is_dir() {
        display::print_as_filename(current_path.as_path(), options);
        return;
    }

    let dir_entries = match current_path.read_dir() {
        Ok(e) => e,
        Err(e) => {
            if options.verbose { println!("Failed to read directory entries for {} because {}.", current_path_str, e); }
            return;
        }
    };

    for dir_entry in dir_entries {
        if let Ok(current_pathbuf) = dir_entry  {
            display::print_as_filename(current_pathbuf.path().as_path(), options);
        }
    }
}

pub fn find (pattern: &str, options: &super::Options) {
    if options.verbose { println!("Looking for: {}, insensitive: {}", pattern, options.insensitive); }

    if pattern.len() == 0 {
        println!("No valid input given.");
        return;
    }

    // Set up search context for searching: the search string and regular expression if needed.
    let s = make_case_insensitive(pattern, options);
    let found_regex;
    let mut search_regex = None;
    match options.search_type {
        super::SearchType::Regex => {
            found_regex = match regex::Regex::new(pattern) {
                Ok(r) => r,
                Err(e) => {
                    println!("Failed to parse regular expression: {}", e);
                    return;
                }
            };
            search_regex = Some(&found_regex);
        },
        _ => (),
    };
    let search = SearchContext {
        search: &s[..],
        regex: search_regex,
        options: options,
    };

    // Set up state for searching: the ignore rules and directory queue. Rules are stored in a vector
    // and reference them by rule_index so it doesn't have to store references to rules in subsequent
    // directories found. The rule index is associated with a directory and attached to the directory search queue.
    // This is done because .gitignore rules are merged in root and subsequent ignore files found later in
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
        match ignore::RuleSet::extend(&rule_sets[rule_index], &ignore_path, search.options) {
            Ok(rule_set) => {
                if search.options.verbose { println!("Found a .gitignore: {}", current_path_str); }
                rule_sets.push(rule_set);
                rule_index = rule_sets.len() - 1;
            },
            _ => (),
        };

        let dir_entries = match current_path.path.read_dir() {
            Ok(e) => e,
            Err(e) => {
                if search.options.verbose { println!("Failed to read directory entries for {} because {}", current_path_str, e); }
                continue 'search_loop;
            }
        };

        // Iterate through directory entries.
        for dir_entry in dir_entries {
            match search_dir_entry(&search, dir_entry, &mut rule_sets[rule_index]) {
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

fn search_dir_entry(search: &SearchContext, dir_entry: Result<fs::DirEntry, io::Error>, rule_set: &mut ignore::RuleSet) -> Option<fs::DirEntry> {
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
            if search.options.verbose { println!("Found invalid path string.") }
            return None;
        },
    };

    let is_dir = path.is_dir();
    if let Some(filename) = path.file_name().map(|n| n.to_str()) {
        if rule_set.is_excluded(filename.unwrap(), is_dir, search.options) {
            return None;
        }
    } else {
        if search.options.verbose { println!("Not matching against {} as it has no filename", path_str) }
    }

    let mut s = path_str;
    if search.options.search_names_only {
        s = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => {
                if search.options.verbose { println!("No file name found for {}", path_str); }
                return None;
            }
        }
    }

    let s = make_case_insensitive(s, search.options);
    let found = match search.options.search_type {
        super::SearchType::Regex => regex_path_match_search(&s[..], search),
        super::SearchType::Exact => &s[..] == search.search,
        super::SearchType::Fuzzy => fuzzy_path_match_search(&s[..], search),
    };
    if found {
        display::print_as_path(path.as_path(), search.options);
    }

    // If we're looking at a directory return it to be iterated through.
    if is_dir {
        return Some(dir_entry);
    }
    None
}

fn regex_path_match_search(path_str: &str, search: &SearchContext) -> bool {
    let search_regex = search.regex;
    let r = search_regex.expect("Missing a regular expression!").is_match(path_str);
    if search.options.very_verbose { println!("Regexp matching {} against {}: {}", path_str, search.search, r); }
    r
}

fn fuzzy_path_match_search(path_str: &str, search: &SearchContext) -> bool {
    //!
    //! `fuzzy_path_match_search` attempts to make a fuzzy match based on the following rules:
    //!
    //!     * Words are roughly consecutive UTF8 alphanumeric characters in a path.
    //!     * Words begin at the start of the path or at the first non-alphanumeric character.
    //!     * Non-alphanumeric characters begin words because non-alphanumeric matches must also work.
    //!     * Matches can only begin and continue on the first character of a word.
    //!     * Character by character matching continues in a word until it fails.
    //!     * One a word has failed to match, all subsequent characters of the words are skipped.
    //!     * The match attempts to continue on the next non-alphanumeric character, the start of the next word
    //!         in the path.
    //!
    //! This fuzzy search behavior is based on how IntelliJ's open file fuzzy search works, except
    //! that in the case of this tool, it also matches non-alphanumeric numbers.
    //!
    //! Example matches for a search for the string `shared`:
    //!
    //!     `src/haskell/red.hs` matches because src starts with `s`, haskell matches `ha` and `red`
    //!         in red.hsfinishes the match.
    //!
    //!     `src/shared/foo.js` matches because the word `shared` matches the entire search string.
    //!
    //! Example matches for `foo.js`:
    //!
    //!     `src/foo/bar.js` matches because `foo` matches the `foo` and `.js` matches the `.js` in the path.
    //!
    //!     `src/bar/foo.json` matches because the first six characters of `foo.json` match `foo.js`.
    //!
    let input = search.search;
    let options = search.options;
    if path_str.len() == 0 {
        return false;
    }
    if options.very_verbose { println!("Matching {} against {}", path_str, input); }

    // ** Set pre-loop path search state.

    // The `input_chars` variable is the character iterator for the search input. It is reset whenever
    // the end of `path_chars` is reached while `match_in_progress` is true. If `input_chars`
    // iterates to its end the path matches the input and `fuzzy_path_match_search` returns true.
    let mut input_chars = input.chars();
    // The `path_chars` variable is the character iterator for the path. It is reset to the index
    // stored in `index_matched_at` if path_chars runs out while `match_in_progress` is true. Otherwise
    // it is assumed this patch does not match the search input and `fuzzy_path_match_search` returns false.
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

    // The loop that iterates character by character through input and path, rewinding and retreating,
    // as necessary. `input_chars` is sometimes rewound all the way to its beginning, `path_chars` is
    // only ever reset to the character *after* where the last match first started.
    'pathsearch: loop {
        let current_path_char;
        let next_possible_path_char = path_chars.next();

        if next_possible_path_char.is_some() {
            if !match_in_progress {
                // This is the next character's index, not the current one, it may not exist! We catch that
                // condition in the else if below.
                index_matched_at += 1;
            }
            current_path_char = next_possible_path_char.unwrap();
        } else if index_matched_at == path_str.len() {
            // There was a match in progress, but it started at the last possible character. Matching is over.
            return false;
        } else {
            if !match_in_progress {
                // The loop is finished as it is has run out of characters and is not rewound as
                // no match is in progress.
                return false;
            }
            match_in_progress = false;
            path_chars = path_str.chars();
            path_chars.nth(index_matched_at).unwrap(); // Safe due to else if above.
            input_chars = input.chars();
            current_input_char = input_chars.next().unwrap();
            if options.very_verbose { println!("Resetting search {} against {}, index {}", path_str, input, index_matched_at); }
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
