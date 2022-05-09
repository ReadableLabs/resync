// [dependencies]
// proc-macro2 = { version = "1.0", features = ["span-locations"] }
// quote = "1.0"
// syn = { version = "1.0", features = ["full"] }

use proc_macro2::LineColumn;
use quote::quote;
use syn::ItemImpl;

fn main() {
    let input = r#"
        impl Thing {
            // non-doc comment
            fn f(&self) {}
            // also comment
            fn g(&self) {}
        }
    "#;

    let impl_block: ItemImpl = syn::parse_str(input).unwrap();
    let mut cur = byte_offset(input, impl_block.brace_token.span.start()) + 1;
    for method in impl_block.items {
        let mut tokens = quote!(#method).into_iter();
        let first = tokens.next().unwrap().span();
        let last = tokens.last().map_or(first, |last| last.span());
        let comment = &input[cur..byte_offset(input, first.start())];
        cur = byte_offset(input, last.end());
        println!("comment: {:?}", comment.trim());
    }
}

fn byte_offset(input: &str, location: LineColumn) -> usize {
    let mut offset = 0;
    for _ in 1..location.line {
        offset += input[offset..].find('\n').unwrap() + 1;
    }
    offset + input[offset..]
        .chars()
        .take(location.column)
        .map(char::len_utf8)
        .sum::<usize>()
}
