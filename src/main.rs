use std::io;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Resync")
        .version("0.1")
        .author("Nevin P. <me@nevin.cc>")
        .about("Keep track of out of sync comments")
        .arg(Arg::with_name("dir")
             .short("d")
             .long("dir")
             .help("Sets the directory for resync to work in"))
        .arg(Arg::with_name("sync")
             .short("s")
             .long("sync")
             .help("Syncs the directory"))
        .arg(Arg::with_name("info")
             .short("i")
             .long("info")
             .help("Outputs time each line has changed"))
        .get_matches();

    let working_dir = matches.value_of("dir").unwrap_or("/home/nevin/Desktop/testinit");
    println!("Value of config {}", working_dirl);
    println!("Hello, world!");
}
