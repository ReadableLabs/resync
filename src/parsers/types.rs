use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LineSpan {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SymbolSpan {
    pub start: LineSpan,
    pub end: LineSpan,
}
