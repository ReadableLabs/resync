use std::collections::HashMap;
use std::str;
use crate::info::LineInfo;
use crate::parsers::types::{SymbolSpan};

pub fn get_latest_line(blame_info: &HashMap<usize, LineInfo>, symbol: &SymbolSpan) -> usize {
    let start = symbol.start.line - 1; // because symbol is 1 indexed
    let end = symbol.end.line - 1;

    let mut latest = 0;
    let mut time = 0;
    // let mut latest = blame_info.get(&start).expect("Failed to get initial line");
    for line in start..end {
        let line_info = blame_info.get(&line).expect("Failed to get line at blame");
        if line_info.time > time {
            latest = line;
            time = line_info.time;
        }
    }

    return latest;
    // return latest;
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

