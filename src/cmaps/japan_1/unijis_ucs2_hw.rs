use std::borrow::Cow;

use crate::cmaps::NO_CID_CHARS;
use crate::font::cmap::{CMap, CMapWritingMode, CidRange, Codespace, CodespaceRange};
use crate::font::font::CidSystemInfo;

const CODE_SPACE: [CodespaceRange; 2] = [
    [0..=0, 0..=0, 0..=215, 0..=255],
    [0..=0, 0..=0, 224..=255, 0..=255],
];

const CID_RANGE_H: [CidRange; 4] = [
    CidRange {
        start: 32,
        end: 91,
        cid: 231,
    },
    CidRange {
        start: 92,
        end: 92,
        cid: 8719,
    },
    CidRange {
        start: 93,
        end: 126,
        cid: 292,
    },
    CidRange {
        start: 165,
        end: 165,
        cid: 291,
    },
];

const CID_RANGE_V: [CidRange; 199] = [
    CidRange {
        start: 32,
        end: 91,
        cid: 231,
    },
    CidRange {
        start: 92,
        end: 92,
        cid: 8719,
    },
    CidRange {
        start: 93,
        end: 126,
        cid: 292,
    },
    CidRange {
        start: 165,
        end: 165,
        cid: 291,
    },
    CidRange {
        start: 176,
        end: 176,
        cid: 8269,
    },
    CidRange {
        start: 8208,
        end: 8208,
        cid: 7893,
    },
    CidRange {
        start: 8213,
        end: 8213,
        cid: 7892,
    },
    CidRange {
        start: 8214,
        end: 8214,
        cid: 7895,
    },
    CidRange {
        start: 8216,
        end: 8217,
        cid: 8281,
    },
    CidRange {
        start: 8220,
        end: 8221,
        cid: 8279,
    },
    CidRange {
        start: 8229,
        end: 8229,
        cid: 7898,
    },
    CidRange {
        start: 8230,
        end: 8230,
        cid: 7897,
    },
    CidRange {
        start: 8242,
        end: 8242,
        cid: 8273,
    },
    CidRange {
        start: 8243,
        end: 8243,
        cid: 8283,
    },
    CidRange {
        start: 8592,
        end: 8592,
        cid: 738,
    },
    CidRange {
        start: 8593,
        end: 8593,
        cid: 736,
    },
    CidRange {
        start: 8594,
        end: 8594,
        cid: 739,
    },
    CidRange {
        start: 8595,
        end: 8595,
        cid: 737,
    },
    CidRange {
        start: 8597,
        end: 8597,
        cid: 12214,
    },
    CidRange {
        start: 8644,
        end: 8644,
        cid: 8311,
    },
    CidRange {
        start: 8645,
        end: 8645,
        cid: 8310,
    },
    CidRange {
        start: 8646,
        end: 8646,
        cid: 8312,
    },
    CidRange {
        start: 8678,
        end: 8678,
        cid: 8012,
    },
    CidRange {
        start: 8679,
        end: 8679,
        cid: 8014,
    },
    CidRange {
        start: 8680,
        end: 8680,
        cid: 8011,
    },
    CidRange {
        start: 8681,
        end: 8681,
        cid: 8013,
    },
    CidRange {
        start: 8741,
        end: 8741,
        cid: 7895,
    },
    CidRange {
        start: 8764,
        end: 8764,
        cid: 7894,
    },
    CidRange {
        start: 8943,
        end: 8943,
        cid: 7897,
    },
    CidRange {
        start: 9472,
        end: 9473,
        cid: 7481,
    },
    CidRange {
        start: 9474,
        end: 9475,
        cid: 7479,
    },
    CidRange {
        start: 9476,
        end: 9477,
        cid: 7485,
    },
    CidRange {
        start: 9478,
        end: 9479,
        cid: 7483,
    },
    CidRange {
        start: 9480,
        end: 9481,
        cid: 7489,
    },
    CidRange {
        start: 9482,
        end: 9483,
        cid: 7487,
    },
    CidRange {
        start: 9484,
        end: 9484,
        cid: 7495,
    },
    CidRange {
        start: 9485,
        end: 9485,
        cid: 7497,
    },
    CidRange {
        start: 9486,
        end: 9486,
        cid: 7496,
    },
    CidRange {
        start: 9487,
        end: 9487,
        cid: 7498,
    },
    CidRange {
        start: 9488,
        end: 9488,
        cid: 7503,
    },
    CidRange {
        start: 9489,
        end: 9489,
        cid: 7505,
    },
    CidRange {
        start: 9490,
        end: 9490,
        cid: 7504,
    },
    CidRange {
        start: 9491,
        end: 9491,
        cid: 7506,
    },
    CidRange {
        start: 9492,
        end: 9492,
        cid: 7491,
    },
    CidRange {
        start: 9493,
        end: 9493,
        cid: 7493,
    },
    CidRange {
        start: 9494,
        end: 9494,
        cid: 7492,
    },
    CidRange {
        start: 9495,
        end: 9495,
        cid: 7494,
    },
    CidRange {
        start: 9496,
        end: 9496,
        cid: 7499,
    },
    CidRange {
        start: 9497,
        end: 9497,
        cid: 7501,
    },
    CidRange {
        start: 9498,
        end: 9498,
        cid: 7500,
    },
    CidRange {
        start: 9499,
        end: 9499,
        cid: 7502,
    },
    CidRange {
        start: 9500,
        end: 9500,
        cid: 7523,
    },
    CidRange {
        start: 9501,
        end: 9501,
        cid: 7527,
    },
    CidRange {
        start: 9502,
        end: 9502,
        cid: 7525,
    },
    CidRange {
        start: 9503,
        end: 9503,
        cid: 7524,
    },
    CidRange {
        start: 9504,
        end: 9504,
        cid: 7526,
    },
    CidRange {
        start: 9505,
        end: 9505,
        cid: 7529,
    },
    CidRange {
        start: 9506,
        end: 9506,
        cid: 7528,
    },
    CidRange {
        start: 9507,
        end: 9508,
        cid: 7530,
    },
    CidRange {
        start: 9509,
        end: 9509,
        cid: 7535,
    },
    CidRange {
        start: 9510,
        end: 9510,
        cid: 7533,
    },
    CidRange {
        start: 9511,
        end: 9511,
        cid: 7532,
    },
    CidRange {
        start: 9512,
        end: 9512,
        cid: 7534,
    },
    CidRange {
        start: 9513,
        end: 9513,
        cid: 7537,
    },
    CidRange {
        start: 9514,
        end: 9514,
        cid: 7536,
    },
    CidRange {
        start: 9515,
        end: 9515,
        cid: 7538,
    },
    CidRange {
        start: 9516,
        end: 9516,
        cid: 7515,
    },
    CidRange {
        start: 9517,
        end: 9519,
        cid: 7517,
    },
    CidRange {
        start: 9520,
        end: 9520,
        cid: 7516,
    },
    CidRange {
        start: 9521,
        end: 9523,
        cid: 7520,
    },
    CidRange {
        start: 9524,
        end: 9524,
        cid: 7507,
    },
    CidRange {
        start: 9525,
        end: 9527,
        cid: 7509,
    },
    CidRange {
        start: 9528,
        end: 9528,
        cid: 7508,
    },
    CidRange {
        start: 9529,
        end: 9531,
        cid: 7512,
    },
    CidRange {
        start: 9533,
        end: 9535,
        cid: 7543,
    },
    CidRange {
        start: 9536,
        end: 9536,
        cid: 7541,
    },
    CidRange {
        start: 9537,
        end: 9537,
        cid: 7540,
    },
    CidRange {
        start: 9538,
        end: 9538,
        cid: 7542,
    },
    CidRange {
        start: 9539,
        end: 9539,
        cid: 7547,
    },
    CidRange {
        start: 9540,
        end: 9540,
        cid: 7549,
    },
    CidRange {
        start: 9541,
        end: 9541,
        cid: 7546,
    },
    CidRange {
        start: 9542,
        end: 9542,
        cid: 7548,
    },
    CidRange {
        start: 9543,
        end: 9543,
        cid: 7553,
    },
    CidRange {
        start: 9544,
        end: 9544,
        cid: 7552,
    },
    CidRange {
        start: 9545,
        end: 9546,
        cid: 7550,
    },
    CidRange {
        start: 9756,
        end: 9756,
        cid: 8221,
    },
    CidRange {
        start: 9757,
        end: 9757,
        cid: 8219,
    },
    CidRange {
        start: 9758,
        end: 9758,
        cid: 8222,
    },
    CidRange {
        start: 9759,
        end: 9759,
        cid: 8220,
    },
    CidRange {
        start: 9986,
        end: 9986,
        cid: 12178,
    },
    CidRange {
        start: 10145,
        end: 10145,
        cid: 8209,
    },
    CidRange {
        start: 12289,
        end: 12290,
        cid: 7887,
    },
    CidRange {
        start: 12296,
        end: 12305,
        cid: 7907,
    },
    CidRange {
        start: 12307,
        end: 12307,
        cid: 8270,
    },
    CidRange {
        start: 12308,
        end: 12309,
        cid: 7901,
    },
    CidRange {
        start: 12316,
        end: 12316,
        cid: 7894,
    },
    CidRange {
        start: 12317,
        end: 12317,
        cid: 7956,
    },
    CidRange {
        start: 12319,
        end: 12319,
        cid: 7957,
    },
    CidRange {
        start: 12353,
        end: 12353,
        cid: 7918,
    },
    CidRange {
        start: 12355,
        end: 12355,
        cid: 7919,
    },
    CidRange {
        start: 12357,
        end: 12357,
        cid: 7920,
    },
    CidRange {
        start: 12359,
        end: 12359,
        cid: 7921,
    },
    CidRange {
        start: 12361,
        end: 12361,
        cid: 7922,
    },
    CidRange {
        start: 12387,
        end: 12387,
        cid: 7923,
    },
    CidRange {
        start: 12419,
        end: 12419,
        cid: 7924,
    },
    CidRange {
        start: 12421,
        end: 12421,
        cid: 7925,
    },
    CidRange {
        start: 12423,
        end: 12423,
        cid: 7926,
    },
    CidRange {
        start: 12430,
        end: 12430,
        cid: 7927,
    },
    CidRange {
        start: 12443,
        end: 12443,
        cid: 8272,
    },
    CidRange {
        start: 12444,
        end: 12444,
        cid: 8271,
    },
    CidRange {
        start: 12449,
        end: 12449,
        cid: 7928,
    },
    CidRange {
        start: 12451,
        end: 12451,
        cid: 7929,
    },
    CidRange {
        start: 12453,
        end: 12453,
        cid: 7930,
    },
    CidRange {
        start: 12455,
        end: 12455,
        cid: 7931,
    },
    CidRange {
        start: 12457,
        end: 12457,
        cid: 7932,
    },
    CidRange {
        start: 12483,
        end: 12483,
        cid: 7933,
    },
    CidRange {
        start: 12515,
        end: 12515,
        cid: 7934,
    },
    CidRange {
        start: 12517,
        end: 12517,
        cid: 7935,
    },
    CidRange {
        start: 12519,
        end: 12519,
        cid: 7936,
    },
    CidRange {
        start: 12526,
        end: 12526,
        cid: 7937,
    },
    CidRange {
        start: 12533,
        end: 12534,
        cid: 7938,
    },
    CidRange {
        start: 12540,
        end: 12540,
        cid: 7891,
    },
    CidRange {
        start: 13056,
        end: 13056,
        cid: 8350,
    },
    CidRange {
        start: 13057,
        end: 13058,
        cid: 11958,
    },
    CidRange {
        start: 13059,
        end: 13059,
        cid: 8338,
    },
    CidRange {
        start: 13060,
        end: 13060,
        cid: 11960,
    },
    CidRange {
        start: 13061,
        end: 13061,
        cid: 8333,
    },
    CidRange {
        start: 13062,
        end: 13062,
        cid: 11961,
    },
    CidRange {
        start: 13063,
        end: 13063,
        cid: 11965,
    },
    CidRange {
        start: 13064,
        end: 13064,
        cid: 11963,
    },
    CidRange {
        start: 13065,
        end: 13065,
        cid: 11968,
    },
    CidRange {
        start: 13066,
        end: 13066,
        cid: 11966,
    },
    CidRange {
        start: 13067,
        end: 13067,
        cid: 11970,
    },
    CidRange {
        start: 13068,
        end: 13068,
        cid: 11972,
    },
    CidRange {
        start: 13069,
        end: 13069,
        cid: 7950,
    },
    CidRange {
        start: 13070,
        end: 13075,
        cid: 11973,
    },
    CidRange {
        start: 13076,
        end: 13076,
        cid: 7941,
    },
    CidRange {
        start: 13077,
        end: 13077,
        cid: 8340,
    },
    CidRange {
        start: 13078,
        end: 13078,
        cid: 8330,
    },
    CidRange {
        start: 13079,
        end: 13079,
        cid: 11980,
    },
    CidRange {
        start: 13080,
        end: 13080,
        cid: 8339,
    },
    CidRange {
        start: 13081,
        end: 13081,
        cid: 11982,
    },
    CidRange {
        start: 13082,
        end: 13085,
        cid: 11984,
    },
    CidRange {
        start: 13086,
        end: 13086,
        cid: 8353,
    },
    CidRange {
        start: 13087,
        end: 13089,
        cid: 11988,
    },
    CidRange {
        start: 13090,
        end: 13090,
        cid: 8329,
    },
    CidRange {
        start: 13091,
        end: 13091,
        cid: 8348,
    },
    CidRange {
        start: 13092,
        end: 13092,
        cid: 11991,
    },
    CidRange {
        start: 13093,
        end: 13093,
        cid: 11993,
    },
    CidRange {
        start: 13094,
        end: 13094,
        cid: 7951,
    },
    CidRange {
        start: 13095,
        end: 13095,
        cid: 7945,
    },
    CidRange {
        start: 13096,
        end: 13097,
        cid: 11996,
    },
    CidRange {
        start: 13098,
        end: 13098,
        cid: 8356,
    },
    CidRange {
        start: 13099,
        end: 13099,
        cid: 7953,
    },
    CidRange {
        start: 13101,
        end: 13101,
        cid: 11999,
    },
    CidRange {
        start: 13102,
        end: 13104,
        cid: 12002,
    },
    CidRange {
        start: 13105,
        end: 13105,
        cid: 8358,
    },
    CidRange {
        start: 13106,
        end: 13106,
        cid: 12005,
    },
    CidRange {
        start: 13107,
        end: 13107,
        cid: 8334,
    },
    CidRange {
        start: 13108,
        end: 13109,
        cid: 12008,
    },
    CidRange {
        start: 13110,
        end: 13110,
        cid: 7947,
    },
    CidRange {
        start: 13111,
        end: 13111,
        cid: 12014,
    },
    CidRange {
        start: 13112,
        end: 13112,
        cid: 12016,
    },
    CidRange {
        start: 13113,
        end: 13113,
        cid: 8343,
    },
    CidRange {
        start: 13114,
        end: 13114,
        cid: 12017,
    },
    CidRange {
        start: 13115,
        end: 13115,
        cid: 8349,
    },
    CidRange {
        start: 13116,
        end: 13116,
        cid: 12010,
    },
    CidRange {
        start: 13117,
        end: 13117,
        cid: 12018,
    },
    CidRange {
        start: 13118,
        end: 13120,
        cid: 12020,
    },
    CidRange {
        start: 13121,
        end: 13121,
        cid: 12019,
    },
    CidRange {
        start: 13122,
        end: 13122,
        cid: 8347,
    },
    CidRange {
        start: 13123,
        end: 13126,
        cid: 12023,
    },
    CidRange {
        start: 13127,
        end: 13127,
        cid: 8357,
    },
    CidRange {
        start: 13128,
        end: 13128,
        cid: 12027,
    },
    CidRange {
        start: 13129,
        end: 13129,
        cid: 7940,
    },
    CidRange {
        start: 13130,
        end: 13130,
        cid: 7954,
    },
    CidRange {
        start: 13131,
        end: 13132,
        cid: 12028,
    },
    CidRange {
        start: 13133,
        end: 13133,
        cid: 7943,
    },
    CidRange {
        start: 13134,
        end: 13134,
        cid: 8337,
    },
    CidRange {
        start: 13135,
        end: 13136,
        cid: 12030,
    },
    CidRange {
        start: 13137,
        end: 13137,
        cid: 7948,
    },
    CidRange {
        start: 13138,
        end: 13138,
        cid: 12034,
    },
    CidRange {
        start: 13139,
        end: 13139,
        cid: 12038,
    },
    CidRange {
        start: 13140,
        end: 13140,
        cid: 12035,
    },
    CidRange {
        start: 13141,
        end: 13142,
        cid: 12039,
    },
    CidRange {
        start: 13143,
        end: 13143,
        cid: 8344,
    },
    CidRange {
        start: 13183,
        end: 13183,
        cid: 8324,
    },
    CidRange {
        start: 65288,
        end: 65289,
        cid: 7899,
    },
    CidRange {
        start: 65292,
        end: 65292,
        cid: 8268,
    },
    CidRange {
        start: 65294,
        end: 65294,
        cid: 8274,
    },
    CidRange {
        start: 65309,
        end: 65309,
        cid: 7917,
    },
    CidRange {
        start: 65339,
        end: 65339,
        cid: 7903,
    },
    CidRange {
        start: 65341,
        end: 65341,
        cid: 7904,
    },
    CidRange {
        start: 65343,
        end: 65343,
        cid: 7890,
    },
    CidRange {
        start: 65371,
        end: 65371,
        cid: 7905,
    },
    CidRange {
        start: 65372,
        end: 65372,
        cid: 7896,
    },
    CidRange {
        start: 65373,
        end: 65373,
        cid: 7906,
    },
    CidRange {
        start: 65374,
        end: 65374,
        cid: 7894,
    },
    CidRange {
        start: 65507,
        end: 65507,
        cid: 7889,
    },
];

pub const UNIJIS_UCS2_HW_H: CMap = CMap {
    name: b"UniJIS-UCS2-HW-H",
    cid_system_info: CidSystemInfo {
        registry: b"Adobe",
        ordering: b"Japan1",
        supplement: 4,
    },
    writing_mode: CMapWritingMode::Horizontal,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: Cow::Borrowed(&NO_CID_CHARS),
    cid_range: Cow::Borrowed(&CID_RANGE_H),
};

pub const UNIJIS_UCS2_HW_V: CMap = CMap {
    name: b"UniJIS-UCS2-HW-V",
    cid_system_info: CidSystemInfo {
        registry: b"Adobe",
        ordering: b"Japan1",
        supplement: 4,
    },
    writing_mode: CMapWritingMode::Vertical,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: Cow::Borrowed(&NO_CID_CHARS),
    cid_range: Cow::Borrowed(&CID_RANGE_V),
};
