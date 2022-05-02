use std::borrow::Cow;

use crate::cmaps::cmap::{
    CMap, CMapWritingMode, CidRange, Codespace, CodespaceRange, NO_CID_CHARS, ADOBE_REGISTRY,
};
use crate::font::font::CidSystemInfo;

use super::JAPAN_1;

const CODE_SPACE: [CodespaceRange; 4] = [
    [0..=0, 0..=0, 0..=0, 0..=128],
    [0..=0, 0..=0, 129..=159, 64..=252],
    [0..=0, 0..=0, 0..=0, 160..=223],
    [0..=0, 0..=0, 224..=252, 64..=252],
];

const CID_RANGE_H: [CidRange; 665] = [
    CidRange {
        start: 32,
        end: 126,
        cid: 231,
    },
    CidRange {
        start: 33088,
        end: 33150,
        cid: 633,
    },
    CidRange {
        start: 33152,
        end: 33160,
        cid: 696,
    },
    CidRange {
        start: 33161,
        end: 33161,
        cid: 7478,
    },
    CidRange {
        start: 33162,
        end: 33196,
        cid: 706,
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
        start: 34686,
        end: 34686,
        cid: 8323,
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
        end: 34975,
        cid: 1125,
    },
    CidRange {
        start: 34976,
        end: 34976,
        cid: 7633,
    },
    CidRange {
        start: 34977,
        end: 34992,
        cid: 1127,
    },
    CidRange {
        start: 34993,
        end: 34993,
        cid: 7330,
    },
    CidRange {
        start: 34994,
        end: 35000,
        cid: 1144,
    },
    CidRange {
        start: 35001,
        end: 35001,
        cid: 7634,
    },
    CidRange {
        start: 35002,
        end: 35051,
        cid: 1152,
    },
    CidRange {
        start: 35052,
        end: 35052,
        cid: 7635,
    },
    CidRange {
        start: 35053,
        end: 35056,
        cid: 1203,
    },
    CidRange {
        start: 35057,
        end: 35057,
        cid: 7636,
    },
    CidRange {
        start: 35058,
        end: 35065,
        cid: 1208,
    },
    CidRange {
        start: 35066,
        end: 35066,
        cid: 7637,
    },
    CidRange {
        start: 35067,
        end: 35068,
        cid: 1217,
    },
    CidRange {
        start: 35136,
        end: 35144,
        cid: 1219,
    },
    CidRange {
        start: 35145,
        end: 35145,
        cid: 7638,
    },
    CidRange {
        start: 35146,
        end: 35155,
        cid: 1229,
    },
    CidRange {
        start: 35156,
        end: 35156,
        cid: 7639,
    },
    CidRange {
        start: 35157,
        end: 35159,
        cid: 1240,
    },
    CidRange {
        start: 35160,
        end: 35160,
        cid: 7640,
    },
    CidRange {
        start: 35161,
        end: 35162,
        cid: 1244,
    },
    CidRange {
        start: 35163,
        end: 35164,
        cid: 7641,
    },
    CidRange {
        start: 35165,
        end: 35168,
        cid: 1248,
    },
    CidRange {
        start: 35169,
        end: 35169,
        cid: 7643,
    },
    CidRange {
        start: 35170,
        end: 35198,
        cid: 1253,
    },
    CidRange {
        start: 35200,
        end: 35210,
        cid: 1282,
    },
    CidRange {
        start: 35211,
        end: 35211,
        cid: 7644,
    },
    CidRange {
        start: 35212,
        end: 35237,
        cid: 1294,
    },
    CidRange {
        start: 35238,
        end: 35238,
        cid: 7645,
    },
    CidRange {
        start: 35239,
        end: 35239,
        cid: 7369,
    },
    CidRange {
        start: 35240,
        end: 35240,
        cid: 7646,
    },
    CidRange {
        start: 35241,
        end: 35293,
        cid: 1323,
    },
    CidRange {
        start: 35294,
        end: 35294,
        cid: 7647,
    },
    CidRange {
        start: 35295,
        end: 35319,
        cid: 1377,
    },
    CidRange {
        start: 35320,
        end: 35320,
        cid: 7648,
    },
    CidRange {
        start: 35321,
        end: 35322,
        cid: 1403,
    },
    CidRange {
        start: 35323,
        end: 35323,
        cid: 7649,
    },
    CidRange {
        start: 35324,
        end: 35324,
        cid: 1406,
    },
    CidRange {
        start: 35392,
        end: 35392,
        cid: 1407,
    },
    CidRange {
        start: 35393,
        end: 35393,
        cid: 7650,
    },
    CidRange {
        start: 35394,
        end: 35424,
        cid: 1409,
    },
    CidRange {
        start: 35425,
        end: 35425,
        cid: 6497,
    },
    CidRange {
        start: 35426,
        end: 35431,
        cid: 1441,
    },
    CidRange {
        start: 35432,
        end: 35432,
        cid: 5023,
    },
    CidRange {
        start: 35433,
        end: 35454,
        cid: 1448,
    },
    CidRange {
        start: 35456,
        end: 35460,
        cid: 1470,
    },
    CidRange {
        start: 35461,
        end: 35461,
        cid: 7651,
    },
    CidRange {
        start: 35462,
        end: 35466,
        cid: 1476,
    },
    CidRange {
        start: 35467,
        end: 35467,
        cid: 7652,
    },
    CidRange {
        start: 35468,
        end: 35474,
        cid: 1482,
    },
    CidRange {
        start: 35475,
        end: 35475,
        cid: 7653,
    },
    CidRange {
        start: 35476,
        end: 35477,
        cid: 1490,
    },
    CidRange {
        start: 35478,
        end: 35478,
        cid: 5937,
    },
    CidRange {
        start: 35479,
        end: 35481,
        cid: 1493,
    },
    CidRange {
        start: 35482,
        end: 35482,
        cid: 7654,
    },
    CidRange {
        start: 35483,
        end: 35519,
        cid: 1497,
    },
    CidRange {
        start: 35520,
        end: 35520,
        cid: 7655,
    },
    CidRange {
        start: 35521,
        end: 35521,
        cid: 5490,
    },
    CidRange {
        start: 35522,
        end: 35530,
        cid: 1536,
    },
    CidRange {
        start: 35531,
        end: 35531,
        cid: 7656,
    },
    CidRange {
        start: 35532,
        end: 35535,
        cid: 1546,
    },
    CidRange {
        start: 35536,
        end: 35536,
        cid: 6688,
    },
    CidRange {
        start: 35537,
        end: 35554,
        cid: 1551,
    },
    CidRange {
        start: 35555,
        end: 35555,
        cid: 7657,
    },
    CidRange {
        start: 35556,
        end: 35580,
        cid: 1570,
    },
    CidRange {
        start: 35648,
        end: 35657,
        cid: 1595,
    },
    CidRange {
        start: 35658,
        end: 35658,
        cid: 7658,
    },
    CidRange {
        start: 35659,
        end: 35678,
        cid: 1606,
    },
    CidRange {
        start: 35679,
        end: 35679,
        cid: 7659,
    },
    CidRange {
        start: 35680,
        end: 35710,
        cid: 1627,
    },
    CidRange {
        start: 35712,
        end: 35743,
        cid: 1658,
    },
    CidRange {
        start: 35744,
        end: 35744,
        cid: 7660,
    },
    CidRange {
        start: 35745,
        end: 35751,
        cid: 1691,
    },
    CidRange {
        start: 35752,
        end: 35752,
        cid: 7661,
    },
    CidRange {
        start: 35753,
        end: 35779,
        cid: 1699,
    },
    CidRange {
        start: 35780,
        end: 35780,
        cid: 7474,
    },
    CidRange {
        start: 35781,
        end: 35788,
        cid: 1727,
    },
    CidRange {
        start: 35789,
        end: 35789,
        cid: 7662,
    },
    CidRange {
        start: 35790,
        end: 35818,
        cid: 1736,
    },
    CidRange {
        start: 35819,
        end: 35819,
        cid: 7663,
    },
    CidRange {
        start: 35820,
        end: 35825,
        cid: 1766,
    },
    CidRange {
        start: 35826,
        end: 35826,
        cid: 7664,
    },
    CidRange {
        start: 35827,
        end: 35832,
        cid: 1773,
    },
    CidRange {
        start: 35833,
        end: 35833,
        cid: 7665,
    },
    CidRange {
        start: 35834,
        end: 35834,
        cid: 1780,
    },
    CidRange {
        start: 35835,
        end: 35835,
        cid: 7666,
    },
    CidRange {
        start: 35836,
        end: 35836,
        cid: 1782,
    },
    CidRange {
        start: 35904,
        end: 35906,
        cid: 1783,
    },
    CidRange {
        start: 35907,
        end: 35907,
        cid: 7667,
    },
    CidRange {
        start: 35908,
        end: 35925,
        cid: 1787,
    },
    CidRange {
        start: 35926,
        end: 35926,
        cid: 7668,
    },
    CidRange {
        start: 35927,
        end: 35939,
        cid: 1806,
    },
    CidRange {
        start: 35940,
        end: 35940,
        cid: 7669,
    },
    CidRange {
        start: 35941,
        end: 35948,
        cid: 1820,
    },
    CidRange {
        start: 35949,
        end: 35949,
        cid: 7670,
    },
    CidRange {
        start: 35950,
        end: 35952,
        cid: 1829,
    },
    CidRange {
        start: 35953,
        end: 35953,
        cid: 7671,
    },
    CidRange {
        start: 35954,
        end: 35955,
        cid: 1833,
    },
    CidRange {
        start: 35956,
        end: 35956,
        cid: 7672,
    },
    CidRange {
        start: 35957,
        end: 35961,
        cid: 1836,
    },
    CidRange {
        start: 35962,
        end: 35962,
        cid: 7181,
    },
    CidRange {
        start: 35963,
        end: 35966,
        cid: 1842,
    },
    CidRange {
        start: 35968,
        end: 35971,
        cid: 1846,
    },
    CidRange {
        start: 35972,
        end: 35972,
        cid: 7673,
    },
    CidRange {
        start: 35973,
        end: 35984,
        cid: 1851,
    },
    CidRange {
        start: 35985,
        end: 35985,
        cid: 7674,
    },
    CidRange {
        start: 35986,
        end: 35992,
        cid: 1864,
    },
    CidRange {
        start: 35993,
        end: 35993,
        cid: 7675,
    },
    CidRange {
        start: 35994,
        end: 35997,
        cid: 1872,
    },
    CidRange {
        start: 35998,
        end: 35998,
        cid: 7676,
    },
    CidRange {
        start: 35999,
        end: 36017,
        cid: 1877,
    },
    CidRange {
        start: 36018,
        end: 36018,
        cid: 7677,
    },
    CidRange {
        start: 36019,
        end: 36030,
        cid: 1897,
    },
    CidRange {
        start: 36031,
        end: 36031,
        cid: 7678,
    },
    CidRange {
        start: 36032,
        end: 36092,
        cid: 1910,
    },
    CidRange {
        start: 36160,
        end: 36169,
        cid: 1971,
    },
    CidRange {
        start: 36170,
        end: 36170,
        cid: 7679,
    },
    CidRange {
        start: 36171,
        end: 36181,
        cid: 1982,
    },
    CidRange {
        start: 36182,
        end: 36182,
        cid: 7680,
    },
    CidRange {
        start: 36183,
        end: 36192,
        cid: 1994,
    },
    CidRange {
        start: 36193,
        end: 36193,
        cid: 7681,
    },
    CidRange {
        start: 36194,
        end: 36218,
        cid: 2005,
    },
    CidRange {
        start: 36219,
        end: 36219,
        cid: 5853,
    },
    CidRange {
        start: 36220,
        end: 36222,
        cid: 2031,
    },
    CidRange {
        start: 36224,
        end: 36236,
        cid: 2034,
    },
    CidRange {
        start: 36237,
        end: 36237,
        cid: 7682,
    },
    CidRange {
        start: 36238,
        end: 36243,
        cid: 2048,
    },
    CidRange {
        start: 36244,
        end: 36244,
        cid: 7683,
    },
    CidRange {
        start: 36245,
        end: 36248,
        cid: 2055,
    },
    CidRange {
        start: 36249,
        end: 36249,
        cid: 7684,
    },
    CidRange {
        start: 36250,
        end: 36304,
        cid: 2060,
    },
    CidRange {
        start: 36305,
        end: 36305,
        cid: 7685,
    },
    CidRange {
        start: 36306,
        end: 36324,
        cid: 2116,
    },
    CidRange {
        start: 36325,
        end: 36325,
        cid: 7686,
    },
    CidRange {
        start: 36326,
        end: 36337,
        cid: 2136,
    },
    CidRange {
        start: 36338,
        end: 36338,
        cid: 7687,
    },
    CidRange {
        start: 36339,
        end: 36348,
        cid: 2149,
    },
    CidRange {
        start: 36416,
        end: 36421,
        cid: 2159,
    },
    CidRange {
        start: 36422,
        end: 36422,
        cid: 7688,
    },
    CidRange {
        start: 36423,
        end: 36424,
        cid: 2166,
    },
    CidRange {
        start: 36425,
        end: 36425,
        cid: 7689,
    },
    CidRange {
        start: 36426,
        end: 36426,
        cid: 2169,
    },
    CidRange {
        start: 36427,
        end: 36427,
        cid: 7690,
    },
    CidRange {
        start: 36428,
        end: 36439,
        cid: 2171,
    },
    CidRange {
        start: 36440,
        end: 36440,
        cid: 7691,
    },
    CidRange {
        start: 36441,
        end: 36478,
        cid: 2184,
    },
    CidRange {
        start: 36480,
        end: 36533,
        cid: 2222,
    },
    CidRange {
        start: 36534,
        end: 36534,
        cid: 7692,
    },
    CidRange {
        start: 36535,
        end: 36549,
        cid: 2277,
    },
    CidRange {
        start: 36550,
        end: 36550,
        cid: 7693,
    },
    CidRange {
        start: 36551,
        end: 36551,
        cid: 6441,
    },
    CidRange {
        start: 36552,
        end: 36564,
        cid: 2294,
    },
    CidRange {
        start: 36565,
        end: 36565,
        cid: 7694,
    },
    CidRange {
        start: 36566,
        end: 36570,
        cid: 2308,
    },
    CidRange {
        start: 36571,
        end: 36572,
        cid: 7695,
    },
    CidRange {
        start: 36573,
        end: 36604,
        cid: 2315,
    },
    CidRange {
        start: 36672,
        end: 36681,
        cid: 2347,
    },
    CidRange {
        start: 36682,
        end: 36682,
        cid: 7697,
    },
    CidRange {
        start: 36683,
        end: 36692,
        cid: 2358,
    },
    CidRange {
        start: 36693,
        end: 36693,
        cid: 7698,
    },
    CidRange {
        start: 36694,
        end: 36734,
        cid: 2369,
    },
    CidRange {
        start: 36736,
        end: 36747,
        cid: 2410,
    },
    CidRange {
        start: 36748,
        end: 36749,
        cid: 7699,
    },
    CidRange {
        start: 36750,
        end: 36753,
        cid: 2424,
    },
    CidRange {
        start: 36754,
        end: 36755,
        cid: 7701,
    },
    CidRange {
        start: 36756,
        end: 36770,
        cid: 2430,
    },
    CidRange {
        start: 36771,
        end: 36771,
        cid: 7703,
    },
    CidRange {
        start: 36772,
        end: 36784,
        cid: 2446,
    },
    CidRange {
        start: 36785,
        end: 36785,
        cid: 7704,
    },
    CidRange {
        start: 36786,
        end: 36796,
        cid: 2460,
    },
    CidRange {
        start: 36797,
        end: 36797,
        cid: 7705,
    },
    CidRange {
        start: 36798,
        end: 36818,
        cid: 2472,
    },
    CidRange {
        start: 36819,
        end: 36819,
        cid: 7706,
    },
    CidRange {
        start: 36820,
        end: 36828,
        cid: 2494,
    },
    CidRange {
        start: 36829,
        end: 36829,
        cid: 7707,
    },
    CidRange {
        start: 36830,
        end: 36833,
        cid: 2504,
    },
    CidRange {
        start: 36834,
        end: 36834,
        cid: 7708,
    },
    CidRange {
        start: 36835,
        end: 36860,
        cid: 2509,
    },
    CidRange {
        start: 36928,
        end: 36936,
        cid: 2535,
    },
    CidRange {
        start: 36937,
        end: 36937,
        cid: 7709,
    },
    CidRange {
        start: 36938,
        end: 36983,
        cid: 2545,
    },
    CidRange {
        start: 36984,
        end: 36984,
        cid: 7710,
    },
    CidRange {
        start: 36985,
        end: 36990,
        cid: 2592,
    },
    CidRange {
        start: 36992,
        end: 36992,
        cid: 7711,
    },
    CidRange {
        start: 36993,
        end: 37000,
        cid: 2599,
    },
    CidRange {
        start: 37001,
        end: 37001,
        cid: 7712,
    },
    CidRange {
        start: 37002,
        end: 37023,
        cid: 2608,
    },
    CidRange {
        start: 37024,
        end: 37024,
        cid: 7713,
    },
    CidRange {
        start: 37025,
        end: 37055,
        cid: 2631,
    },
    CidRange {
        start: 37056,
        end: 37056,
        cid: 7714,
    },
    CidRange {
        start: 37057,
        end: 37091,
        cid: 2663,
    },
    CidRange {
        start: 37092,
        end: 37092,
        cid: 7715,
    },
    CidRange {
        start: 37093,
        end: 37102,
        cid: 2699,
    },
    CidRange {
        start: 37103,
        end: 37104,
        cid: 7716,
    },
    CidRange {
        start: 37105,
        end: 37110,
        cid: 2711,
    },
    CidRange {
        start: 37111,
        end: 37112,
        cid: 7718,
    },
    CidRange {
        start: 37113,
        end: 37116,
        cid: 2719,
    },
    CidRange {
        start: 37184,
        end: 37189,
        cid: 2723,
    },
    CidRange {
        start: 37190,
        end: 37190,
        cid: 7720,
    },
    CidRange {
        start: 37191,
        end: 37191,
        cid: 6766,
    },
    CidRange {
        start: 37192,
        end: 37207,
        cid: 2731,
    },
    CidRange {
        start: 37208,
        end: 37208,
        cid: 7721,
    },
    CidRange {
        start: 37209,
        end: 37226,
        cid: 2748,
    },
    CidRange {
        start: 37227,
        end: 37227,
        cid: 7722,
    },
    CidRange {
        start: 37228,
        end: 37229,
        cid: 2767,
    },
    CidRange {
        start: 37230,
        end: 37230,
        cid: 7723,
    },
    CidRange {
        start: 37231,
        end: 37245,
        cid: 2770,
    },
    CidRange {
        start: 37246,
        end: 37246,
        cid: 7724,
    },
    CidRange {
        start: 37248,
        end: 37256,
        cid: 2786,
    },
    CidRange {
        start: 37257,
        end: 37257,
        cid: 7725,
    },
    CidRange {
        start: 37258,
        end: 37306,
        cid: 2796,
    },
    CidRange {
        start: 37307,
        end: 37307,
        cid: 7726,
    },
    CidRange {
        start: 37308,
        end: 37322,
        cid: 2846,
    },
    CidRange {
        start: 37323,
        end: 37323,
        cid: 7727,
    },
    CidRange {
        start: 37324,
        end: 37337,
        cid: 2862,
    },
    CidRange {
        start: 37338,
        end: 37338,
        cid: 7728,
    },
    CidRange {
        start: 37339,
        end: 37344,
        cid: 2877,
    },
    CidRange {
        start: 37345,
        end: 37345,
        cid: 7729,
    },
    CidRange {
        start: 37346,
        end: 37356,
        cid: 2884,
    },
    CidRange {
        start: 37357,
        end: 37357,
        cid: 7730,
    },
    CidRange {
        start: 37358,
        end: 37362,
        cid: 2896,
    },
    CidRange {
        start: 37363,
        end: 37364,
        cid: 7731,
    },
    CidRange {
        start: 37365,
        end: 37370,
        cid: 2903,
    },
    CidRange {
        start: 37371,
        end: 37371,
        cid: 7733,
    },
    CidRange {
        start: 37372,
        end: 37372,
        cid: 2910,
    },
    CidRange {
        start: 37440,
        end: 37445,
        cid: 2911,
    },
    CidRange {
        start: 37446,
        end: 37446,
        cid: 7734,
    },
    CidRange {
        start: 37447,
        end: 37447,
        cid: 2918,
    },
    CidRange {
        start: 37448,
        end: 37449,
        cid: 7735,
    },
    CidRange {
        start: 37450,
        end: 37451,
        cid: 2921,
    },
    CidRange {
        start: 37452,
        end: 37453,
        cid: 7737,
    },
    CidRange {
        start: 37454,
        end: 37467,
        cid: 2925,
    },
    CidRange {
        start: 37468,
        end: 37468,
        cid: 7739,
    },
    CidRange {
        start: 37469,
        end: 37502,
        cid: 2940,
    },
    CidRange {
        start: 37504,
        end: 37519,
        cid: 2974,
    },
    CidRange {
        start: 37520,
        end: 37520,
        cid: 7740,
    },
    CidRange {
        start: 37521,
        end: 37524,
        cid: 2991,
    },
    CidRange {
        start: 37525,
        end: 37525,
        cid: 7741,
    },
    CidRange {
        start: 37526,
        end: 37531,
        cid: 2996,
    },
    CidRange {
        start: 37532,
        end: 37532,
        cid: 7742,
    },
    CidRange {
        start: 37533,
        end: 37562,
        cid: 3003,
    },
    CidRange {
        start: 37563,
        end: 37563,
        cid: 7743,
    },
    CidRange {
        start: 37564,
        end: 37573,
        cid: 3034,
    },
    CidRange {
        start: 37574,
        end: 37574,
        cid: 7744,
    },
    CidRange {
        start: 37575,
        end: 37575,
        cid: 3045,
    },
    CidRange {
        start: 37576,
        end: 37576,
        cid: 7745,
    },
    CidRange {
        start: 37577,
        end: 37578,
        cid: 3047,
    },
    CidRange {
        start: 37579,
        end: 37579,
        cid: 7746,
    },
    CidRange {
        start: 37580,
        end: 37580,
        cid: 3050,
    },
    CidRange {
        start: 37581,
        end: 37581,
        cid: 7747,
    },
    CidRange {
        start: 37582,
        end: 37592,
        cid: 3052,
    },
    CidRange {
        start: 37593,
        end: 37593,
        cid: 4533,
    },
    CidRange {
        start: 37594,
        end: 37628,
        cid: 3064,
    },
    CidRange {
        start: 37696,
        end: 37696,
        cid: 3099,
    },
    CidRange {
        start: 37697,
        end: 37697,
        cid: 7748,
    },
    CidRange {
        start: 37698,
        end: 37701,
        cid: 3101,
    },
    CidRange {
        start: 37702,
        end: 37702,
        cid: 7749,
    },
    CidRange {
        start: 37703,
        end: 37708,
        cid: 3106,
    },
    CidRange {
        start: 37709,
        end: 37709,
        cid: 7750,
    },
    CidRange {
        start: 37710,
        end: 37716,
        cid: 3113,
    },
    CidRange {
        start: 37717,
        end: 37717,
        cid: 7751,
    },
    CidRange {
        start: 37718,
        end: 37725,
        cid: 3121,
    },
    CidRange {
        start: 37726,
        end: 37726,
        cid: 7752,
    },
    CidRange {
        start: 37727,
        end: 37734,
        cid: 3130,
    },
    CidRange {
        start: 37735,
        end: 37735,
        cid: 7753,
    },
    CidRange {
        start: 37736,
        end: 37737,
        cid: 3139,
    },
    CidRange {
        start: 37738,
        end: 37738,
        cid: 7754,
    },
    CidRange {
        start: 37739,
        end: 37743,
        cid: 3142,
    },
    CidRange {
        start: 37744,
        end: 37745,
        cid: 7755,
    },
    CidRange {
        start: 37746,
        end: 37749,
        cid: 3149,
    },
    CidRange {
        start: 37750,
        end: 37750,
        cid: 5855,
    },
    CidRange {
        start: 37751,
        end: 37758,
        cid: 3154,
    },
    CidRange {
        start: 37760,
        end: 37763,
        cid: 3162,
    },
    CidRange {
        start: 37764,
        end: 37764,
        cid: 7757,
    },
    CidRange {
        start: 37765,
        end: 37773,
        cid: 3167,
    },
    CidRange {
        start: 37774,
        end: 37774,
        cid: 5200,
    },
    CidRange {
        start: 37775,
        end: 37778,
        cid: 3177,
    },
    CidRange {
        start: 37779,
        end: 37779,
        cid: 5430,
    },
    CidRange {
        start: 37780,
        end: 37783,
        cid: 3182,
    },
    CidRange {
        start: 37784,
        end: 37784,
        cid: 7758,
    },
    CidRange {
        start: 37785,
        end: 37819,
        cid: 3187,
    },
    CidRange {
        start: 37820,
        end: 37820,
        cid: 7759,
    },
    CidRange {
        start: 37821,
        end: 37823,
        cid: 3223,
    },
    CidRange {
        start: 37824,
        end: 37824,
        cid: 7760,
    },
    CidRange {
        start: 37825,
        end: 37841,
        cid: 3227,
    },
    CidRange {
        start: 37842,
        end: 37843,
        cid: 7761,
    },
    CidRange {
        start: 37844,
        end: 37848,
        cid: 3246,
    },
    CidRange {
        start: 37849,
        end: 37850,
        cid: 7763,
    },
    CidRange {
        start: 37851,
        end: 37854,
        cid: 3253,
    },
    CidRange {
        start: 37855,
        end: 37855,
        cid: 7765,
    },
    CidRange {
        start: 37856,
        end: 37859,
        cid: 3258,
    },
    CidRange {
        start: 37860,
        end: 37861,
        cid: 7766,
    },
    CidRange {
        start: 37862,
        end: 37863,
        cid: 3264,
    },
    CidRange {
        start: 37864,
        end: 37864,
        cid: 7768,
    },
    CidRange {
        start: 37865,
        end: 37875,
        cid: 3267,
    },
    CidRange {
        start: 37876,
        end: 37876,
        cid: 6893,
    },
    CidRange {
        start: 37877,
        end: 37884,
        cid: 3279,
    },
    CidRange {
        start: 37952,
        end: 37959,
        cid: 3287,
    },
    CidRange {
        start: 37960,
        end: 37960,
        cid: 7769,
    },
    CidRange {
        start: 37961,
        end: 37975,
        cid: 3296,
    },
    CidRange {
        start: 37976,
        end: 37976,
        cid: 7770,
    },
    CidRange {
        start: 37977,
        end: 38005,
        cid: 3312,
    },
    CidRange {
        start: 38006,
        end: 38006,
        cid: 7771,
    },
    CidRange {
        start: 38007,
        end: 38014,
        cid: 3342,
    },
    CidRange {
        start: 38016,
        end: 38022,
        cid: 3350,
    },
    CidRange {
        start: 38023,
        end: 38023,
        cid: 7772,
    },
    CidRange {
        start: 38024,
        end: 38024,
        cid: 6537,
    },
    CidRange {
        start: 38025,
        end: 38025,
        cid: 7773,
    },
    CidRange {
        start: 38026,
        end: 38028,
        cid: 3360,
    },
    CidRange {
        start: 38029,
        end: 38029,
        cid: 7774,
    },
    CidRange {
        start: 38030,
        end: 38049,
        cid: 3364,
    },
    CidRange {
        start: 38050,
        end: 38050,
        cid: 7775,
    },
    CidRange {
        start: 38051,
        end: 38059,
        cid: 3385,
    },
    CidRange {
        start: 38060,
        end: 38060,
        cid: 7776,
    },
    CidRange {
        start: 38061,
        end: 38061,
        cid: 3395,
    },
    CidRange {
        start: 38062,
        end: 38062,
        cid: 7777,
    },
    CidRange {
        start: 38063,
        end: 38097,
        cid: 3397,
    },
    CidRange {
        start: 38098,
        end: 38098,
        cid: 7778,
    },
    CidRange {
        start: 38099,
        end: 38111,
        cid: 3433,
    },
    CidRange {
        start: 38112,
        end: 38112,
        cid: 7779,
    },
    CidRange {
        start: 38113,
        end: 38130,
        cid: 3447,
    },
    CidRange {
        start: 38131,
        end: 38131,
        cid: 7780,
    },
    CidRange {
        start: 38132,
        end: 38140,
        cid: 3466,
    },
    CidRange {
        start: 38208,
        end: 38208,
        cid: 3475,
    },
    CidRange {
        start: 38209,
        end: 38210,
        cid: 7781,
    },
    CidRange {
        start: 38211,
        end: 38221,
        cid: 3478,
    },
    CidRange {
        start: 38222,
        end: 38222,
        cid: 7783,
    },
    CidRange {
        start: 38223,
        end: 38223,
        cid: 5179,
    },
    CidRange {
        start: 38224,
        end: 38224,
        cid: 3491,
    },
    CidRange {
        start: 38225,
        end: 38225,
        cid: 7784,
    },
    CidRange {
        start: 38226,
        end: 38227,
        cid: 3493,
    },
    CidRange {
        start: 38228,
        end: 38228,
        cid: 7785,
    },
    CidRange {
        start: 38229,
        end: 38238,
        cid: 3496,
    },
    CidRange {
        start: 38239,
        end: 38239,
        cid: 7786,
    },
    CidRange {
        start: 38240,
        end: 38252,
        cid: 3507,
    },
    CidRange {
        start: 38253,
        end: 38253,
        cid: 7787,
    },
    CidRange {
        start: 38254,
        end: 38255,
        cid: 3521,
    },
    CidRange {
        start: 38256,
        end: 38256,
        cid: 7788,
    },
    CidRange {
        start: 38257,
        end: 38270,
        cid: 3524,
    },
    CidRange {
        start: 38272,
        end: 38336,
        cid: 3538,
    },
    CidRange {
        start: 38337,
        end: 38337,
        cid: 7789,
    },
    CidRange {
        start: 38338,
        end: 38346,
        cid: 3604,
    },
    CidRange {
        start: 38347,
        end: 38347,
        cid: 7790,
    },
    CidRange {
        start: 38348,
        end: 38359,
        cid: 3614,
    },
    CidRange {
        start: 38360,
        end: 38360,
        cid: 7791,
    },
    CidRange {
        start: 38361,
        end: 38390,
        cid: 3627,
    },
    CidRange {
        start: 38391,
        end: 38391,
        cid: 7792,
    },
    CidRange {
        start: 38392,
        end: 38396,
        cid: 3658,
    },
    CidRange {
        start: 38464,
        end: 38464,
        cid: 3663,
    },
    CidRange {
        start: 38465,
        end: 38465,
        cid: 7793,
    },
    CidRange {
        start: 38466,
        end: 38471,
        cid: 3665,
    },
    CidRange {
        start: 38472,
        end: 38472,
        cid: 7794,
    },
    CidRange {
        start: 38473,
        end: 38505,
        cid: 3672,
    },
    CidRange {
        start: 38506,
        end: 38506,
        cid: 7795,
    },
    CidRange {
        start: 38507,
        end: 38526,
        cid: 3706,
    },
    CidRange {
        start: 38528,
        end: 38537,
        cid: 3726,
    },
    CidRange {
        start: 38538,
        end: 38538,
        cid: 7475,
    },
    CidRange {
        start: 38539,
        end: 38543,
        cid: 3737,
    },
    CidRange {
        start: 38544,
        end: 38544,
        cid: 7796,
    },
    CidRange {
        start: 38545,
        end: 38552,
        cid: 3743,
    },
    CidRange {
        start: 38553,
        end: 38553,
        cid: 4143,
    },
    CidRange {
        start: 38554,
        end: 38602,
        cid: 3752,
    },
    CidRange {
        start: 38603,
        end: 38603,
        cid: 7797,
    },
    CidRange {
        start: 38604,
        end: 38614,
        cid: 3802,
    },
    CidRange {
        start: 38615,
        end: 38615,
        cid: 7798,
    },
    CidRange {
        start: 38616,
        end: 38620,
        cid: 3814,
    },
    CidRange {
        start: 38621,
        end: 38621,
        cid: 7799,
    },
    CidRange {
        start: 38622,
        end: 38623,
        cid: 3820,
    },
    CidRange {
        start: 38624,
        end: 38624,
        cid: 7800,
    },
    CidRange {
        start: 38625,
        end: 38646,
        cid: 3823,
    },
    CidRange {
        start: 38647,
        end: 38647,
        cid: 6453,
    },
    CidRange {
        start: 38648,
        end: 38648,
        cid: 7801,
    },
    CidRange {
        start: 38649,
        end: 38649,
        cid: 3847,
    },
    CidRange {
        start: 38650,
        end: 38650,
        cid: 7802,
    },
    CidRange {
        start: 38651,
        end: 38651,
        cid: 3849,
    },
    CidRange {
        start: 38652,
        end: 38652,
        cid: 7803,
    },
    CidRange {
        start: 38720,
        end: 38736,
        cid: 3851,
    },
    CidRange {
        start: 38737,
        end: 38737,
        cid: 7804,
    },
    CidRange {
        start: 38738,
        end: 38766,
        cid: 3869,
    },
    CidRange {
        start: 38767,
        end: 38767,
        cid: 7805,
    },
    CidRange {
        start: 38768,
        end: 38770,
        cid: 3899,
    },
    CidRange {
        start: 38771,
        end: 38771,
        cid: 7806,
    },
    CidRange {
        start: 38772,
        end: 38776,
        cid: 3903,
    },
    CidRange {
        start: 38777,
        end: 38777,
        cid: 7476,
    },
    CidRange {
        start: 38778,
        end: 38782,
        cid: 3909,
    },
    CidRange {
        start: 38784,
        end: 38792,
        cid: 3914,
    },
    CidRange {
        start: 38793,
        end: 38793,
        cid: 7807,
    },
    CidRange {
        start: 38794,
        end: 38856,
        cid: 3924,
    },
    CidRange {
        start: 38857,
        end: 38857,
        cid: 7808,
    },
    CidRange {
        start: 38858,
        end: 38903,
        cid: 3988,
    },
    CidRange {
        start: 38904,
        end: 38905,
        cid: 7809,
    },
    CidRange {
        start: 38906,
        end: 38908,
        cid: 4036,
    },
    CidRange {
        start: 38976,
        end: 38976,
        cid: 7811,
    },
    CidRange {
        start: 38977,
        end: 38991,
        cid: 4040,
    },
    CidRange {
        start: 38992,
        end: 38992,
        cid: 7812,
    },
    CidRange {
        start: 38993,
        end: 38996,
        cid: 4056,
    },
    CidRange {
        start: 38997,
        end: 38997,
        cid: 6007,
    },
    CidRange {
        start: 38998,
        end: 38999,
        cid: 4061,
    },
    CidRange {
        start: 39000,
        end: 39000,
        cid: 7813,
    },
    CidRange {
        start: 39001,
        end: 39026,
        cid: 4064,
    },
    CidRange {
        start: 39071,
        end: 39123,
        cid: 4090,
    },
    CidRange {
        start: 39124,
        end: 39124,
        cid: 3751,
    },
    CidRange {
        start: 39125,
        end: 39164,
        cid: 4144,
    },
    CidRange {
        start: 39232,
        end: 39259,
        cid: 4184,
    },
    CidRange {
        start: 39260,
        end: 39260,
        cid: 7814,
    },
    CidRange {
        start: 39261,
        end: 39269,
        cid: 4213,
    },
    CidRange {
        start: 39270,
        end: 39270,
        cid: 7815,
    },
    CidRange {
        start: 39271,
        end: 39273,
        cid: 4223,
    },
    CidRange {
        start: 39274,
        end: 39274,
        cid: 7816,
    },
    CidRange {
        start: 39275,
        end: 39275,
        cid: 4227,
    },
    CidRange {
        start: 39276,
        end: 39276,
        cid: 7817,
    },
    CidRange {
        start: 39277,
        end: 39294,
        cid: 4229,
    },
    CidRange {
        start: 39296,
        end: 39420,
        cid: 4247,
    },
    CidRange {
        start: 39488,
        end: 39502,
        cid: 4372,
    },
    CidRange {
        start: 39503,
        end: 39503,
        cid: 7818,
    },
    CidRange {
        start: 39504,
        end: 39512,
        cid: 4388,
    },
    CidRange {
        start: 39513,
        end: 39513,
        cid: 7819,
    },
    CidRange {
        start: 39514,
        end: 39534,
        cid: 4398,
    },
    CidRange {
        start: 39535,
        end: 39535,
        cid: 7820,
    },
    CidRange {
        start: 39536,
        end: 39548,
        cid: 4420,
    },
    CidRange {
        start: 39549,
        end: 39549,
        cid: 7821,
    },
    CidRange {
        start: 39550,
        end: 39550,
        cid: 4434,
    },
    CidRange {
        start: 39552,
        end: 39562,
        cid: 4435,
    },
    CidRange {
        start: 39563,
        end: 39563,
        cid: 7822,
    },
    CidRange {
        start: 39564,
        end: 39617,
        cid: 4447,
    },
    CidRange {
        start: 39618,
        end: 39618,
        cid: 7823,
    },
    CidRange {
        start: 39619,
        end: 39649,
        cid: 4502,
    },
    CidRange {
        start: 39650,
        end: 39650,
        cid: 3063,
    },
    CidRange {
        start: 39651,
        end: 39676,
        cid: 4534,
    },
    CidRange {
        start: 39744,
        end: 39771,
        cid: 4560,
    },
    CidRange {
        start: 39772,
        end: 39772,
        cid: 7824,
    },
    CidRange {
        start: 39773,
        end: 39806,
        cid: 4589,
    },
    CidRange {
        start: 39808,
        end: 39810,
        cid: 4623,
    },
    CidRange {
        start: 39811,
        end: 39811,
        cid: 7825,
    },
    CidRange {
        start: 39812,
        end: 39839,
        cid: 4627,
    },
    CidRange {
        start: 39840,
        end: 39840,
        cid: 7826,
    },
    CidRange {
        start: 39841,
        end: 39919,
        cid: 4656,
    },
    CidRange {
        start: 39920,
        end: 39920,
        cid: 7827,
    },
    CidRange {
        start: 39921,
        end: 39932,
        cid: 4736,
    },
    CidRange {
        start: 40000,
        end: 40062,
        cid: 4748,
    },
    CidRange {
        start: 40064,
        end: 40097,
        cid: 4811,
    },
    CidRange {
        start: 40098,
        end: 40098,
        cid: 7828,
    },
    CidRange {
        start: 40099,
        end: 40188,
        cid: 4846,
    },
    CidRange {
        start: 40256,
        end: 40318,
        cid: 4936,
    },
    CidRange {
        start: 40320,
        end: 40320,
        cid: 7829,
    },
    CidRange {
        start: 40321,
        end: 40331,
        cid: 5000,
    },
    CidRange {
        start: 40332,
        end: 40332,
        cid: 7830,
    },
    CidRange {
        start: 40333,
        end: 40343,
        cid: 5012,
    },
    CidRange {
        start: 40344,
        end: 40344,
        cid: 1447,
    },
    CidRange {
        start: 40345,
        end: 40374,
        cid: 5024,
    },
    CidRange {
        start: 40375,
        end: 40375,
        cid: 7831,
    },
    CidRange {
        start: 40376,
        end: 40394,
        cid: 5055,
    },
    CidRange {
        start: 40395,
        end: 40395,
        cid: 7832,
    },
    CidRange {
        start: 40396,
        end: 40444,
        cid: 5075,
    },
    CidRange {
        start: 40512,
        end: 40547,
        cid: 5124,
    },
    CidRange {
        start: 40548,
        end: 40548,
        cid: 7833,
    },
    CidRange {
        start: 40549,
        end: 40552,
        cid: 5161,
    },
    CidRange {
        start: 40553,
        end: 40553,
        cid: 7834,
    },
    CidRange {
        start: 40554,
        end: 40566,
        cid: 5166,
    },
    CidRange {
        start: 40567,
        end: 40567,
        cid: 3490,
    },
    CidRange {
        start: 40568,
        end: 40574,
        cid: 5180,
    },
    CidRange {
        start: 40576,
        end: 40586,
        cid: 5187,
    },
    CidRange {
        start: 40587,
        end: 40587,
        cid: 7835,
    },
    CidRange {
        start: 40588,
        end: 40588,
        cid: 5199,
    },
    CidRange {
        start: 40589,
        end: 40589,
        cid: 3176,
    },
    CidRange {
        start: 40590,
        end: 40595,
        cid: 5201,
    },
    CidRange {
        start: 40596,
        end: 40596,
        cid: 7836,
    },
    CidRange {
        start: 40597,
        end: 40700,
        cid: 5208,
    },
    CidRange {
        start: 40768,
        end: 40830,
        cid: 5312,
    },
    CidRange {
        start: 40832,
        end: 40886,
        cid: 5375,
    },
    CidRange {
        start: 40887,
        end: 40887,
        cid: 3181,
    },
    CidRange {
        start: 40888,
        end: 40909,
        cid: 5431,
    },
    CidRange {
        start: 40910,
        end: 40910,
        cid: 7837,
    },
    CidRange {
        start: 40911,
        end: 40946,
        cid: 5454,
    },
    CidRange {
        start: 40947,
        end: 40947,
        cid: 1535,
    },
    CidRange {
        start: 40948,
        end: 40956,
        cid: 5491,
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
        end: 57490,
        cid: 5563,
    },
    CidRange {
        start: 57491,
        end: 57491,
        cid: 7838,
    },
    CidRange {
        start: 57492,
        end: 57507,
        cid: 5583,
    },
    CidRange {
        start: 57508,
        end: 57508,
        cid: 7839,
    },
    CidRange {
        start: 57509,
        end: 57564,
        cid: 5600,
    },
    CidRange {
        start: 57565,
        end: 57565,
        cid: 7840,
    },
    CidRange {
        start: 57566,
        end: 57587,
        cid: 5657,
    },
    CidRange {
        start: 57588,
        end: 57588,
        cid: 7477,
    },
    CidRange {
        start: 57589,
        end: 57596,
        cid: 5680,
    },
    CidRange {
        start: 57664,
        end: 57673,
        cid: 5688,
    },
    CidRange {
        start: 57674,
        end: 57674,
        cid: 7841,
    },
    CidRange {
        start: 57675,
        end: 57678,
        cid: 5699,
    },
    CidRange {
        start: 57679,
        end: 57680,
        cid: 7842,
    },
    CidRange {
        start: 57681,
        end: 57726,
        cid: 5705,
    },
    CidRange {
        start: 57728,
        end: 57768,
        cid: 5751,
    },
    CidRange {
        start: 57769,
        end: 57769,
        cid: 7844,
    },
    CidRange {
        start: 57770,
        end: 57829,
        cid: 5793,
    },
    CidRange {
        start: 57830,
        end: 57830,
        cid: 2030,
    },
    CidRange {
        start: 57831,
        end: 57831,
        cid: 5854,
    },
    CidRange {
        start: 57832,
        end: 57832,
        cid: 3153,
    },
    CidRange {
        start: 57833,
        end: 57836,
        cid: 5856,
    },
    CidRange {
        start: 57837,
        end: 57837,
        cid: 7845,
    },
    CidRange {
        start: 57838,
        end: 57852,
        cid: 5861,
    },
    CidRange {
        start: 57920,
        end: 57960,
        cid: 5876,
    },
    CidRange {
        start: 57961,
        end: 57961,
        cid: 7846,
    },
    CidRange {
        start: 57962,
        end: 57970,
        cid: 5918,
    },
    CidRange {
        start: 57971,
        end: 57971,
        cid: 7847,
    },
    CidRange {
        start: 57972,
        end: 57980,
        cid: 5928,
    },
    CidRange {
        start: 57981,
        end: 57981,
        cid: 1492,
    },
    CidRange {
        start: 57982,
        end: 57982,
        cid: 5938,
    },
    CidRange {
        start: 57984,
        end: 58038,
        cid: 5939,
    },
    CidRange {
        start: 58039,
        end: 58039,
        cid: 7848,
    },
    CidRange {
        start: 58040,
        end: 58051,
        cid: 5995,
    },
    CidRange {
        start: 58052,
        end: 58052,
        cid: 4060,
    },
    CidRange {
        start: 58053,
        end: 58081,
        cid: 6008,
    },
    CidRange {
        start: 58082,
        end: 58082,
        cid: 7849,
    },
    CidRange {
        start: 58083,
        end: 58091,
        cid: 6038,
    },
    CidRange {
        start: 58092,
        end: 58092,
        cid: 7850,
    },
    CidRange {
        start: 58093,
        end: 58108,
        cid: 6048,
    },
    CidRange {
        start: 58176,
        end: 58199,
        cid: 6064,
    },
    CidRange {
        start: 58200,
        end: 58200,
        cid: 7851,
    },
    CidRange {
        start: 58201,
        end: 58201,
        cid: 6089,
    },
    CidRange {
        start: 58202,
        end: 58202,
        cid: 7852,
    },
    CidRange {
        start: 58203,
        end: 58212,
        cid: 6091,
    },
    CidRange {
        start: 58213,
        end: 58213,
        cid: 7853,
    },
    CidRange {
        start: 58214,
        end: 58238,
        cid: 6102,
    },
    CidRange {
        start: 58240,
        end: 58307,
        cid: 6127,
    },
    CidRange {
        start: 58308,
        end: 58308,
        cid: 7854,
    },
    CidRange {
        start: 58309,
        end: 58364,
        cid: 6196,
    },
    CidRange {
        start: 58432,
        end: 58494,
        cid: 6252,
    },
    CidRange {
        start: 58496,
        end: 58499,
        cid: 6315,
    },
    CidRange {
        start: 58500,
        end: 58500,
        cid: 7855,
    },
    CidRange {
        start: 58501,
        end: 58504,
        cid: 6320,
    },
    CidRange {
        start: 58505,
        end: 58505,
        cid: 7856,
    },
    CidRange {
        start: 58506,
        end: 58513,
        cid: 6325,
    },
    CidRange {
        start: 58514,
        end: 58514,
        cid: 7857,
    },
    CidRange {
        start: 58515,
        end: 58545,
        cid: 6334,
    },
    CidRange {
        start: 58546,
        end: 58546,
        cid: 7858,
    },
    CidRange {
        start: 58547,
        end: 58552,
        cid: 6366,
    },
    CidRange {
        start: 58553,
        end: 58553,
        cid: 7859,
    },
    CidRange {
        start: 58554,
        end: 58609,
        cid: 6373,
    },
    CidRange {
        start: 58610,
        end: 58610,
        cid: 7860,
    },
    CidRange {
        start: 58611,
        end: 58620,
        cid: 6430,
    },
    CidRange {
        start: 58688,
        end: 58688,
        cid: 6440,
    },
    CidRange {
        start: 58689,
        end: 58689,
        cid: 2293,
    },
    CidRange {
        start: 58690,
        end: 58700,
        cid: 6442,
    },
    CidRange {
        start: 58701,
        end: 58701,
        cid: 3845,
    },
    CidRange {
        start: 58702,
        end: 58714,
        cid: 6454,
    },
    CidRange {
        start: 58715,
        end: 58715,
        cid: 7861,
    },
    CidRange {
        start: 58716,
        end: 58744,
        cid: 6468,
    },
    CidRange {
        start: 58745,
        end: 58745,
        cid: 1440,
    },
    CidRange {
        start: 58746,
        end: 58750,
        cid: 6498,
    },
    CidRange {
        start: 58752,
        end: 58785,
        cid: 6503,
    },
    CidRange {
        start: 58786,
        end: 58786,
        cid: 3358,
    },
    CidRange {
        start: 58787,
        end: 58788,
        cid: 6538,
    },
    CidRange {
        start: 58789,
        end: 58789,
        cid: 7862,
    },
    CidRange {
        start: 58790,
        end: 58810,
        cid: 6541,
    },
    CidRange {
        start: 58811,
        end: 58811,
        cid: 7863,
    },
    CidRange {
        start: 58812,
        end: 58860,
        cid: 6563,
    },
    CidRange {
        start: 58861,
        end: 58861,
        cid: 7864,
    },
    CidRange {
        start: 58862,
        end: 58876,
        cid: 6613,
    },
    CidRange {
        start: 58944,
        end: 58960,
        cid: 6628,
    },
    CidRange {
        start: 58961,
        end: 58961,
        cid: 7865,
    },
    CidRange {
        start: 58962,
        end: 59003,
        cid: 6646,
    },
    CidRange {
        start: 59004,
        end: 59004,
        cid: 1550,
    },
    CidRange {
        start: 59005,
        end: 59006,
        cid: 6689,
    },
    CidRange {
        start: 59008,
        end: 59013,
        cid: 6691,
    },
    CidRange {
        start: 59014,
        end: 59014,
        cid: 7866,
    },
    CidRange {
        start: 59015,
        end: 59029,
        cid: 6698,
    },
    CidRange {
        start: 59030,
        end: 59030,
        cid: 7867,
    },
    CidRange {
        start: 59031,
        end: 59082,
        cid: 6714,
    },
    CidRange {
        start: 59083,
        end: 59083,
        cid: 2730,
    },
    CidRange {
        start: 59084,
        end: 59110,
        cid: 6767,
    },
    CidRange {
        start: 59111,
        end: 59111,
        cid: 7868,
    },
    CidRange {
        start: 59112,
        end: 59121,
        cid: 6795,
    },
    CidRange {
        start: 59122,
        end: 59122,
        cid: 7869,
    },
    CidRange {
        start: 59123,
        end: 59132,
        cid: 6806,
    },
    CidRange {
        start: 59200,
        end: 59244,
        cid: 6816,
    },
    CidRange {
        start: 59245,
        end: 59245,
        cid: 7870,
    },
    CidRange {
        start: 59246,
        end: 59262,
        cid: 6862,
    },
    CidRange {
        start: 59264,
        end: 59275,
        cid: 6879,
    },
    CidRange {
        start: 59276,
        end: 59276,
        cid: 7871,
    },
    CidRange {
        start: 59277,
        end: 59277,
        cid: 6892,
    },
    CidRange {
        start: 59278,
        end: 59278,
        cid: 7872,
    },
    CidRange {
        start: 59279,
        end: 59302,
        cid: 6894,
    },
    CidRange {
        start: 59303,
        end: 59303,
        cid: 7873,
    },
    CidRange {
        start: 59304,
        end: 59322,
        cid: 6919,
    },
    CidRange {
        start: 59323,
        end: 59323,
        cid: 7874,
    },
    CidRange {
        start: 59324,
        end: 59348,
        cid: 6939,
    },
    CidRange {
        start: 59349,
        end: 59349,
        cid: 7875,
    },
    CidRange {
        start: 59350,
        end: 59388,
        cid: 6965,
    },
    CidRange {
        start: 59456,
        end: 59518,
        cid: 7004,
    },
    CidRange {
        start: 59520,
        end: 59524,
        cid: 7067,
    },
    CidRange {
        start: 59525,
        end: 59525,
        cid: 7876,
    },
    CidRange {
        start: 59526,
        end: 59568,
        cid: 7073,
    },
    CidRange {
        start: 59569,
        end: 59569,
        cid: 7877,
    },
    CidRange {
        start: 59570,
        end: 59586,
        cid: 7117,
    },
    CidRange {
        start: 59587,
        end: 59587,
        cid: 7878,
    },
    CidRange {
        start: 59588,
        end: 59598,
        cid: 7135,
    },
    CidRange {
        start: 59599,
        end: 59599,
        cid: 7879,
    },
    CidRange {
        start: 59600,
        end: 59604,
        cid: 7147,
    },
    CidRange {
        start: 59605,
        end: 59605,
        cid: 7880,
    },
    CidRange {
        start: 59606,
        end: 59633,
        cid: 7153,
    },
    CidRange {
        start: 59634,
        end: 59634,
        cid: 1841,
    },
    CidRange {
        start: 59635,
        end: 59635,
        cid: 7881,
    },
    CidRange {
        start: 59636,
        end: 59644,
        cid: 7183,
    },
    CidRange {
        start: 59712,
        end: 59774,
        cid: 7192,
    },
    CidRange {
        start: 59776,
        end: 59818,
        cid: 7255,
    },
    CidRange {
        start: 59819,
        end: 59819,
        cid: 7882,
    },
    CidRange {
        start: 59820,
        end: 59833,
        cid: 7299,
    },
    CidRange {
        start: 59834,
        end: 59834,
        cid: 7883,
    },
    CidRange {
        start: 59835,
        end: 59850,
        cid: 7314,
    },
    CidRange {
        start: 59851,
        end: 59851,
        cid: 1143,
    },
    CidRange {
        start: 59852,
        end: 59852,
        cid: 7884,
    },
    CidRange {
        start: 59853,
        end: 59889,
        cid: 7332,
    },
    CidRange {
        start: 59890,
        end: 59890,
        cid: 1321,
    },
    CidRange {
        start: 59891,
        end: 59900,
        cid: 7370,
    },
    CidRange {
        start: 59968,
        end: 60015,
        cid: 7380,
    },
    CidRange {
        start: 60016,
        end: 60016,
        cid: 7885,
    },
    CidRange {
        start: 60017,
        end: 60030,
        cid: 7429,
    },
    CidRange {
        start: 60032,
        end: 60060,
        cid: 7443,
    },
    CidRange {
        start: 60061,
        end: 60061,
        cid: 7886,
    },
    CidRange {
        start: 60062,
        end: 60062,
        cid: 7473,
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
];

const CID_RANGE_V: [CidRange; 39] = [
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
        start: 33098,
        end: 33098,
        cid: 8272,
    },
    CidRange {
        start: 33099,
        end: 33099,
        cid: 8271,
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
        start: 33125,
        end: 33125,
        cid: 8281,
    },
    CidRange {
        start: 33126,
        end: 33126,
        cid: 8276,
    },
    CidRange {
        start: 33127,
        end: 33127,
        cid: 8279,
    },
    CidRange {
        start: 33128,
        end: 33128,
        cid: 8278,
    },
    CidRange {
        start: 33129,
        end: 33146,
        cid: 7899,
    },
    CidRange {
        start: 33163,
        end: 33163,
        cid: 8269,
    },
    CidRange {
        start: 33164,
        end: 33164,
        cid: 8273,
    },
    CidRange {
        start: 33165,
        end: 33165,
        cid: 8283,
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
        start: 34655,
        end: 34670,
        cid: 7940,
    },
    CidRange {
        start: 34688,
        end: 34689,
        cid: 7956,
    },
];

pub const EXT_RKSJ_H: CMap = CMap {
    name: Cow::Borrowed(b"Ext-RKSJ-H"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(JAPAN_1),
        supplement: 2,
    },
    writing_mode: CMapWritingMode::Horizontal,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: Cow::Borrowed(&NO_CID_CHARS),
    cid_range: Cow::Borrowed(&CID_RANGE_H),
};

pub const EXT_RKSJ_V: CMap = CMap {
    name: Cow::Borrowed(b"Ext-RKSJ-V"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(JAPAN_1),
        supplement: 2,
    },
    writing_mode: CMapWritingMode::Vertical,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: Cow::Borrowed(&NO_CID_CHARS),
    cid_range: Cow::Borrowed(&CID_RANGE_V),
};
