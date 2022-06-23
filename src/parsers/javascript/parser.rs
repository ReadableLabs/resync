use std::{path::PathBuf, fs::read_to_string, ops::Deref, borrow::Borrow, any::{Any, type_name}};
use rslint_parser::{ast::BracketExpr, parse_text, AstNode, SyntaxToken, SyntaxNode, util, SyntaxNodeExt, SyntaxKind, Syntax};

use crate::parsers::Parser;

pub struct JsParser;

impl Parser for JsParser {
    fn parse(&self, file: &PathBuf) -> Result<Vec<(crate::parsers::types::SymbolSpan, crate::parsers::types::SymbolSpan)>, &str> {
        let text = read_to_string(file).unwrap();

        let parse = parse_text(text.as_str(), 0);

        let nodes = parse.syntax().children().into_iter();

        for node in nodes {
            // println!("{:#?}", node.contains_comments());

            if node.contains_comments() {
                for descendant in node.descendants() {
                    println!("{}", descendant);
                    println!("{:#?}", node.kind());
                    println!("descendant");
                }
                // println!("{:#?}", node.text_range().start())
                // for child in node.children().into_iter() {
                //     println!("{}", child);
                //     println!("child");
                // }
            }
        }

        panic!("Not implemented");
    }
}
