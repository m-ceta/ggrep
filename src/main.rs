extern crate regex;
use regex::Regex;

use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::env;
use std::io::{BufRead, BufReader};

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        return;
    }

    let start_dir = &args[1];
    let file_exp = &args[2];
    let search_exp = &args[3];

    if let Ok(re_file) = Regex::new(file_exp) {

        if let Ok(re_search) = Regex::new(search_exp) {

            let mut found = Vec::new();
            search_file(PathBuf::from(start_dir), &re_file, &mut found);

            while let Some(found_file) = found.pop() {
                search_contents(found_file, &re_search);
            }
        }
    }
}

fn search_file(path: PathBuf, re_file: &Regex, found: &mut Vec<PathBuf>) {
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(dir) = entry {
                    search_file(dir.path(), re_file, found);
                }
            }
        }
    } 
    else {
        if let Some(os_name) = path.file_name() {
            if let Some(name) = os_name.to_str() {
                if re_file.is_match(name) {
                    found.push(path);
                }
            }
        }
    }
}

fn search_contents(path: PathBuf, re_search: &Regex) {
    if let Some(os_name) = path.file_name() {
        if let Some(name) = os_name.to_str() {
            if let Ok(file) = File::open(&path) {
                let mut i = 1;
                for result in BufReader::new(file).lines() {
                    if let Ok(l) = result {
                        if re_search.is_match(l.as_str()) {
                            println!("{}({}): {}", name, i, l);
                        }
                    }
                    i += 1;
                }
            }
        }
    }
}


