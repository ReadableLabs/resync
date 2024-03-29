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
use git2::Repository;
use std::env::current_dir;
use std::fs::{File, remove_file};

fn main() {
    let matches = Command::new("Resync")
        .version("1.3")
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
        .arg(Arg::new("reset-db")
            .short('r')
            .long("reset-db")
            .help("Resets resyncs internal db"))
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

    let debug = matches.is_present("reset-db");

    let repo = Repository::discover(working_dir).expect("Failed to open repository");
    let porcelain = matches.is_present("porcelain");

    let config = Config::new(porcelain);
    let db = config.open_db(debug);

    // make .file, sync, and then delete file to make sure branch is made
    let temp_file = working_dir.join(".resync");


    if File::create(&temp_file).is_err() && porcelain == false {
        println!("Failed creating resync temp file. Repo might not be synced");
    };

    match sync::sync(&repo) {
        Ok(_result) => {
        },
        Err(e) => {
            if porcelain != true {
                println!("Failed to sync. Error: {}", e);
            }
        }
    }

    if remove_file(&temp_file).is_err() && porcelain == false  {
        println!("Failed removing resync temp file. You can remove it manually by deleting '.resync' in the project root.");
    };

    if porcelain == false {
        println!("Searching for out of sync comments...");
    }

    let patterns = [".git", ".swp", "node_modules"]; // TODO: add global pattern list, or read gitignore

    let ac = AhoCorasick::new(&patterns);

    let mut checker = Checker::new(repo, working_dir.to_path_buf(), ac, porcelain, db);

    if matches.is_present("check-file") {
        let mut ignore_files: Vec<String> = Vec::new();
        let file = matches.value_of("check-file").unwrap(); // file is relative path
        let full_path = Path::join(working_dir, file);

        checker.check_file(full_path, &mut ignore_files);

        std::process::exit(0);
    }

    checker.check_working_dir();
}

