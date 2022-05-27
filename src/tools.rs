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
    for line in start..=end {
        // println!("{}", line);
        let line_info = blame_info.get(&line).expect("Failed to get line at blame");
        if line_info.time > time {
            latest = line;
            time = line_info.time;
        }
    }

    return latest;
    // return latest;
}

/// Checks if one commit makes up more than x percent of a function.
/// 
/// Eg: if commit aaaa doesn't make up more than 40% of a function, it returns false
pub fn check_control(blame_info: &HashMap<usize, LineInfo>, symbol: &SymbolSpan) -> bool {
    let mut map: HashMap<u64, f32> = HashMap::new();
    let mut total_lines: f32 = 0.0;

    for line in symbol.start.line..symbol.end.line {
        let line_info = blame_info.get(&line).expect("Failed to get line");
        let count = map.entry(line_info.time).or_insert(0.0);
        *count += 1.0;
        total_lines += 1.0;
    }

    for (_time, amount) in &map {
        let control = amount / total_lines;
        if control > 0.40 {
            return true;
        }
    }

    return false;
}

pub fn print_symbol(function: &SymbolSpan, comment: &SymbolSpan, file: &Path, language: &str) {
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

    PrettyPrinter::new()
    .input_file(file)
    .language(language)
    .line_numbers(true)
    .line_ranges(LineRanges::from(ranges))
    .print()
    .unwrap();
    println!("");
}

pub fn unix_time_diff(current: u128, prev: u128) -> String {
    let elapsed = current - prev;

    let s_per_min = 60;
    let s_per_hour= s_per_min * 60;
    let s_per_day= s_per_hour * 24;
    let s_per_month= s_per_day * 30;
    let s_per_year = s_per_day * 365;

    if elapsed < s_per_min {
        return format!("{} seconds ago", elapsed / 1000);
    }

    else if elapsed < s_per_hour {
        return format!("{} minutes ago", elapsed / s_per_min);
    }

    else if elapsed < s_per_day {
        return format!("{} hours ago", elapsed / s_per_hour);
    }

    else if elapsed < s_per_month {
        return format!("{} days ago", elapsed / s_per_day);
    }

    else if elapsed < s_per_year {
        return format!("{} months ago", elapsed / s_per_month);
    }

    else {
        return format!("{} years ago", elapsed / s_per_year);
    }
}