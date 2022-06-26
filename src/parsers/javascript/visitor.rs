use std::{rc::Rc, borrow::Borrow};

use swc_common::{SourceFile, Span};
use swc_ecma_visit::{Fold, VisitAll, Visit};

use crate::parsers::types::SymbolSpan;

pub struct JsVisitor {
    pub fm: Rc<SourceFile>,
    pub spans: Vec<Span>,
    pub symbols: Vec<(SymbolSpan, SymbolSpan)>,
}

impl Visit for JsVisitor {
    fn visit_decl(&mut self,n: &swc_ecma_ast::Decl) {
    }
}

impl Fold for JsVisitor
{
    fn fold_decl(&mut self, decl:swc_ecma_ast::Decl) -> swc_ecma_ast::Decl {
        println!("got on decl");
        let span = match decl {
            swc_ecma_ast::Decl::Class(ref e) => {
                // for member in e.class.body.iter() {
                //     self.fold_class_member(*member);
                // }
                e.class.span
            },
            swc_ecma_ast::Decl::Fn(ref e) => e.function.span,
            swc_ecma_ast::Decl::Var(ref e) => e.span,
            swc_ecma_ast::Decl::TsInterface(ref e) => e.span,
            swc_ecma_ast::Decl::TsTypeAlias(ref e) => e.span,
            swc_ecma_ast::Decl::TsEnum(ref e) => e.span,
            swc_ecma_ast::Decl::TsModule(ref e) => e.span,
        };

        // if decl.is_class() {
        //     self.fold_class_members(decl.as_class().unwrap().class.body);
        //     println!("{:#?}", decl.as_class().unwrap().class.body);
        // }

        println!("{:#?}", span);
        println!("folding");
        self.spans.push(span);
        decl
    }

    fn fold_class_members(&mut self,n:Vec<swc_ecma_ast::ClassMember>) -> Vec<swc_ecma_ast::ClassMember> {
        n
    }

    fn fold_class(&mut self,n:swc_ecma_ast::Class) -> swc_ecma_ast::Class {
        println!("folding class");
        n
    }

    fn fold_class_member(&mut self, member: swc_ecma_ast::ClassMember) -> swc_ecma_ast::ClassMember {
        println!("got here");
        member
    }

    fn fold_class_method(&mut self, method: swc_ecma_ast::ClassMethod) -> swc_ecma_ast::ClassMethod {
        println!("method decl");
        println!("{:#?}", method.span);
        self.spans.push(method.span);
        method
    }
}