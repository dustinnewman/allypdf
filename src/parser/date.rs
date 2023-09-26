use std::convert::TryFrom;

use crate::error::{PdfError, Result};
use crate::util::{slice_to_numeric, MINUS, PLUS, SQUOTE, Z};

const DATE_PREFIX: &[u8] = b"D:";

enum UniversalTimeOffset {
    Plus,
    Minus,
    Zero,
}

impl Default for UniversalTimeOffset {
    fn default() -> Self {
        Self::Zero
    }
}

impl From<UniversalTimeOffset> for i8 {
    fn from(value: UniversalTimeOffset) -> Self {
        match value {
            UniversalTimeOffset::Plus | UniversalTimeOffset::Zero => 1,
            UniversalTimeOffset::Minus => -1,
        }
    }
}

impl TryFrom<&u8> for UniversalTimeOffset {
    type Error = PdfError;

    fn try_from(byte: &u8) -> Result<Self> {
        match *byte {
            PLUS => Ok(Self::Plus),
            MINUS => Ok(Self::Minus),
            Z => Ok(Self::Zero),
            _ => Err(PdfError::DateParsingError),
        }
    }
}

#[derive(Debug)]
pub struct Date {
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    ut_offset_hour: i8,
    ut_offset_minute: i8,
}

impl Default for Date {
    fn default() -> Self {
        Self {
            year: Default::default(),
            month: 1,
            day: 1,
            hour: Default::default(),
            minute: Default::default(),
            second: Default::default(),
            ut_offset_hour: Default::default(),
            ut_offset_minute: Default::default(),
        }
    }
}

impl TryFrom<&[u8]> for Date {
    type Error = PdfError;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.get(0..2).ok_or(PdfError::DateParsingError)? != DATE_PREFIX {
            return Err(PdfError::DateParsingError);
        }
        let year = value
            .get(2..6)
            .and_then(|slice| slice_to_numeric(slice, 10))
            .ok_or(PdfError::DateParsingError)?;
        let month = value
            .get(6..8)
            .and_then(|slice| slice_to_numeric(slice, 10))
            .unwrap_or(1) as u8;
        let day = value
            .get(8..10)
            .and_then(|slice| slice_to_numeric(slice, 10))
            .unwrap_or(1) as u8;
        let hour = value
            .get(10..12)
            .and_then(|slice| slice_to_numeric(slice, 10))
            .unwrap_or_default() as u8;
        let minute = value
            .get(12..14)
            .and_then(|slice| slice_to_numeric(slice, 10))
            .unwrap_or_default() as u8;
        let second = value
            .get(14..16)
            .and_then(|slice| slice_to_numeric(slice, 10))
            .unwrap_or_default() as u8;
        let ut_offset = value
            .get(16)
            .and_then(|byte| UniversalTimeOffset::try_from(byte).ok())
            .unwrap_or_default() as i8;
        let ut_offset_hour = value
            .get(17..19)
            .and_then(|slice| slice_to_numeric(slice, 10))
            .unwrap_or_default();
        let ut_offset_hour = (ut_offset_hour as i8) * ut_offset;
        let apostrophe = value.get(20).map_or(false, |slice| *slice == SQUOTE);
        let ut_offset_minute = if apostrophe {
            value
                .get(21..23)
                .and_then(|slice| slice_to_numeric(slice, 10))
                .unwrap_or_default()
        } else {
            0
        };
        let ut_offset_minute = (ut_offset_minute as i8) * ut_offset;
        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            ut_offset_hour,
            ut_offset_minute,
        })
    }
}
