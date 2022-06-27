use std::{rc::Rc, borrow::Borrow};

use swc_common::{SourceFile, Span};
use swc_ecma_visit::{Fold, VisitAll, Visit, FoldWith};

use crate::parsers::types::{SymbolSpan, LineSpan};

pub struct JsVisitor<'a> {
    pub spans: Vec<Span>,
    pub symbols: Vec<SymbolSpan>,
    pub text: &'a str,
}

impl<'a> JsVisitor<'a> {
    pub fn new(text: &'a str) -> Self {
        JsVisitor {
            spans: Vec::new(),
            symbols: Vec::new(),
            text: text
        }
    }
}

impl<'a> Visit for JsVisitor<'a> {
    fn visit_decl(&mut self, decl: &swc_ecma_ast::Decl) {
        // println!("got on decl");
        let span = match decl {
            swc_ecma_ast::Decl::Class(ref e) => {
                for member in e.class.body.iter() {
                    self.visit_class_member(member);
                }
                e.class.span
            },

            swc_ecma_ast::Decl::Fn(ref e) => e.function.span,
            swc_ecma_ast::Decl::Var(ref e) => e.span,
            swc_ecma_ast::Decl::TsInterface(ref e) => e.span,
            swc_ecma_ast::Decl::TsTypeAlias(ref e) => e.span,
            swc_ecma_ast::Decl::TsEnum(ref e) => e.span,
            swc_ecma_ast::Decl::TsModule(ref e) => e.span,
        };

        self.symbols.push(to_symbol_span(&self.text, span.lo.0, span.hi.0));
    }

    fn visit_class_member(&mut self, member: &swc_ecma_ast::ClassMember) {
        let span = match member {
            swc_ecma_ast::ClassMember::Constructor(ref e) => e.span,
            swc_ecma_ast::ClassMember::Method(ref e) => e.span,
            swc_ecma_ast::ClassMember::PrivateMethod(e) => e.span,
            swc_ecma_ast::ClassMember::ClassProp(e) => e.span,
            swc_ecma_ast::ClassMember::PrivateProp(e) => e.span,
            swc_ecma_ast::ClassMember::TsIndexSignature(e) => e.span,
            swc_ecma_ast::ClassMember::Empty(e) => e.span,
            swc_ecma_ast::ClassMember::StaticBlock(e) => e.span,
        };
        self.symbols.push(to_symbol_span(&self.text, span.lo.0, span.hi.0));
        // println!("visiting class member");
    }

}

fn to_symbol_span(text: &str, start: u32, end: u32) -> SymbolSpan {
    return SymbolSpan {
        start: to_line_span(&text, usize::try_from(start).unwrap(), true),
        end: to_line_span(&text, usize::try_from(end).unwrap(), false)}
}


// Make return result
fn to_line_span(text: &str, offset: usize, start: bool) -> LineSpan {
    let mut char_idx: usize = 0;
    let mut line_idx: usize = 0;

    if start == true {
        line_idx += 1;
    }

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

    // this happens if file doesn't have empty line at the end
    return LineSpan {
        character: char_idx,
        line: line_dx
    };
    // panic!("Failed to get line span");
}
