use jpeg2000::decode::{Codec, ColorSpace, DecodeConfig};

pub fn jpx_decode(data: &[u8]) -> Option<Vec<u8>> {
    Some(
        jpeg2000::decode::from_memory(
            data,
            Codec::JPX,
            DecodeConfig {
                default_colorspace: Some(ColorSpace::SRGB),
                discard_level: 0,
            },
            None,
        )
        .ok()?
        .raw_pixels(),
    )
}

mod tests{
    #[cfg(test)]
    fn test_jpx_1() {}
}
