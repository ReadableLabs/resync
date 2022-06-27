use std::fs::read_to_string;
use std::{str, path::PathBuf};
use std::vec::Vec;
use crate::parsers::{
    types::SymbolSpan,
    rust::
    visitor::RsVisitor,
    Parser};
use syn::visit::Visit;

pub struct RsParser;

impl Parser for RsParser {
    fn parse(&self, file: &PathBuf) -> Result<Vec<(SymbolSpan, SymbolSpan)>, &str> {
        let file_input = match read_to_string(&file) {
            Ok(read) => read,
            Err(e) => {
                return Err("Failed to read file");
            }
        };
        let ast = match syn::parse_file(&file_input) {
            Ok(ast) => ast,
            Err(_) => {
                return Err("Failed to parse file");
            }
        };

        let mut visitor = RsVisitor { symbols: Vec::new() };
        visitor.visit_file(&ast);

        // for symbol in &visitor.symbols {
        //     println!("{:#?}", symbol.0);
        // }

        Ok(visitor.symbols)
    }
}

