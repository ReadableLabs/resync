use nom_locate::{LocatedSpan};


/// The comment type of a function
#[derive(Debug)]
pub enum CommentType {
    Docstring,
    Free
    // Inline todo
}

/// not used
/// See https://docs.rs/nom_locate/latest/nom_locate/struct.LocatedSpan.html
pub type Span<'a> = LocatedSpan<&'a str>;

/// deprecated
/// The symbol position for either a comment or a function. Is used as range.
pub struct SymbolPosition<'a> {
    pub start:  Span<'a>,
    pub end:    Span<'a>
}

#[derive(Debug)]
pub struct LineSpan {
    pub line: usize,
    pub character: usize,
}


#[derive(Debug)]
pub struct SymbolSpan {
    pub start: LineSpan,
    pub end: LineSpan,
}

