use std::borrow::Cow;

use crate::cmap::{
    CMap, CMapWritingMode, CidRange, Codespace, CodespaceRange, ADOBE_REGISTRY, NO_BASE_FONT_CHARS,
    NO_CID_CHARS,
};
use crate::font::cid_font::CidSystemInfo;

use super::KOREA_1;

const CODE_SPACE: [CodespaceRange; 2] = [
    [0..=0, 0..=0, 0..=0, 0..=128],
    [0..=0, 0..=0, 161..=254, 161..=254],
];

const CID_RANGE_H: [CidRange; 467] = [
    CidRange {
        start: 32,
        end: 126,
        cid: 8094,
    },
    CidRange {
        start: 41377,
        end: 41470,
        cid: 101,
    },
    CidRange {
        start: 41633,
        end: 41701,
        cid: 195,
    },
    CidRange {
        start: 41889,
        end: 41982,
        cid: 264,
    },
    CidRange {
        start: 42145,
        end: 42195,
        cid: 358,
    },
    CidRange {
        start: 42197,
        end: 42238,
        cid: 409,
    },
    CidRange {
        start: 42401,
        end: 42410,
        cid: 451,
    },
    CidRange {
        start: 42416,
        end: 42425,
        cid: 461,
    },
    CidRange {
        start: 42433,
        end: 42456,
        cid: 471,
    },
    CidRange {
        start: 42465,
        end: 42488,
        cid: 495,
    },
    CidRange {
        start: 42657,
        end: 42724,
        cid: 519,
    },
    CidRange {
        start: 42913,
        end: 42991,
        cid: 587,
    },
    CidRange {
        start: 43169,
        end: 43172,
        cid: 666,
    },
    CidRange {
        start: 43174,
        end: 43174,
        cid: 670,
    },
    CidRange {
        start: 43176,
        end: 43183,
        cid: 671,
    },
    CidRange {
        start: 43185,
        end: 43262,
        cid: 679,
    },
    CidRange {
        start: 43425,
        end: 43518,
        cid: 757,
    },
    CidRange {
        start: 43681,
        end: 43763,
        cid: 851,
    },
    CidRange {
        start: 43937,
        end: 44022,
        cid: 934,
    },
    CidRange {
        start: 44193,
        end: 44225,
        cid: 1020,
    },
    CidRange {
        start: 44241,
        end: 44273,
        cid: 1053,
    },
    CidRange {
        start: 45217,
        end: 45310,
        cid: 1086,
    },
    CidRange {
        start: 45473,
        end: 45566,
        cid: 1180,
    },
    CidRange {
        start: 45729,
        end: 45822,
        cid: 1274,
    },
    CidRange {
        start: 45985,
        end: 46078,
        cid: 1368,
    },
    CidRange {
        start: 46241,
        end: 46334,
        cid: 1462,
    },
    CidRange {
        start: 46497,
        end: 46590,
        cid: 1556,
    },
    CidRange {
        start: 46753,
        end: 46846,
        cid: 1650,
    },
    CidRange {
        start: 47009,
        end: 47102,
        cid: 1744,
    },
    CidRange {
        start: 47265,
        end: 47358,
        cid: 1838,
    },
    CidRange {
        start: 47521,
        end: 47614,
        cid: 1932,
    },
    CidRange {
        start: 47777,
        end: 47870,
        cid: 2026,
    },
    CidRange {
        start: 48033,
        end: 48126,
        cid: 2120,
    },
    CidRange {
        start: 48289,
        end: 48382,
        cid: 2214,
    },
    CidRange {
        start: 48545,
        end: 48638,
        cid: 2308,
    },
    CidRange {
        start: 48801,
        end: 48894,
        cid: 2402,
    },
    CidRange {
        start: 49057,
        end: 49150,
        cid: 2496,
    },
    CidRange {
        start: 49313,
        end: 49406,
        cid: 2590,
    },
    CidRange {
        start: 49569,
        end: 49662,
        cid: 2684,
    },
    CidRange {
        start: 49825,
        end: 49918,
        cid: 2778,
    },
    CidRange {
        start: 50081,
        end: 50174,
        cid: 2872,
    },
    CidRange {
        start: 50337,
        end: 50430,
        cid: 2966,
    },
    CidRange {
        start: 50593,
        end: 50686,
        cid: 3060,
    },
    CidRange {
        start: 50849,
        end: 50942,
        cid: 3154,
    },
    CidRange {
        start: 51105,
        end: 51198,
        cid: 3248,
    },
    CidRange {
        start: 51361,
        end: 51454,
        cid: 3342,
    },
    CidRange {
        start: 51873,
        end: 51966,
        cid: 3436,
    },
    CidRange {
        start: 52129,
        end: 52175,
        cid: 3530,
    },
    CidRange {
        start: 52176,
        end: 52176,
        cid: 4116,
    },
    CidRange {
        start: 52177,
        end: 52181,
        cid: 3577,
    },
    CidRange {
        start: 52182,
        end: 52182,
        cid: 3678,
    },
    CidRange {
        start: 52183,
        end: 52198,
        cid: 3582,
    },
    CidRange {
        start: 52199,
        end: 52199,
        cid: 7053,
    },
    CidRange {
        start: 52200,
        end: 52222,
        cid: 3598,
    },
    CidRange {
        start: 52385,
        end: 52478,
        cid: 3621,
    },
    CidRange {
        start: 52641,
        end: 52686,
        cid: 3715,
    },
    CidRange {
        start: 52687,
        end: 52687,
        cid: 3460,
    },
    CidRange {
        start: 52688,
        end: 52711,
        cid: 3761,
    },
    CidRange {
        start: 52712,
        end: 52712,
        cid: 7900,
    },
    CidRange {
        start: 52713,
        end: 52734,
        cid: 3785,
    },
    CidRange {
        start: 52897,
        end: 52908,
        cid: 3807,
    },
    CidRange {
        start: 52909,
        end: 52909,
        cid: 3802,
    },
    CidRange {
        start: 52910,
        end: 52990,
        cid: 3819,
    },
    CidRange {
        start: 53153,
        end: 53242,
        cid: 3900,
    },
    CidRange {
        start: 53243,
        end: 53243,
        cid: 3902,
    },
    CidRange {
        start: 53244,
        end: 53246,
        cid: 3990,
    },
    CidRange {
        start: 53409,
        end: 53409,
        cid: 3993,
    },
    CidRange {
        start: 53410,
        end: 53410,
        cid: 3946,
    },
    CidRange {
        start: 53411,
        end: 53431,
        cid: 3994,
    },
    CidRange {
        start: 53432,
        end: 53432,
        cid: 3946,
    },
    CidRange {
        start: 53433,
        end: 53455,
        cid: 4015,
    },
    CidRange {
        start: 53456,
        end: 53456,
        cid: 3708,
    },
    CidRange {
        start: 53457,
        end: 53468,
        cid: 4038,
    },
    CidRange {
        start: 53469,
        end: 53469,
        cid: 4131,
    },
    CidRange {
        start: 53470,
        end: 53502,
        cid: 4050,
    },
    CidRange {
        start: 53665,
        end: 53715,
        cid: 4083,
    },
    CidRange {
        start: 53716,
        end: 53716,
        cid: 4374,
    },
    CidRange {
        start: 53717,
        end: 53717,
        cid: 4156,
    },
    CidRange {
        start: 53718,
        end: 53719,
        cid: 4134,
    },
    CidRange {
        start: 53720,
        end: 53720,
        cid: 4375,
    },
    CidRange {
        start: 53721,
        end: 53722,
        cid: 4136,
    },
    CidRange {
        start: 53723,
        end: 53728,
        cid: 4376,
    },
    CidRange {
        start: 53729,
        end: 53729,
        cid: 4138,
    },
    CidRange {
        start: 53730,
        end: 53730,
        cid: 5800,
    },
    CidRange {
        start: 53731,
        end: 53733,
        cid: 4382,
    },
    CidRange {
        start: 53734,
        end: 53734,
        cid: 4386,
    },
    CidRange {
        start: 53735,
        end: 53735,
        cid: 4139,
    },
    CidRange {
        start: 53736,
        end: 53739,
        cid: 4387,
    },
    CidRange {
        start: 53740,
        end: 53740,
        cid: 4140,
    },
    CidRange {
        start: 53741,
        end: 53741,
        cid: 4391,
    },
    CidRange {
        start: 53742,
        end: 53742,
        cid: 4141,
    },
    CidRange {
        start: 53743,
        end: 53744,
        cid: 4394,
    },
    CidRange {
        start: 53745,
        end: 53745,
        cid: 4142,
    },
    CidRange {
        start: 53746,
        end: 53746,
        cid: 4396,
    },
    CidRange {
        start: 53747,
        end: 53749,
        cid: 4143,
    },
    CidRange {
        start: 53750,
        end: 53750,
        cid: 4399,
    },
    CidRange {
        start: 53751,
        end: 53753,
        cid: 4146,
    },
    CidRange {
        start: 53754,
        end: 53754,
        cid: 4403,
    },
    CidRange {
        start: 53755,
        end: 53755,
        cid: 4149,
    },
    CidRange {
        start: 53756,
        end: 53757,
        cid: 4406,
    },
    CidRange {
        start: 53758,
        end: 53758,
        cid: 4409,
    },
    CidRange {
        start: 53921,
        end: 53921,
        cid: 4150,
    },
    CidRange {
        start: 53922,
        end: 53923,
        cid: 4410,
    },
    CidRange {
        start: 53924,
        end: 53926,
        cid: 4151,
    },
    CidRange {
        start: 53927,
        end: 53930,
        cid: 4412,
    },
    CidRange {
        start: 53931,
        end: 53931,
        cid: 4419,
    },
    CidRange {
        start: 53932,
        end: 53932,
        cid: 4154,
    },
    CidRange {
        start: 53933,
        end: 53933,
        cid: 4420,
    },
    CidRange {
        start: 53934,
        end: 53937,
        cid: 4155,
    },
    CidRange {
        start: 53938,
        end: 53938,
        cid: 4424,
    },
    CidRange {
        start: 53939,
        end: 53949,
        cid: 4159,
    },
    CidRange {
        start: 53950,
        end: 53950,
        cid: 4511,
    },
    CidRange {
        start: 53951,
        end: 53953,
        cid: 4170,
    },
    CidRange {
        start: 53954,
        end: 53955,
        cid: 4513,
    },
    CidRange {
        start: 53956,
        end: 53956,
        cid: 4517,
    },
    CidRange {
        start: 53957,
        end: 53957,
        cid: 4173,
    },
    CidRange {
        start: 53958,
        end: 53962,
        cid: 4518,
    },
    CidRange {
        start: 53963,
        end: 53963,
        cid: 4524,
    },
    CidRange {
        start: 53964,
        end: 53964,
        cid: 4174,
    },
    CidRange {
        start: 53965,
        end: 53966,
        cid: 4525,
    },
    CidRange {
        start: 53967,
        end: 53972,
        cid: 4528,
    },
    CidRange {
        start: 53973,
        end: 53975,
        cid: 4535,
    },
    CidRange {
        start: 53976,
        end: 53976,
        cid: 4175,
    },
    CidRange {
        start: 53977,
        end: 53978,
        cid: 4541,
    },
    CidRange {
        start: 53979,
        end: 53981,
        cid: 4176,
    },
    CidRange {
        start: 53982,
        end: 53983,
        cid: 4545,
    },
    CidRange {
        start: 53984,
        end: 53984,
        cid: 4179,
    },
    CidRange {
        start: 53985,
        end: 53985,
        cid: 4547,
    },
    CidRange {
        start: 53986,
        end: 53986,
        cid: 4550,
    },
    CidRange {
        start: 53987,
        end: 53987,
        cid: 4180,
    },
    CidRange {
        start: 53988,
        end: 53988,
        cid: 4564,
    },
    CidRange {
        start: 53989,
        end: 53992,
        cid: 4566,
    },
    CidRange {
        start: 53993,
        end: 53994,
        cid: 4571,
    },
    CidRange {
        start: 53995,
        end: 53995,
        cid: 4576,
    },
    CidRange {
        start: 53996,
        end: 53999,
        cid: 4181,
    },
    CidRange {
        start: 54000,
        end: 54003,
        cid: 4604,
    },
    CidRange {
        start: 54004,
        end: 54005,
        cid: 4609,
    },
    CidRange {
        start: 54006,
        end: 54006,
        cid: 4185,
    },
    CidRange {
        start: 54007,
        end: 54008,
        cid: 4611,
    },
    CidRange {
        start: 54009,
        end: 54014,
        cid: 4186,
    },
    CidRange {
        start: 54177,
        end: 54270,
        cid: 4192,
    },
    CidRange {
        start: 54433,
        end: 54501,
        cid: 4286,
    },
    CidRange {
        start: 54502,
        end: 54502,
        cid: 4318,
    },
    CidRange {
        start: 54503,
        end: 54523,
        cid: 4355,
    },
    CidRange {
        start: 54524,
        end: 54524,
        cid: 4136,
    },
    CidRange {
        start: 54525,
        end: 54526,
        cid: 4376,
    },
    CidRange {
        start: 54689,
        end: 54692,
        cid: 4378,
    },
    CidRange {
        start: 54693,
        end: 54693,
        cid: 5800,
    },
    CidRange {
        start: 54694,
        end: 54698,
        cid: 4382,
    },
    CidRange {
        start: 54699,
        end: 54699,
        cid: 4139,
    },
    CidRange {
        start: 54700,
        end: 54701,
        cid: 4387,
    },
    CidRange {
        start: 54702,
        end: 54702,
        cid: 4192,
    },
    CidRange {
        start: 54703,
        end: 54782,
        cid: 4389,
    },
    CidRange {
        start: 54945,
        end: 54967,
        cid: 4469,
    },
    CidRange {
        start: 54968,
        end: 54968,
        cid: 4167,
    },
    CidRange {
        start: 54969,
        end: 54988,
        cid: 4492,
    },
    CidRange {
        start: 54989,
        end: 54989,
        cid: 4172,
    },
    CidRange {
        start: 54990,
        end: 55038,
        cid: 4512,
    },
    CidRange {
        start: 55201,
        end: 55242,
        cid: 4561,
    },
    CidRange {
        start: 55243,
        end: 55243,
        cid: 5552,
    },
    CidRange {
        start: 55244,
        end: 55267,
        cid: 4603,
    },
    CidRange {
        start: 55268,
        end: 55268,
        cid: 6424,
    },
    CidRange {
        start: 55269,
        end: 55294,
        cid: 4627,
    },
    CidRange {
        start: 55457,
        end: 55550,
        cid: 4653,
    },
    CidRange {
        start: 55713,
        end: 55806,
        cid: 4747,
    },
    CidRange {
        start: 55969,
        end: 56062,
        cid: 4841,
    },
    CidRange {
        start: 56225,
        end: 56260,
        cid: 4935,
    },
    CidRange {
        start: 56261,
        end: 56261,
        cid: 5151,
    },
    CidRange {
        start: 56262,
        end: 56291,
        cid: 4971,
    },
    CidRange {
        start: 56292,
        end: 56292,
        cid: 4922,
    },
    CidRange {
        start: 56293,
        end: 56318,
        cid: 5001,
    },
    CidRange {
        start: 56481,
        end: 56484,
        cid: 5027,
    },
    CidRange {
        start: 56485,
        end: 56485,
        cid: 7518,
    },
    CidRange {
        start: 56486,
        end: 56574,
        cid: 5031,
    },
    CidRange {
        start: 56737,
        end: 56740,
        cid: 5120,
    },
    CidRange {
        start: 56741,
        end: 56741,
        cid: 5079,
    },
    CidRange {
        start: 56742,
        end: 56788,
        cid: 5124,
    },
    CidRange {
        start: 56789,
        end: 56789,
        cid: 5109,
    },
    CidRange {
        start: 56790,
        end: 56819,
        cid: 5171,
    },
    CidRange {
        start: 56820,
        end: 56820,
        cid: 7607,
    },
    CidRange {
        start: 56821,
        end: 56830,
        cid: 5201,
    },
    CidRange {
        start: 56993,
        end: 57083,
        cid: 5211,
    },
    CidRange {
        start: 57084,
        end: 57084,
        cid: 5584,
    },
    CidRange {
        start: 57085,
        end: 57085,
        cid: 5302,
    },
    CidRange {
        start: 57086,
        end: 57086,
        cid: 5367,
    },
    CidRange {
        start: 57249,
        end: 57266,
        cid: 5303,
    },
    CidRange {
        start: 57267,
        end: 57267,
        cid: 7083,
    },
    CidRange {
        start: 57268,
        end: 57312,
        cid: 5321,
    },
    CidRange {
        start: 57313,
        end: 57313,
        cid: 5362,
    },
    CidRange {
        start: 57314,
        end: 57319,
        cid: 5366,
    },
    CidRange {
        start: 57320,
        end: 57320,
        cid: 5485,
    },
    CidRange {
        start: 57321,
        end: 57342,
        cid: 5372,
    },
    CidRange {
        start: 57505,
        end: 57584,
        cid: 5394,
    },
    CidRange {
        start: 57585,
        end: 57585,
        cid: 6001,
    },
    CidRange {
        start: 57586,
        end: 57598,
        cid: 5474,
    },
    CidRange {
        start: 57761,
        end: 57772,
        cid: 5487,
    },
    CidRange {
        start: 57773,
        end: 57773,
        cid: 5460,
    },
    CidRange {
        start: 57774,
        end: 57836,
        cid: 5499,
    },
    CidRange {
        start: 57837,
        end: 57837,
        cid: 5317,
    },
    CidRange {
        start: 57838,
        end: 57854,
        cid: 5562,
    },
    CidRange {
        start: 58017,
        end: 58110,
        cid: 5579,
    },
    CidRange {
        start: 58273,
        end: 58356,
        cid: 5673,
    },
    CidRange {
        start: 58357,
        end: 58357,
        cid: 7009,
    },
    CidRange {
        start: 58358,
        end: 58366,
        cid: 5757,
    },
    CidRange {
        start: 58529,
        end: 58529,
        cid: 7370,
    },
    CidRange {
        start: 58530,
        end: 58536,
        cid: 5766,
    },
    CidRange {
        start: 58537,
        end: 58537,
        cid: 5678,
    },
    CidRange {
        start: 58538,
        end: 58622,
        cid: 5773,
    },
    CidRange {
        start: 58785,
        end: 58797,
        cid: 5858,
    },
    CidRange {
        start: 58798,
        end: 58798,
        cid: 5874,
    },
    CidRange {
        start: 58799,
        end: 58800,
        cid: 5871,
    },
    CidRange {
        start: 58801,
        end: 58802,
        cid: 4425,
    },
    CidRange {
        start: 58803,
        end: 58808,
        cid: 5873,
    },
    CidRange {
        start: 58809,
        end: 58809,
        cid: 4427,
    },
    CidRange {
        start: 58810,
        end: 58810,
        cid: 5879,
    },
    CidRange {
        start: 58811,
        end: 58812,
        cid: 4429,
    },
    CidRange {
        start: 58813,
        end: 58819,
        cid: 5880,
    },
    CidRange {
        start: 58820,
        end: 58820,
        cid: 4431,
    },
    CidRange {
        start: 58821,
        end: 58829,
        cid: 5887,
    },
    CidRange {
        start: 58830,
        end: 58830,
        cid: 4435,
    },
    CidRange {
        start: 58831,
        end: 58831,
        cid: 5896,
    },
    CidRange {
        start: 58832,
        end: 58832,
        cid: 4436,
    },
    CidRange {
        start: 58833,
        end: 58833,
        cid: 5897,
    },
    CidRange {
        start: 58834,
        end: 58834,
        cid: 4437,
    },
    CidRange {
        start: 58835,
        end: 58837,
        cid: 5898,
    },
    CidRange {
        start: 58838,
        end: 58838,
        cid: 4439,
    },
    CidRange {
        start: 58839,
        end: 58873,
        cid: 5901,
    },
    CidRange {
        start: 58874,
        end: 58875,
        cid: 4442,
    },
    CidRange {
        start: 58876,
        end: 58876,
        cid: 4159,
    },
    CidRange {
        start: 58877,
        end: 58877,
        cid: 5936,
    },
    CidRange {
        start: 58878,
        end: 58878,
        cid: 4444,
    },
    CidRange {
        start: 59041,
        end: 59041,
        cid: 4447,
    },
    CidRange {
        start: 59042,
        end: 59043,
        cid: 5937,
    },
    CidRange {
        start: 59044,
        end: 59044,
        cid: 4449,
    },
    CidRange {
        start: 59045,
        end: 59046,
        cid: 5939,
    },
    CidRange {
        start: 59047,
        end: 59047,
        cid: 4450,
    },
    CidRange {
        start: 59048,
        end: 59052,
        cid: 5941,
    },
    CidRange {
        start: 59053,
        end: 59053,
        cid: 4453,
    },
    CidRange {
        start: 59054,
        end: 59054,
        cid: 5946,
    },
    CidRange {
        start: 59055,
        end: 59057,
        cid: 4455,
    },
    CidRange {
        start: 59058,
        end: 59058,
        cid: 5947,
    },
    CidRange {
        start: 59059,
        end: 59059,
        cid: 4458,
    },
    CidRange {
        start: 59060,
        end: 59062,
        cid: 5948,
    },
    CidRange {
        start: 59063,
        end: 59064,
        cid: 4459,
    },
    CidRange {
        start: 59065,
        end: 59067,
        cid: 5951,
    },
    CidRange {
        start: 59068,
        end: 59068,
        cid: 4463,
    },
    CidRange {
        start: 59069,
        end: 59075,
        cid: 5954,
    },
    CidRange {
        start: 59076,
        end: 59076,
        cid: 4160,
    },
    CidRange {
        start: 59077,
        end: 59077,
        cid: 5961,
    },
    CidRange {
        start: 59078,
        end: 59079,
        cid: 4465,
    },
    CidRange {
        start: 59080,
        end: 59081,
        cid: 5962,
    },
    CidRange {
        start: 59082,
        end: 59082,
        cid: 4161,
    },
    CidRange {
        start: 59083,
        end: 59089,
        cid: 5964,
    },
    CidRange {
        start: 59090,
        end: 59090,
        cid: 4468,
    },
    CidRange {
        start: 59091,
        end: 59093,
        cid: 5971,
    },
    CidRange {
        start: 59094,
        end: 59094,
        cid: 4469,
    },
    CidRange {
        start: 59095,
        end: 59096,
        cid: 5974,
    },
    CidRange {
        start: 59097,
        end: 59097,
        cid: 4470,
    },
    CidRange {
        start: 59098,
        end: 59099,
        cid: 5976,
    },
    CidRange {
        start: 59100,
        end: 59100,
        cid: 4162,
    },
    CidRange {
        start: 59101,
        end: 59102,
        cid: 5978,
    },
    CidRange {
        start: 59103,
        end: 59103,
        cid: 4471,
    },
    CidRange {
        start: 59104,
        end: 59104,
        cid: 5980,
    },
    CidRange {
        start: 59105,
        end: 59105,
        cid: 4472,
    },
    CidRange {
        start: 59106,
        end: 59107,
        cid: 5981,
    },
    CidRange {
        start: 59108,
        end: 59108,
        cid: 4474,
    },
    CidRange {
        start: 59109,
        end: 59109,
        cid: 4473,
    },
    CidRange {
        start: 59110,
        end: 59110,
        cid: 4475,
    },
    CidRange {
        start: 59111,
        end: 59111,
        cid: 5983,
    },
    CidRange {
        start: 59112,
        end: 59112,
        cid: 4476,
    },
    CidRange {
        start: 59113,
        end: 59113,
        cid: 5984,
    },
    CidRange {
        start: 59114,
        end: 59115,
        cid: 4478,
    },
    CidRange {
        start: 59116,
        end: 59116,
        cid: 6447,
    },
    CidRange {
        start: 59117,
        end: 59118,
        cid: 5985,
    },
    CidRange {
        start: 59119,
        end: 59119,
        cid: 4481,
    },
    CidRange {
        start: 59120,
        end: 59120,
        cid: 5987,
    },
    CidRange {
        start: 59121,
        end: 59121,
        cid: 4482,
    },
    CidRange {
        start: 59122,
        end: 59122,
        cid: 5460,
    },
    CidRange {
        start: 59123,
        end: 59124,
        cid: 5988,
    },
    CidRange {
        start: 59125,
        end: 59125,
        cid: 4483,
    },
    CidRange {
        start: 59126,
        end: 59126,
        cid: 4163,
    },
    CidRange {
        start: 59127,
        end: 59127,
        cid: 4166,
    },
    CidRange {
        start: 59128,
        end: 59128,
        cid: 5990,
    },
    CidRange {
        start: 59129,
        end: 59129,
        cid: 4485,
    },
    CidRange {
        start: 59130,
        end: 59134,
        cid: 5991,
    },
    CidRange {
        start: 59297,
        end: 59297,
        cid: 4487,
    },
    CidRange {
        start: 59298,
        end: 59301,
        cid: 5996,
    },
    CidRange {
        start: 59302,
        end: 59302,
        cid: 4488,
    },
    CidRange {
        start: 59303,
        end: 59304,
        cid: 6000,
    },
    CidRange {
        start: 59305,
        end: 59305,
        cid: 4489,
    },
    CidRange {
        start: 59306,
        end: 59306,
        cid: 4491,
    },
    CidRange {
        start: 59307,
        end: 59307,
        cid: 6002,
    },
    CidRange {
        start: 59308,
        end: 59308,
        cid: 4167,
    },
    CidRange {
        start: 59309,
        end: 59309,
        cid: 4493,
    },
    CidRange {
        start: 59310,
        end: 59311,
        cid: 6003,
    },
    CidRange {
        start: 59312,
        end: 59312,
        cid: 4494,
    },
    CidRange {
        start: 59313,
        end: 59326,
        cid: 6005,
    },
    CidRange {
        start: 59327,
        end: 59327,
        cid: 4495,
    },
    CidRange {
        start: 59328,
        end: 59328,
        cid: 6019,
    },
    CidRange {
        start: 59329,
        end: 59329,
        cid: 7783,
    },
    CidRange {
        start: 59330,
        end: 59333,
        cid: 6020,
    },
    CidRange {
        start: 59334,
        end: 59334,
        cid: 4497,
    },
    CidRange {
        start: 59335,
        end: 59335,
        cid: 4499,
    },
    CidRange {
        start: 59336,
        end: 59338,
        cid: 6024,
    },
    CidRange {
        start: 59339,
        end: 59339,
        cid: 4501,
    },
    CidRange {
        start: 59340,
        end: 59340,
        cid: 6027,
    },
    CidRange {
        start: 59341,
        end: 59341,
        cid: 4502,
    },
    CidRange {
        start: 59342,
        end: 59342,
        cid: 6028,
    },
    CidRange {
        start: 59343,
        end: 59344,
        cid: 4503,
    },
    CidRange {
        start: 59345,
        end: 59346,
        cid: 6029,
    },
    CidRange {
        start: 59347,
        end: 59347,
        cid: 4506,
    },
    CidRange {
        start: 59348,
        end: 59358,
        cid: 6031,
    },
    CidRange {
        start: 59359,
        end: 59359,
        cid: 4508,
    },
    CidRange {
        start: 59360,
        end: 59363,
        cid: 6042,
    },
    CidRange {
        start: 59364,
        end: 59364,
        cid: 4509,
    },
    CidRange {
        start: 59365,
        end: 59365,
        cid: 6046,
    },
    CidRange {
        start: 59366,
        end: 59366,
        cid: 4510,
    },
    CidRange {
        start: 59367,
        end: 59382,
        cid: 6047,
    },
    CidRange {
        start: 59383,
        end: 59383,
        cid: 5797,
    },
    CidRange {
        start: 59384,
        end: 59390,
        cid: 6063,
    },
    CidRange {
        start: 59553,
        end: 59622,
        cid: 6070,
    },
    CidRange {
        start: 59623,
        end: 59624,
        cid: 4551,
    },
    CidRange {
        start: 59625,
        end: 59631,
        cid: 6140,
    },
    CidRange {
        start: 59632,
        end: 59632,
        cid: 4553,
    },
    CidRange {
        start: 59633,
        end: 59633,
        cid: 4180,
    },
    CidRange {
        start: 59634,
        end: 59638,
        cid: 6147,
    },
    CidRange {
        start: 59639,
        end: 59639,
        cid: 4555,
    },
    CidRange {
        start: 59640,
        end: 59640,
        cid: 6152,
    },
    CidRange {
        start: 59641,
        end: 59641,
        cid: 5800,
    },
    CidRange {
        start: 59642,
        end: 59642,
        cid: 6153,
    },
    CidRange {
        start: 59643,
        end: 59643,
        cid: 4556,
    },
    CidRange {
        start: 59644,
        end: 59645,
        cid: 6154,
    },
    CidRange {
        start: 59646,
        end: 59646,
        cid: 4557,
    },
    CidRange {
        start: 59809,
        end: 59814,
        cid: 6156,
    },
    CidRange {
        start: 59815,
        end: 59815,
        cid: 4560,
    },
    CidRange {
        start: 59816,
        end: 59819,
        cid: 6162,
    },
    CidRange {
        start: 59820,
        end: 59820,
        cid: 4561,
    },
    CidRange {
        start: 59821,
        end: 59851,
        cid: 6166,
    },
    CidRange {
        start: 59852,
        end: 59852,
        cid: 4563,
    },
    CidRange {
        start: 59853,
        end: 59894,
        cid: 6197,
    },
    CidRange {
        start: 59895,
        end: 59895,
        cid: 7988,
    },
    CidRange {
        start: 59896,
        end: 59902,
        cid: 6239,
    },
    CidRange {
        start: 60065,
        end: 60096,
        cid: 6246,
    },
    CidRange {
        start: 60097,
        end: 60097,
        cid: 6123,
    },
    CidRange {
        start: 60098,
        end: 60132,
        cid: 6278,
    },
    CidRange {
        start: 60133,
        end: 60133,
        cid: 4577,
    },
    CidRange {
        start: 60134,
        end: 60147,
        cid: 6313,
    },
    CidRange {
        start: 60148,
        end: 60148,
        cid: 4183,
    },
    CidRange {
        start: 60149,
        end: 60150,
        cid: 6327,
    },
    CidRange {
        start: 60151,
        end: 60151,
        cid: 4579,
    },
    CidRange {
        start: 60152,
        end: 60155,
        cid: 6329,
    },
    CidRange {
        start: 60156,
        end: 60156,
        cid: 4581,
    },
    CidRange {
        start: 60157,
        end: 60157,
        cid: 6333,
    },
    CidRange {
        start: 60158,
        end: 60158,
        cid: 4582,
    },
    CidRange {
        start: 60321,
        end: 60323,
        cid: 6334,
    },
    CidRange {
        start: 60324,
        end: 60324,
        cid: 4584,
    },
    CidRange {
        start: 60325,
        end: 60326,
        cid: 6337,
    },
    CidRange {
        start: 60327,
        end: 60327,
        cid: 4586,
    },
    CidRange {
        start: 60328,
        end: 60328,
        cid: 6339,
    },
    CidRange {
        start: 60329,
        end: 60329,
        cid: 4588,
    },
    CidRange {
        start: 60330,
        end: 60330,
        cid: 4184,
    },
    CidRange {
        start: 60331,
        end: 60345,
        cid: 6340,
    },
    CidRange {
        start: 60346,
        end: 60347,
        cid: 4590,
    },
    CidRange {
        start: 60348,
        end: 60348,
        cid: 6355,
    },
    CidRange {
        start: 60349,
        end: 60349,
        cid: 4592,
    },
    CidRange {
        start: 60350,
        end: 60352,
        cid: 6356,
    },
    CidRange {
        start: 60353,
        end: 60353,
        cid: 4593,
    },
    CidRange {
        start: 60354,
        end: 60354,
        cid: 4595,
    },
    CidRange {
        start: 60355,
        end: 60357,
        cid: 6359,
    },
    CidRange {
        start: 60358,
        end: 60359,
        cid: 4596,
    },
    CidRange {
        start: 60360,
        end: 60363,
        cid: 6362,
    },
    CidRange {
        start: 60364,
        end: 60364,
        cid: 4599,
    },
    CidRange {
        start: 60365,
        end: 60366,
        cid: 6366,
    },
    CidRange {
        start: 60367,
        end: 60369,
        cid: 4600,
    },
    CidRange {
        start: 60370,
        end: 60370,
        cid: 5552,
    },
    CidRange {
        start: 60371,
        end: 60375,
        cid: 6368,
    },
    CidRange {
        start: 60376,
        end: 60376,
        cid: 4603,
    },
    CidRange {
        start: 60377,
        end: 60414,
        cid: 6373,
    },
    CidRange {
        start: 60577,
        end: 60581,
        cid: 6411,
    },
    CidRange {
        start: 60582,
        end: 60582,
        cid: 4614,
    },
    CidRange {
        start: 60583,
        end: 60583,
        cid: 4616,
    },
    CidRange {
        start: 60584,
        end: 60585,
        cid: 6416,
    },
    CidRange {
        start: 60586,
        end: 60586,
        cid: 4618,
    },
    CidRange {
        start: 60587,
        end: 60590,
        cid: 6418,
    },
    CidRange {
        start: 60591,
        end: 60591,
        cid: 5950,
    },
    CidRange {
        start: 60592,
        end: 60593,
        cid: 4620,
    },
    CidRange {
        start: 60594,
        end: 60594,
        cid: 4187,
    },
    CidRange {
        start: 60595,
        end: 60596,
        cid: 6422,
    },
    CidRange {
        start: 60597,
        end: 60597,
        cid: 4625,
    },
    CidRange {
        start: 60598,
        end: 60599,
        cid: 6424,
    },
    CidRange {
        start: 60600,
        end: 60600,
        cid: 4627,
    },
    CidRange {
        start: 60601,
        end: 60601,
        cid: 6426,
    },
    CidRange {
        start: 60602,
        end: 60602,
        cid: 4629,
    },
    CidRange {
        start: 60603,
        end: 60607,
        cid: 6427,
    },
    CidRange {
        start: 60608,
        end: 60609,
        cid: 4632,
    },
    CidRange {
        start: 60610,
        end: 60612,
        cid: 6432,
    },
    CidRange {
        start: 60613,
        end: 60613,
        cid: 4634,
    },
    CidRange {
        start: 60614,
        end: 60614,
        cid: 4636,
    },
    CidRange {
        start: 60615,
        end: 60616,
        cid: 6435,
    },
    CidRange {
        start: 60617,
        end: 60618,
        cid: 4188,
    },
    CidRange {
        start: 60619,
        end: 60628,
        cid: 6437,
    },
    CidRange {
        start: 60629,
        end: 60629,
        cid: 4638,
    },
    CidRange {
        start: 60630,
        end: 60636,
        cid: 6447,
    },
    CidRange {
        start: 60637,
        end: 60638,
        cid: 4640,
    },
    CidRange {
        start: 60639,
        end: 60640,
        cid: 6454,
    },
    CidRange {
        start: 60641,
        end: 60641,
        cid: 4642,
    },
    CidRange {
        start: 60642,
        end: 60643,
        cid: 6456,
    },
    CidRange {
        start: 60644,
        end: 60644,
        cid: 4644,
    },
    CidRange {
        start: 60645,
        end: 60646,
        cid: 6458,
    },
    CidRange {
        start: 60647,
        end: 60648,
        cid: 4645,
    },
    CidRange {
        start: 60649,
        end: 60662,
        cid: 6460,
    },
    CidRange {
        start: 60663,
        end: 60664,
        cid: 4647,
    },
    CidRange {
        start: 60665,
        end: 60665,
        cid: 6474,
    },
    CidRange {
        start: 60666,
        end: 60666,
        cid: 4650,
    },
    CidRange {
        start: 60667,
        end: 60670,
        cid: 6475,
    },
    CidRange {
        start: 60833,
        end: 60835,
        cid: 4653,
    },
    CidRange {
        start: 60836,
        end: 60909,
        cid: 6479,
    },
    CidRange {
        start: 60910,
        end: 60910,
        cid: 5351,
    },
    CidRange {
        start: 60911,
        end: 60926,
        cid: 6553,
    },
    CidRange {
        start: 61089,
        end: 61146,
        cid: 6569,
    },
    CidRange {
        start: 61147,
        end: 61147,
        cid: 6494,
    },
    CidRange {
        start: 61148,
        end: 61182,
        cid: 6627,
    },
    CidRange {
        start: 61345,
        end: 61438,
        cid: 6662,
    },
    CidRange {
        start: 61601,
        end: 61694,
        cid: 6756,
    },
    CidRange {
        start: 61857,
        end: 61950,
        cid: 6850,
    },
    CidRange {
        start: 62113,
        end: 62140,
        cid: 6944,
    },
    CidRange {
        start: 62141,
        end: 62141,
        cid: 5731,
    },
    CidRange {
        start: 62142,
        end: 62201,
        cid: 6972,
    },
    CidRange {
        start: 62202,
        end: 62202,
        cid: 5771,
    },
    CidRange {
        start: 62203,
        end: 62206,
        cid: 7032,
    },
    CidRange {
        start: 62369,
        end: 62384,
        cid: 7036,
    },
    CidRange {
        start: 62385,
        end: 62385,
        cid: 4191,
    },
    CidRange {
        start: 62386,
        end: 62462,
        cid: 7052,
    },
    CidRange {
        start: 62625,
        end: 62630,
        cid: 7129,
    },
    CidRange {
        start: 62631,
        end: 62631,
        cid: 6484,
    },
    CidRange {
        start: 62632,
        end: 62701,
        cid: 7135,
    },
    CidRange {
        start: 62702,
        end: 62702,
        cid: 6684,
    },
    CidRange {
        start: 62703,
        end: 62718,
        cid: 7205,
    },
    CidRange {
        start: 62881,
        end: 62974,
        cid: 7221,
    },
    CidRange {
        start: 63137,
        end: 63219,
        cid: 7315,
    },
    CidRange {
        start: 63220,
        end: 63220,
        cid: 4279,
    },
    CidRange {
        start: 63221,
        end: 63221,
        cid: 7398,
    },
    CidRange {
        start: 63222,
        end: 63222,
        cid: 7139,
    },
    CidRange {
        start: 63223,
        end: 63230,
        cid: 7399,
    },
    CidRange {
        start: 63393,
        end: 63415,
        cid: 7407,
    },
    CidRange {
        start: 63416,
        end: 63416,
        cid: 4247,
    },
    CidRange {
        start: 63417,
        end: 63431,
        cid: 7430,
    },
    CidRange {
        start: 63432,
        end: 63432,
        cid: 4266,
    },
    CidRange {
        start: 63433,
        end: 63442,
        cid: 7445,
    },
    CidRange {
        start: 63443,
        end: 63443,
        cid: 4340,
    },
    CidRange {
        start: 63444,
        end: 63486,
        cid: 7455,
    },
    CidRange {
        start: 63649,
        end: 63706,
        cid: 7498,
    },
    CidRange {
        start: 63707,
        end: 63707,
        cid: 7572,
    },
    CidRange {
        start: 63708,
        end: 63727,
        cid: 7556,
    },
    CidRange {
        start: 63728,
        end: 63728,
        cid: 5088,
    },
    CidRange {
        start: 63729,
        end: 63742,
        cid: 7576,
    },
    CidRange {
        start: 63905,
        end: 63998,
        cid: 7590,
    },
    CidRange {
        start: 64161,
        end: 64161,
        cid: 7709,
    },
    CidRange {
        start: 64162,
        end: 64162,
        cid: 3558,
    },
    CidRange {
        start: 64163,
        end: 64229,
        cid: 7684,
    },
    CidRange {
        start: 64230,
        end: 64230,
        cid: 3644,
    },
    CidRange {
        start: 64231,
        end: 64254,
        cid: 7751,
    },
    CidRange {
        start: 64417,
        end: 64510,
        cid: 7775,
    },
    CidRange {
        start: 64673,
        end: 64680,
        cid: 7869,
    },
    CidRange {
        start: 64681,
        end: 64681,
        cid: 3815,
    },
    CidRange {
        start: 64682,
        end: 64766,
        cid: 7877,
    },
    CidRange {
        start: 64929,
        end: 65022,
        cid: 7962,
    },
];

const CID_RANGE_V: [CidRange; 16] = [
    CidRange {
        start: 41378,
        end: 41379,
        cid: 8056,
    },
    CidRange {
        start: 41381,
        end: 41381,
        cid: 8058,
    },
    CidRange {
        start: 41382,
        end: 41382,
        cid: 8320,
    },
    CidRange {
        start: 41385,
        end: 41387,
        cid: 8059,
    },
    CidRange {
        start: 41389,
        end: 41389,
        cid: 8062,
    },
    CidRange {
        start: 41394,
        end: 41405,
        cid: 8063,
    },
    CidRange {
        start: 41451,
        end: 41451,
        cid: 8075,
    },
    CidRange {
        start: 41889,
        end: 41889,
        cid: 8076,
    },
    CidRange {
        start: 41896,
        end: 41897,
        cid: 8077,
    },
    CidRange {
        start: 41900,
        end: 41900,
        cid: 8079,
    },
    CidRange {
        start: 41902,
        end: 41902,
        cid: 8080,
    },
    CidRange {
        start: 41914,
        end: 41919,
        cid: 8081,
    },
    CidRange {
        start: 41947,
        end: 41947,
        cid: 8087,
    },
    CidRange {
        start: 41949,
        end: 41949,
        cid: 8088,
    },
    CidRange {
        start: 41951,
        end: 41951,
        cid: 8089,
    },
    CidRange {
        start: 41979,
        end: 41982,
        cid: 8090,
    },
];

pub const KSC_EUC_H: CMap = CMap {
    name: Cow::Borrowed(b"KSC-EUC-H"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(KOREA_1),
        supplement: 0,
    },
    writing_mode: CMapWritingMode::Horizontal,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: NO_CID_CHARS,
    cid_range: Cow::Borrowed(&CID_RANGE_H),
    base_font_chars: NO_BASE_FONT_CHARS,
};

pub const KSC_EUC_V: CMap = CMap {
    name: Cow::Borrowed(b"KSC-EUC-V"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(KOREA_1),
        supplement: 0,
    },
    writing_mode: CMapWritingMode::Vertical,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: NO_CID_CHARS,
    cid_range: Cow::Borrowed(&CID_RANGE_V),
    base_font_chars: NO_BASE_FONT_CHARS,
};
