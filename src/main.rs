mod info;
mod parsers; // base::get_parser
mod tools;
mod sync;

use std::path::Path;
use aho_corasick::AhoCorasick;
use std::io::{stdin, stdout, Read, Write};
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

    let working_dir = Path::new(matches.value_of("dir").unwrap_or("/home/nevin/Desktop/testinit"));

    if matches.is_present("sync") {
        match sync::sync(working_dir) {
            Ok(result) => {
                println!("Succesfully synced. {}", result);
            }
            Err(e) => {
                println!("Failed to sync. {}", e);
            }
        }
        println!("synced");
        println!("Value of config {}", "hi");
    }

    if matches.is_present("check-dir") {
        let patterns = [".git", ".swp", "node_modules"];

        let ac = AhoCorasick::new(&patterns);
        // read gitignore if there is one
        for file in WalkDir::new(working_dir).into_iter().filter_map(|e| e.ok()) {
            let f = file.path().to_str().unwrap();
            if ac.is_match(f) {
                continue;
            }

            // check if file is directory
            let ext = match file.path().extension() {
                Some(ext) => ext.to_str().unwrap(),
                None => {
                    continue;
                }
            };

            // println!("{}", file.file_name().to_os_string().into_string().unwrap());
            // println!("{}", file.path().to_str().unwrap());
            // let file = matches.value_of("check").expect("Error: no file given");
            // let full_dir = working_dir.join(file);
            // let commit_diff = info::get_commit_diff(working_dir, "56454c97", "affe6a76").unwrap();
            // println!("changed commits, {}", commit_diff);

            // let ext = file.path().extension().and_then(OsStr::to_str).unwrap();

            // parser should be first, and then try to read
            let parser = match get_parser(file.path(), &patterns) {
                Some(parser) => parser,
                _ => {
                    continue;
                }
            };
            let read = match read_to_string(file.path()) {
                Ok(read) => read,
                Err(_) => {
                    println!("Failed to read file {}, skipping", file.path().display());
                    continue;
                }
            };
            let lines: Vec<&str> = read.split("\n").collect();

            let relative_path = diff_paths(file.path(), working_dir).unwrap();
            // println!("{}", relative_path.display());

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

                if function_info.time > comment_info.time {
                    if check_control(&blame_lines, &function) {
                        continue;
                    }
                    // println!("latest line - {} - {}", comment_info.time, function_info.time);
                    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                    let time_diff = unix_time_diff(current_time.into(), comment_info.time.into());
                    println!("{}", time_diff);
                    let commit_diff = info::get_commit_diff(&repo, &Oid::from_str(&function_info.commit).unwrap(), &Oid::from_str(&comment_info.commit).unwrap()).expect("Failed to get commit diff");
                    println!("{} commits since update", commit_diff);
                    // if commit_diff < 20 {
                    //     continue;
                    // }
                    let line = function.start.line - 1;
                    let character = function.start.character;
                    let file_path = file.path();
                    println!("{}:{}:{}", file.path().display(), line, character);
                    // print_symbol(&lines, &comment, &file_path, ext);
                    // println!("");
                    // println!("Is out of sync with...");
                    print_symbol(&lines, &function, &comment, &file_path, ext);
                    println!("");
                    // let mut stdout = stdout();
                    // stdout.flush().unwrap();
                    // stdin().read(&mut [0]).unwrap();
                }
            }

            // let all_funs = get_all_functions(Span::new(&read));

            /*
            for (comment, function) in all_funs {
                let comment_time = get_max_time(&blame_lines, &comment);
                let function_time = get_max_time(&blame_lines, &function);

                if comment_time < function_time {
                    let line_number = function.start.location_line() - 1;
                    let char_number = function.start.get_column();
                    println!("{}:{}:{}", full_dir.display(), line_number, char_number);
                    print_comment(&lines, &comment);
                    println!("");
                    println!("Is out of sync with...");
                    print_function(&lines, &function);
                    println!("");
                }
            }
            */
        }
    }

    // if matches.is_present("check-file") {
        // match info::get_line_info(working_dir, Path::new("myFile.txt")) {
        //     Ok(_) => {
                println!("succesfully got blame");
                // for (key, value) in lines {
                    // println!("{}:{}", key, value);
                // }
        //     }
        //     Err(e) => {
        //         println!("Error blaming {}", e);
        //     }
        // }
    // }

    println!("Hello, world!");
}

