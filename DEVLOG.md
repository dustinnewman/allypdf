# August 4, 2022
Parsed True Type fonts and put inside font descriptor instead of raw stream. (TODO: Type 1 and OpenType fonts) Now use that parsed font to get text (i.e. actual bytes or Unicode) from True Type fonts in show_text() function inside graphics engine

# August 1, 2022
Working on show text: getting bytes to be properly looked up with the current font. Consult PDF spec for byte -> glyph mapping?