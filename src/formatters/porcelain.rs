use std::ffi::OsStr;
use std::path::PathBuf;

use crate::parsers::types::SymbolSpan;

use crate::formatters::Formatter;

pub struct PorcelainFormatter;

impl Formatter for PorcelainFormatter {
    fn output(&self, _: &SymbolSpan, comment: &SymbolSpan, file: &PathBuf, _: &str, time_diff: &String, commit_diff: &usize) {
        let file_name = file.file_name().and_then(OsStr::to_str).unwrap();
        println!("{}\n{}\n{}\n{}\n{}\n{}", time_diff, commit_diff, file.display(), file_name, comment.start.line, comment.end.line);
    }
}