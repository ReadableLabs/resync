pub mod terminal;
pub mod porcelain;
pub mod single_line;

use std::path::PathBuf;

use crate::{formatters::{
    porcelain::PorcelainFormatter,
    terminal::TerminalFormatter,
    single_line::SingleLineFormatter
}, parsers::types::SymbolSpan};

pub trait Formatter {
    fn output(&self, function: &SymbolSpan, comment: &SymbolSpan, file: &PathBuf, language: &str, time_diff: &String, commit_diff: &usize);
}

pub fn get_formatter(porcelain: &bool) -> Box< dyn Formatter> {
    if *porcelain {
        Box::new(SingleLineFormatter {})
    }

    else {
        Box::new(TerminalFormatter {})
    }
}