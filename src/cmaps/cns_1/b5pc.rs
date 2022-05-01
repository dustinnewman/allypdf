use std::borrow::Cow;

use crate::cmaps::NO_CID_CHARS;
use crate::font::cmap::{CMap, CMapWritingMode, CidRange, Codespace, CodespaceRange};
use crate::font::font::CidSystemInfo;

const CODE_SPACE: [CodespaceRange; 3] = [
    [0..=0, 0..=0, 0..=0, 0..=128],
    [0..=0, 0..=0, 161..=252, 64..=254],
    [0..=0, 0..=0, 0..=0, 253..=255],
];

const CID_RANGE_H: [CidRange; 247] = [
    CidRange {
        start: 32,
        end: 126,
        cid: 1,
    },
    CidRange {
        start: 128,
        end: 128,
        cid: 61,
    },
    CidRange {
        start: 41280,
        end: 41304,
        cid: 99,
    },
    CidRange {
        start: 41305,
        end: 41308,
        cid: 13743,
    },
    CidRange {
        start: 41309,
        end: 41342,
        cid: 128,
    },
    CidRange {
        start: 41377,
        end: 41461,
        cid: 162,
    },
    CidRange {
        start: 41462,
        end: 41462,
        cid: 248,
    },
    CidRange {
        start: 41463,
        end: 41463,
        cid: 247,
    },
    CidRange {
        start: 41464,
        end: 41470,
        cid: 249,
    },
    CidRange {
        start: 41536,
        end: 41598,
        cid: 256,
    },
    CidRange {
        start: 41633,
        end: 41726,
        cid: 319,
    },
    CidRange {
        start: 41792,
        end: 41854,
        cid: 413,
    },
    CidRange {
        start: 41889,
        end: 41915,
        cid: 476,
    },
    CidRange {
        start: 41917,
        end: 41919,
        cid: 503,
    },
    CidRange {
        start: 41920,
        end: 41952,
        cid: 562,
    },
    CidRange {
        start: 42048,
        end: 42110,
        cid: 595,
    },
    CidRange {
        start: 42145,
        end: 42238,
        cid: 658,
    },
    CidRange {
        start: 42304,
        end: 42366,
        cid: 752,
    },
    CidRange {
        start: 42401,
        end: 42494,
        cid: 815,
    },
    CidRange {
        start: 42560,
        end: 42622,
        cid: 909,
    },
    CidRange {
        start: 42657,
        end: 42750,
        cid: 972,
    },
    CidRange {
        start: 42816,
        end: 42878,
        cid: 1066,
    },
    CidRange {
        start: 42913,
        end: 43006,
        cid: 1129,
    },
    CidRange {
        start: 43072,
        end: 43134,
        cid: 1223,
    },
    CidRange {
        start: 43169,
        end: 43262,
        cid: 1286,
    },
    CidRange {
        start: 43328,
        end: 43390,
        cid: 1380,
    },
    CidRange {
        start: 43425,
        end: 43518,
        cid: 1443,
    },
    CidRange {
        start: 43584,
        end: 43646,
        cid: 1537,
    },
    CidRange {
        start: 43681,
        end: 43774,
        cid: 1600,
    },
    CidRange {
        start: 43840,
        end: 43902,
        cid: 1694,
    },
    CidRange {
        start: 43937,
        end: 44030,
        cid: 1757,
    },
    CidRange {
        start: 44096,
        end: 44158,
        cid: 1851,
    },
    CidRange {
        start: 44193,
        end: 44285,
        cid: 1914,
    },
    CidRange {
        start: 44286,
        end: 44286,
        cid: 2431,
    },
    CidRange {
        start: 44352,
        end: 44414,
        cid: 2007,
    },
    CidRange {
        start: 44449,
        end: 44542,
        cid: 2070,
    },
    CidRange {
        start: 44608,
        end: 44670,
        cid: 2164,
    },
    CidRange {
        start: 44705,
        end: 44798,
        cid: 2227,
    },
    CidRange {
        start: 44864,
        end: 44926,
        cid: 2321,
    },
    CidRange {
        start: 44961,
        end: 45007,
        cid: 2384,
    },
    CidRange {
        start: 45008,
        end: 45054,
        cid: 2432,
    },
    CidRange {
        start: 45120,
        end: 45182,
        cid: 2479,
    },
    CidRange {
        start: 45217,
        end: 45310,
        cid: 2542,
    },
    CidRange {
        start: 45376,
        end: 45438,
        cid: 2636,
    },
    CidRange {
        start: 45473,
        end: 45566,
        cid: 2699,
    },
    CidRange {
        start: 45632,
        end: 45694,
        cid: 2793,
    },
    CidRange {
        start: 45729,
        end: 45822,
        cid: 2856,
    },
    CidRange {
        start: 45888,
        end: 45950,
        cid: 2950,
    },
    CidRange {
        start: 45985,
        end: 46078,
        cid: 3013,
    },
    CidRange {
        start: 46144,
        end: 46206,
        cid: 3107,
    },
    CidRange {
        start: 46241,
        end: 46334,
        cid: 3170,
    },
    CidRange {
        start: 46400,
        end: 46462,
        cid: 3264,
    },
    CidRange {
        start: 46497,
        end: 46590,
        cid: 3327,
    },
    CidRange {
        start: 46656,
        end: 46718,
        cid: 3421,
    },
    CidRange {
        start: 46753,
        end: 46846,
        cid: 3484,
    },
    CidRange {
        start: 46912,
        end: 46974,
        cid: 3578,
    },
    CidRange {
        start: 47009,
        end: 47102,
        cid: 3641,
    },
    CidRange {
        start: 47168,
        end: 47230,
        cid: 3735,
    },
    CidRange {
        start: 47265,
        end: 47358,
        cid: 3798,
    },
    CidRange {
        start: 47424,
        end: 47486,
        cid: 3892,
    },
    CidRange {
        start: 47521,
        end: 47614,
        cid: 3955,
    },
    CidRange {
        start: 47680,
        end: 47742,
        cid: 4049,
    },
    CidRange {
        start: 47777,
        end: 47870,
        cid: 4112,
    },
    CidRange {
        start: 47936,
        end: 47998,
        cid: 4206,
    },
    CidRange {
        start: 48033,
        end: 48071,
        cid: 4269,
    },
    CidRange {
        start: 48072,
        end: 48126,
        cid: 4309,
    },
    CidRange {
        start: 48192,
        end: 48254,
        cid: 4364,
    },
    CidRange {
        start: 48289,
        end: 48382,
        cid: 4427,
    },
    CidRange {
        start: 48448,
        end: 48510,
        cid: 4521,
    },
    CidRange {
        start: 48545,
        end: 48638,
        cid: 4584,
    },
    CidRange {
        start: 48704,
        end: 48721,
        cid: 4678,
    },
    CidRange {
        start: 48722,
        end: 48722,
        cid: 4308,
    },
    CidRange {
        start: 48723,
        end: 48766,
        cid: 4696,
    },
    CidRange {
        start: 48801,
        end: 48894,
        cid: 4740,
    },
    CidRange {
        start: 48960,
        end: 49022,
        cid: 4834,
    },
    CidRange {
        start: 49057,
        end: 49150,
        cid: 4897,
    },
    CidRange {
        start: 49216,
        end: 49278,
        cid: 4991,
    },
    CidRange {
        start: 49313,
        end: 49406,
        cid: 5054,
    },
    CidRange {
        start: 49472,
        end: 49534,
        cid: 5148,
    },
    CidRange {
        start: 49569,
        end: 49578,
        cid: 5211,
    },
    CidRange {
        start: 49579,
        end: 49662,
        cid: 5222,
    },
    CidRange {
        start: 49728,
        end: 49790,
        cid: 5306,
    },
    CidRange {
        start: 49825,
        end: 49866,
        cid: 5369,
    },
    CidRange {
        start: 49867,
        end: 49867,
        cid: 5221,
    },
    CidRange {
        start: 49868,
        end: 49918,
        cid: 5411,
    },
    CidRange {
        start: 49984,
        end: 50016,
        cid: 5462,
    },
    CidRange {
        start: 50017,
        end: 50046,
        cid: 5496,
    },
    CidRange {
        start: 50081,
        end: 50104,
        cid: 5526,
    },
    CidRange {
        start: 50105,
        end: 50105,
        cid: 5551,
    },
    CidRange {
        start: 50106,
        end: 50106,
        cid: 5550,
    },
    CidRange {
        start: 50107,
        end: 50174,
        cid: 5552,
    },
    CidRange {
        start: 50240,
        end: 50261,
        cid: 5620,
    },
    CidRange {
        start: 50262,
        end: 50262,
        cid: 5495,
    },
    CidRange {
        start: 50263,
        end: 50302,
        cid: 5642,
    },
    CidRange {
        start: 50337,
        end: 50430,
        cid: 5682,
    },
    CidRange {
        start: 50496,
        end: 50558,
        cid: 5776,
    },
    CidRange {
        start: 50593,
        end: 50686,
        cid: 5839,
    },
    CidRange {
        start: 50752,
        end: 50814,
        cid: 5933,
    },
    CidRange {
        start: 51520,
        end: 51529,
        cid: 5996,
    },
    CidRange {
        start: 51530,
        end: 51530,
        cid: 628,
    },
    CidRange {
        start: 51531,
        end: 51563,
        cid: 6006,
    },
    CidRange {
        start: 51564,
        end: 51582,
        cid: 6040,
    },
    CidRange {
        start: 51617,
        end: 51645,
        cid: 6059,
    },
    CidRange {
        start: 51646,
        end: 51646,
        cid: 6039,
    },
    CidRange {
        start: 51647,
        end: 51692,
        cid: 6088,
    },
    CidRange {
        start: 51693,
        end: 51710,
        cid: 6135,
    },
    CidRange {
        start: 51776,
        end: 51838,
        cid: 6153,
    },
    CidRange {
        start: 51873,
        end: 51958,
        cid: 6216,
    },
    CidRange {
        start: 51959,
        end: 51959,
        cid: 6134,
    },
    CidRange {
        start: 51960,
        end: 51966,
        cid: 6302,
    },
    CidRange {
        start: 52032,
        end: 52094,
        cid: 6309,
    },
    CidRange {
        start: 52129,
        end: 52222,
        cid: 6372,
    },
    CidRange {
        start: 52288,
        end: 52350,
        cid: 6466,
    },
    CidRange {
        start: 52385,
        end: 52478,
        cid: 6529,
    },
    CidRange {
        start: 52544,
        end: 52606,
        cid: 6623,
    },
    CidRange {
        start: 52641,
        end: 52734,
        cid: 6686,
    },
    CidRange {
        start: 52800,
        end: 52862,
        cid: 6780,
    },
    CidRange {
        start: 52897,
        end: 52990,
        cid: 6843,
    },
    CidRange {
        start: 53056,
        end: 53118,
        cid: 6937,
    },
    CidRange {
        start: 53153,
        end: 53246,
        cid: 7000,
    },
    CidRange {
        start: 53312,
        end: 53374,
        cid: 7094,
    },
    CidRange {
        start: 53409,
        end: 53502,
        cid: 7157,
    },
    CidRange {
        start: 53568,
        end: 53630,
        cid: 7251,
    },
    CidRange {
        start: 53665,
        end: 53758,
        cid: 7314,
    },
    CidRange {
        start: 53824,
        end: 53886,
        cid: 7408,
    },
    CidRange {
        start: 53921,
        end: 54014,
        cid: 7471,
    },
    CidRange {
        start: 54080,
        end: 54142,
        cid: 7565,
    },
    CidRange {
        start: 54177,
        end: 54270,
        cid: 7628,
    },
    CidRange {
        start: 54336,
        end: 54398,
        cid: 7722,
    },
    CidRange {
        start: 54433,
        end: 54526,
        cid: 7785,
    },
    CidRange {
        start: 54592,
        end: 54654,
        cid: 7879,
    },
    CidRange {
        start: 54689,
        end: 54782,
        cid: 7942,
    },
    CidRange {
        start: 54848,
        end: 54910,
        cid: 8036,
    },
    CidRange {
        start: 54945,
        end: 54987,
        cid: 8099,
    },
    CidRange {
        start: 54988,
        end: 54988,
        cid: 8788,
    },
    CidRange {
        start: 54989,
        end: 55038,
        cid: 8143,
    },
    CidRange {
        start: 55104,
        end: 55161,
        cid: 8193,
    },
    CidRange {
        start: 55162,
        end: 55162,
        cid: 8889,
    },
    CidRange {
        start: 55163,
        end: 55166,
        cid: 8251,
    },
    CidRange {
        start: 55201,
        end: 55294,
        cid: 8255,
    },
    CidRange {
        start: 55360,
        end: 55422,
        cid: 8349,
    },
    CidRange {
        start: 55457,
        end: 55550,
        cid: 8412,
    },
    CidRange {
        start: 55616,
        end: 55678,
        cid: 8506,
    },
    CidRange {
        start: 55713,
        end: 55806,
        cid: 8569,
    },
    CidRange {
        start: 55872,
        end: 55934,
        cid: 8663,
    },
    CidRange {
        start: 55969,
        end: 56030,
        cid: 8726,
    },
    CidRange {
        start: 56031,
        end: 56031,
        cid: 8142,
    },
    CidRange {
        start: 56032,
        end: 56062,
        cid: 8789,
    },
    CidRange {
        start: 56128,
        end: 56190,
        cid: 8820,
    },
    CidRange {
        start: 56225,
        end: 56230,
        cid: 8883,
    },
    CidRange {
        start: 56231,
        end: 56318,
        cid: 8890,
    },
    CidRange {
        start: 56384,
        end: 56446,
        cid: 8978,
    },
    CidRange {
        start: 56481,
        end: 56574,
        cid: 9041,
    },
    CidRange {
        start: 56640,
        end: 56702,
        cid: 9135,
    },
    CidRange {
        start: 56737,
        end: 56827,
        cid: 9198,
    },
    CidRange {
        start: 56828,
        end: 56828,
        cid: 9089,
    },
    CidRange {
        start: 56829,
        end: 56830,
        cid: 9289,
    },
    CidRange {
        start: 56896,
        end: 56958,
        cid: 9291,
    },
    CidRange {
        start: 56993,
        end: 57086,
        cid: 9354,
    },
    CidRange {
        start: 57152,
        end: 57214,
        cid: 9448,
    },
    CidRange {
        start: 57249,
        end: 57342,
        cid: 9511,
    },
    CidRange {
        start: 57408,
        end: 57470,
        cid: 9605,
    },
    CidRange {
        start: 57505,
        end: 57598,
        cid: 9668,
    },
    CidRange {
        start: 57664,
        end: 57726,
        cid: 9762,
    },
    CidRange {
        start: 57761,
        end: 57854,
        cid: 9825,
    },
    CidRange {
        start: 57920,
        end: 57982,
        cid: 9919,
    },
    CidRange {
        start: 58017,
        end: 58110,
        cid: 9982,
    },
    CidRange {
        start: 58176,
        end: 58238,
        cid: 10076,
    },
    CidRange {
        start: 58273,
        end: 58366,
        cid: 10139,
    },
    CidRange {
        start: 58432,
        end: 58494,
        cid: 10233,
    },
    CidRange {
        start: 58529,
        end: 58622,
        cid: 10296,
    },
    CidRange {
        start: 58688,
        end: 58750,
        cid: 10390,
    },
    CidRange {
        start: 58785,
        end: 58878,
        cid: 10453,
    },
    CidRange {
        start: 58944,
        end: 59006,
        cid: 10547,
    },
    CidRange {
        start: 59041,
        end: 59134,
        cid: 10610,
    },
    CidRange {
        start: 59200,
        end: 59262,
        cid: 10704,
    },
    CidRange {
        start: 59297,
        end: 59390,
        cid: 10767,
    },
    CidRange {
        start: 59456,
        end: 59518,
        cid: 10861,
    },
    CidRange {
        start: 59553,
        end: 59554,
        cid: 10924,
    },
    CidRange {
        start: 59555,
        end: 59646,
        cid: 10927,
    },
    CidRange {
        start: 59712,
        end: 59765,
        cid: 11019,
    },
    CidRange {
        start: 59766,
        end: 59774,
        cid: 11074,
    },
    CidRange {
        start: 59809,
        end: 59902,
        cid: 11083,
    },
    CidRange {
        start: 59968,
        end: 60030,
        cid: 11177,
    },
    CidRange {
        start: 60065,
        end: 60158,
        cid: 11240,
    },
    CidRange {
        start: 60224,
        end: 60250,
        cid: 11334,
    },
    CidRange {
        start: 60251,
        end: 60286,
        cid: 11362,
    },
    CidRange {
        start: 60321,
        end: 60400,
        cid: 11398,
    },
    CidRange {
        start: 60401,
        end: 60401,
        cid: 10926,
    },
    CidRange {
        start: 60402,
        end: 60414,
        cid: 11478,
    },
    CidRange {
        start: 60480,
        end: 60542,
        cid: 11491,
    },
    CidRange {
        start: 60577,
        end: 60637,
        cid: 11554,
    },
    CidRange {
        start: 60638,
        end: 60638,
        cid: 11073,
    },
    CidRange {
        start: 60639,
        end: 60670,
        cid: 11615,
    },
    CidRange {
        start: 60736,
        end: 60798,
        cid: 11647,
    },
    CidRange {
        start: 60833,
        end: 60841,
        cid: 11710,
    },
    CidRange {
        start: 60842,
        end: 60926,
        cid: 11720,
    },
    CidRange {
        start: 60992,
        end: 61054,
        cid: 11805,
    },
    CidRange {
        start: 61089,
        end: 61162,
        cid: 11868,
    },
    CidRange {
        start: 61163,
        end: 61163,
        cid: 12308,
    },
    CidRange {
        start: 61164,
        end: 61182,
        cid: 11942,
    },
    CidRange {
        start: 61248,
        end: 61310,
        cid: 11961,
    },
    CidRange {
        start: 61345,
        end: 61438,
        cid: 12024,
    },
    CidRange {
        start: 61504,
        end: 61525,
        cid: 12118,
    },
    CidRange {
        start: 61526,
        end: 61526,
        cid: 11719,
    },
    CidRange {
        start: 61527,
        end: 61566,
        cid: 12140,
    },
    CidRange {
        start: 61601,
        end: 61642,
        cid: 12180,
    },
    CidRange {
        start: 61643,
        end: 61643,
        cid: 11361,
    },
    CidRange {
        start: 61644,
        end: 61694,
        cid: 12222,
    },
    CidRange {
        start: 61760,
        end: 61794,
        cid: 12273,
    },
    CidRange {
        start: 61795,
        end: 61802,
        cid: 12309,
    },
    CidRange {
        start: 61803,
        end: 61803,
        cid: 12640,
    },
    CidRange {
        start: 61804,
        end: 61822,
        cid: 12317,
    },
    CidRange {
        start: 61857,
        end: 61950,
        cid: 12336,
    },
    CidRange {
        start: 62016,
        end: 62055,
        cid: 12430,
    },
    CidRange {
        start: 62056,
        end: 62056,
        cid: 12783,
    },
    CidRange {
        start: 62057,
        end: 62078,
        cid: 12470,
    },
    CidRange {
        start: 62113,
        end: 62146,
        cid: 12492,
    },
    CidRange {
        start: 62147,
        end: 62206,
        cid: 12527,
    },
    CidRange {
        start: 62272,
        end: 62324,
        cid: 12587,
    },
    CidRange {
        start: 62325,
        end: 62334,
        cid: 12641,
    },
    CidRange {
        start: 62369,
        end: 62462,
        cid: 12651,
    },
    CidRange {
        start: 62528,
        end: 62565,
        cid: 12745,
    },
    CidRange {
        start: 62566,
        end: 62590,
        cid: 12784,
    },
    CidRange {
        start: 62625,
        end: 62644,
        cid: 12809,
    },
    CidRange {
        start: 62645,
        end: 62645,
        cid: 12526,
    },
    CidRange {
        start: 62646,
        end: 62716,
        cid: 12829,
    },
    CidRange {
        start: 62717,
        end: 62718,
        cid: 12901,
    },
    CidRange {
        start: 62784,
        end: 62846,
        cid: 12903,
    },
    CidRange {
        start: 62881,
        end: 62974,
        cid: 12966,
    },
    CidRange {
        start: 63040,
        end: 63074,
        cid: 13060,
    },
    CidRange {
        start: 63075,
        end: 63075,
        cid: 12900,
    },
    CidRange {
        start: 63076,
        end: 63102,
        cid: 13095,
    },
    CidRange {
        start: 63137,
        end: 63230,
        cid: 13122,
    },
    CidRange {
        start: 63296,
        end: 63358,
        cid: 13216,
    },
    CidRange {
        start: 63393,
        end: 63486,
        cid: 13279,
    },
    CidRange {
        start: 63552,
        end: 63614,
        cid: 13373,
    },
    CidRange {
        start: 63649,
        end: 63742,
        cid: 13436,
    },
    CidRange {
        start: 63808,
        end: 63862,
        cid: 13530,
    },
    CidRange {
        start: 63863,
        end: 63870,
        cid: 13586,
    },
    CidRange {
        start: 63905,
        end: 63939,
        cid: 13594,
    },
    CidRange {
        start: 63940,
        end: 63940,
        cid: 13585,
    },
    CidRange {
        start: 63941,
        end: 63941,
        cid: 13629,
    },
    CidRange {
        start: 63942,
        end: 63942,
        cid: 13641,
    },
    CidRange {
        start: 63943,
        end: 63953,
        cid: 13630,
    },
    CidRange {
        start: 63954,
        end: 63957,
        cid: 13642,
    },
    CidRange {
        start: 253,
        end: 255,
        cid: 96,
    },
];

const CID_RANGE_V: [CidRange; 12] = [
    CidRange {
        start: 41291,
        end: 41291,
        cid: 13646,
    },
    CidRange {
        start: 41306,
        end: 41306,
        cid: 13743,
    },
    CidRange {
        start: 41308,
        end: 41308,
        cid: 13745,
    },
    CidRange {
        start: 41309,
        end: 41310,
        cid: 130,
    },
    CidRange {
        start: 41313,
        end: 41314,
        cid: 134,
    },
    CidRange {
        start: 41317,
        end: 41318,
        cid: 138,
    },
    CidRange {
        start: 41321,
        end: 41322,
        cid: 142,
    },
    CidRange {
        start: 41325,
        end: 41326,
        cid: 146,
    },
    CidRange {
        start: 41329,
        end: 41330,
        cid: 150,
    },
    CidRange {
        start: 41333,
        end: 41334,
        cid: 154,
    },
    CidRange {
        start: 41337,
        end: 41338,
        cid: 158,
    },
    CidRange {
        start: 41443,
        end: 41443,
        cid: 13647,
    },
];

pub const B5PC_H: CMap = CMap {
    name: b"B5pc-H",
    cid_system_info: CidSystemInfo {
        registry: b"Adobe",
        ordering: b"CNS1",
        supplement: 0,
    },
    writing_mode: CMapWritingMode::Horizontal,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: Cow::Borrowed(&NO_CID_CHARS),
    cid_range: Cow::Borrowed(&CID_RANGE_H),
};

pub const B5PC_V: CMap = CMap {
    name: b"B5pc-V",
    cid_system_info: CidSystemInfo {
        registry: b"Adobe",
        ordering: b"CNS1",
        supplement: 0,
    },
    writing_mode: CMapWritingMode::Vertical,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: Cow::Borrowed(&NO_CID_CHARS),
    cid_range: Cow::Borrowed(&CID_RANGE_V),
};
