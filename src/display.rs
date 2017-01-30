use std::path;
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;
use fileinfo::FileInfo;
use std::ffi::OsStr;

pub fn print_as_path(path: &path::Path, options: &super::Options) {
    let mut s = path.to_str().unwrap_or("");
    if s.starts_with("./") {
        s = &s[2..];
    }
    print_path(path, s, options);
}

pub fn print_as_filename(path: &path::Path, options: &super::Options) {
    let s = path.file_name().unwrap_or(OsStr::new("")).to_str().unwrap_or("");
    print_path(path, s, options);
}

pub fn print_path(path: &path::Path, s: &str, options: &super::Options) {
    if options.no_colors {
        println!("{}", s);
        return;
    }

    let info = match FileInfo::new(path) {
        Ok(i) => i,
        Err(e) => {
            if options.verbose { println!("Error getting metadata for {}: {}", s, e) }
            println!("{}", Plain.bg(Red).fg(White).paint(s));
            return;
        }
    };

    if info.is_dir() {
        println!("{}", Blue.paint(s));
    } else if info.is_symbolic_link() {
        println!("{}", Magenta.paint(s));
    } else if info.everyone_can_do_everything() {
        println!("{}", Plain.bg(Green).fg(Black).paint(s));
    } else if info.is_executable() {
        println!("{}", Red.paint(s));
    } else {
        println!("{}", s);
    }
}