pub mod types;
pub mod rust;
pub mod javascript;

use crate::parsers::{
    rust::parser::RsParser,
    javascript::parser::JsParser,
    types::SymbolSpan};
use std::{vec::Vec, path::PathBuf};
use aho_corasick::AhoCorasick;
use std::path::Path;

pub fn get_parser(file: &PathBuf, ignore_patterns: &[&str]) -> Option<Box<dyn Parser>> {
    let ac = AhoCorasick::new(ignore_patterns);
    let f = file.to_str().unwrap();

    if ac.is_match(f) {
        return None;
    }

    let extension = match file.extension() {
        Some(ext) => {
            ext.to_str().unwrap()
        },
        _ => {
            return None;
        }
    };

    // just use ecma instead of js or ts or any of these, and do the check in the file
    match extension {
        "js" => Some(Box::new(JsParser {ts: false})),
        "jsx" => Some(Box::new(JsParser {ts: false})),
        "ts" => Some(Box::new(JsParser {ts: true})),
        /*
        "tsx" => Box::new(TsParser {}),
        */
        "rs" => Some(Box::new(RsParser {})),
        _ => {
            // println!("Language '{}' for {} not supported, continuing.", extension, f);
            None
        }
    }
}

pub trait Parser {
    fn parse(&self, file: &PathBuf) -> Result<Vec<(SymbolSpan, SymbolSpan)>, &str>;
}