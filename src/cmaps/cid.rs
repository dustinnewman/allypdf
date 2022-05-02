// In simple fonts, character codes are only 8-bit and can thus only address
// 256 glyphs. In composite fonts, however, we can have multi-byte character
// codes from 2 to 4 bytes.
pub type CharCode = u32;
// "In previous versions of PDF, it was recommended that the maximum value of
// a CID (character identifier) was limited to 65,535." (PDF Annex C.2)
// However, we will use u32 to be safe. If a PDF works for CID 65,535, why
// shouldn't it work for 65,536?
pub type Cid = u32;

pub trait CharCodeToCid {
    fn get_cid(&self, char_code: CharCode) -> Option<Cid>;
}

pub trait CharCodeToGlyphName {
    fn get_glyph_name(&self, char_code: CharCode) -> Option<&[u8]>;
}
