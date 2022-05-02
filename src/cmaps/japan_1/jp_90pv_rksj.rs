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

const CID_RANGE_H: [CidRange; 263] = [
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
        end: 34131,
        cid: 7555,
    },
    CidRange {
        start: 34142,
        end: 34161,
        cid: 8071,
    },
    CidRange {
        start: 34172,
        end: 34174,
        cid: 8286,
    },
    CidRange {
        start: 34176,
        end: 34181,
        cid: 8289,
    },
    CidRange {
        start: 34193,
        end: 34202,
        cid: 8061,
    },
    CidRange {
        start: 34207,
        end: 34216,
        cid: 7575,
    },
    CidRange {
        start: 34217,
        end: 34218,
        cid: 8225,
    },
    CidRange {
        start: 34219,
        end: 34221,
        cid: 8295,
    },
    CidRange {
        start: 34227,
        end: 34236,
        cid: 8092,
    },
    CidRange {
        start: 34237,
        end: 34241,
        cid: 8298,
    },
    CidRange {
        start: 34267,
        end: 34292,
        cid: 8112,
    },
    CidRange {
        start: 34368,
        end: 34368,
        cid: 7601,
    },
    CidRange {
        start: 34369,
        end: 34369,
        cid: 8186,
    },
    CidRange {
        start: 34370,
        end: 34370,
        cid: 7602,
    },
    CidRange {
        start: 34371,
        end: 34371,
        cid: 8020,
    },
    CidRange {
        start: 34372,
        end: 34372,
        cid: 8022,
    },
    CidRange {
        start: 34373,
        end: 34373,
        cid: 8303,
    },
    CidRange {
        start: 34374,
        end: 34374,
        cid: 7607,
    },
    CidRange {
        start: 34375,
        end: 34375,
        cid: 8023,
    },
    CidRange {
        start: 34376,
        end: 34376,
        cid: 7603,
    },
    CidRange {
        start: 34377,
        end: 34377,
        cid: 8021,
    },
    CidRange {
        start: 34378,
        end: 34378,
        cid: 7604,
    },
    CidRange {
        start: 34379,
        end: 34379,
        cid: 8304,
    },
    CidRange {
        start: 34380,
        end: 34381,
        cid: 7605,
    },
    CidRange {
        start: 34382,
        end: 34382,
        cid: 8037,
    },
    CidRange {
        start: 34383,
        end: 34389,
        cid: 8024,
    },
    CidRange {
        start: 34390,
        end: 34390,
        cid: 8305,
    },
    CidRange {
        start: 34391,
        end: 34391,
        cid: 8036,
    },
    CidRange {
        start: 34392,
        end: 34393,
        cid: 8034,
    },
    CidRange {
        start: 34394,
        end: 34396,
        cid: 8031,
    },
    CidRange {
        start: 34397,
        end: 34397,
        cid: 8306,
    },
    CidRange {
        start: 34459,
        end: 34461,
        cid: 7610,
    },
    CidRange {
        start: 34462,
        end: 34462,
        cid: 8307,
    },
    CidRange {
        start: 34463,
        end: 34463,
        cid: 8018,
    },
    CidRange {
        start: 34464,
        end: 34465,
        cid: 8016,
    },
    CidRange {
        start: 34466,
        end: 34466,
        cid: 8019,
    },
    CidRange {
        start: 34467,
        end: 34467,
        cid: 8211,
    },
    CidRange {
        start: 34468,
        end: 34468,
        cid: 8213,
    },
    CidRange {
        start: 34469,
        end: 34469,
        cid: 8212,
    },
    CidRange {
        start: 34470,
        end: 34470,
        cid: 8214,
    },
    CidRange {
        start: 34483,
        end: 34483,
        cid: 8058,
    },
    CidRange {
        start: 34484,
        end: 34484,
        cid: 8056,
    },
    CidRange {
        start: 34485,
        end: 34485,
        cid: 8308,
    },
    CidRange {
        start: 34503,
        end: 34506,
        cid: 8219,
    },
    CidRange {
        start: 34507,
        end: 34510,
        cid: 8309,
    },
    CidRange {
        start: 34511,
        end: 34511,
        cid: 8014,
    },
    CidRange {
        start: 34512,
        end: 34512,
        cid: 8013,
    },
    CidRange {
        start: 34513,
        end: 34513,
        cid: 8012,
    },
    CidRange {
        start: 34514,
        end: 34514,
        cid: 8011,
    },
    CidRange {
        start: 34515,
        end: 34518,
        cid: 8206,
    },
    CidRange {
        start: 34624,
        end: 34630,
        cid: 8197,
    },
    CidRange {
        start: 34631,
        end: 34631,
        cid: 8150,
    },
    CidRange {
        start: 34632,
        end: 34632,
        cid: 8204,
    },
    CidRange {
        start: 34633,
        end: 34633,
        cid: 8145,
    },
    CidRange {
        start: 34634,
        end: 34634,
        cid: 8138,
    },
    CidRange {
        start: 34635,
        end: 34635,
        cid: 7620,
    },
    CidRange {
        start: 34636,
        end: 34636,
        cid: 8151,
    },
    CidRange {
        start: 34637,
        end: 34637,
        cid: 7618,
    },
    CidRange {
        start: 34638,
        end: 34638,
        cid: 8146,
    },
    CidRange {
        start: 34639,
        end: 34639,
        cid: 8141,
    },
    CidRange {
        start: 34640,
        end: 34640,
        cid: 7619,
    },
    CidRange {
        start: 34641,
        end: 34641,
        cid: 8149,
    },
    CidRange {
        start: 34642,
        end: 34642,
        cid: 8147,
    },
    CidRange {
        start: 34643,
        end: 34643,
        cid: 8143,
    },
    CidRange {
        start: 34644,
        end: 34644,
        cid: 8148,
    },
    CidRange {
        start: 34645,
        end: 34645,
        cid: 8144,
    },
    CidRange {
        start: 34646,
        end: 34647,
        cid: 8139,
    },
    CidRange {
        start: 34648,
        end: 34648,
        cid: 8142,
    },
    CidRange {
        start: 34705,
        end: 34706,
        cid: 8317,
    },
    CidRange {
        start: 34707,
        end: 34711,
        cid: 7613,
    },
    CidRange {
        start: 34712,
        end: 34712,
        cid: 8154,
    },
    CidRange {
        start: 34713,
        end: 34713,
        cid: 8165,
    },
    CidRange {
        start: 34714,
        end: 34714,
        cid: 8319,
    },
    CidRange {
        start: 34715,
        end: 34715,
        cid: 8158,
    },
    CidRange {
        start: 34716,
        end: 34716,
        cid: 8191,
    },
    CidRange {
        start: 34717,
        end: 34717,
        cid: 8320,
    },
    CidRange {
        start: 34718,
        end: 34718,
        cid: 8223,
    },
    CidRange {
        start: 34719,
        end: 34719,
        cid: 7585,
    },
    CidRange {
        start: 34720,
        end: 34720,
        cid: 8038,
    },
    CidRange {
        start: 34721,
        end: 34721,
        cid: 7588,
    },
    CidRange {
        start: 34722,
        end: 34722,
        cid: 7586,
    },
    CidRange {
        start: 34723,
        end: 34723,
        cid: 8039,
    },
    CidRange {
        start: 34724,
        end: 34724,
        cid: 8183,
    },
    CidRange {
        start: 34725,
        end: 34726,
        cid: 8327,
    },
    CidRange {
        start: 34727,
        end: 34727,
        cid: 8042,
    },
    CidRange {
        start: 34728,
        end: 34728,
        cid: 7592,
    },
    CidRange {
        start: 34729,
        end: 34730,
        cid: 8040,
    },
    CidRange {
        start: 34731,
        end: 34731,
        cid: 7590,
    },
    CidRange {
        start: 34732,
        end: 34732,
        cid: 7593,
    },
    CidRange {
        start: 34733,
        end: 34733,
        cid: 7599,
    },
    CidRange {
        start: 34734,
        end: 34734,
        cid: 8046,
    },
    CidRange {
        start: 34735,
        end: 34735,
        cid: 8044,
    },
    CidRange {
        start: 34736,
        end: 34736,
        cid: 7595,
    },
    CidRange {
        start: 34737,
        end: 34737,
        cid: 8045,
    },
    CidRange {
        start: 34738,
        end: 34738,
        cid: 8043,
    },
    CidRange {
        start: 34739,
        end: 34739,
        cid: 7596,
    },
    CidRange {
        start: 34740,
        end: 34740,
        cid: 8047,
    },
    CidRange {
        start: 34741,
        end: 34741,
        cid: 7598,
    },
    CidRange {
        start: 34749,
        end: 34749,
        cid: 8048,
    },
    CidRange {
        start: 34750,
        end: 34751,
        cid: 8051,
    },
    CidRange {
        start: 34752,
        end: 34753,
        cid: 8049,
    },
    CidRange {
        start: 34789,
        end: 34791,
        cid: 7621,
    },
    CidRange {
        start: 34792,
        end: 34792,
        cid: 8323,
    },
    CidRange {
        start: 34810,
        end: 34810,
        cid: 8054,
    },
    CidRange {
        start: 34811,
        end: 34812,
        cid: 8321,
    },
    CidRange {
        start: 34880,
        end: 34880,
        cid: 7624,
    },
    CidRange {
        start: 34881,
        end: 34882,
        cid: 7629,
    },
    CidRange {
        start: 34900,
        end: 34901,
        cid: 7608,
    },
    CidRange {
        start: 34920,
        end: 34920,
        cid: 7958,
    },
    CidRange {
        start: 34922,
        end: 34925,
        cid: 8313,
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
        start: 60225,
        end: 60226,
        cid: 7887,
    },
    CidRange {
        start: 60240,
        end: 60241,
        cid: 7889,
    },
    CidRange {
        start: 60251,
        end: 60253,
        cid: 7891,
    },
    CidRange {
        start: 60256,
        end: 60260,
        cid: 7894,
    },
    CidRange {
        start: 60265,
        end: 60282,
        cid: 7899,
    },
    CidRange {
        start: 60289,
        end: 60289,
        cid: 7917,
    },
    CidRange {
        start: 60575,
        end: 60575,
        cid: 7918,
    },
    CidRange {
        start: 60577,
        end: 60577,
        cid: 7919,
    },
    CidRange {
        start: 60579,
        end: 60579,
        cid: 7920,
    },
    CidRange {
        start: 60581,
        end: 60581,
        cid: 7921,
    },
    CidRange {
        start: 60583,
        end: 60583,
        cid: 7922,
    },
    CidRange {
        start: 60609,
        end: 60609,
        cid: 7923,
    },
    CidRange {
        start: 60641,
        end: 60641,
        cid: 7924,
    },
    CidRange {
        start: 60643,
        end: 60643,
        cid: 7925,
    },
    CidRange {
        start: 60645,
        end: 60645,
        cid: 7926,
    },
    CidRange {
        start: 60652,
        end: 60652,
        cid: 7927,
    },
    CidRange {
        start: 60736,
        end: 60736,
        cid: 7928,
    },
    CidRange {
        start: 60738,
        end: 60738,
        cid: 7929,
    },
    CidRange {
        start: 60740,
        end: 60740,
        cid: 7930,
    },
    CidRange {
        start: 60742,
        end: 60742,
        cid: 7931,
    },
    CidRange {
        start: 60744,
        end: 60744,
        cid: 7932,
    },
    CidRange {
        start: 60770,
        end: 60770,
        cid: 7933,
    },
    CidRange {
        start: 60803,
        end: 60803,
        cid: 7934,
    },
    CidRange {
        start: 60805,
        end: 60805,
        cid: 7935,
    },
    CidRange {
        start: 60807,
        end: 60807,
        cid: 7936,
    },
    CidRange {
        start: 60814,
        end: 60814,
        cid: 7937,
    },
    CidRange {
        start: 60821,
        end: 60822,
        cid: 7938,
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

pub const JAPAN_1_90PV_RKSJ_H: CMap = CMap {
    name: b"90pv-RKSJ-H",
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
