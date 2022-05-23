mod info;
mod parsers;
mod tools;
mod sync;

use std::{path::Path, any::Any};
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
             .help("Checks a specific file for out of sync comments")
            .takes_value(true))
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
    let porcelain = matches.is_present("porcelain");
    println!("{}", porcelain);

    if matches.is_present("sync") {
        match sync::sync(working_dir) {
            Ok(result) => {
                println!("Succesfully synced {}", result);
            },
            Err(e) => {
                println!("Failed to sync. Error: {}", e);
            }
        }
    }

    if matches.is_present("check-dir") {
        let patterns = [".git", ".swp", "node_modules"]; // TODO: add global pattern list, or read gitignore

        let ac = AhoCorasick::new(&patterns);
        for file in WalkDir::new(working_dir).into_iter().filter_map(|e| e.ok()) {
            check_file(&repo, &working_dir, &file.path(), &ac, &porcelain);
        }
    }

    if matches.is_present("check-file") {
        let patterns = [".git", ".swp", "node_modules"]; // TODO: add global pattern list, or read gitignore
        let ac = AhoCorasick::new(&patterns);

        let file = matches.value_of("check-file").unwrap(); // file is relative path
        // get parent dir from working dir before doing this
        let full_path = Path::join(working_dir, file);

        check_file(&repo, &working_dir, &full_path, &ac, &porcelain);
    }
    println!("Hello, world!");
}

