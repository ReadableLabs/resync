use std::str;
use std::vec::Vec;
use crate::parsers::{
    types::{CommentType, SymbolPosition, Span, SymbolSpan, LineSpan},
    rust::{
        visitor::RsVisitor,
        tools::{get_attrs_item, get_attrs_impl_item, get_comment_range}},
    base::Parser};
use syn::{Attribute, Expr, File, ItemFn, Item, ImplItem, visit::{self, Visit}};
use syn::spanned::Spanned;

pub struct RsParser;

impl Parser for RsParser {
    #[proc_macro_derive(MyMacro)]
    fn parse(&self, file_input: &str) -> Result<Vec<(SymbolSpan, SymbolSpan)>, &str> {
        // println!("using rust parser");
        let ast = match syn::parse_file(file_input) {
            Ok(ast) => ast,
            Err(_) => {
                return Err("Failed to parse file");
            }
        };

        // file parser maybe
        let mut visitor = RsVisitor { symbols: Vec::new() };
        visitor.visit_file(&ast);
        // println!("{:#?}", visitor.symbols);

        // println!("{}", visitor.symbols.len());

        Ok(visitor.symbols)
    }
}

