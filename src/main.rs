use std::path::Path;
use std::fs::{read_to_string, File};
use std::io::{self, prelude::*, BufReader};
use clap::{Arg, Command};
use resync::sync;
use resync::info;
use resync::parser::{get_fun_range, get_all_functions};
use nom::Finish;
use nom::error::ParseError;
use resync::parser::Span;

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
        let read = read_to_string("/home/nevin/Desktop/testinit/myFile.txt").expect("Failed to read file");

        let file = File::open("/home/nevin/Desktop/testinit/myFile.txt").expect("Failed to find file");
        let reader = BufReader::new(file);

        let mut lines = reader.lines();
        let third = lines.nth(3).unwrap();

        println!("line - {}", third.unwrap());

        let blame_lines = info::get_line_info(working_dir, Path::new("myFile.txt")).expect("Error blaming file");
        let all_funs = get_all_functions(Span::new(&read));

        for (comment, function) in all_funs {
            let mut comment_line = comment.start.location_line() - 1; // because it's 1 indexed
            let mut comment_end_line = comment.end.location_line() - 1;

            let mut max_comment_time = 0;
            for line in comment_line..comment_end_line {
                if blame_lines.get(&line).expect("Failed to get line at blame") > &max_comment_time {
                    max_comment_time = *blame_lines.get(&line).expect("Failed to get line of blame");
                }
            }

            let mut function_line = function.start.location_line() - 1;
            let mut function_end_line = function.end.location_line() - 1;

            let mut max_function_time = 0;
            for line in function_line..function_end_line {
                if blame_lines.get(&line).expect("Failed to get line at blame") > &max_function_time {
                    max_function_time = *blame_lines.get(&line).expect("Failed to get line of blame");
                }
            }

            if max_comment_time < max_function_time {
                // TODO: print comment from ranges
                println!("{} - {} is out of sync", function_line, function_end_line);
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

