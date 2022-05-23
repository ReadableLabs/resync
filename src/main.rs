mod info;
mod parsers;
mod tools;
mod sync;

use std::path::Path;
use aho_corasick::AhoCorasick;
use resync::check::check_file;
use std::fs::{read_to_string};
use clap::{Arg, Command};
use walkdir::WalkDir;
use tools::{get_latest_line, print_symbol, check_control};
use parsers::base::get_parser;
use git2::{Repository, Oid};
use pathdiff::diff_paths;
use std::time::SystemTime;

use crate::tools::unix_time_diff;

fn main() {
    let matches = Command::new("Resync")
        .version("0.1")
        .author("Nevin P. <me@nevin.cc>")
        .about("Keep track of out of sync comments")
        .arg(Arg::new("dir")
             .short('d')
             .long("dir")
             .help("Sets working dir")
             .takes_value(true))
        .arg(Arg::new("sync")
             .short('s')
             .long("sync")
             .help("Updates resync git branch with current working branch information (used with extension)"))
        .arg(Arg::new("check-dir")
             .short('c')
             .long("check-dir")
             .help("Checks a specific directory recursively for out of sync comments")
             .takes_value(false)
             )
        .arg(Arg::new("check-file")
             .short('i')
             .long("check-file")
             .help("Checks a specific file for out of sync comments"))
        .arg(Arg::new("porcelain")
             .short('p')
             .long("porcelain")
             .help("Output out of sync comments in format designed for machine consumption"))
        .arg(Arg::new("no-resync-branch")
             .short('m')
             .long("no-resync-branch")
             .help("Don't use or create a resync branch (out of sync comments won't be updated until you commit to your own branch)"))
        .get_matches();

        // get default dir: TODO
    let working_dir = Path::new(matches.value_of("dir").unwrap_or("/home/nevin/Desktop/testinit"));
    let repo = Repository::open(working_dir).expect("Failed to open repository");

    if matches.is_present("sync") {
        match sync::sync(working_dir) {
            Ok(result) => {
                println!("Succesfully synced {}", result);
            }
            Err(e) => {
                println!("Failed to sync. Error: {}", e);
            }
        }
    }

    if matches.is_present("check-dir") {
        let patterns = [".git", ".swp", "node_modules"]; // TODO: add global pattern list, or read gitignore

        let ac = AhoCorasick::new(&patterns);
        for file in WalkDir::new(working_dir).into_iter().filter_map(|e| e.ok()) {
            check_file(&repo, &working_dir, &file.path(), &ac);
            /*
            let f = file.path().to_str().unwrap();
            if ac.is_match(f) {
                continue;
            }

            let file_path = file.path();

            // check if file is directory
            let ext = match file_path.extension() {
                Some(ext) => ext.to_str().unwrap(),
                None => {
                    continue;
                }
            };

            let parser = match get_parser(&file_path, &patterns) {
                Some(parser) => parser,
                _ => {
                    continue;
                }
            };

            let read = match read_to_string(file_path) {
                Ok(read) => read,
                Err(_) => {
                    println!("Failed to read file {}, skipping", file.path().display());
                    continue;
                }
            };

            let relative_path = diff_paths(file.path(), working_dir).unwrap();

            let repo = Repository::open(working_dir).expect("Failed to open repo");

            let blame_lines = match info::get_line_info(&repo, &relative_path) {
                Ok(lines) => lines,
                Err(e) => {
                    println!("{}", e);
                    println!("Failed checking {}, continuing", file.path().display());
                    continue;
                }
            };
            let all_funs = match parser.parse(&read) {
                Ok(funs) => funs,
                Err(e) => {
                    println!("Failed to parse file. Error {}. Skipping", e);
                    continue;
                }
            };

            // make a module which checks all of these, checkall, which you can implement
            for (comment, function) in all_funs {
                let comment_idx = get_latest_line(&blame_lines, &comment);
                let fun_idx = get_latest_line(&blame_lines, &function);

                let comment_info = blame_lines.get(&comment_idx).expect("Failed to get comment from blame lines");
                let function_info = blame_lines.get(&fun_idx).expect("Failed to get function from blame lines");

                if function_info.time <= comment_info.time {
                    continue;
                }

                if check_control(&blame_lines, &function) {
                    continue;
                }

                let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                let time_diff = unix_time_diff(current_time.into(), comment_info.time.into());
                println!("{}", time_diff);
                let commit_diff = info::get_commit_diff(&repo, &Oid::from_str(&comment_info.commit).unwrap()).expect("Failed to get commit diff");
                println!("{} commits since update", commit_diff);

                let line = function.start.line - 1;
                let character = function.start.character;
                println!("{}:{}:{}", file_path.display(), line, character);

                print_symbol(&function, &comment, &file_path, ext);
                println!("");
            }
            */
        }
    }

    println!("Hello, world!");
}

