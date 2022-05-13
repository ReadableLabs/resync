use crate::parsers::{
    //javascript::JsParser,
    //typescript::TsParser,
    rust::parser::RsParser,
    types::{Span, SymbolPosition, SymbolSpan}};
use std::vec::Vec;
use std::process::exit;

pub fn get_parser(language: &str) -> Option<Box<dyn Parser>> {
    match language {
        /*
        "js" => Box::new(JsParser {}),
        "jsx" => Box::new(JsParser {}),
        "ts" => Box::new(TsParser {}),
        "tsx" => Box::new(TsParser {}),
        */
        "rs" => Some(Box::new(RsParser {})),
        _ => {
            println!("Error: language '{}' not supported. Please open an issue at https://github.com/ReadableLabs/resync, or consider opening a pull request to add it", language);
            None
        }
    }
}

pub trait Parser {
    fn parse(&self, file_input: &str) -> Vec<(SymbolSpan, SymbolSpan)>;
}
