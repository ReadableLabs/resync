use std::{path::PathBuf, fs::read_to_string, ops::Deref, borrow::Borrow};

use crate::parsers::Parser;
use dprint_swc_ecma_ast_view::{with_ast_view_for_module, ClassDecl};
use dprint_swc_ext::view::{ProgramInfo, Comments, ModuleDecl};
use swc_common::{self, sync::Lrc, SourceMap, Spanned, EqIgnoreSpan, comments::SingleThreadedComments};
use swc_ecma_ast::{ModuleItem, Program};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser as ECMAParser, StringInput, Syntax, EsConfig, token::TokenAndSpan};

use deno_ast::{parse_module, SourceTextInfo, ParseParams, MediaType::JavaScript};
use std::sync::Arc;

pub struct JsParser;

impl Parser for JsParser {
    fn parse(&self, file: &PathBuf) -> Result<Vec<(crate::parsers::types::SymbolSpan, crate::parsers::types::SymbolSpan)>, &str> {
        // first 
        let text = read_to_string(file).expect("Failed to read file");

        // let comments = SingleThreadedComments::default();

        // let cm: Lrc<SourceMap> = Default::default();
        // let fm = cm.load_file(file.as_path()).expect("Failed to load file");
        // let lexer = Lexer::new(
        //     Syntax::Es(Default::default()),
        //     Default::default(),
        //     StringInput::from(&*fm),
        //     Some(&comments)
        // );

        // let mut parser = swc_ecma_parser::Parser::new_from(lexer);

        // let module = parser.parse_module().expect("Failed to parse module");
        // println!("{}", parser.input());

        // for i in module.body.iter() {
        // }



        // use rslint
        let text_info = SourceTextInfo::new(text.into());
        let parsed_source = parse_module(ParseParams {
            specifier: "file:///my_file.js".to_string(),
            media_type: JavaScript,
            text_info: text_info.clone(),
            capture_tokens: true,
            maybe_syntax: None,
            scope_analysis: false,
        }).expect("Should parse");

        let program = parsed_source.program();

        // let module = program.module().clone();


        // let (leading_comments, trailing_comments) = parsed_source.comments().as_single_threaded().take_all();
        // // let program: swc_ecmascript::ast::Program = Program::Module(parsed_source.module().to_owned());
        // let tokens = parsed_source.tokens();

        // let program_ref: dprint_swc_ext::view::ProgramRef = dprint_swc_ext::view::ProgramRef::Module(parsed_source.module());

        // let program_info = ProgramInfo {
        //     program: program_ref,
        //     text_info: Some(&text_info),
        //     comments: None,
        //     // comments: Some(&comments),
        //     tokens: Some(&tokens)
        // };

        // dprint_swc_ecma_ast_view::with_ast_view(program_info, |program| {
        //     for child in program.children() {
        //         println!("child")
        //     }
        // });



        // dprint_swc_ecma_ast_view::with_ast_view(program_info, |program| {
        // });

        // println!("{:#?}", comments.leading_map());

        // let i = parsed_source.tokens().body.iter();

        // let i = parsed_source.comments().get_vec();

        // for item in i {
        //     println!("{:#?}", item);
        // }
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
