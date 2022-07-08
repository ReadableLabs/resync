use std::{path::PathBuf, fs::read_to_string};

use swc_common::{sync::Lrc, comments::SingleThreadedComments};
use swc_common::SourceMap;
use swc_ecma_parser::{lexer::Lexer, Parser as SwcParser, StringInput, Syntax};
use swc_ecma_visit::Visit;

use crate::parsers::{Parser, types::SymbolSpan, javascript::{visitor::JsVisitor, comment_parser::parse_comments}};

pub struct JsParser {
    pub ts: bool,
}

impl Parser for JsParser {
    fn parse(&self, file: &PathBuf) -> Result<Vec<(crate::parsers::types::SymbolSpan, crate::parsers::types::SymbolSpan)>, &str> {
        let text = read_to_string(&file).expect("Failed to read file");
        let comments: SingleThreadedComments = Default::default();
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.load_file(file).expect("Failed to load file");

        let syntax = match self.ts {
            true => Syntax::Typescript(Default::default()),
            false => Syntax::Es(Default::default())
        };

        let lexer = Lexer::new(
            syntax,
            // Syntax::Es(Default::default()),
            Default::default(),
            StringInput::from(&*fm),
            Some(&comments)
        );

        let mut parser = SwcParser::new_from(lexer);

        let module = match parser.parse_module() {
            Ok(module) => module,
            Err(_) => {
                return Err("Failed to parse file");
            }
        }; //.expect(
        let mut visitor = JsVisitor::new(&text);

        visitor.visit_module(&module);

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

        // println!("done");

        Ok(matched_symbols)
    }
}


    

