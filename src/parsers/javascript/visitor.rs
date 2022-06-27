use std::{rc::Rc, borrow::Borrow};

use swc_common::{SourceFile, Span};
use swc_ecma_visit::{Fold, VisitAll, Visit, FoldWith};

use crate::parsers::types::SymbolSpan;

pub struct JsVisitor {
    pub fm: Rc<SourceFile>,
    pub spans: Vec<Span>,
    pub symbols: Vec<(SymbolSpan, SymbolSpan)>,
}

impl Visit for JsVisitor {
    fn visit_decl(&mut self, decl: &swc_ecma_ast::Decl) {
        println!("got on decl");
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

        self.spans.push(span);
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
        self.spans.push(span);
        println!("visiting class member");
    }

}
