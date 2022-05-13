use syn::{Item, ImplItem, Attribute};
use crate::parsers::types::{SymbolSpan, LineSpan};
use std::vec::Vec;

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

/// Gets the range of a comment, and returns none if there is no comment in the attribute
pub fn get_comment_range(attrs: &Vec<Attribute>) -> Option<SymbolSpan> {
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

