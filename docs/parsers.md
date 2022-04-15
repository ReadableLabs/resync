### PARSER DESIGN

### Table Of Contents
- About Parsers
- How they work in detail
- Writing a parser
- FAQ


### About Parsers
Resyncs parsers are responsible for getting all the comment + code pairs for a file, and then returning a vector containing them. All parsers are written using (nom - a parser combinator library)[https://github.com/Geal/nom], and use (nom\_locate)[https://github.com/fflorent/nom\_locate] for getting the position of the symbols.

The parsers themselves don't adhere to all the language standards - they just need to be able to get comments + functions. This means that if you were writing a parser for javascript, you wouldn't have to match all the language features. You would only have to parse for keywords such as ")" + whitespace + "{", or "=>" + whitespace + "{" to find functions.

```
# Comment			 Function
Vec<(SymbolPosition, SymbolPosition)>
```
See (types.rs)[TODO] to see what you need.

### How they work in detail

TODO - talk about what they implement and they're used.

Resync parsers work by first getting a comment, and then getting either the function beneath it, or the next few lines of code as decided by the user (see (config.md)[TODO] to see how to get this value).

Resyncs parsers work in a way that all they need to implement IResyncParser. All this needs to return is a vector containing tuples of SymbolPositions. The vector can be empty if no symbols are found.


### Writin a parser
TODO


### FAQ

Q: Why not just use other parsers (like ANTLR) - why make your own?
A: I wanted Resync to be able to run with a small file size, as well as be able to run with no packages the user would need to install manually. If we include a different library for parsing each file type, implementation will be more complicated, and the file size will be greatly impacted.

The idea is that we can have one interface which is created with a language, and then just call parser.parse() and automatically get the correct language parser, or an error if none is found.

