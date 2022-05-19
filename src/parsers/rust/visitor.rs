use crate::parsers::{
    types::{SymbolSpan, LineSpan},
    rust::tools::{get_attrs_item, get_attrs_impl_item, get_comment_range}};
use syn::{Item, ImplItem, visit::{self, Visit}};
use syn::spanned::Spanned;

/// keeps track of all the code + comment pairs
pub struct RsVisitor {
    pub symbols: Vec<(SymbolSpan, SymbolSpan)>,
}

impl<'ast> Visit<'ast> for RsVisitor {
    fn visit_item(&mut self, node: &'ast Item) {

        let fun = &node.span();
        // println!("{:#?}", span);

        let attrs = match get_attrs_item(node) {
            Some(attrs) => attrs,
            _ => {
                return visit::visit_item(self, node);
            }
        };

        let comment = match get_comment_range(attrs) {
            Some(comment) => comment,
            _ => {
                return visit::visit_item(self, node);
            }
        };

        let function = SymbolSpan {
            start: LineSpan {
                line: comment.end.line + 1,
                character: fun.start().column
            },
            end: LineSpan {
                line: fun.end().line,
                character: fun.end().column
            }
        };

        self.symbols.push((comment, function));

        return visit::visit_item(self, node);
    }

    fn visit_impl_item(&mut self, node: &'ast ImplItem) {
        let fun = node.span();

        let attrs = match get_attrs_impl_item(node) {
            Some(attrs) => attrs,
            _ => {
                return visit::visit_impl_item(self, node);
            }
        };

        let comment = match get_comment_range(attrs) {
            Some(comment) => comment,
            _ => {
                return visit::visit_impl_item(self, node);
            }
        };

        let function = SymbolSpan {
            start: LineSpan {
                line: comment.end.line + 1,
                character: fun.start().column
            },
            end: LineSpan {
                line: fun.end().line,
                character: fun.end().column
            }
        };

        self.symbols.push((comment, function));
    }
}

