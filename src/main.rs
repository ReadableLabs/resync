use std::path::Path;
use std::fs::{read_to_string};
use clap::{Arg, Command};
use resync::sync;
use resync::info;
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
             .takes_value(true)
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
        let file = matches.value_of("check").expect("Error: no file given");
        let full_dir = working_dir.join(file);
        let read = read_to_string(working_dir.join(file)).expect("Failed to read file");
        let lines: Vec<&str> = read.split("\n").collect();

        let blame_lines = info::get_line_info(working_dir, Path::new(file)).expect("Error blaming file");

        let ext = Path::new(file).extension().and_then(OsStr::to_str).expect("Failed to find file extension");
        println!("ext: {}", ext);

        let parser = get_parser(ext);
        let all_funs = parser.parse(Span::new(&read));
        // let all_funs = get_all_functions(Span::new(&read));

        for (comment, function) in all_funs {
            /*
            let comment_line = comment.start.location_line() - 1; // because it's 1 indexed
            let comment_end_line = comment.end.location_line() - 1;

            let mut max_comment_time = 0;
            for line in comment_line..comment_end_line {
                if blame_lines.get(&line).expect("Failed to get line at blame") > &max_comment_time {
                    max_comment_time = *blame_lines.get(&line).expect("Failed to get line of blame");
                }
            }

            let function_line = function.start.location_line() - 1;
            let function_end_line = function.end.location_line() - 1;

            let mut max_function_time = 0;
            for line in function_line..function_end_line {
                if blame_lines.get(&line).expect("Failed to get line at blame") > &max_function_time {
                    max_function_time = *blame_lines.get(&line).expect("Failed to get line of blame");
                }
            }
            */
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
                /*
                for line in comment_line..comment_end_line {
                    println!("{}", lines.nth(usize::try_from(comment_line + line).unwrap()).unwrap());
                }
                println!("Is out of sync with...");
                for function_line in function_line..function_end_line { // may break, make sure is less than end line
                    println!("{}", line);
                    println!("{}", lines.nth(usize::try_from(function_line + line).unwrap()).unwrap());
                }
                // TODO: print comment from ranges
                println!("{} - {} is out of sync", function_line, function_end_line);
                */
            }
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

