use std::collections::HashMap;
use std::str;
use std::str::{Lines};
use crate::types::{SymbolPosition};

pub fn get_max_time(blame_info: &HashMap<u32, u64>, symbol: &SymbolPosition) -> u64 {
    let mut max_time = 0;
    let start = symbol.start.location_line() - 1; // because symbol is 1 indexed
    let end = symbol.end.location_line() - 1;

    let mut time = 0;
    for line in start..end {
        let line_info = blame_info.get(&line).expect("Failed to get line at blame");
        if line_info > &time{
            time = *line_info;
        }
    }
    return time;
}

pub fn print_comment(lines: &Vec<&str>, comment: &SymbolPosition) {
    let comment_start = comment.start.location_line() - 1;
    let comment_end = comment.end.location_line() - 1;

    for line in comment_start..comment_end {
        println!("{}: |{}", line, lines[usize::try_from(line).unwrap()]);
    }
}

pub fn print_function(lines: &Vec<&str>, function: &SymbolPosition) {
    let function_start = function.start.location_line() - 1;
    let function_end = function.end.location_line() - 1;

    // do +3 and show line numbers
    for line in function_start..function_end {
        println!("{}: |{}", line, lines[usize::try_from(line).unwrap()]);
    }

}

