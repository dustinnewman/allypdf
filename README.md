# PDF parser
## Modules
### parser
`Lexer` parses a list of raw bytes (`Vec<u8>`) into a list of tokens. These are basic things like numbers, strings, and even basic punctuation delimiters (`(`, `]`). Tokens do not consider context or surrounding tokens at all. `Parser` then takes this list of tokens and turns it into a list of objects. Objects do consider context. For example, a number by itself is just a number but two numbers followed by `obj` (e.g. `4 0 obj`) is an indirect object definition.

### filter
The `filter` module encompasses all the different stream filters defined in the PDF spec. These include (non-exhaustively) ASCII 85, Flate Decode, JPX, etc. A filter turns a list of raw bytes into of... also raw bytes, but hopefully with more structure (e.g. PDF content stream operators). The output from these filters is usually (in the case of stream content for example) re-parsed by the `parser` module into `Operations`.

### operators
An `Operator` is a PDF content stream operator (defined in spec) type. Things like showing text, drawing lines, moving around the page, these are all base operators. `Operations` by contrast are operators with arguments. So, while the `ShowText` operator exists by itself, the `ShowText` operation needs the string to show as an input. 

The `operators` module also handles various related types that are needed by `Operations`, but also by other modules. For example, PDF colors, rectangles, and paths are all needed by many modules to render and convert PDFs, but they are defined in the `operators` module since that is the first time we have enough context to actually construct them.

### document
The `document` module is supposed to take the output from the `parser` module (a list of objects) and give it a hierarchy and structure, as defined in the PDF spec. For example, the `parser` returns a list of objects, but `document` will construct the PDF catalog, page hierarchy, and resource dictionaries. This module is probably the first one to be optimized if needed due to this. The thinking, as of writing, is to produce the essential PDF structure (resources, contents, fonts, etc.) all at once and then if those structures are needed later, we have access to them. This is supposed to simplify handling things so you don't need to worry if, e.g. the resource dictionary is fully parsed.

### render
The `render` module acts as a platform-neutral canvas for a PDF. Let me explain. PDF content is written in absolute coordinates on the page. It is primarily a graphic format, concerned with pixel-perfect rendering and placement of elements. This comes at the cost of a more semantic markup which categorizes things like headers, sections, figures, etc. If it wanted to, a PDF writer could write all content on a PDF page backwards, placing the text at the end of page at the beginning of the content stream and then working "up"/backwards. Since PDF content can be written in any order, we cannot just write HTML output-equivalents as we go, since we could end up with the exact opposite placement. To solve this, the `render` module is supposed to act an easily-parsed, easily-sorted canvas where we can more easily and predictably parse the text/images/elements of a page.

### output
In contrast to the `render` module, the `output` module does not write things out in absolute, pixel-based terms, but tries to establish logical order in an output format. For example, an HTML output would not want to just mimic the `position: absolute;` style of PDF, but rather try and convert letters into words and words into paragraphs. We can have HTML or text output for example.

### font
PDF fonts are complex structures with many different encodings, edge cases, types, subtypes, and files. To encapsulate this, we use the `font` module.
