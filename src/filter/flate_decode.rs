use std::io::Read;
use flate2::read::DeflateDecoder;

pub fn flate_decode(content: &[u8]) -> Option<Vec<u8>> {
   let mut deflater = DeflateDecoder::new(content);
   let mut vec = vec![];
   deflater.read_to_end(&mut vec).ok()?;
   Some(vec)
}
