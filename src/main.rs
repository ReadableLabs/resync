use std::path::Path;
use clap::{Arg, Command};
use resync::sync;
use resync::info;
use std::env;

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
        .get_matches();


    // let path = match env::current_dir() {
    //     Ok(path) => {
    //         println!(path.display());
    //     }
    //     Err(e) => {
    //         println!("{}", e);
    //     }

    // }
    let working_dir = match matches.value_of("dir") {
        Some(dir) => {
            println!("{}", dir);
            dir
            // Ok(dir);
        },
        None => {
            // Ok("");
            println!("it be none");
            "hi"
        },
    };
    println!("{}", working_dir);
    // let working_dir = Path::new(matches.value_of("dir").unwrap_or(&path.into_os_string().into_string().unwrap()));

    /*
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

    */
    println!("Hello, world!");
}
