use aho_corasick::AhoCorasick;
use pathdiff::diff_paths;

use crate::tools::{get_latest_line, print_symbol, check_control, unix_time_diff};
use crate::parsers::base::get_parser;
use crate::info::{get_line_info, get_commit_diff};
use std::path::Path;
use std::fs::{read_to_string};
use std::time::SystemTime;
use git2::{Repository, Oid};

pub fn check_file(repo: &Repository, working_dir: &Path, file: &Path, ac: &AhoCorasick, porcelain: &bool) {
    let patterns = [".git", ".swp", "node_modules"]; // TODO: add global pattern list, or read gitignore
    // let f = file.path().to_str().unwrap();
    if ac.is_match(file.to_str().unwrap()) {
        return;
    }

    // check if file is directory
    let ext = match file.extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => {
            return;
        }
    };

    let parser = match get_parser(file, &patterns) {
        Some(parser) => parser,
        _ => {
            return;
        }
    };

    let read = match read_to_string(file) {
        Ok(read) => read,
        Err(_) => {
            println!("Failed to read file {}, skipping", file.display());
            return;
        }
    };

    let relative_path = diff_paths(file, working_dir).unwrap();

    // let repo = Repository::open(working_dir).expect("Failed to open repo");

    let blame_lines = match get_line_info(&repo, &relative_path) {
        Ok(lines) => lines,
        Err(e) => {

            if *porcelain != true {
                println!("{}", e);
                println!("Failed checking {}, continuing", file.display());
            }
            return;
        }
    };
    let all_funs = match parser.parse(&read) {
        Ok(funs) => funs,
        Err(e) => {
            println!("Failed to parse file. Error {}. Skipping", e);
            return;
        }
    };

    // make a module which checks all of these, checkall, which you can implement
    for (comment, function) in all_funs {
        let comment_idx = get_latest_line(&blame_lines, &comment);
        let fun_idx = get_latest_line(&blame_lines, &function);

        let comment_info = blame_lines.get(&comment_idx).expect("Failed to get comment from blame lines");
        let function_info = blame_lines.get(&fun_idx).expect("Failed to get function from blame lines");

        // if the comment has been edited before, or at the same time as the function has
        if function_info.time <= comment_info.time {
            continue;
        }

        if check_control(&blame_lines, &function) {
            continue;
        }

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let time_diff = unix_time_diff(current_time.into(), comment_info.time.into());
        println!("{}", time_diff);
        let commit_diff = get_commit_diff(&repo, &Oid::from_str(&comment_info.commit).unwrap()).expect("Failed to get commit diff");
        println!("{} commits since update", commit_diff);

        let line = function.start.line - 1;
        let character = function.start.character;
        println!("{}:{}:{}", file.display(), line, character);

        print_symbol(&function, &comment, &file, ext, &porcelain);
    }
}