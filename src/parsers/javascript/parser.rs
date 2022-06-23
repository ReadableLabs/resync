use std::{path::PathBuf, fs::read_to_string, ops::Deref, borrow::Borrow, any::{Any, type_name}};
use rslint_parser::{ast::BracketExpr, parse_text, AstNode, SyntaxToken, SyntaxNode, util, SyntaxNodeExt, SyntaxKind, Syntax, JsLanguage, SyntaxNodeChildren, JsNum};

use crate::parsers::{Parser, types::SymbolSpan};

pub struct JsParser;

impl Parser for JsParser {
    fn parse(&self, file: &PathBuf) -> Result<Vec<(crate::parsers::types::SymbolSpan, crate::parsers::types::SymbolSpan)>, &str> {
        let text = read_to_string(file).unwrap();

        let parse = parse_text(text.as_str(), 0);

        let nodes = parse.syntax().children().into_iter();

        for node in nodes {
            let parent = node;

            while parent.children().into_iter().count() {
                let child = parent.children();
                // println!("{}", child);
            }

            if !node.contains_comments() {
                continue;
            }

            let tokens = node.tokens();

            let comment = tokens.iter().find(|tok| tok.kind() == SyntaxKind::COMMENT).expect("Failed to find comment");
            println!("{}", comment.parent());

            // if node.contains_comments() {
            //     for descendant in node.descendants() {
            //         println!("{}", descendant);
            //         println!("{:#?}", node.kind());
            //         println!("descendant");
            //     }
            //     // println!("{:#?}", node.text_range().start())
            //     // for child in node.children().into_iter() {
            //     //     println!("{}", child);
            //     //     println!("child");
            //     // }
            // }
        }
        panic!("Not implemented");
    }
}

fn recursive_visitor(node: SyntaxNode) -> SyntaxNodeChildren {
    return node.children();
}

fn check_all_children(nodes: SyntaxNodeChildren) -> Vec<SymbolSpan> {
    for node in nodes {
    }
    return Vec::new();
}
