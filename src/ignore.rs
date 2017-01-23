use glob::Pattern;
use glob::PatternError;
use glob::MatchOptions;
use std::io;
use std::io::BufRead;
use std::path;
use std::fs;

#[derive(Clone)]
pub struct RuleSet {
    rules: Vec<RuleSetPattern>,
}

pub enum IgnoreError {
    Io(io::Error)
}

pub enum RuleSetError {
    Pattern(PatternError),
    NoLength,
}

#[derive(Clone)]
struct RuleSetPattern {
    pattern: Pattern,
    is_dir: bool,
}

impl RuleSetPattern {
    fn new (new_path: &str) -> Result<RuleSetPattern, RuleSetError> {
        let mut path = new_path.trim();
        let is_dir = path.ends_with('/');
        if is_dir {
            path = &path[..path.len() - 1]
        }
        if path.len() == 0 {
            return Err(RuleSetError::NoLength);
        }
        let pattern = try!(Pattern::new(path).map_err(RuleSetError::Pattern));
        Ok(RuleSetPattern {
            pattern: pattern,
            is_dir: false,
        })
    }
}

impl RuleSet {

    pub fn new_default() -> RuleSet {
        let mut rules: Vec<RuleSetPattern> = vec![];
        match RuleSetPattern::new(".git/") {
            Ok(r) => rules.push(r),
            _ => (),
        }
        RuleSet {
            rules: rules
        }
    }

    pub fn new (ignore_path: &path::Path, options: &super::Options) -> Result<RuleSet, IgnoreError> {

        let f = try!(fs::File::open(ignore_path).map_err(IgnoreError::Io));
        if options.verbose { println!("Found {:?} an ignore file.", ignore_path) }

        let buffer = io::BufReader::new(&f);
        let mut rules: Vec<RuleSetPattern> = vec![];
        for line in buffer.lines() {
            let l = try!(line.map_err(IgnoreError::Io));
            let r = match RuleSetPattern::new(&l[..]) {
                Ok(r) => r,
                _ => continue // TODO: support ! rule negations.
            };
            if options.verbose { println!("Found rule: {}.", l) }
            rules.push(r);
        }

        Ok(RuleSet {
            rules: rules
        })
    }

    pub fn extend (rule_set: &RuleSet, ignore_path: &path::Path, options: &super::Options) -> Result<RuleSet, IgnoreError> {
        let mut new_set = try!(RuleSet::new(ignore_path, options));
        new_set.rules.extend(rule_set.rules.clone());
        Ok(new_set)
    }

    pub fn is_excluded (&self, path: &str, is_dir: bool, options: &super::Options) -> bool {
        for rule_set_pattern in &self.rules {
            if rule_set_pattern.is_dir && !is_dir {
                continue;
            }
            let match_options = MatchOptions {
                case_sensitive: true, // .gitignore is case insensitive, but fe isn't.
                require_literal_separator: false,
                require_literal_leading_dot: false
            };
            if rule_set_pattern.pattern.matches_with(path, &match_options) {
                if options.verbose { println!("{} is ignored because it matches {}", path, rule_set_pattern.pattern) }
                return true;
            } else {
                if options.very_verbose { println!("{} is not ignored because it doesn't match {}", path, rule_set_pattern.pattern) }
            }
        }
        false
    }
}