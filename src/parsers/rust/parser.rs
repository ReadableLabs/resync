use std::str;
use std::vec::Vec;
use crate::parsers::{
    types::{CommentType, SymbolPosition, Span, SymbolSpan, LineSpan},
    base::Parser};
use syn::{Attribute, Expr, Result, File, ItemFn, Item, ImplItem, visit::{self, Visit}};
use syn::spanned::Spanned;

// use syn::__private::ToTokens;

pub struct RsParser;

struct FnVisitor {
    symbols: Vec<(SymbolSpan, SymbolSpan)>,
}

// will try to get these merged into upstream syn so this code doesn't have to be here.
// Also if you have an idea of how I can walk the entire syntax tree without repetitive code all
// help is welcome
pub fn get_attrs_impl_item(item: &ImplItem) -> Option<&Vec<Attribute>> {
    match item {
        ImplItem::Const(syn::ImplItemConst { attrs, ..})
            | ImplItem::Method(syn::ImplItemMethod { attrs, ..})
            | ImplItem::Type(syn::ImplItemType { attrs, .. })
            | ImplItem::Macro(syn::ImplItemMacro { attrs, .. }) => {
                Some(attrs)
            },
            _ => None
    }
}

/// Returns a shared reference to the attributes of the item.
pub fn get_attrs_item(item: &Item) -> Option<&Vec<Attribute>> {
    match item {
        Item::Const(syn::ItemConst { attrs, .. })
        | Item::Enum(syn::ItemEnum { attrs, .. })
        | Item::ExternCrate(syn::ItemExternCrate { attrs, .. })
        | Item::Fn(syn::ItemFn { attrs, .. })
        | Item::ForeignMod(syn::ItemForeignMod { attrs, .. })
        | Item::Impl(syn::ItemImpl { attrs, .. })
        | Item::Macro(syn::ItemMacro { attrs, .. })
        | Item::Macro2(syn::ItemMacro2 { attrs, .. })
        | Item::Mod(syn::ItemMod { attrs, .. })
        | Item::Static(syn::ItemStatic { attrs, .. })
        | Item::Struct(syn::ItemStruct { attrs, .. })
        | Item::Trait(syn::ItemTrait { attrs, .. })
        | Item::TraitAlias(syn::ItemTraitAlias { attrs, .. })
        | Item::Type(syn::ItemType { attrs, .. })
        | Item::Union(syn::ItemUnion { attrs, .. }) => {
            Some(attrs)
        },
        _ => None
    }
}

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item(&mut self, node: &'ast Item) {
        println!("{:#?}", node);

        let fun = node.span();
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

        println!("{:#?}", comment);
        println!("{:#?}", function);

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

        println!("{:#?}", comment);
        println!("{:#?}", function);

        self.symbols.push((comment, function));
    }
}

impl Parser for RsParser {
    // essentially check the attr for comment, and then check the span of the item.
    #[proc_macro_derive(MyMacro)]
    fn parse(&self, file_input: &str) -> Vec<(SymbolSpan, SymbolSpan)> {
        // let mut vec = Vec::new();
        println!("using rust parser");
        let ast = syn::parse_file(file_input).unwrap();
        let mut visitor = FnVisitor { symbols: Vec::new() };
        visitor.visit_file(&ast);

        println!("{}", visitor.symbols.len());

        return visitor.symbols;
    }
}

fn get_comment_range(attrs: &Vec<syn::Attribute>) -> Option<SymbolSpan> {
    if attrs.len() <= 0 {
        return None;
    }
    let mut start = attrs[0].path.get_ident().expect("Failed to get identifier").span().start();
    let mut end = attrs[0].path.get_ident().expect("Failed to get identifier").span().end();

    for attr in attrs {
        let ident = match attr.path.get_ident() {
            Some(i) => i,
            _ => {
                continue;
            }
        };

        if ident.to_string() != "doc" {
            continue;
        }

        let span = ident.span();

        if span.start().line < start.line {
            start = span.start();
        }

        if span.end().line > end.line {
            end = span.end();
        }
    }

    return Some(SymbolSpan {
        start: LineSpan {
            line: start.line,
            character: start.column
        },
        end: LineSpan {
            line: end.line,
            character: end.column
        }
    });
}

