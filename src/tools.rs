use std::collections::HashMap;
use std::str;
use crate::parsers::types::{SymbolPosition, SymbolSpan, LineSpan};

pub fn get_max_time(blame_info: &HashMap<usize, u64>, symbol: &SymbolSpan) -> u64 {
    let start = symbol.start.line - 1; // because symbol is 1 indexed
    let end = symbol.end.line - 1;

    let mut time = 0;
    for line in start..end {
        let line_info = *blame_info.get(&line).expect("Failed to get line at blame");
        if line_info > time {
            time = line_info;
        }
    }
    return time;
}

pub fn print_comment(lines: &Vec<&str>, comment: &SymbolSpan) {
    let comment_start = comment.start.line - 1;
    let comment_end = comment.end.line;

    for line in comment_start..comment_end {
        println!("{}: |{}", line, lines[usize::try_from(line).unwrap()]);
    }
}

pub fn print_function(lines: &Vec<&str>, function: &SymbolSpan) {
    let function_start = function.start.line - 1;
    let function_end = function.end.line;

    // do +3 and show line numbers
    for line in function_start..function_end {
        println!("{}: |{}", line, lines[usize::try_from(line).unwrap()]);
    }

}

