mod ascii_hex_decode;
mod ascii_85_decode;
mod lzw_decode;
mod flate_decode;
mod run_length_decode;
// TODO: For LZW and Flate, you need a separate PNG parser after the data has
// been decoded. Luckily, I don't believe you need to pass any of those params
// into the decoding functions themselves and you can just do the PNG parsing
// Take a look at Chrome's "PDFium" for an example.
// TOOD: Look how Chrome handles the "predictor" component. I didn't see it
// anywhere in their code, but all the other PDF parsing libraries use it.
mod ccitt_fax_decode;
mod jbig2_decode;
mod dct_decode;
mod jpx_decode;
mod crypt_decode;

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

pub fn decode(content: &[u8], filter: Filter) -> Option<Vec<u8>> {
    match filter {
        Filter::AsciiHexDecode => ascii_hex_decode::ascii_hex_decode(content),
        Filter::Ascii85Decode => ascii_85_decode::ascii_85_decode(content),
        Filter::LZWDecode => {
            let data = unsafe {
                std::mem::transmute::<[u8], [u32]>(content)
            };
            lzw_decode::lwz_decode(&data)
        },
        Filter::FlateDecode => flate_decode::flate_decode(content),
        Filter::RunLengthDecode => run_length_decode::run_length_decode(content),
        Filter::CCITTFaxDecode => ccitt_fax_decode::ccitt_fax_decode(content),
        Filter::JBIG2Decode => jbig2_decode::jbig2_decode(content),
        Filter::DCTDecode => dct_decode::dct_decode(content),
        Filter::JPXDecode => jpx_decode::jpx_decode(content),
        Filter::Crypt => crypt_decode::crypt_decode(content),
    }
}
