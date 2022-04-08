use nom_locate::{LocatedSpan};


/// The comment type of a function
/// Docstring - the comment is over a function
/// Free - the comment is over a random peice of code
/// Inline (TODO) - inline comment
#[derive(Debug)]
pub enum CommentType {
    Docstring,
    Free
    // Inline todo
}

/// See https://docs.rs/nom_locate/latest/nom_locate/struct.LocatedSpan.html
pub type Span<'a> = LocatedSpan<&'a str>;

/// The symbol position for either a comment or a function. Is used as range.
pub struct SymbolPosition<'a> {
    pub start:  Span<'a>,
    pub end:    Span<'a>
}
