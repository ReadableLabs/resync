use std::{path::PathBuf, fs::read_to_string};

use crate::parsers::Parser;
use swc_common::{self, sync::Lrc, SourceMap, Spanned, EqIgnoreSpan};
use swc_ecma_ast::{ModuleItem, Program};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser as ECMAParser, StringInput, Syntax, EsConfig, token::TokenAndSpan};

use deno_ast::{parse_module, SourceTextInfo, ParseParams, MediaType::JavaScript};
use std::sync::Arc;

pub struct JsParser;

impl Parser for JsParser {
    fn parse(&self, file: &PathBuf) -> Result<Vec<(crate::parsers::types::SymbolSpan, crate::parsers::types::SymbolSpan)>, &str> {
        let text = read_to_string(file).expect("Failed to read file");
        let text_info = SourceTextInfo::new(text.into());
        let parsed_source = parse_module(ParseParams {
            specifier: "file:///my_file.js".to_string(),
            media_type: JavaScript,
            text_info,
            capture_tokens: true,
            maybe_syntax: None,
            scope_analysis: false,
        }).expect("Should parse");

        let comments = parsed_source.comments();

        // println!("{:#?}", comments.leading_map());

        // let i = parsed_source.tokens().body.iter();

        let i = parsed_source.comments().get_vec();

        for item in i {
            println!("{:#?}", item);
        }
        // let source_text = Arc::new(text.as_str());

        // let text_info = SourceTextInfo::new(source_text);

        // let parsed_source = parse_module(ParseParams {
        //     specifier: "file://my-file.js".to_string(),
        //     media_type: JavaScript,
        //     text_info,
        //     capture_tokens: true,
        //     maybe_syntax: None,
        //     scope_analysis: false,
        // }).expect("Failed to parse");

        // println!("#{:#?}", parsed_source.comments());

        panic!("Not implemented");

    }
}
