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

pub fn check_control(blame_info: &HashMap<usize, LineInfo>, symbol: &SymbolSpan) -> bool {
    let mut map: HashMap<u64, f32> = HashMap::new();
    let mut total_lines: f32 = 0.0;
    for line in symbol.start.line..symbol.end.line {
        let line_info = blame_info.get(&line).expect("Failed to get line");
        let count = map.entry(line_info.time).or_insert(0.0);
        *count += 1.0;
        total_lines += 1.0;
    }

    for (time, amount) in &map {
        let control = amount / total_lines;
        if control > 0.40 {
            return true;
        }
    }

    return false;
}

pub fn print_symbol(lines: &Vec<&str>, function: &SymbolSpan, comment: &SymbolSpan, file: &Path, language: &str) {
    let function_start = function.start.line;
    let function_end = function.end.line;

    let comment_start = comment.start.line;
    let comment_end = comment.end.line;

    let new_function_end = match function_start + 3 >= function_end {
        true => function_end,
        false => function_start + 3,
    };

    let ranges = vec!(
    LineRange::new(
        function_start,
        new_function_end
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

pub fn unix_time_diff(current: u128, prev: u128) -> String {
    let elapsed = current - prev;

    let msPerMin = 60 * 1000;
    let msPerHour= msPerMin * 60;
    let msPerDay= msPerHour * 24;
    let msPerMonth= msPerDay * 30;
    let msPerYear = msPerDay * 365;

    if elapsed < msPerMin {
        return format!("{} seconds ago", elapsed / 1000);
    }

    else if elapsed < msPerHour {
        return format!("{} minutes ago", elapsed / msPerMin);
    }

    else if elapsed < msPerDay {
        return format!("{} hours ago", elapsed / msPerHour);
    }

    else if elapsed < msPerMonth {
        return format!("{} days ago", elapsed / msPerDay);
    }

    else if elapsed < msPerYear {
        return format!("{} months ago", elapsed / msPerMonth);
    }

    else {
        return format!("{} years ago", elapsed / msPerYear);
    }
}