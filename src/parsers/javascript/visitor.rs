use swc_ecma_visit::Fold;

use crate::parsers::types::SymbolSpan;

pub struct JsVisitor {
    pub symbols: Vec<(SymbolSpan, SymbolSpan)>,
}

impl Fold for JsVisitor
{
    fn fold_decl(&mut self, decl:swc_ecma_ast::Decl) -> swc_ecma_ast::Decl {
        println!("got on decl");
        let span = match decl {
            swc_ecma_ast::Decl::Class(ref e) => e.class.span,
            swc_ecma_ast::Decl::Fn(ref e) => e.function.span,
            swc_ecma_ast::Decl::Var(ref e) => e.span,
            swc_ecma_ast::Decl::TsInterface(ref e) => e.span,
            swc_ecma_ast::Decl::TsTypeAlias(ref e) => e.span,
            swc_ecma_ast::Decl::TsEnum(ref e) => e.span,
            swc_ecma_ast::Decl::TsModule(ref e) => e.span,
        };

        println!("{}", span.lo().0);
        self.fold_decl(decl)
    }
}