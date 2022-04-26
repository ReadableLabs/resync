use std::str;
use std::vec::Vec;
use crate::parsers::{
    types::{CommentType, SymbolPosition, Span},
    base::Parser};
use syn::{Expr, Result};
use syn::spanned::Spanned;
use proc_macro;

// use syn::__private::ToTokens;

pub struct RsParser;

impl Parser for RsParser {
    #[proc_macro_derive(MyMacro)]
    fn parse(&self, file_input: &str) -> bool {
        println!("using rust parser");
        let ast = syn::parse_file(file_input).unwrap();
        // println!("{:#?}", ast);
        println!("{:#?}", ast.items[0]);
        // println!("{:#?}", &ast.items[0].span().start());
        match &ast.items[0] {
            syn::Item::Fn(expr) => {
                println!("{:#?}", expr.attrs.len());
                println!("{:#?}", expr.span().start());
                println!("fun");
            },
            _ => {
                println!("none");
            }
        }
        return true;
    }
}
