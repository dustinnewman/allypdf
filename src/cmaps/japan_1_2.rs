use std::{borrow::Cow, ops::RangeInclusive};

use crate::font::cmap::{
    CMapFile, CMapWritingMode, CidRange, Codespace, CodespaceRange, DEFAULT_CODE_SPACE_RANGE,
};
use crate::font::font::CidSystemInfo;

const CODE_SPACE: [CodespaceRange; 1] = [[
    DEFAULT_CODE_SPACE_RANGE,
    DEFAULT_CODE_SPACE_RANGE,
    RangeInclusive::new(0, 34),
    RangeInclusive::new(0, 255),
]];
const CID_RANGE: [CidRange; 35] = [
    CidRange {
        start: 0,
        end: 255,
        cid: 0,
    },
    CidRange {
        start: 256,
        end: 511,
        cid: 256,
    },
    CidRange {
        start: 512,
        end: 767,
        cid: 512,
    },
    CidRange {
        start: 768,
        end: 1023,
        cid: 768,
    },
    CidRange {
        start: 1024,
        end: 1279,
        cid: 1024,
    },
    CidRange {
        start: 1280,
        end: 1535,
        cid: 1280,
    },
    CidRange {
        start: 1536,
        end: 1791,
        cid: 1536,
    },
    CidRange {
        start: 1792,
        end: 2047,
        cid: 1792,
    },
    CidRange {
        start: 2048,
        end: 2303,
        cid: 2048,
    },
    CidRange {
        start: 2304,
        end: 2559,
        cid: 2304,
    },
    CidRange {
        start: 2560,
        end: 2815,
        cid: 2560,
    },
    CidRange {
        start: 2816,
        end: 3071,
        cid: 2816,
    },
    CidRange {
        start: 3072,
        end: 3327,
        cid: 3072,
    },
    CidRange {
        start: 3328,
        end: 3583,
        cid: 3328,
    },
    CidRange {
        start: 3584,
        end: 3839,
        cid: 3584,
    },
    CidRange {
        start: 3840,
        end: 4095,
        cid: 3840,
    },
    CidRange {
        start: 4096,
        end: 4351,
        cid: 4096,
    },
    CidRange {
        start: 4352,
        end: 4607,
        cid: 4352,
    },
    CidRange {
        start: 4608,
        end: 4863,
        cid: 4608,
    },
    CidRange {
        start: 4864,
        end: 5119,
        cid: 4864,
    },
    CidRange {
        start: 5120,
        end: 5375,
        cid: 5120,
    },
    CidRange {
        start: 5376,
        end: 5631,
        cid: 5376,
    },
    CidRange {
        start: 5632,
        end: 5887,
        cid: 5632,
    },
    CidRange {
        start: 5888,
        end: 6143,
        cid: 5888,
    },
    CidRange {
        start: 6144,
        end: 6399,
        cid: 6144,
    },
    CidRange {
        start: 6400,
        end: 6655,
        cid: 6400,
    },
    CidRange {
        start: 6656,
        end: 6911,
        cid: 6656,
    },
    CidRange {
        start: 6912,
        end: 7167,
        cid: 6912,
    },
    CidRange {
        start: 7168,
        end: 7423,
        cid: 7168,
    },
    CidRange {
        start: 7424,
        end: 7679,
        cid: 7424,
    },
    CidRange {
        start: 7680,
        end: 7935,
        cid: 7680,
    },
    CidRange {
        start: 7936,
        end: 8191,
        cid: 7936,
    },
    CidRange {
        start: 8192,
        end: 8447,
        cid: 8192,
    },
    CidRange {
        start: 8448,
        end: 8703,
        cid: 8448,
    },
    CidRange {
        start: 8704,
        end: 8719,
        cid: 8704,
    },
];

pub const JAPAN_1_2_90MS_RKSJ_H: CMapFile = CMapFile {
    name: b"90ms-RKSJ-H",
    cid_system_info: CidSystemInfo {
        registry: b"Adobe",
        ordering: b"Japan1",
        supplement: 2,
    },
    writing_mode: CMapWritingMode::Horizontal,
    cmap: None,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_range: Cow::Borrowed(&CID_RANGE),
};
