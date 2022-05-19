use std::collections::HashMap;
use std::str;
use std::path::Path;
use crate::info::LineInfo;
use crate::parsers::types::{SymbolSpan};
use bat::{
    line_range::{LineRange, LineRanges},
    PrettyPrinter};

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

pub fn print_symbol(lines: &Vec<&str>, function: &SymbolSpan, comment: &SymbolSpan, file: &Path, language: &str) {
    let function_start = function.start.line;
    let function_end = function.end.line;

    let comment_start = comment.start.line;
    let comment_end = comment.end.line;

    let ranges = vec!(
    LineRange::new(
        function_start,
        function_end
    ),
    LineRange::new(
        comment_start,
        comment_end
    ));

    let function_range = vec!(LineRange::new(function_start, function_end));

    PrettyPrinter::new()
        .input_file(file)
        .language(language)
        .line_numbers(true)
        .line_ranges(LineRanges::from(ranges))
        .print()
        .unwrap();

    // do +3 and show line numbers
    // for line in function_start..function_end {
    //     println!("{}: |{}", line, lines[usize::try_from(line).unwrap()]);
    // }

}

