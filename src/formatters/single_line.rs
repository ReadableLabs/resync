use std::{path::PathBuf, ffi::OsStr};

use crate::{formatters::Formatter, parsers::types::SymbolSpan};

pub struct SingleLineFormatter;

impl Formatter for SingleLineFormatter {
    fn output(&self, _: &SymbolSpan, comment: &SymbolSpan, file: &PathBuf, _: &str, time_diff: &String, commit_diff: &usize) {
        let file_name = file.file_name().and_then(OsStr::to_str).unwrap();
        println!("{}\t{}\t{}\t{}\t{}\t{}", time_diff, commit_diff, file.display(), file_name, comment.start.line, comment.end.line);
    }
}

