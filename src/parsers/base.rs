use crate::parsers::{
    javascript::JsParser,
    typescript::TsParser,
    types::{Span, SymbolPosition}};
use std::vec::Vec;
use std::process::exit;

pub fn get_parser(language: &str) -> Box<dyn Parser> {
    match language {
        "js" => Box::new(JsParser {}),
        "jsx" => Box::new(JsParser {}),
        "ts" => Box::new(TsParser {}),
        "tsx" => Box::new(TsParser {}),
        _ => {
            println!("Error: language not supported. Please open an issue at https://github.com/ReadableLabs/resync, or consider opening a pull request to add it");
            exit(-1);
        }
    }
}

pub trait Parser {
    fn parse<'a>(&self, file_input: Span<'a>) -> Vec<(SymbolPosition<'a>, SymbolPosition<'a>)>;
}
