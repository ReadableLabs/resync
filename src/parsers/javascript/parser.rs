use std::{path::PathBuf, fs::read_to_string, ops::Deref, borrow::Borrow, any::{Any, type_name}};
use rslint_parser::{ast::BracketExpr, parse_text, AstNode, SyntaxToken, SyntaxNode, util, SyntaxNodeExt, SyntaxKind, Syntax, JsLanguage, SyntaxNodeChildren, JsNum};

use crate::parsers::{Parser, types::{SymbolSpan, LineSpan}};

pub struct JsParser;

impl Parser for JsParser {
    fn parse(&self, file: &PathBuf) -> Result<Vec<(crate::parsers::types::SymbolSpan, crate::parsers::types::SymbolSpan)>, &str> {
        let text = read_to_string(file).unwrap();

        let mut stack: Vec<SyntaxNode> = Vec::new();

        let parse = parse_text(text.as_str(), 0);

        let nodes = parse.syntax().children().into_iter();

        let symbols: Vec<(SymbolSpan, SymbolSpan)> = Vec::new();

        for item in nodes {
            for child in item.children() {
                println!("{}", child);
                for other_child in child.children() {
                    println!("other child");
                    println!("{}", other_child);
                }
                println!("Ok");
            }
            // println!("{}", item);
            // visit(item);
            // let children = item.children().map(|child| stack.push(child));
            // and for each one of their children
            // children.map(|n| stack.push(n));
            // let comments = tokens.iter().filter(|token| token.kind() == SyntaxKind::COMMENT);

            // for comment in comments {
            //     // println!("{}", comment.siblings_with_tokens(rslint_parser::Direction::Next).next().unwrap());
            //     let range = comment.text_range();
            //     let parent_range = comment.parent().text_range();

            //     // kind of hacky, can't add one after into
            //     let mut fun_start: usize = range.end().into();
            //     fun_start += 1;

            //     let fun_end: usize = parent_range.end().into();

            //     // can be usize
            //     let comment_start: usize = range.start().into();
            //     let comment_end: usize = range.end().into();

            //     // println!("{:#?}", to_symbol_span(&text, comment_start, comment_end));
            //     // println!("{:#?}", to_symbol_span(&text, fun_start, fun_end));

            //     let comment_symbol = to_symbol_span(&text, comment_start, comment_end);
            //     let fun_symbol = to_symbol_span(&text, fun_start, fun_end);

            //     symbols.push((comment_symbol, fun_symbol));
            }
            Ok(symbols)
        }
    }


            // if !parent.contains_comments() {
            //     continue;
            // }

            // while parent.children().into_iter().count() > 0 {
            //     parent = parent.children().next().unwrap();
            //     // println!("{}", child);
            // }

            // if !node.contains_comments() {
            //     continue;
            // }

            // let tokens = node.tokens();

            // let comment = tokens.iter().find(|tok| tok.kind() == SyntaxKind::COMMENT).expect("Failed to find comment");
            // println!("{}", comment.parent());

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


fn to_line_span(text: &str, offset: usize) -> LineSpan {
    let mut char_idx: usize = 0;
    let mut line_idx: usize = 0;

    for (idx, char) in text.chars().enumerate() {
        if idx == offset {
            return LineSpan {
                character: char_idx,
                line: line_idx
            }
        }
        char_idx += 1;
        if char == '\n' {
            char_idx = 0;
            line_idx += 1;
        }
    }

    panic!("Failed to get line span");
}

fn visit(node: SyntaxNode) {
    println!("{}", node);
    node.children().for_each(visit);
}

fn to_symbol_span(text: &str, start: usize, end: usize) -> SymbolSpan {
    SymbolSpan {
        start: to_line_span(&text, start),
        end: to_line_span(&text, end)
    }
}
