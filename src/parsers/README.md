## Parsers

Comment
- Text
- Args\[\] (Will be none)
- Return

Code
- Type (Function/Free)
- Params\[\] (Will be None if function type is free)
- Return\[\] (Will be none)
- Range +2 down if empty, function range

Most parsers start by getting docstring comments, the info of them, as well as the correct function information.

Comment
- Args
- Return
- Notes
- Span

Function
- Args
- Return
- Notes
- Span

Next, inline comments are gotten.

Comment
- Span

Code
- Span (5 lines down)

Note if the types will be one or diff

Most parsers have a TON of dependencies, and the api for calling them is not universal. The objects they return each have to be mapped out, and converted into resync types which can work in the program. To avoid this, I just write a simple comment/function parser using nom, for each language.

The last thing I want is a 200mb executable with 10,000 dependencies.
Resync also has a very specific api. Teh comments each can have docstring args, return types. This functionality would still have to be coded in, adding even more complexity.

## See (parsers.md)[../../parsers.md]
