extern crate regex;
use regex::Regex;

use std::fs;
use std::fs::{File, OpenOptions};
use std::path;
use std::path::Path;
use std::env;

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

            let found_files = search_file(&Path::new(&start_dir), &re_file);

            for found_file in found_files {
                let mut found_contents: Vec<&String> = Vec::new();
                search_contents(&found_file, &re_search, &mut found_contents);

                if let Some(os_name) = found_file.file_name() {
                    if let Some(name) = os_name.to_str() {
                        println!("<{}>", name);
                        for content in found_contents {
                            println!("   {}", content);
                        }
                    }
                }
            }
        }
    }
}

fn search_file<'a>(path: &'a Path, re_file: &'a Regex) -> &'a Vec<&'a Path> {
    let mut found: Vec<&Path> = Vec::new();
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(dir) = entry {
                    let ret = search_file(&dir.path(), re_file);
                    found.append(&mut ret);
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
    &found
}

fn search_contents(path: &Path, re_search: &Regex, found: &mut Vec<&String>) {

}


