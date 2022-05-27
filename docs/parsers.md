## Parsers

Table of Contents

- About parsers
- Writing a new parser
- FAQ

### About parsers

Resync uses parsers to get comment and symbol pairs from source code. It then checks the ranges of the found symbols to see if a comment is out of sync. The parsers aren't used for getting any specific things such as arguments or function names, they're just used for getting the ranges of both comments, and the symbol that the comment refers to.

### Writing a parser

All parsers implement `Parser` found in `parsers/base.rs`. When you call `get_parser`, the function checks the extension of the file passed in, and matches it to the specific parser.

To add a parser, implement `Parser`, make sure the parser returns a vector containing a tuple of `SymbolSpan`s. The first `SymbolSpan` in the tuple is the range of the comment, and the second `SymbolSpan` is the range of whichever symbol came below it.

If you need an example of a parser which implements `Parser`, check `parsers/rust/parser.rs`.

### FAQ

- Which library should I use for parsing?
  - Whichever one is easiest (and preferably lightweight) for parsing said language. The rust parser uses syn but for typescript you could use swc.
