pub mod info;
pub mod parsers;
pub mod tools;
pub mod sync;
pub mod config;
pub mod formatters;

use std::path::Path;
use aho_corasick::AhoCorasick;
use resync::check::Checker;
use resync::config::Config;
use clap::{Arg, Command};
use walkdir::WalkDir;
use git2::Repository;
use std::env::current_dir;
use std::fs::{File, remove_file};
use dirs;

fn main() {
    let matches = Command::new("Resync")
        .version("0.1")
        .author("Nevin Puri <me@nevin.cc>")
        .about("Easily find out of sync comments")
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
        .arg(Arg::new("dir")
             .help("Sets working dir")
             .short('d')
             .long("dir")
            .takes_value(true))
        .get_matches();

    let current_dir_pathbuf = current_dir().unwrap();
    let current_dir = current_dir_pathbuf.as_path();
    // get default dir: TODO
    let working_dir = match matches.value_of("dir") {
        Some(value) => Path::new(value),
        None => current_dir,
    };

    let config = Config {};
    let db = config.open_db(true);

    let repo = Repository::open(working_dir).expect("Failed to open repository");
    let porcelain = matches.is_present("porcelain");

    // make .file, sync, and then delete file to make sure branch is made
    let temp_file = working_dir.join(".resync");

    File::create(&temp_file).unwrap();

    match sync::sync(working_dir) {
        Ok(_result) => {
            // if porcelain != true {
            //     println!("Succesfully synced {}", result);
            // }
        },
        Err(e) => {
            if porcelain != true {
                println!("Failed to sync. Error: {}", e);
            }
        }
    }

    remove_file(&temp_file).unwrap(); // check if result is none or not

    if porcelain == false {
        println!("Searching for out of sync comments...");
    }

    let patterns = [".git", ".swp", "node_modules"]; // TODO: add global pattern list, or read gitignore

    let ac = AhoCorasick::new(&patterns);

    let mut checker = Checker::new(repo, working_dir.to_path_buf(), ac, porcelain, db);

    if matches.is_present("check-file") {
        let file = matches.value_of("check-file").unwrap(); // file is relative path
        // get parent dir from working dir before doing this
        let full_path = Path::join(working_dir, file);

        checker.check_file(full_path);
        // check_file(&repo, &working_dir, &full_path, &ac, &porcelain);

        std::process::exit(0);
    }

    checker.check_working_dir();


    // println!("Hello, world!");
}

