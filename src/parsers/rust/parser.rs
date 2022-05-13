use std::str;
use std::vec::Vec;
use crate::parsers::{
    types::{CommentType, SymbolPosition, Span, SymbolSpan, LineSpan},
    rust::visitor::RsVisitor,
    base::Parser};
use syn::{Attribute, Expr, Result, File, ItemFn, Item, ImplItem};
use syn::spanned::Spanned;

pub struct RsParser;

impl Parser for RsParser {
    #[proc_macro_derive(MyMacro)]
    fn parse(&self, file_input: &str) -> Vec<(SymbolSpan, SymbolSpan)> {
        println!("using rust parser");
        let ast = syn::parse_file(file_input).unwrap();
        // file parser maybe
        let mut visitor = RsVisitor { symbols: Vec::new() };
        visitor.visit_file(&ast);
        println!("{:#?}", visitor.symbols);

        println!("{}", visitor.symbols.len());

        return visitor.symbols;
    }
}

