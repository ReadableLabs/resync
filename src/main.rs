use std::path::Path;
use clap::{Arg, Command};
use resync::sync;
use resync::info;
use resync::parser::get_fun_range;
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

    let working_dir = Path::new(matches.value_of("dir").unwrap_or("/home/nevin/Desktop/testinit"));
    println!("{}", working_dir.display());
    // let parsed = get_fun_range(Span::new("myFun2 = () => {\nthis is text inside of a function\n}")).unwrap();
    let parsed2 = get_fun_range(Span::new("myFun2() {\nthis is text inside of a function\n}")).unwrap();
    // let text = parsed.1.start_pos.location_line();
    // let second = parsed.1.end_pos.location_line();
    // println!("{}:{}", text, second);
    /*
    println!("{}", parsed.1.mid.location_offset());
    println!("{}", parsed.1.end.location_line());
    */
    // println!("{}", std::str::from_utf8(hi).unwrap());
    // let working_dir = Path::new(matches.value_of("dir").unwrap_or(&path.into_os_string().into_string().unwrap()));

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
