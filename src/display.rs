use std::path;
use term_painter::ToStyle;
use term_painter::Color::*;
use fileinfo::FileInfo;

pub fn print(path: &path::Path, options: &super::Options) {
    let mut s = path.to_str().unwrap_or("");
    if s.starts_with("./") {
        s = &s[2..];
    }

    let info = match FileInfo::new(path) {
        Ok(i) => i,
        Err(e) => {
            if options.verbose { println!("Error getting metadata for {}: {}", s, e) }
            println!("{}", s);
            return;
        }
    };

    if info.is_dir() {
        println!("{}", Blue.paint(s));
    } else if info.is_executable() {
        println!("{}", Red.paint(s));
    } else {
        println!("{}", s);
    }
}