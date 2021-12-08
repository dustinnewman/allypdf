use crate::error::PdfError;
use crate::parser::parser::{Dictionary, Name};
use std::convert::TryFrom;

mod ascii_85_decode;
mod ascii_hex_decode;
mod flate_decode;
mod lzw_decode;
mod run_length_decode;
// TODO: For LZW and Flate, you need a separate PNG parser after the data has
// been decoded. Luckily, I don't believe you need to pass any of those params
// into the decoding functions themselves and you can just do the PNG parsing
// Take a look at Chrome's "PDFium" for an example.
// TOOD: Look how Chrome handles the "predictor" component. I didn't see it
// anywhere in their code, but all the other PDF parsing libraries use it.
mod ccitt_fax_decode;
mod crypt_decode;
mod dct_decode;
mod jbig2_decode;
mod jpx_decode;

pub const ASCII_HEX_DECODE: &[u8] = b"AsciiHexDecode";
pub const ASCII_85_DECODE: &[u8] = b"Ascii85Decode";
pub const LZW_DECODE: &[u8] = b"LZWDecode";
pub const FLATE_DECODE: &[u8] = b"FlateDecode";
pub const RUN_LENGTH_DECODE: &[u8] = b"RunLengthDecode";
pub const CCITT_FAX_DECODE: &[u8] = b"CCITTFaxDecode";
pub const JBIG_2_DECODE: &[u8] = b"JBIG2Decode";
pub const DCT_DECODE: &[u8] = b"DCTDecode";
pub const JPX_DECODE: &[u8] = b"JPXDecode";
pub const CRYPT_DECODE: &[u8] = b"Crypt";

#[derive(Debug, PartialEq)]
pub enum Filter {
    AsciiHexDecode,
    Ascii85Decode,
    LZWDecode,
    FlateDecode,
    RunLengthDecode,
    CCITTFaxDecode,
    JBIG2Decode,
    DCTDecode,
    JPXDecode,
    Crypt,
}

impl TryFrom<&Name> for Filter {
    type Error = PdfError;

    fn try_from(name: &Name) -> Result<Self, Self::Error> {
        let name: &[u8] = &name;
        let filter = match name {
            ASCII_HEX_DECODE => Filter::AsciiHexDecode,
            ASCII_85_DECODE => Filter::Ascii85Decode,
            LZW_DECODE => Filter::LZWDecode,
            FLATE_DECODE => Filter::FlateDecode,
            RUN_LENGTH_DECODE => Filter::RunLengthDecode,
            CCITT_FAX_DECODE => Filter::CCITTFaxDecode,
            JBIG_2_DECODE => Filter::JBIG2Decode,
            DCT_DECODE => Filter::DCTDecode,
            JPX_DECODE => Filter::JPXDecode,
            CRYPT_DECODE => Filter::Crypt,
            _ => {
                return Err(PdfError::Other {
                    msg: "Name to filter conversion failed.".to_string(),
                })
            }
        };
        Ok(filter)
    }
}

pub fn decode(content: &[u8], filter: Filter, _params: &Dictionary) -> Option<Vec<u8>> {
    match filter {
        Filter::AsciiHexDecode => ascii_hex_decode::ascii_hex_decode(content),
        Filter::Ascii85Decode => ascii_85_decode::ascii_85_decode(content),
        Filter::LZWDecode => {
            let mut data = vec![];
            let iter = content.chunks(4);
            for quadruple in iter {
                let bytes: [u8; 4] = match quadruple {
                    [a] => [*a, 0, 0, 0],
                    [a, b] => [*a, *b, 0, 0],
                    [a, b, c] => [*a, *b, *c, 0],
                    [a, b, c, d] => [*a, *b, *c, *d],
                    _ => return None,
                };
                let combo = u32::from_be_bytes(bytes);
                data.push(combo);
            }
            lzw_decode::lwz_decode(&data)
        }
        Filter::FlateDecode => flate_decode::flate_decode(content),
        Filter::RunLengthDecode => run_length_decode::run_length_decode(content),
        Filter::CCITTFaxDecode => ccitt_fax_decode::ccitt_fax_decode(content),
        Filter::JBIG2Decode => jbig2_decode::jbig2_decode(content),
        Filter::DCTDecode => dct_decode::dct_decode(content),
        Filter::JPXDecode => jpx_decode::jpx_decode(content),
        Filter::Crypt => crypt_decode::crypt_decode(content),
    }
}
