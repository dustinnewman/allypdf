use std::borrow::Cow;

use crate::cmaps::cmap::{
    CMap, CMapWritingMode, CidRange, Codespace, CodespaceRange, NO_CID_CHARS, ADOBE_REGISTRY, NO_BASE_FONT_CHARS
};
use crate::font::font::CidSystemInfo;

use super::JAPAN_1;

const CODE_SPACE: [CodespaceRange; 4] = [
    [0..=0, 0..=0, 0..=0, 0..=128],
    [0..=0, 0..=0, 129..=159, 64..=252],
    [0..=0, 0..=0, 0..=0, 160..=223],
    [0..=0, 0..=0, 224..=252, 64..=252],
];

const CID_RANGE_H: [CidRange; 170] = [
    CidRange {
        start: 32,
        end: 126,
        cid: 1,
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
        start: 34624,
        end: 34653,
        cid: 7555,
    },
    CidRange {
        start: 34655,
        end: 34656,
        cid: 7585,
    },
    CidRange {
        start: 34657,
        end: 34657,
        cid: 8038,
    },
    CidRange {
        start: 34658,
        end: 34658,
        cid: 7588,
    },
    CidRange {
        start: 34659,
        end: 34659,
        cid: 8040,
    },
    CidRange {
        start: 34660,
        end: 34660,
        cid: 7590,
    },
    CidRange {
        start: 34661,
        end: 34661,
        cid: 8042,
    },
    CidRange {
        start: 34662,
        end: 34663,
        cid: 7592,
    },
    CidRange {
        start: 34664,
        end: 34664,
        cid: 8044,
    },
    CidRange {
        start: 34665,
        end: 34666,
        cid: 7595,
    },
    CidRange {
        start: 34667,
        end: 34667,
        cid: 8043,
    },
    CidRange {
        start: 34668,
        end: 34669,
        cid: 7598,
    },
    CidRange {
        start: 34670,
        end: 34670,
        cid: 8047,
    },
    CidRange {
        start: 34671,
        end: 34677,
        cid: 7601,
    },
    CidRange {
        start: 34686,
        end: 34686,
        cid: 8323,
    },
    CidRange {
        start: 34688,
        end: 34691,
        cid: 7608,
    },
    CidRange {
        start: 34692,
        end: 34692,
        cid: 8055,
    },
    CidRange {
        start: 34693,
        end: 34703,
        cid: 7613,
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
        start: 60736,
        end: 60798,
        cid: 8359,
    },
    CidRange {
        start: 60800,
        end: 60851,
        cid: 8422,
    },
    CidRange {
        start: 60852,
        end: 60852,
        cid: 1993,
    },
    CidRange {
        start: 60853,
        end: 60924,
        cid: 8474,
    },
    CidRange {
        start: 60992,
        end: 61054,
        cid: 8546,
    },
    CidRange {
        start: 61056,
        end: 61164,
        cid: 8609,
    },
    CidRange {
        start: 61167,
        end: 61176,
        cid: 8092,
    },
    CidRange {
        start: 61177,
        end: 61177,
        cid: 751,
    },
    CidRange {
        start: 61178,
        end: 61180,
        cid: 8005,
    },
    CidRange {
        start: 64064,
        end: 64073,
        cid: 8092,
    },
    CidRange {
        start: 64074,
        end: 64083,
        cid: 7575,
    },
    CidRange {
        start: 64084,
        end: 64084,
        cid: 751,
    },
    CidRange {
        start: 64085,
        end: 64087,
        cid: 8005,
    },
    CidRange {
        start: 64088,
        end: 64088,
        cid: 7618,
    },
    CidRange {
        start: 64089,
        end: 64089,
        cid: 7610,
    },
    CidRange {
        start: 64090,
        end: 64090,
        cid: 8055,
    },
    CidRange {
        start: 64091,
        end: 64091,
        cid: 768,
    },
    CidRange {
        start: 64092,
        end: 64126,
        cid: 8359,
    },
    CidRange {
        start: 64128,
        end: 64207,
        cid: 8394,
    },
    CidRange {
        start: 64208,
        end: 64208,
        cid: 1993,
    },
    CidRange {
        start: 64209,
        end: 64252,
        cid: 8474,
    },
    CidRange {
        start: 64320,
        end: 64382,
        cid: 8518,
    },
    CidRange {
        start: 64384,
        end: 64508,
        cid: 8581,
    },
    CidRange {
        start: 64576,
        end: 64587,
        cid: 8706,
    },
];

const CID_RANGE_V: [CidRange; 78] = [
    CidRange {
        start: 33089,
        end: 33090,
        cid: 7887,
    },
    CidRange {
        start: 33091,
        end: 33091,
        cid: 8268,
    },
    CidRange {
        start: 33092,
        end: 33092,
        cid: 8274,
    },
    CidRange {
        start: 33104,
        end: 33105,
        cid: 7889,
    },
    CidRange {
        start: 33115,
        end: 33117,
        cid: 7891,
    },
    CidRange {
        start: 33120,
        end: 33124,
        cid: 7894,
    },
    CidRange {
        start: 33129,
        end: 33146,
        cid: 7899,
    },
    CidRange {
        start: 33153,
        end: 33153,
        cid: 7917,
    },
    CidRange {
        start: 33192,
        end: 33192,
        cid: 739,
    },
    CidRange {
        start: 33193,
        end: 33193,
        cid: 738,
    },
    CidRange {
        start: 33194,
        end: 33195,
        cid: 736,
    },
    CidRange {
        start: 33196,
        end: 33196,
        cid: 8270,
    },
    CidRange {
        start: 33439,
        end: 33439,
        cid: 7918,
    },
    CidRange {
        start: 33441,
        end: 33441,
        cid: 7919,
    },
    CidRange {
        start: 33443,
        end: 33443,
        cid: 7920,
    },
    CidRange {
        start: 33445,
        end: 33445,
        cid: 7921,
    },
    CidRange {
        start: 33447,
        end: 33447,
        cid: 7922,
    },
    CidRange {
        start: 33473,
        end: 33473,
        cid: 7923,
    },
    CidRange {
        start: 33505,
        end: 33505,
        cid: 7924,
    },
    CidRange {
        start: 33507,
        end: 33507,
        cid: 7925,
    },
    CidRange {
        start: 33509,
        end: 33509,
        cid: 7926,
    },
    CidRange {
        start: 33516,
        end: 33516,
        cid: 7927,
    },
    CidRange {
        start: 33600,
        end: 33600,
        cid: 7928,
    },
    CidRange {
        start: 33602,
        end: 33602,
        cid: 7929,
    },
    CidRange {
        start: 33604,
        end: 33604,
        cid: 7930,
    },
    CidRange {
        start: 33606,
        end: 33606,
        cid: 7931,
    },
    CidRange {
        start: 33608,
        end: 33608,
        cid: 7932,
    },
    CidRange {
        start: 33634,
        end: 33634,
        cid: 7933,
    },
    CidRange {
        start: 33667,
        end: 33667,
        cid: 7934,
    },
    CidRange {
        start: 33669,
        end: 33669,
        cid: 7935,
    },
    CidRange {
        start: 33671,
        end: 33671,
        cid: 7936,
    },
    CidRange {
        start: 33678,
        end: 33678,
        cid: 7937,
    },
    CidRange {
        start: 33685,
        end: 33686,
        cid: 7938,
    },
    CidRange {
        start: 33951,
        end: 33951,
        cid: 7481,
    },
    CidRange {
        start: 33952,
        end: 33952,
        cid: 7479,
    },
    CidRange {
        start: 33953,
        end: 33953,
        cid: 7495,
    },
    CidRange {
        start: 33954,
        end: 33954,
        cid: 7503,
    },
    CidRange {
        start: 33955,
        end: 33955,
        cid: 7499,
    },
    CidRange {
        start: 33956,
        end: 33956,
        cid: 7491,
    },
    CidRange {
        start: 33957,
        end: 33957,
        cid: 7523,
    },
    CidRange {
        start: 33958,
        end: 33958,
        cid: 7515,
    },
    CidRange {
        start: 33959,
        end: 33959,
        cid: 7531,
    },
    CidRange {
        start: 33960,
        end: 33960,
        cid: 7507,
    },
    CidRange {
        start: 33961,
        end: 33961,
        cid: 7539,
    },
    CidRange {
        start: 33962,
        end: 33962,
        cid: 7482,
    },
    CidRange {
        start: 33963,
        end: 33963,
        cid: 7480,
    },
    CidRange {
        start: 33964,
        end: 33964,
        cid: 7498,
    },
    CidRange {
        start: 33965,
        end: 33965,
        cid: 7506,
    },
    CidRange {
        start: 33966,
        end: 33966,
        cid: 7502,
    },
    CidRange {
        start: 33967,
        end: 33967,
        cid: 7494,
    },
    CidRange {
        start: 33968,
        end: 33968,
        cid: 7530,
    },
    CidRange {
        start: 33969,
        end: 33969,
        cid: 7522,
    },
    CidRange {
        start: 33970,
        end: 33970,
        cid: 7538,
    },
    CidRange {
        start: 33971,
        end: 33971,
        cid: 7514,
    },
    CidRange {
        start: 33972,
        end: 33972,
        cid: 7554,
    },
    CidRange {
        start: 33973,
        end: 33973,
        cid: 7526,
    },
    CidRange {
        start: 33974,
        end: 33974,
        cid: 7519,
    },
    CidRange {
        start: 33975,
        end: 33975,
        cid: 7534,
    },
    CidRange {
        start: 33976,
        end: 33976,
        cid: 7511,
    },
    CidRange {
        start: 33977,
        end: 33977,
        cid: 7545,
    },
    CidRange {
        start: 33978,
        end: 33978,
        cid: 7527,
    },
    CidRange {
        start: 33979,
        end: 33979,
        cid: 7516,
    },
    CidRange {
        start: 33980,
        end: 33980,
        cid: 7535,
    },
    CidRange {
        start: 33981,
        end: 33981,
        cid: 7508,
    },
    CidRange {
        start: 33982,
        end: 33982,
        cid: 7542,
    },
    CidRange {
        start: 34655,
        end: 34656,
        cid: 7940,
    },
    CidRange {
        start: 34657,
        end: 34657,
        cid: 8329,
    },
    CidRange {
        start: 34658,
        end: 34658,
        cid: 7943,
    },
    CidRange {
        start: 34659,
        end: 34659,
        cid: 8339,
    },
    CidRange {
        start: 34660,
        end: 34660,
        cid: 7945,
    },
    CidRange {
        start: 34661,
        end: 34661,
        cid: 8338,
    },
    CidRange {
        start: 34662,
        end: 34663,
        cid: 7947,
    },
    CidRange {
        start: 34664,
        end: 34664,
        cid: 8344,
    },
    CidRange {
        start: 34665,
        end: 34666,
        cid: 7950,
    },
    CidRange {
        start: 34667,
        end: 34667,
        cid: 8348,
    },
    CidRange {
        start: 34668,
        end: 34669,
        cid: 7953,
    },
    CidRange {
        start: 34670,
        end: 34670,
        cid: 8349,
    },
    CidRange {
        start: 34688,
        end: 34689,
        cid: 7956,
    },
];

pub const JAPAN_1_90MSP_RKSJ_H: CMap = CMap {
    name: Cow::Borrowed(b"90msp-RKSJ-H"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(JAPAN_1),
        supplement: 2,
    },
    writing_mode: CMapWritingMode::Horizontal,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: NO_CID_CHARS,
    cid_range: Cow::Borrowed(&CID_RANGE_H),
    base_font_chars: NO_BASE_FONT_CHARS,
};

pub const JAPAN_1_90MSP_RKSJ_V: CMap = CMap {
    name: Cow::Borrowed(b"90msp-RKSJ-V"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(JAPAN_1),
        supplement: 2,
    },
    writing_mode: CMapWritingMode::Vertical,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: NO_CID_CHARS,
    cid_range: Cow::Borrowed(&CID_RANGE_V),
    base_font_chars: NO_BASE_FONT_CHARS,
};
