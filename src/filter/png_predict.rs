pub enum PNGPredictorType {
    None,
    PNG,
    Flate,
}

pub fn get_predictor(predictor: u8) -> PNGPredictorType {
    match predictor {
        x if x >= 10 => PNGPredictorType::PNG,
        2 => PNGPredictorType::Flate,
        _ => PNGPredictorType::None,
    }
}