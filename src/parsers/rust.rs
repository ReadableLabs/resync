use std::str;
use std::vec::Vec;
use crate::parsers::{
    types::{CommentType, SymbolPosition, Span, SymbolSpan, LineSpan},
    base::Parser};
use syn::{Attribute, Expr, Result, File, ItemFn, Item, visit::{self, Visit}};
use syn::spanned::Spanned;

// use syn::__private::ToTokens;

pub struct RsParser;

struct FnVisitor {
    symbols: Vec<(SymbolSpan, SymbolSpan)>,
}

/// Returns a shared reference to the attributes of the item.
pub fn get_attrs(item: &Item) -> Option<&Vec<Attribute>> {
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
            println!("ok");
            Some(attrs)
        },
        _ => None
    }
}

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item(&mut self, node: &'ast Item) {
        match node {
            Item::Fn(item) => {

                let span = item.span();
                let attrs = &item.attrs;

                // return item, get attr, get span from item, return both
                println!("GOT ITEM SPAN");
                println!("GOT ITEM SPAN");
                println!("GOT ITEM SPAN");
                println!("GOT ITEM SPAN");
                println!("GOT ITEM SPAN");
                println!("GOT ITEM SPAN");
                println!("GOT ITEM SPAN");
                println!("GOT ITEM SPAN");
                println!("GOT ITEM SPAN");
                println!("GOT ITEM SPAN");
            },
            _ => {
            }
        }
        let attrs = match get_attrs(node) {
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

        // and then subtract the span
        // println!("{:#?}", item.span());
        // println!("{:#?}", attrs);
        // let mut tokens = quote!(#node);
        // println!("{:#?}", tokens);
        return visit::visit_item(self, node);
    }
}

impl Parser for RsParser {
    // essentially check the attr for comment, and then check the span of the item.
    #[proc_macro_derive(MyMacro)]
    fn parse(&self, file_input: &str) -> Vec<(SymbolSpan, SymbolSpan)> {
        let mut vec = Vec::new();
        println!("using rust parser");
        let ast = syn::parse_file(file_input).unwrap();
        let mut visitor = FnVisitor { symbols: Vec::new() };
        visitor.visit_file(&ast);

        println!("{}", visitor.symbols.len());

        for attr in &ast.attrs {
        }

        for item in &ast.items {
            match item {
                syn::Item::Fn(expr) => {
                    match parseFun(expr) {
                        Some(symbol) => {
                            vec.push(symbol);
                        },
                        _ => {
                            println!("no symbol")
                        }
                    };
                },
                syn::Item::Impl(expr) => {
                    let mut symbols = parseImpl(&expr);
                    vec.append(&mut symbols);
                    // println!("impl");
                }
                _ => {
                    // println!("none");
                }
            }
        }

        /*
        for symbol in &vec {
            println!("{:#?}", symbol);
        }
        */

        return vec;
    }
}

fn parseImpl(expr: &syn::ItemImpl) -> Vec<(SymbolSpan, SymbolSpan)> {
    let mut vec: Vec<(SymbolSpan, SymbolSpan)> = Vec::new();
    if expr.attrs.len() > 0 {
        match get_comment_range(&expr.attrs) {
            Some(comment) => {

                let fun = expr.span();

                let function = SymbolSpan {
                    start: LineSpan {
                        line: fun.start().line,
                        character: fun.start().column
                    },
                    end: LineSpan {
                        line: fun.end().line,
                        character: fun.end().column
                    }
                };

                vec.push((comment, function));
            },
            _ => {
            }
        }
        // check for comment above or in impl, and add that into the vec
    }

    for item in &expr.items {
        match item {
            // now it's basically a function
            syn::ImplItem::Method(method) => {
                let comment = match get_comment_range(&method.attrs) {
                    Some(comment) => comment,
                    _ => {
                        continue;
                    }
                };

                let fun = method.block.span();

                let function = SymbolSpan {
                    start: LineSpan {
                        line: fun.start().line,
                        character: fun.start().column
                    },
                    end: LineSpan {
                        line: fun.end().line,
                        character: fun.end().column
                    }
                };

                vec.push((comment, function))
            },
            _ => {}
        }
    }

    return vec;
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

fn parseFun(expr: &syn::ItemFn) -> Option<(SymbolSpan, SymbolSpan)> {
    if expr.attrs.len() <= 0 {
        return None;
    }

    let comment = match get_comment_range(&expr.attrs) {
        Some(comment) => comment,
        _ => {
            return None;
        }
    };

    let fun = expr.block.span();

    let function = SymbolSpan {
        start: LineSpan {
            line: fun.start().line,
            character: fun.start().column
        },
        end: {
            LineSpan {
                line: fun.end().line,
                character: fun.end().column
            }
        }
    };

    // println!("{:#?}", comment);
    // println!("{:#?}", function);
    return Some((comment, function));
    // println!("{:#?}", expr.block.span().start().line);
    // println!("{:#?}", expr.block.span().end().line);
    // println!("{:#?}", expr.attrs[0].path.segments[0].ident.to_string());
    // println!("{:#?}", expr);
    // println!("{:#?}", expr.span().start());
    // println!("{:#?}", expr.span().end());
    // println!("fun");
}
