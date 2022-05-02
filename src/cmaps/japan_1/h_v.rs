use std::borrow::Cow;

use crate::cmaps::cmap::{
    CMap, CMapWritingMode, CidRange, Codespace, CodespaceRange, NO_CID_CHARS, ADOBE_REGISTRY,
};
use crate::font::font::CidSystemInfo;

use super::JAPAN_1;

const CODE_SPACE: [CodespaceRange; 1] = [[0..=0, 0..=0, 33..=126, 33..=126]];

const CID_RANGE_H: [CidRange; 118] = [
    CidRange {
        start: 8481,
        end: 8574,
        cid: 633,
    },
    CidRange {
        start: 8737,
        end: 8750,
        cid: 727,
    },
    CidRange {
        start: 8762,
        end: 8769,
        cid: 741,
    },
    CidRange {
        start: 8778,
        end: 8784,
        cid: 749,
    },
    CidRange {
        start: 8796,
        end: 8810,
        cid: 756,
    },
    CidRange {
        start: 8818,
        end: 8825,
        cid: 771,
    },
    CidRange {
        start: 8830,
        end: 8830,
        cid: 779,
    },
    CidRange {
        start: 9008,
        end: 9017,
        cid: 780,
    },
    CidRange {
        start: 9025,
        end: 9050,
        cid: 790,
    },
    CidRange {
        start: 9057,
        end: 9082,
        cid: 816,
    },
    CidRange {
        start: 9249,
        end: 9331,
        cid: 842,
    },
    CidRange {
        start: 9505,
        end: 9590,
        cid: 925,
    },
    CidRange {
        start: 9761,
        end: 9784,
        cid: 1011,
    },
    CidRange {
        start: 9793,
        end: 9816,
        cid: 1035,
    },
    CidRange {
        start: 10017,
        end: 10049,
        cid: 1059,
    },
    CidRange {
        start: 10065,
        end: 10097,
        cid: 1092,
    },
    CidRange {
        start: 10273,
        end: 10273,
        cid: 7479,
    },
    CidRange {
        start: 10274,
        end: 10274,
        cid: 7481,
    },
    CidRange {
        start: 10275,
        end: 10275,
        cid: 7491,
    },
    CidRange {
        start: 10276,
        end: 10276,
        cid: 7495,
    },
    CidRange {
        start: 10277,
        end: 10277,
        cid: 7503,
    },
    CidRange {
        start: 10278,
        end: 10278,
        cid: 7499,
    },
    CidRange {
        start: 10279,
        end: 10279,
        cid: 7507,
    },
    CidRange {
        start: 10280,
        end: 10280,
        cid: 7523,
    },
    CidRange {
        start: 10281,
        end: 10281,
        cid: 7515,
    },
    CidRange {
        start: 10282,
        end: 10282,
        cid: 7531,
    },
    CidRange {
        start: 10283,
        end: 10283,
        cid: 7539,
    },
    CidRange {
        start: 10284,
        end: 10284,
        cid: 7480,
    },
    CidRange {
        start: 10285,
        end: 10285,
        cid: 7482,
    },
    CidRange {
        start: 10286,
        end: 10286,
        cid: 7494,
    },
    CidRange {
        start: 10287,
        end: 10287,
        cid: 7498,
    },
    CidRange {
        start: 10288,
        end: 10288,
        cid: 7506,
    },
    CidRange {
        start: 10289,
        end: 10289,
        cid: 7502,
    },
    CidRange {
        start: 10290,
        end: 10290,
        cid: 7514,
    },
    CidRange {
        start: 10291,
        end: 10291,
        cid: 7530,
    },
    CidRange {
        start: 10292,
        end: 10292,
        cid: 7522,
    },
    CidRange {
        start: 10293,
        end: 10293,
        cid: 7538,
    },
    CidRange {
        start: 10294,
        end: 10294,
        cid: 7554,
    },
    CidRange {
        start: 10295,
        end: 10295,
        cid: 7511,
    },
    CidRange {
        start: 10296,
        end: 10296,
        cid: 7526,
    },
    CidRange {
        start: 10297,
        end: 10297,
        cid: 7519,
    },
    CidRange {
        start: 10298,
        end: 10298,
        cid: 7534,
    },
    CidRange {
        start: 10299,
        end: 10299,
        cid: 7542,
    },
    CidRange {
        start: 10300,
        end: 10300,
        cid: 7508,
    },
    CidRange {
        start: 10301,
        end: 10301,
        cid: 7527,
    },
    CidRange {
        start: 10302,
        end: 10302,
        cid: 7516,
    },
    CidRange {
        start: 10303,
        end: 10303,
        cid: 7535,
    },
    CidRange {
        start: 10304,
        end: 10304,
        cid: 7545,
    },
    CidRange {
        start: 12321,
        end: 12414,
        cid: 1125,
    },
    CidRange {
        start: 12577,
        end: 12670,
        cid: 1219,
    },
    CidRange {
        start: 12833,
        end: 12926,
        cid: 1313,
    },
    CidRange {
        start: 13089,
        end: 13182,
        cid: 1407,
    },
    CidRange {
        start: 13345,
        end: 13438,
        cid: 1501,
    },
    CidRange {
        start: 13601,
        end: 13694,
        cid: 1595,
    },
    CidRange {
        start: 13857,
        end: 13950,
        cid: 1689,
    },
    CidRange {
        start: 14113,
        end: 14206,
        cid: 1783,
    },
    CidRange {
        start: 14369,
        end: 14462,
        cid: 1877,
    },
    CidRange {
        start: 14625,
        end: 14718,
        cid: 1971,
    },
    CidRange {
        start: 14881,
        end: 14974,
        cid: 2065,
    },
    CidRange {
        start: 15137,
        end: 15230,
        cid: 2159,
    },
    CidRange {
        start: 15393,
        end: 15486,
        cid: 2253,
    },
    CidRange {
        start: 15649,
        end: 15742,
        cid: 2347,
    },
    CidRange {
        start: 15905,
        end: 15998,
        cid: 2441,
    },
    CidRange {
        start: 16161,
        end: 16254,
        cid: 2535,
    },
    CidRange {
        start: 16417,
        end: 16510,
        cid: 2629,
    },
    CidRange {
        start: 16673,
        end: 16766,
        cid: 2723,
    },
    CidRange {
        start: 16929,
        end: 17022,
        cid: 2817,
    },
    CidRange {
        start: 17185,
        end: 17278,
        cid: 2911,
    },
    CidRange {
        start: 17441,
        end: 17534,
        cid: 3005,
    },
    CidRange {
        start: 17697,
        end: 17790,
        cid: 3099,
    },
    CidRange {
        start: 17953,
        end: 18046,
        cid: 3193,
    },
    CidRange {
        start: 18209,
        end: 18302,
        cid: 3287,
    },
    CidRange {
        start: 18465,
        end: 18558,
        cid: 3381,
    },
    CidRange {
        start: 18721,
        end: 18814,
        cid: 3475,
    },
    CidRange {
        start: 18977,
        end: 19070,
        cid: 3569,
    },
    CidRange {
        start: 19233,
        end: 19326,
        cid: 3663,
    },
    CidRange {
        start: 19489,
        end: 19582,
        cid: 3757,
    },
    CidRange {
        start: 19745,
        end: 19838,
        cid: 3851,
    },
    CidRange {
        start: 20001,
        end: 20094,
        cid: 3945,
    },
    CidRange {
        start: 20257,
        end: 20307,
        cid: 4039,
    },
    CidRange {
        start: 20513,
        end: 20606,
        cid: 4090,
    },
    CidRange {
        start: 20769,
        end: 20862,
        cid: 4184,
    },
    CidRange {
        start: 21025,
        end: 21118,
        cid: 4278,
    },
    CidRange {
        start: 21281,
        end: 21374,
        cid: 4372,
    },
    CidRange {
        start: 21537,
        end: 21630,
        cid: 4466,
    },
    CidRange {
        start: 21793,
        end: 21886,
        cid: 4560,
    },
    CidRange {
        start: 22049,
        end: 22142,
        cid: 4654,
    },
    CidRange {
        start: 22305,
        end: 22398,
        cid: 4748,
    },
    CidRange {
        start: 22561,
        end: 22654,
        cid: 4842,
    },
    CidRange {
        start: 22817,
        end: 22910,
        cid: 4936,
    },
    CidRange {
        start: 23073,
        end: 23166,
        cid: 5030,
    },
    CidRange {
        start: 23329,
        end: 23422,
        cid: 5124,
    },
    CidRange {
        start: 23585,
        end: 23678,
        cid: 5218,
    },
    CidRange {
        start: 23841,
        end: 23934,
        cid: 5312,
    },
    CidRange {
        start: 24097,
        end: 24190,
        cid: 5406,
    },
    CidRange {
        start: 24353,
        end: 24446,
        cid: 5500,
    },
    CidRange {
        start: 24609,
        end: 24702,
        cid: 5594,
    },
    CidRange {
        start: 24865,
        end: 24958,
        cid: 5688,
    },
    CidRange {
        start: 25121,
        end: 25214,
        cid: 5782,
    },
    CidRange {
        start: 25377,
        end: 25470,
        cid: 5876,
    },
    CidRange {
        start: 25633,
        end: 25726,
        cid: 5970,
    },
    CidRange {
        start: 25889,
        end: 25982,
        cid: 6064,
    },
    CidRange {
        start: 26145,
        end: 26238,
        cid: 6158,
    },
    CidRange {
        start: 26401,
        end: 26494,
        cid: 6252,
    },
    CidRange {
        start: 26657,
        end: 26750,
        cid: 6346,
    },
    CidRange {
        start: 26913,
        end: 27006,
        cid: 6440,
    },
    CidRange {
        start: 27169,
        end: 27262,
        cid: 6534,
    },
    CidRange {
        start: 27425,
        end: 27518,
        cid: 6628,
    },
    CidRange {
        start: 27681,
        end: 27774,
        cid: 6722,
    },
    CidRange {
        start: 27937,
        end: 28030,
        cid: 6816,
    },
    CidRange {
        start: 28193,
        end: 28286,
        cid: 6910,
    },
    CidRange {
        start: 28449,
        end: 28542,
        cid: 7004,
    },
    CidRange {
        start: 28705,
        end: 28798,
        cid: 7098,
    },
    CidRange {
        start: 28961,
        end: 29054,
        cid: 7192,
    },
    CidRange {
        start: 29217,
        end: 29310,
        cid: 7286,
    },
    CidRange {
        start: 29473,
        end: 29566,
        cid: 7380,
    },
    CidRange {
        start: 29729,
        end: 29732,
        cid: 7474,
    },
    CidRange {
        start: 29733,
        end: 29734,
        cid: 8284,
    },
];

const CID_RANGE_V: [CidRange; 27] = [
    CidRange {
        start: 8482,
        end: 8483,
        cid: 7887,
    },
    CidRange {
        start: 8497,
        end: 8498,
        cid: 7889,
    },
    CidRange {
        start: 8508,
        end: 8510,
        cid: 7891,
    },
    CidRange {
        start: 8513,
        end: 8517,
        cid: 7894,
    },
    CidRange {
        start: 8522,
        end: 8539,
        cid: 7899,
    },
    CidRange {
        start: 8545,
        end: 8545,
        cid: 7917,
    },
    CidRange {
        start: 9249,
        end: 9249,
        cid: 7918,
    },
    CidRange {
        start: 9251,
        end: 9251,
        cid: 7919,
    },
    CidRange {
        start: 9253,
        end: 9253,
        cid: 7920,
    },
    CidRange {
        start: 9255,
        end: 9255,
        cid: 7921,
    },
    CidRange {
        start: 9257,
        end: 9257,
        cid: 7922,
    },
    CidRange {
        start: 9283,
        end: 9283,
        cid: 7923,
    },
    CidRange {
        start: 9315,
        end: 9315,
        cid: 7924,
    },
    CidRange {
        start: 9317,
        end: 9317,
        cid: 7925,
    },
    CidRange {
        start: 9319,
        end: 9319,
        cid: 7926,
    },
    CidRange {
        start: 9326,
        end: 9326,
        cid: 7927,
    },
    CidRange {
        start: 9505,
        end: 9505,
        cid: 7928,
    },
    CidRange {
        start: 9507,
        end: 9507,
        cid: 7929,
    },
    CidRange {
        start: 9509,
        end: 9509,
        cid: 7930,
    },
    CidRange {
        start: 9511,
        end: 9511,
        cid: 7931,
    },
    CidRange {
        start: 9513,
        end: 9513,
        cid: 7932,
    },
    CidRange {
        start: 9539,
        end: 9539,
        cid: 7933,
    },
    CidRange {
        start: 9571,
        end: 9571,
        cid: 7934,
    },
    CidRange {
        start: 9573,
        end: 9573,
        cid: 7935,
    },
    CidRange {
        start: 9575,
        end: 9575,
        cid: 7936,
    },
    CidRange {
        start: 9582,
        end: 9582,
        cid: 7937,
    },
    CidRange {
        start: 9589,
        end: 9590,
        cid: 7938,
    },
];

pub const H: CMap = CMap {
    name: Cow::Borrowed(b"H"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(JAPAN_1),
        supplement: 1,
    },
    writing_mode: CMapWritingMode::Horizontal,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: Cow::Borrowed(&NO_CID_CHARS),
    cid_range: Cow::Borrowed(&CID_RANGE_H),
};

pub const V: CMap = CMap {
    name: Cow::Borrowed(b"V"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(JAPAN_1),
        supplement: 1,
    },
    writing_mode: CMapWritingMode::Vertical,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: Cow::Borrowed(&NO_CID_CHARS),
    cid_range: Cow::Borrowed(&CID_RANGE_V),
};
