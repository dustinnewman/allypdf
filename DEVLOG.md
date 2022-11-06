# October 20, 2022
Kind of figured out stream decoding. You only need to worry about length for font objects. And even then, only really for Type 1 fonts. So I moved this into font_descriptor function inside document.

# October 16, 2022
Need to work on two things next: decode stream dictionaries after the fact (not in parser, but rather in document when you have all objects already) (motivating example: << /Length 143 0 R >>stream...endstream) and then the bfchars part of the CMaps in TrueType fonts. Will they always be UTF16? Or is that only hello_salam.pdf? fraud_proofs.pdf does not seem to use UTF16 but need to check it out.

# October 15, 2022
Finally got the ownership of CMap parser to work with streams. show_text still needs work for fonts.

# October 9, 2022
Take the bytes from the show_text operation and use the encoding to get the character from the glyph name.

# August 4, 2022
Parsed True Type fonts and put inside font descriptor instead of raw stream. (TODO: Type 1 and OpenType fonts) Now use that parsed font to get text (i.e. actual bytes or Unicode) from True Type fonts in show_text() function inside graphics engine

# August 1, 2022
Working on show text: getting bytes to be properly looked up with the current font. Consult PDF spec for byte -> glyph mapping?