use std::str;
use std::vec::Vec;
use crate::parsers::{
    types::{CommentType, SymbolPosition, Span, SymbolSpan, LineSpan},
    base::Parser};
use syn::{Expr, Result};
use syn::spanned::Spanned;

// use syn::__private::ToTokens;

pub struct RsParser;

impl Parser for RsParser {
    #[proc_macro_derive(MyMacro)]
    fn parse(&self, file_input: &str) -> Vec<(SymbolSpan, SymbolSpan)> {
        let mut vec = Vec::new();
        println!("using rust parser");
        let ast = syn::parse_file(file_input).unwrap();

        for item in &ast.items {
            match item {
                syn::Item::Fn(expr) => {
                    match parseFun(expr) {
                        Some(symbol) => {
                            vec.push(symbol);
                        },
                        _ => {
                            println!("no symbol")
                        }
                    };
                },
                _ => {
                    println!("none");
                }
            }
        }
        return vec;
    }
}

fn parseFun(expr: &syn::ItemFn) -> Option<(SymbolSpan, SymbolSpan)> {
    if expr.attrs.len() <= 0 {
        println!("len is none");
        return None;
    }

    let mut start = expr.attrs[0].path.get_ident().expect("Failed to get identifier").span().start();
    let mut end = expr.attrs[0].path.get_ident().expect("Failed to get identifier").span().end();
    for attr in &expr.attrs {
        let ident = match attr.path.get_ident() {
            Some(i) => i,
            _ => {
                continue;
            }
        };

        if ident.to_string() != "doc" {
            continue;
        }

        println!("ident");

        let span = ident.span();

        if span.start().line < start.line {
            start = span.start();
        }

        if span.end().line > end.line {
            end = span.end();
        }

        println!("{:#?}", ident.span().start().line);
        println!("{:#?}", ident.span().end().line);
    }
    let comment = SymbolSpan {
        start: LineSpan {
            line: start.line,
            character: start.column
        },
        end: LineSpan {
            line: end.line,
            character: end.column
        }
    };

    let fun = expr.block.span();

    let function = SymbolSpan {
        start: LineSpan {
            line: fun.start().line,
            character: fun.start().column
        },
        end: {
            LineSpan {
                line: fun.end().line,
                character: fun.end().column
            }
        }
    };

    println!("{:#?}", comment);
    println!("{:#?}", function);
    return Some((comment, function));
    // println!("{:#?}", expr.block.span().start().line);
    // println!("{:#?}", expr.block.span().end().line);
    // println!("{:#?}", expr.attrs[0].path.segments[0].ident.to_string());
    // println!("{:#?}", expr);
    // println!("{:#?}", expr.span().start());
    // println!("{:#?}", expr.span().end());
    // println!("fun");
}
