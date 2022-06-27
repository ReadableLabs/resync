use std::{path::PathBuf, fs::read_to_string, ops::Deref, borrow::Borrow, any::{Any, type_name}};

use swc_common::{sync::Lrc, comments::SingleThreadedComments, Spanned};
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, FilePathMapping, SourceMap,
};
use swc_ecma_ast::{ClassDecl, VarDecl, FnDecl};
use swc_ecma_parser::{lexer::Lexer, Parser as SwcParser, StringInput, Syntax};
use swc_ecma_visit::{Fold, FoldWith, Visit};

use crate::parsers::{Parser, types::{SymbolSpan, LineSpan}, javascript::{visitor::JsVisitor, comment_parser::parse_comments}};

pub struct JsParser;

impl Parser for JsParser {
    fn parse(&self, file: &PathBuf) -> Result<Vec<(crate::parsers::types::SymbolSpan, crate::parsers::types::SymbolSpan)>, &str> {
        let text = read_to_string(&file).expect("Failed to read file");
        let comments: SingleThreadedComments = Default::default();
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.load_file(file).expect("Failed to load file");

        let lexer = Lexer::new(
            Syntax::Es(Default::default()),
            Default::default(),
            StringInput::from(&*fm),
            Some(&comments)
        );

        let mut parser = SwcParser::new_from(lexer);

        let module = parser.parse_module().expect("Failed to parse module");
        let mut visitor = JsVisitor::new(&text);

        visitor.visit_module(&module);
        // visitor.spans.iter().for_each(|span| {
            // println!("{:#?}", to_symbol_span(&text, span.lo.0, span.hi.0));
        // });

        let comments = parse_comments(&text);

        let mut matched_symbols: Vec<(SymbolSpan, SymbolSpan)> = Vec::new();

        for comment in &comments {
            // println!("comment");
            // println!("{:#?}", comment.end.line);

            for symbol in &visitor.symbols {
                if symbol.start.line - 1 == comment.end.line {
                    matched_symbols.push((comment.clone(), symbol.clone()));
                }

                // this will speed up search but will not allow classes to be found
                // else if symbol.start.line - 1 > comment.end.line {
                //     break;
                // }
            }
        }

        println!("done");

        Ok(matched_symbols)
    }
}


    

