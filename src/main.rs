use std::path::Path;
use aho_corasick::AhoCorasick;
use std::fs::{read_to_string};
use clap::{Arg, Command};
use walkdir::WalkDir;
use resync::sync;
use resync::info;
use pathdiff::diff_paths;
use resync::parsers::base::get_parser;
use resync::parsers::types::Span;
use resync::tools::{get_max_time, print_comment, print_function};
use std::ffi::OsStr;

fn main() {
    let matches = Command::new("Resync")
        .version("0.1")
        .author("Nevin P. <me@nevin.cc>")
        .about("Keep track of out of sync comments")
        .arg(Arg::new("dir")
             .short('d')
             .long("dir")
             .help("Sets the directory for resync to work in")
             .takes_value(true))
        .arg(Arg::new("sync")
             .short('s')
             .long("sync")
             .help("Syncs the directory"))
        .arg(Arg::new("check")
             .short('c')
             .long("check")
             .help("Checks a specific file for out of sync comments")
             .takes_value(false)
             )
        .arg(Arg::new("info")
             .short('i')
             .long("info")
             .help("Outputs time each line has changed"))
        .arg(Arg::new("porcelain")
             .short('p')
             .long("porcelain")
             .help("Format designed for machine consumption"))
        .arg(Arg::new("m")
             .short('m')
             .long("use-master-branch")
             .help("Whether or not to use master branch to get blame data"))
        .get_matches();

    // replace it all with a character very specific, so the line ranges don't get messed up, and
    // then repalce all the specific chars with nothing at once

    let working_dir = Path::new(matches.value_of("dir").unwrap_or("/home/nevin/Desktop/testinit"));
    // get current working dir
    println!("{}", working_dir.display());
    // let all_funs = get_all_functions(Span::new("/*asdgasdgasdg\n*/\npublic function myFun2() {\nsome code\n}\n /*jadskg*/ my code part three ajkdshadskjgadshgjahgdsj\nasdgadshugadsg\n /*sidg*/\npublic function myFun3() {\nsome more code\n}\n /*hoiasdhgoisag*/\npublic function myFun4() {\nasdgasagsdgdas\n}"));

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

    if matches.is_present("check") {
        let patterns = [".git", ".swp", "node_modules"];

        let ac = AhoCorasick::new(&patterns);
        // read gitignore if there is one
        for file in WalkDir::new(working_dir).into_iter().filter_map(|e| e.ok()) {
            let f = file.path().to_str().unwrap();
            if ac.is_match(f) {
                continue;
            }
            match file.path().extension() {
                Some(ext) => {},
                None => {
                    continue;
                }
            }

            println!("{}", f);

            // println!("{}", file.file_name().to_os_string().into_string().unwrap());
            // println!("{}", file.path().to_str().unwrap());
            // let file = matches.value_of("check").expect("Error: no file given");
            // let full_dir = working_dir.join(file);
            let read = read_to_string(file.path()).expect("Failed to read file");
            let lines: Vec<&str> = read.split("\n").collect();

            let relative_path = diff_paths(file.path(), working_dir).unwrap();
            println!("{}", relative_path.display());

            let blame_lines = info::get_line_info(working_dir, &relative_path).expect("Error blaming file");

            let ext = file.path().extension().and_then(OsStr::to_str).unwrap();

            let parser = match get_parser(ext) {
                Some(parser) => parser,
                _ => {
                    continue;
                }
            };
            let all_funs = parser.parse(&read);

            for (comment, function) in all_funs {
                let comment_time = get_max_time(&blame_lines, &comment);
                
                let function_time = get_max_time(&blame_lines, &function);

                if function_time > comment_time {
                    let line = function.start.line - 1;
                    let character = function.start.character;
                    println!("{}:{}:{}", file.path().display(), line, character);
                    print_comment(&lines, &comment);
                    println!("");
                    println!("Is out of sync with...");
                    print_function(&lines, &function);
                    println!("");
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

    if matches.is_present("info") {
        match info::get_line_info(working_dir, Path::new("myFile.txt")) {
            Ok(lines) => {
                println!("succesfully got blame");
                for (key, value) in lines {
                    println!("{}:{}", key, value);
                }
            }
            Err(e) => {
                println!("Error blaming {}", e);
            }
        }
    }

    println!("Hello, world!");
}

