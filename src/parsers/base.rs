use crate::parsers::{
    rust::parser::RsParser,
    types::SymbolSpan};
use std::vec::Vec;
use aho_corasick::AhoCorasick;
use std::path::Path;

pub fn get_parser(file: &Path, ignore_patterns: &[&str]) -> Option<Box<dyn Parser>> {
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

    match extension {
        /*
        "js" => Box::new(JsParser {}),
        "jsx" => Box::new(JsParser {}),
        "ts" => Box::new(TsParser {}),
        "tsx" => Box::new(TsParser {}),
        */
        "rs" => Some(Box::new(RsParser {})),
        _ => {
            println!("Language '{}' for {} not supported, continuing.", extension, f);
            None
        }
    }
}

pub trait Parser {
    fn parse(&self, file_input: &str) -> Vec<(SymbolSpan, SymbolSpan)>;
}
