use swc_common::Span;
use swc_ecma_visit::Visit;

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

    fn visit_expr(&mut self, expr: &swc_ecma_ast::Expr) {
        let span = match expr {
            swc_ecma_ast::Expr::This(e) => e.span,
            swc_ecma_ast::Expr::Array(e) => e.span,
            swc_ecma_ast::Expr::Object(e) => e.span,
            swc_ecma_ast::Expr::Fn(e) => e.function.span,
            swc_ecma_ast::Expr::Unary(e) => e.span,
            swc_ecma_ast::Expr::Update(e) => e.span,
            swc_ecma_ast::Expr::Bin(e) => e.span,
            swc_ecma_ast::Expr::Assign(e) => e.span,
            swc_ecma_ast::Expr::Member(e) => e.span,
            swc_ecma_ast::Expr::SuperProp(e) => e.span,
            swc_ecma_ast::Expr::Cond(e) => e.span,
            swc_ecma_ast::Expr::Call(e) => e.span,
            swc_ecma_ast::Expr::New(e) => e.span,
            swc_ecma_ast::Expr::Seq(e) => e.span,
            swc_ecma_ast::Expr::Ident(e) => e.span,
            swc_ecma_ast::Expr::Tpl(e) => e.span,
            swc_ecma_ast::Expr::TaggedTpl(e) => e.span,
            swc_ecma_ast::Expr::Arrow(e) => e.span,
            swc_ecma_ast::Expr::Class(e) => e.class.span,
            swc_ecma_ast::Expr::Yield(e) => e.span,
            swc_ecma_ast::Expr::MetaProp(e) => e.span,
            swc_ecma_ast::Expr::Await(e) => e.span,
            swc_ecma_ast::Expr::Paren(e) => e.span,
            swc_ecma_ast::Expr::JSXMember(e) => e.prop.span,
            swc_ecma_ast::Expr::JSXNamespacedName(e) => e.ns.span,
            swc_ecma_ast::Expr::JSXEmpty(e) => e.span,
            swc_ecma_ast::Expr::JSXElement(e) => e.span,
            swc_ecma_ast::Expr::JSXFragment(e) => e.span,
            swc_ecma_ast::Expr::TsTypeAssertion(e) => e.span,
            swc_ecma_ast::Expr::TsConstAssertion(e) => e.span,
            swc_ecma_ast::Expr::TsNonNull(e) => e.span,
            swc_ecma_ast::Expr::TsAs(e) => e.span,
            swc_ecma_ast::Expr::TsInstantiation(e) => e.span,
            swc_ecma_ast::Expr::PrivateName(e) => e.span,
            swc_ecma_ast::Expr::OptChain(e) => e.span,
            swc_ecma_ast::Expr::Invalid(e) => e.span,
            swc_ecma_ast::Expr::Lit(e) => {
                match e {
                    swc_ecma_ast::Lit::Str(n) => n.span,
                    swc_ecma_ast::Lit::Bool(n) => n.span,
                    swc_ecma_ast::Lit::Null(n) => n.span,
                    swc_ecma_ast::Lit::Num(n) => n.span,
                    swc_ecma_ast::Lit::BigInt(n) => n.span,
                    swc_ecma_ast::Lit::Regex(n) => n.span,
                    swc_ecma_ast::Lit::JSXText(n) => n.span,
                }
            },
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
        line: line_idx
    };
    // panic!("Failed to get line span");
}
