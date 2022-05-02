use std::borrow::Cow;

use crate::cmaps::cmap::{
    CMap, CMapWritingMode, CidRange, Codespace, CodespaceRange, NO_CID_CHARS,
};
use crate::font::font::CidSystemInfo;

const CODE_SPACE: [CodespaceRange; 5] = [
    [0..=0, 0..=0, 0..=0, 0..=128],
    [0..=0, 0..=0, 129..=159, 64..=252],
    [0..=0, 0..=0, 0..=0, 160..=223],
    [0..=0, 0..=0, 224..=252, 64..=252],
    [0..=0, 0..=0, 0..=0, 253..=255],
];

const CID_RANGE_H: [CidRange; 222] = [
    CidRange {
        start: 32,
        end: 126,
        cid: 1,
    },
    CidRange {
        start: 128,
        end: 128,
        cid: 97,
    },
    CidRange {
        start: 33088,
        end: 33150,
        cid: 633,
    },
    CidRange {
        start: 33152,
        end: 33196,
        cid: 696,
    },
    CidRange {
        start: 33208,
        end: 33215,
        cid: 741,
    },
    CidRange {
        start: 33224,
        end: 33230,
        cid: 749,
    },
    CidRange {
        start: 33242,
        end: 33256,
        cid: 756,
    },
    CidRange {
        start: 33264,
        end: 33271,
        cid: 771,
    },
    CidRange {
        start: 33276,
        end: 33276,
        cid: 779,
    },
    CidRange {
        start: 33359,
        end: 33368,
        cid: 780,
    },
    CidRange {
        start: 33376,
        end: 33401,
        cid: 790,
    },
    CidRange {
        start: 33409,
        end: 33434,
        cid: 816,
    },
    CidRange {
        start: 33439,
        end: 33521,
        cid: 842,
    },
    CidRange {
        start: 33600,
        end: 33662,
        cid: 925,
    },
    CidRange {
        start: 33664,
        end: 33686,
        cid: 988,
    },
    CidRange {
        start: 33695,
        end: 33718,
        cid: 1011,
    },
    CidRange {
        start: 33727,
        end: 33750,
        cid: 1035,
    },
    CidRange {
        start: 33856,
        end: 33888,
        cid: 1059,
    },
    CidRange {
        start: 33904,
        end: 33918,
        cid: 1092,
    },
    CidRange {
        start: 33920,
        end: 33937,
        cid: 1107,
    },
    CidRange {
        start: 33951,
        end: 33951,
        cid: 7479,
    },
    CidRange {
        start: 33952,
        end: 33952,
        cid: 7481,
    },
    CidRange {
        start: 33953,
        end: 33953,
        cid: 7491,
    },
    CidRange {
        start: 33954,
        end: 33954,
        cid: 7495,
    },
    CidRange {
        start: 33955,
        end: 33955,
        cid: 7503,
    },
    CidRange {
        start: 33956,
        end: 33956,
        cid: 7499,
    },
    CidRange {
        start: 33957,
        end: 33957,
        cid: 7507,
    },
    CidRange {
        start: 33958,
        end: 33958,
        cid: 7523,
    },
    CidRange {
        start: 33959,
        end: 33959,
        cid: 7515,
    },
    CidRange {
        start: 33960,
        end: 33960,
        cid: 7531,
    },
    CidRange {
        start: 33961,
        end: 33961,
        cid: 7539,
    },
    CidRange {
        start: 33962,
        end: 33962,
        cid: 7480,
    },
    CidRange {
        start: 33963,
        end: 33963,
        cid: 7482,
    },
    CidRange {
        start: 33964,
        end: 33964,
        cid: 7494,
    },
    CidRange {
        start: 33965,
        end: 33965,
        cid: 7498,
    },
    CidRange {
        start: 33966,
        end: 33966,
        cid: 7506,
    },
    CidRange {
        start: 33967,
        end: 33967,
        cid: 7502,
    },
    CidRange {
        start: 33968,
        end: 33968,
        cid: 7514,
    },
    CidRange {
        start: 33969,
        end: 33969,
        cid: 7530,
    },
    CidRange {
        start: 33970,
        end: 33970,
        cid: 7522,
    },
    CidRange {
        start: 33971,
        end: 33971,
        cid: 7538,
    },
    CidRange {
        start: 33972,
        end: 33972,
        cid: 7554,
    },
    CidRange {
        start: 33973,
        end: 33973,
        cid: 7511,
    },
    CidRange {
        start: 33974,
        end: 33974,
        cid: 7526,
    },
    CidRange {
        start: 33975,
        end: 33975,
        cid: 7519,
    },
    CidRange {
        start: 33976,
        end: 33976,
        cid: 7534,
    },
    CidRange {
        start: 33977,
        end: 33977,
        cid: 7542,
    },
    CidRange {
        start: 33978,
        end: 33978,
        cid: 7508,
    },
    CidRange {
        start: 33979,
        end: 33979,
        cid: 7527,
    },
    CidRange {
        start: 33980,
        end: 33980,
        cid: 7516,
    },
    CidRange {
        start: 33981,
        end: 33981,
        cid: 7535,
    },
    CidRange {
        start: 33982,
        end: 33982,
        cid: 7545,
    },
    CidRange {
        start: 34112,
        end: 34174,
        cid: 232,
    },
    CidRange {
        start: 34176,
        end: 34176,
        cid: 390,
    },
    CidRange {
        start: 34177,
        end: 34206,
        cid: 296,
    },
    CidRange {
        start: 34207,
        end: 34269,
        cid: 327,
    },
    CidRange {
        start: 34270,
        end: 34300,
        cid: 391,
    },
    CidRange {
        start: 34368,
        end: 34430,
        cid: 422,
    },
    CidRange {
        start: 34432,
        end: 34449,
        cid: 485,
    },
    CidRange {
        start: 34450,
        end: 34450,
        cid: 295,
    },
    CidRange {
        start: 34451,
        end: 34462,
        cid: 503,
    },
    CidRange {
        start: 34466,
        end: 34541,
        cid: 7479,
    },
    CidRange {
        start: 34624,
        end: 34653,
        cid: 7555,
    },
    CidRange {
        start: 34655,
        end: 34677,
        cid: 7585,
    },
    CidRange {
        start: 34688,
        end: 34703,
        cid: 7608,
    },
    CidRange {
        start: 34704,
        end: 34704,
        cid: 762,
    },
    CidRange {
        start: 34705,
        end: 34705,
        cid: 761,
    },
    CidRange {
        start: 34706,
        end: 34706,
        cid: 769,
    },
    CidRange {
        start: 34707,
        end: 34713,
        cid: 7624,
    },
    CidRange {
        start: 34714,
        end: 34714,
        cid: 768,
    },
    CidRange {
        start: 34715,
        end: 34716,
        cid: 7631,
    },
    CidRange {
        start: 34975,
        end: 35068,
        cid: 1125,
    },
    CidRange {
        start: 35136,
        end: 35198,
        cid: 1219,
    },
    CidRange {
        start: 35200,
        end: 35324,
        cid: 1282,
    },
    CidRange {
        start: 35392,
        end: 35454,
        cid: 1407,
    },
    CidRange {
        start: 35456,
        end: 35580,
        cid: 1470,
    },
    CidRange {
        start: 35648,
        end: 35710,
        cid: 1595,
    },
    CidRange {
        start: 35712,
        end: 35836,
        cid: 1658,
    },
    CidRange {
        start: 35904,
        end: 35966,
        cid: 1783,
    },
    CidRange {
        start: 35968,
        end: 36092,
        cid: 1846,
    },
    CidRange {
        start: 36160,
        end: 36222,
        cid: 1971,
    },
    CidRange {
        start: 36224,
        end: 36348,
        cid: 2034,
    },
    CidRange {
        start: 36416,
        end: 36478,
        cid: 2159,
    },
    CidRange {
        start: 36480,
        end: 36604,
        cid: 2222,
    },
    CidRange {
        start: 36672,
        end: 36734,
        cid: 2347,
    },
    CidRange {
        start: 36736,
        end: 36860,
        cid: 2410,
    },
    CidRange {
        start: 36928,
        end: 36990,
        cid: 2535,
    },
    CidRange {
        start: 36992,
        end: 37116,
        cid: 2598,
    },
    CidRange {
        start: 37184,
        end: 37246,
        cid: 2723,
    },
    CidRange {
        start: 37248,
        end: 37372,
        cid: 2786,
    },
    CidRange {
        start: 37440,
        end: 37502,
        cid: 2911,
    },
    CidRange {
        start: 37504,
        end: 37628,
        cid: 2974,
    },
    CidRange {
        start: 37696,
        end: 37758,
        cid: 3099,
    },
    CidRange {
        start: 37760,
        end: 37884,
        cid: 3162,
    },
    CidRange {
        start: 37952,
        end: 38014,
        cid: 3287,
    },
    CidRange {
        start: 38016,
        end: 38140,
        cid: 3350,
    },
    CidRange {
        start: 38208,
        end: 38270,
        cid: 3475,
    },
    CidRange {
        start: 38272,
        end: 38396,
        cid: 3538,
    },
    CidRange {
        start: 38464,
        end: 38526,
        cid: 3663,
    },
    CidRange {
        start: 38528,
        end: 38652,
        cid: 3726,
    },
    CidRange {
        start: 38720,
        end: 38782,
        cid: 3851,
    },
    CidRange {
        start: 38784,
        end: 38908,
        cid: 3914,
    },
    CidRange {
        start: 38976,
        end: 39026,
        cid: 4039,
    },
    CidRange {
        start: 39071,
        end: 39164,
        cid: 4090,
    },
    CidRange {
        start: 39232,
        end: 39294,
        cid: 4184,
    },
    CidRange {
        start: 39296,
        end: 39420,
        cid: 4247,
    },
    CidRange {
        start: 39488,
        end: 39550,
        cid: 4372,
    },
    CidRange {
        start: 39552,
        end: 39676,
        cid: 4435,
    },
    CidRange {
        start: 39744,
        end: 39806,
        cid: 4560,
    },
    CidRange {
        start: 39808,
        end: 39932,
        cid: 4623,
    },
    CidRange {
        start: 40000,
        end: 40062,
        cid: 4748,
    },
    CidRange {
        start: 40064,
        end: 40188,
        cid: 4811,
    },
    CidRange {
        start: 40256,
        end: 40318,
        cid: 4936,
    },
    CidRange {
        start: 40320,
        end: 40444,
        cid: 4999,
    },
    CidRange {
        start: 40512,
        end: 40574,
        cid: 5124,
    },
    CidRange {
        start: 40576,
        end: 40700,
        cid: 5187,
    },
    CidRange {
        start: 40768,
        end: 40830,
        cid: 5312,
    },
    CidRange {
        start: 40832,
        end: 40956,
        cid: 5375,
    },
    CidRange {
        start: 160,
        end: 223,
        cid: 326,
    },
    CidRange {
        start: 57408,
        end: 57470,
        cid: 5500,
    },
    CidRange {
        start: 57472,
        end: 57596,
        cid: 5563,
    },
    CidRange {
        start: 57664,
        end: 57726,
        cid: 5688,
    },
    CidRange {
        start: 57728,
        end: 57852,
        cid: 5751,
    },
    CidRange {
        start: 57920,
        end: 57982,
        cid: 5876,
    },
    CidRange {
        start: 57984,
        end: 58108,
        cid: 5939,
    },
    CidRange {
        start: 58176,
        end: 58238,
        cid: 6064,
    },
    CidRange {
        start: 58240,
        end: 58364,
        cid: 6127,
    },
    CidRange {
        start: 58432,
        end: 58494,
        cid: 6252,
    },
    CidRange {
        start: 58496,
        end: 58620,
        cid: 6315,
    },
    CidRange {
        start: 58688,
        end: 58750,
        cid: 6440,
    },
    CidRange {
        start: 58752,
        end: 58876,
        cid: 6503,
    },
    CidRange {
        start: 58944,
        end: 59006,
        cid: 6628,
    },
    CidRange {
        start: 59008,
        end: 59132,
        cid: 6691,
    },
    CidRange {
        start: 59200,
        end: 59262,
        cid: 6816,
    },
    CidRange {
        start: 59264,
        end: 59388,
        cid: 6879,
    },
    CidRange {
        start: 59456,
        end: 59518,
        cid: 7004,
    },
    CidRange {
        start: 59520,
        end: 59644,
        cid: 7067,
    },
    CidRange {
        start: 59712,
        end: 59774,
        cid: 7192,
    },
    CidRange {
        start: 59776,
        end: 59900,
        cid: 7255,
    },
    CidRange {
        start: 59968,
        end: 60030,
        cid: 7380,
    },
    CidRange {
        start: 60032,
        end: 60066,
        cid: 7443,
    },
    CidRange {
        start: 60067,
        end: 60068,
        cid: 8284,
    },
    CidRange {
        start: 60224,
        end: 60224,
        cid: 633,
    },
    CidRange {
        start: 60225,
        end: 60226,
        cid: 7887,
    },
    CidRange {
        start: 60227,
        end: 60239,
        cid: 636,
    },
    CidRange {
        start: 60240,
        end: 60241,
        cid: 7889,
    },
    CidRange {
        start: 60242,
        end: 60250,
        cid: 651,
    },
    CidRange {
        start: 60251,
        end: 60253,
        cid: 7891,
    },
    CidRange {
        start: 60254,
        end: 60255,
        cid: 663,
    },
    CidRange {
        start: 60256,
        end: 60260,
        cid: 7894,
    },
    CidRange {
        start: 60261,
        end: 60264,
        cid: 670,
    },
    CidRange {
        start: 60265,
        end: 60282,
        cid: 7899,
    },
    CidRange {
        start: 60283,
        end: 60286,
        cid: 692,
    },
    CidRange {
        start: 60288,
        end: 60288,
        cid: 696,
    },
    CidRange {
        start: 60289,
        end: 60289,
        cid: 7917,
    },
    CidRange {
        start: 60290,
        end: 60332,
        cid: 698,
    },
    CidRange {
        start: 60344,
        end: 60351,
        cid: 741,
    },
    CidRange {
        start: 60360,
        end: 60366,
        cid: 749,
    },
    CidRange {
        start: 60378,
        end: 60392,
        cid: 756,
    },
    CidRange {
        start: 60400,
        end: 60407,
        cid: 771,
    },
    CidRange {
        start: 60412,
        end: 60412,
        cid: 779,
    },
    CidRange {
        start: 60495,
        end: 60504,
        cid: 780,
    },
    CidRange {
        start: 60512,
        end: 60537,
        cid: 790,
    },
    CidRange {
        start: 60545,
        end: 60570,
        cid: 816,
    },
    CidRange {
        start: 60575,
        end: 60575,
        cid: 7918,
    },
    CidRange {
        start: 60576,
        end: 60576,
        cid: 843,
    },
    CidRange {
        start: 60577,
        end: 60577,
        cid: 7919,
    },
    CidRange {
        start: 60578,
        end: 60578,
        cid: 845,
    },
    CidRange {
        start: 60579,
        end: 60579,
        cid: 7920,
    },
    CidRange {
        start: 60580,
        end: 60580,
        cid: 847,
    },
    CidRange {
        start: 60581,
        end: 60581,
        cid: 7921,
    },
    CidRange {
        start: 60582,
        end: 60582,
        cid: 849,
    },
    CidRange {
        start: 60583,
        end: 60583,
        cid: 7922,
    },
    CidRange {
        start: 60584,
        end: 60608,
        cid: 851,
    },
    CidRange {
        start: 60609,
        end: 60609,
        cid: 7923,
    },
    CidRange {
        start: 60610,
        end: 60640,
        cid: 877,
    },
    CidRange {
        start: 60641,
        end: 60641,
        cid: 7924,
    },
    CidRange {
        start: 60642,
        end: 60642,
        cid: 909,
    },
    CidRange {
        start: 60643,
        end: 60643,
        cid: 7925,
    },
    CidRange {
        start: 60644,
        end: 60644,
        cid: 911,
    },
    CidRange {
        start: 60645,
        end: 60645,
        cid: 7926,
    },
    CidRange {
        start: 60646,
        end: 60651,
        cid: 913,
    },
    CidRange {
        start: 60652,
        end: 60652,
        cid: 7927,
    },
    CidRange {
        start: 60653,
        end: 60657,
        cid: 920,
    },
    CidRange {
        start: 60736,
        end: 60736,
        cid: 7928,
    },
    CidRange {
        start: 60737,
        end: 60737,
        cid: 926,
    },
    CidRange {
        start: 60738,
        end: 60738,
        cid: 7929,
    },
    CidRange {
        start: 60739,
        end: 60739,
        cid: 928,
    },
    CidRange {
        start: 60740,
        end: 60740,
        cid: 7930,
    },
    CidRange {
        start: 60741,
        end: 60741,
        cid: 930,
    },
    CidRange {
        start: 60742,
        end: 60742,
        cid: 7931,
    },
    CidRange {
        start: 60743,
        end: 60743,
        cid: 932,
    },
    CidRange {
        start: 60744,
        end: 60744,
        cid: 7932,
    },
    CidRange {
        start: 60745,
        end: 60769,
        cid: 934,
    },
    CidRange {
        start: 60770,
        end: 60770,
        cid: 7933,
    },
    CidRange {
        start: 60771,
        end: 60798,
        cid: 960,
    },
    CidRange {
        start: 60800,
        end: 60802,
        cid: 988,
    },
    CidRange {
        start: 60803,
        end: 60803,
        cid: 7934,
    },
    CidRange {
        start: 60804,
        end: 60804,
        cid: 992,
    },
    CidRange {
        start: 60805,
        end: 60805,
        cid: 7935,
    },
    CidRange {
        start: 60806,
        end: 60806,
        cid: 994,
    },
    CidRange {
        start: 60807,
        end: 60807,
        cid: 7936,
    },
    CidRange {
        start: 60808,
        end: 60813,
        cid: 996,
    },
    CidRange {
        start: 60814,
        end: 60814,
        cid: 7937,
    },
    CidRange {
        start: 60815,
        end: 60820,
        cid: 1003,
    },
    CidRange {
        start: 60821,
        end: 60822,
        cid: 7938,
    },
    CidRange {
        start: 60831,
        end: 60854,
        cid: 1011,
    },
    CidRange {
        start: 60863,
        end: 60886,
        cid: 1035,
    },
    CidRange {
        start: 60992,
        end: 61021,
        cid: 7555,
    },
    CidRange {
        start: 61023,
        end: 61038,
        cid: 7940,
    },
    CidRange {
        start: 61039,
        end: 61045,
        cid: 7601,
    },
    CidRange {
        start: 61056,
        end: 61057,
        cid: 7956,
    },
    CidRange {
        start: 61058,
        end: 61071,
        cid: 7610,
    },
    CidRange {
        start: 61072,
        end: 61072,
        cid: 762,
    },
    CidRange {
        start: 61073,
        end: 61073,
        cid: 761,
    },
    CidRange {
        start: 61074,
        end: 61074,
        cid: 769,
    },
    CidRange {
        start: 61075,
        end: 61081,
        cid: 7624,
    },
    CidRange {
        start: 61082,
        end: 61082,
        cid: 768,
    },
    CidRange {
        start: 61083,
        end: 61084,
        cid: 7631,
    },
    CidRange {
        start: 253,
        end: 253,
        cid: 152,
    },
    CidRange {
        start: 254,
        end: 254,
        cid: 228,
    },
    CidRange {
        start: 255,
        end: 255,
        cid: 124,
    },
];

pub const JAPAN_1_83PV_RKSJ_H: CMap = CMap {
    name: b"83pv-RKSJ-H",
    cid_system_info: CidSystemInfo {
        registry: b"Adobe",
        ordering: b"Japan1",
        supplement: 1,
    },
    writing_mode: CMapWritingMode::Horizontal,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: Cow::Borrowed(&NO_CID_CHARS),
    cid_range: Cow::Borrowed(&CID_RANGE_H),
};
