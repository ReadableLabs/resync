use std::path::PathBuf;

use crate::{formatters::Formatter, parsers::types::SymbolSpan, tools::print_symbol};

pub struct TerminalFormatter;

impl Formatter for TerminalFormatter {
    fn output(&self, function: &SymbolSpan, comment: &SymbolSpan, file: &PathBuf, language: &str, time_diff: &String, commit_diff: &usize) {
        println!("{}", time_diff);
        println!("{} commits since update", commit_diff); // change red green or yellow text changed a lot, little, or changed
        println!("{}:{}:{}", file.display(), function.start.line - 1, function.start.character);
        print_symbol(&function, &comment, &file, &language, &time_diff, &commit_diff);
    }
}

