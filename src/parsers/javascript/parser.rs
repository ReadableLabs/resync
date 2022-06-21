use std::path::PathBuf;

use crate::parsers::Parser;
use swc_common::{self, sync::Lrc, SourceMap};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser as ECMAParser, StringInput, Syntax, EsConfig};

pub struct JsParser;

impl Parser for JsParser {
    fn parse(&self, file: &PathBuf) -> Result<Vec<(crate::parsers::types::SymbolSpan, crate::parsers::types::SymbolSpan)>, &str> {
        let cm: Lrc<SourceMap> = Default::default();

        let fm = cm.load_file(file).unwrap();

        let lexer = Lexer::new(
            Syntax::Es(EsConfig {
                jsx: false,
                ..Default::default()
            }),
            Default::default(),
            StringInput::from(&*fm),
            None
        );

        let mut parser = ECMAParser::new_from(lexer);

        
        panic!("Not implemented");
    }
}