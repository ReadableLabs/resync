use std::path::Path;
use clap::{Arg, Command};
use resync::sync;
use resync::info;

fn main() {
    let matches = Command::new("Resync")
        .version("0.1")
        .author("Nevin P. <me@nevin.cc>")
        .about("Keep track of out of sync comments")
        .arg(Arg::new("dir")
             .short('d')
             .long("dir")
             .help("Sets the directory for resync to work in"))
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
        .get_matches();

    let working_dir = Path::new(matches.value_of("dir").unwrap_or("/home/nevin/Desktop/testinit"));

    if matches.is_present("sync") {
        match sync::sync(working_dir) {
            Ok(result) => {
                println!("Succesfully synced, result {}", result);
            }
            Err(e) => {
                println!("Failed to sync, error {}", e);
            }
        }
        println!("synced");
        println!("Value of config {}", "hi");
    }

    if matches.is_present("info") {
        match info::get_line_info(working_dir, Path::new("myFile.txt")) {
            Ok(lines) => {
                println!("succesfully got blame");
            }
            Err(e) => {
                println!("Error blaming {}", e);
            }
        }
    }

    println!("Hello, world!");
}
