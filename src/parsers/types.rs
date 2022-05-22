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
