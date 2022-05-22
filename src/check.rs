use aho_corasick::AhoCorasick;
use pathdiff::diff_paths;

use crate::tools::{get_latest_line, print_symbol, check_control, unix_time_diff};
use crate::parsers::base::get_parser;
use crate::info::{get_line_info, get_commit_diff};
use std::path::Path;
use std::fs::{read_to_string};
use std::time::SystemTime;
use git2::{Repository, Oid};

pub fn check_file(repo: &Repository, working_dir: &Path, file: &Path) {
    let patterns = [".git", ".swp", "node_modules"]; // TODO: add global pattern list, or read gitignore

    let ac = AhoCorasick::new(&patterns);
    let f = file.to_str().unwrap();
    if ac.is_match(f) {
        return;
    }

    let ext = match file.extension() {
        Some(ext) => ext.to_str().unwrap(),
        _ => {
            return;
        }
    };

    let parser = match get_parser(&file, &patterns) {
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

    let blame_lines = match get_line_info(&repo, &relative_path) {
        Ok(lines) => lines,
        Err(e) => {
            println!("{}", e);
            println!("Failed checking blame for {}. Error: {}, skipping", file.display(), e);
            return;
        }
    };

    let all_funs = match parser.parse(&read) {
        Ok(funs) => funs,
        Err(e) => {
            println!("Failed to parse file {}. Error: {}, skipping", file.display(), e);
            return;
        }
    };

    for (comment, function) in all_funs {
        let comment_line = get_latest_line(&blame_lines, &comment);
        let fun_line = get_latest_line(&blame_lines, &function);

        let comment_info = blame_lines.get(&comment_line).expect("Failed to get comment info from blame lines");
        let fun_info = blame_lines.get(&fun_line).expect("Failed to get function info from blame lines");

        if fun_info.time < comment_info.time {
            return;
        }

        if check_control(&blame_lines, &function) {
            println!("passed control");
            return;
        }

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        let time_diff = unix_time_diff(current_time.into(), comment_info.time.into());
        let commit_diff = get_commit_diff(&repo, &Oid::from_str(&comment_info.commit).unwrap()).expect("Failed to get commit diff");

        let line = function.start.line - 1;
        let character = function.start.character;

        println!("{}", time_diff);
        println!("{} commits since update", commit_diff);
        println!("{}:{}:{}", file.display(), line, character);
        print_symbol(&function, &comment, &file, ext);
        println!("");
    }

}