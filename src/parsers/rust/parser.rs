use std::str;
use std::vec::Vec;
use crate::parsers::{
    types::SymbolSpan,
    rust::
    visitor::RsVisitor,
    base::Parser};
use syn::visit::Visit;

pub struct RsParser;

impl Parser for RsParser {
    fn parse(&self, file_input: &str) -> Result<Vec<(SymbolSpan, SymbolSpan)>, &str> {
        let ast = match syn::parse_file(file_input) {
            Ok(ast) => ast,
            Err(_) => {
                return Err("Failed to parse file");
            }
        };

        let mut visitor = RsVisitor { symbols: Vec::new() };
        visitor.visit_file(&ast);

        Ok(visitor.symbols)
    }
}

