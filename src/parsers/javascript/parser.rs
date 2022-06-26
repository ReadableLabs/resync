use std::{path::PathBuf, fs::read_to_string, ops::Deref, borrow::Borrow, any::{Any, type_name}};

use swc_common::{sync::Lrc, comments::SingleThreadedComments, Spanned};
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, FilePathMapping, SourceMap,
};
use swc_ecma_ast::{ClassDecl, VarDecl, FnDecl};
use swc_ecma_parser::{lexer::Lexer, Parser as SwcParser, StringInput, Syntax};
use swc_ecma_visit::{Fold, FoldWith};

use crate::parsers::{Parser, types::{SymbolSpan, LineSpan}, javascript::visitor::JsVisitor};

pub struct JsParser;

impl Parser for JsParser {
    fn parse(&self, file: &PathBuf) -> Result<Vec<(crate::parsers::types::SymbolSpan, crate::parsers::types::SymbolSpan)>, &str> {
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
        let mut visitor = JsVisitor {
            fm,
            symbols: Vec::new()
        };

        visitor.fold_module(module);

        println!("done");
        // println!("{:#?}", module);

        panic!("Not implemented");
        }
    }


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