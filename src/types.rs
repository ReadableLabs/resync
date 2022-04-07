use nom_locate::{LocatedSpan};

#[derive(Debug)]
pub enum CommentType {
    Docstring,
    Free
}

pub type Span<'a> = LocatedSpan<&'a str>;

pub struct SymbolPosition<'a> {
    pub start:  Span<'a>,
    pub end:    Span<'a>
}
