use std::borrow::Cow;

use crate::cmaps::cmap::{
    CMap, CMapWritingMode, CidRange, Codespace, CodespaceRange, ADOBE_REGISTRY, NO_BASE_FONT_CHARS,
    NO_CID_CHARS,
};
use crate::font::font::CidSystemInfo;

use super::CNS_1;

const CODE_SPACE: [CodespaceRange; 5] = [
    [0..=0, 0..=0, 0..=0, 0..=128],
    [142..=142, 161..=161, 161..=254, 161..=254],
    [142..=142, 162..=162, 161..=254, 161..=254],
    [142..=142, 163..=163, 161..=254, 161..=254],
    [0..=0, 0..=0, 161..=254, 161..=254],
];

const CID_RANGE_H: [CidRange; 395] = [
    CidRange {
        start: 32,
        end: 126,
        cid: 13648,
    },
    CidRange {
        start: 2392957345,
        end: 2392957438,
        cid: 99,
    },
    CidRange {
        start: 2392957601,
        end: 2392957694,
        cid: 193,
    },
    CidRange {
        start: 2392957857,
        end: 2392957902,
        cid: 287,
    },
    CidRange {
        start: 2392958113,
        end: 2392958206,
        cid: 333,
    },
    CidRange {
        start: 2392958369,
        end: 2392958444,
        cid: 427,
    },
    CidRange {
        start: 2392958446,
        end: 2392958448,
        cid: 503,
    },
    CidRange {
        start: 2392958625,
        end: 2392958654,
        cid: 506,
    },
    CidRange {
        start: 2392958881,
        end: 2392958881,
        cid: 595,
    },
    CidRange {
        start: 2392958882,
        end: 2392958884,
        cid: 536,
    },
    CidRange {
        start: 2392958885,
        end: 2392958885,
        cid: 596,
    },
    CidRange {
        start: 2392958886,
        end: 2392958886,
        cid: 539,
    },
    CidRange {
        start: 2392958887,
        end: 2392958887,
        cid: 602,
    },
    CidRange {
        start: 2392958888,
        end: 2392958888,
        cid: 540,
    },
    CidRange {
        start: 2392958889,
        end: 2392958892,
        cid: 603,
    },
    CidRange {
        start: 2392958893,
        end: 2392958895,
        cid: 541,
    },
    CidRange {
        start: 2392958896,
        end: 2392958896,
        cid: 607,
    },
    CidRange {
        start: 2392958897,
        end: 2392958897,
        cid: 5998,
    },
    CidRange {
        start: 2392958898,
        end: 2392958898,
        cid: 608,
    },
    CidRange {
        start: 2392958899,
        end: 2392958899,
        cid: 610,
    },
    CidRange {
        start: 2392958900,
        end: 2392958900,
        cid: 544,
    },
    CidRange {
        start: 2392958901,
        end: 2392958901,
        cid: 611,
    },
    CidRange {
        start: 2392958902,
        end: 2392958902,
        cid: 5999,
    },
    CidRange {
        start: 2392958903,
        end: 2392958903,
        cid: 545,
    },
    CidRange {
        start: 2392958904,
        end: 2392958905,
        cid: 612,
    },
    CidRange {
        start: 2392958906,
        end: 2392958906,
        cid: 546,
    },
    CidRange {
        start: 2392958907,
        end: 2392958907,
        cid: 6000,
    },
    CidRange {
        start: 2392958908,
        end: 2392958908,
        cid: 547,
    },
    CidRange {
        start: 2392958909,
        end: 2392958909,
        cid: 614,
    },
    CidRange {
        start: 2392958910,
        end: 2392958910,
        cid: 633,
    },
    CidRange {
        start: 2392958911,
        end: 2392958911,
        cid: 6005,
    },
    CidRange {
        start: 2392958912,
        end: 2392958913,
        cid: 634,
    },
    CidRange {
        start: 2392958914,
        end: 2392958914,
        cid: 548,
    },
    CidRange {
        start: 2392958915,
        end: 2392958918,
        cid: 636,
    },
    CidRange {
        start: 2392958919,
        end: 2392958919,
        cid: 549,
    },
    CidRange {
        start: 2392958920,
        end: 2392958923,
        cid: 642,
    },
    CidRange {
        start: 2392958924,
        end: 2392958924,
        cid: 6006,
    },
    CidRange {
        start: 2392958925,
        end: 2392958925,
        cid: 646,
    },
    CidRange {
        start: 2392958926,
        end: 2392958926,
        cid: 550,
    },
    CidRange {
        start: 2392958927,
        end: 2392958928,
        cid: 648,
    },
    CidRange {
        start: 2392958929,
        end: 2392958930,
        cid: 652,
    },
    CidRange {
        start: 2392958931,
        end: 2392958933,
        cid: 551,
    },
    CidRange {
        start: 2392958934,
        end: 2392958936,
        cid: 654,
    },
    CidRange {
        start: 2392958937,
        end: 2392958938,
        cid: 554,
    },
    CidRange {
        start: 2392958939,
        end: 2392958939,
        cid: 6007,
    },
    CidRange {
        start: 2392958940,
        end: 2392958943,
        cid: 720,
    },
    CidRange {
        start: 2392958944,
        end: 2392958944,
        cid: 725,
    },
    CidRange {
        start: 2392958945,
        end: 2392958945,
        cid: 556,
    },
    CidRange {
        start: 2392958946,
        end: 2392958949,
        cid: 726,
    },
    CidRange {
        start: 2392958950,
        end: 2392958950,
        cid: 557,
    },
    CidRange {
        start: 2392958951,
        end: 2392958957,
        cid: 730,
    },
    CidRange {
        start: 2392958958,
        end: 2392958958,
        cid: 6026,
    },
    CidRange {
        start: 2392958959,
        end: 2392958962,
        cid: 737,
    },
    CidRange {
        start: 2392958963,
        end: 2392958963,
        cid: 6028,
    },
    CidRange {
        start: 2392958964,
        end: 2392958968,
        cid: 741,
    },
    CidRange {
        start: 2392958969,
        end: 2392958969,
        cid: 6029,
    },
    CidRange {
        start: 2392958970,
        end: 2392958973,
        cid: 746,
    },
    CidRange {
        start: 2392958974,
        end: 2392958974,
        cid: 854,
    },
    CidRange {
        start: 2392959137,
        end: 2392959142,
        cid: 855,
    },
    CidRange {
        start: 2392959143,
        end: 2392959143,
        cid: 862,
    },
    CidRange {
        start: 2392959144,
        end: 2392959144,
        cid: 866,
    },
    CidRange {
        start: 2392959145,
        end: 2392959146,
        cid: 558,
    },
    CidRange {
        start: 2392959147,
        end: 2392959154,
        cid: 867,
    },
    CidRange {
        start: 2392959155,
        end: 2392959155,
        cid: 6066,
    },
    CidRange {
        start: 2392959156,
        end: 2392959158,
        cid: 875,
    },
    CidRange {
        start: 2392959159,
        end: 2392959162,
        cid: 1014,
    },
    CidRange {
        start: 2392959163,
        end: 2392959163,
        cid: 6162,
    },
    CidRange {
        start: 2392959164,
        end: 2392959166,
        cid: 1018,
    },
    CidRange {
        start: 2392959167,
        end: 2392959171,
        cid: 1022,
    },
    CidRange {
        start: 2392959172,
        end: 2392959180,
        cid: 1029,
    },
    CidRange {
        start: 2392959181,
        end: 2392959181,
        cid: 6163,
    },
    CidRange {
        start: 2392959182,
        end: 2392959182,
        cid: 6168,
    },
    CidRange {
        start: 2392959183,
        end: 2392959186,
        cid: 1039,
    },
    CidRange {
        start: 2392959187,
        end: 2392959187,
        cid: 6169,
    },
    CidRange {
        start: 2392959188,
        end: 2392959193,
        cid: 1288,
    },
    CidRange {
        start: 2392959194,
        end: 2392959194,
        cid: 6375,
    },
    CidRange {
        start: 2392959195,
        end: 2392959202,
        cid: 1294,
    },
    CidRange {
        start: 2392959203,
        end: 2392959203,
        cid: 560,
    },
    CidRange {
        start: 2392959204,
        end: 2392959204,
        cid: 1307,
    },
    CidRange {
        start: 2392959205,
        end: 2392959207,
        cid: 1312,
    },
    CidRange {
        start: 2392959208,
        end: 2392959211,
        cid: 1686,
    },
    CidRange {
        start: 2392959212,
        end: 2392959212,
        cid: 561,
    },
    CidRange {
        start: 2392959213,
        end: 2392959216,
        cid: 1695,
    },
    CidRange {
        start: 2392959217,
        end: 2392959227,
        cid: 2086,
    },
    CidRange {
        start: 2392959228,
        end: 2392959230,
        cid: 2549,
    },
    CidRange {
        start: 2392959393,
        end: 2392959393,
        cid: 7731,
    },
    CidRange {
        start: 2392959394,
        end: 2392959394,
        cid: 2552,
    },
    CidRange {
        start: 2392959395,
        end: 2392959395,
        cid: 7732,
    },
    CidRange {
        start: 2392959396,
        end: 2392959397,
        cid: 2553,
    },
    CidRange {
        start: 2392959398,
        end: 2392959403,
        cid: 3041,
    },
    CidRange {
        start: 2392959404,
        end: 2392959406,
        cid: 3515,
    },
    CidRange {
        start: 2392959407,
        end: 2392959407,
        cid: 9056,
    },
    CidRange {
        start: 2392959408,
        end: 2392959408,
        cid: 9746,
    },
    CidRange {
        start: 2392959409,
        end: 2392959411,
        cid: 3963,
    },
    CidRange {
        start: 2392959412,
        end: 2392959413,
        cid: 4352,
    },
    CidRange {
        start: 2392959414,
        end: 2392959414,
        cid: 4745,
    },
    CidRange {
        start: 2392959415,
        end: 2392959416,
        cid: 5042,
    },
    CidRange {
        start: 2392959417,
        end: 2392959417,
        cid: 12045,
    },
    CidRange {
        start: 2392965793,
        end: 2392965825,
        cid: 562,
    },
    CidRange {
        start: 2392966305,
        end: 2392966398,
        cid: 595,
    },
    CidRange {
        start: 2392966561,
        end: 2392966654,
        cid: 689,
    },
    CidRange {
        start: 2392966817,
        end: 2392966910,
        cid: 783,
    },
    CidRange {
        start: 2392967073,
        end: 2392967166,
        cid: 877,
    },
    CidRange {
        start: 2392967329,
        end: 2392967422,
        cid: 971,
    },
    CidRange {
        start: 2392967585,
        end: 2392967678,
        cid: 1065,
    },
    CidRange {
        start: 2392967841,
        end: 2392967934,
        cid: 1159,
    },
    CidRange {
        start: 2392968097,
        end: 2392968190,
        cid: 1253,
    },
    CidRange {
        start: 2392968353,
        end: 2392968446,
        cid: 1347,
    },
    CidRange {
        start: 2392968609,
        end: 2392968702,
        cid: 1441,
    },
    CidRange {
        start: 2392968865,
        end: 2392968958,
        cid: 1535,
    },
    CidRange {
        start: 2392969121,
        end: 2392969214,
        cid: 1629,
    },
    CidRange {
        start: 2392969377,
        end: 2392969470,
        cid: 1723,
    },
    CidRange {
        start: 2392969633,
        end: 2392969726,
        cid: 1817,
    },
    CidRange {
        start: 2392969889,
        end: 2392969982,
        cid: 1911,
    },
    CidRange {
        start: 2392970145,
        end: 2392970238,
        cid: 2005,
    },
    CidRange {
        start: 2392970401,
        end: 2392970494,
        cid: 2099,
    },
    CidRange {
        start: 2392970657,
        end: 2392970750,
        cid: 2193,
    },
    CidRange {
        start: 2392970913,
        end: 2392971006,
        cid: 2287,
    },
    CidRange {
        start: 2392971169,
        end: 2392971262,
        cid: 2381,
    },
    CidRange {
        start: 2392971425,
        end: 2392971518,
        cid: 2475,
    },
    CidRange {
        start: 2392971681,
        end: 2392971774,
        cid: 2569,
    },
    CidRange {
        start: 2392971937,
        end: 2392972030,
        cid: 2663,
    },
    CidRange {
        start: 2392972193,
        end: 2392972286,
        cid: 2757,
    },
    CidRange {
        start: 2392972449,
        end: 2392972542,
        cid: 2851,
    },
    CidRange {
        start: 2392972705,
        end: 2392972798,
        cid: 2945,
    },
    CidRange {
        start: 2392972961,
        end: 2392973054,
        cid: 3039,
    },
    CidRange {
        start: 2392973217,
        end: 2392973310,
        cid: 3133,
    },
    CidRange {
        start: 2392973473,
        end: 2392973566,
        cid: 3227,
    },
    CidRange {
        start: 2392973729,
        end: 2392973822,
        cid: 3321,
    },
    CidRange {
        start: 2392973985,
        end: 2392974078,
        cid: 3415,
    },
    CidRange {
        start: 2392974241,
        end: 2392974334,
        cid: 3509,
    },
    CidRange {
        start: 2392974497,
        end: 2392974590,
        cid: 3603,
    },
    CidRange {
        start: 2392974753,
        end: 2392974846,
        cid: 3697,
    },
    CidRange {
        start: 2392975009,
        end: 2392975102,
        cid: 3791,
    },
    CidRange {
        start: 2392975265,
        end: 2392975358,
        cid: 3885,
    },
    CidRange {
        start: 2392975521,
        end: 2392975614,
        cid: 3979,
    },
    CidRange {
        start: 2392975777,
        end: 2392975870,
        cid: 4073,
    },
    CidRange {
        start: 2392976033,
        end: 2392976126,
        cid: 4167,
    },
    CidRange {
        start: 2392976289,
        end: 2392976382,
        cid: 4261,
    },
    CidRange {
        start: 2392976545,
        end: 2392976638,
        cid: 4355,
    },
    CidRange {
        start: 2392976801,
        end: 2392976894,
        cid: 4449,
    },
    CidRange {
        start: 2392977057,
        end: 2392977150,
        cid: 4543,
    },
    CidRange {
        start: 2392977313,
        end: 2392977406,
        cid: 4637,
    },
    CidRange {
        start: 2392977569,
        end: 2392977662,
        cid: 4731,
    },
    CidRange {
        start: 2392977825,
        end: 2392977918,
        cid: 4825,
    },
    CidRange {
        start: 2392978081,
        end: 2392978174,
        cid: 4919,
    },
    CidRange {
        start: 2392978337,
        end: 2392978430,
        cid: 5013,
    },
    CidRange {
        start: 2392978593,
        end: 2392978686,
        cid: 5107,
    },
    CidRange {
        start: 2392978849,
        end: 2392978942,
        cid: 5201,
    },
    CidRange {
        start: 2392979105,
        end: 2392979198,
        cid: 5295,
    },
    CidRange {
        start: 2392979361,
        end: 2392979454,
        cid: 5389,
    },
    CidRange {
        start: 2392979617,
        end: 2392979710,
        cid: 5483,
    },
    CidRange {
        start: 2392979873,
        end: 2392979966,
        cid: 5577,
    },
    CidRange {
        start: 2392980129,
        end: 2392980222,
        cid: 5671,
    },
    CidRange {
        start: 2392980385,
        end: 2392980478,
        cid: 5765,
    },
    CidRange {
        start: 2392980641,
        end: 2392980734,
        cid: 5859,
    },
    CidRange {
        start: 2392980897,
        end: 2392980939,
        cid: 5953,
    },
    CidRange {
        start: 2393022881,
        end: 2393022974,
        cid: 5996,
    },
    CidRange {
        start: 2393023137,
        end: 2393023230,
        cid: 6090,
    },
    CidRange {
        start: 2393023393,
        end: 2393023486,
        cid: 6184,
    },
    CidRange {
        start: 2393023649,
        end: 2393023742,
        cid: 6278,
    },
    CidRange {
        start: 2393023905,
        end: 2393023998,
        cid: 6372,
    },
    CidRange {
        start: 2393024161,
        end: 2393024254,
        cid: 6466,
    },
    CidRange {
        start: 2393024417,
        end: 2393024510,
        cid: 6560,
    },
    CidRange {
        start: 2393024673,
        end: 2393024766,
        cid: 6654,
    },
    CidRange {
        start: 2393024929,
        end: 2393025022,
        cid: 6748,
    },
    CidRange {
        start: 2393025185,
        end: 2393025278,
        cid: 6842,
    },
    CidRange {
        start: 2393025441,
        end: 2393025534,
        cid: 6936,
    },
    CidRange {
        start: 2393025697,
        end: 2393025790,
        cid: 7030,
    },
    CidRange {
        start: 2393025953,
        end: 2393026046,
        cid: 7124,
    },
    CidRange {
        start: 2393026209,
        end: 2393026302,
        cid: 7218,
    },
    CidRange {
        start: 2393026465,
        end: 2393026558,
        cid: 7312,
    },
    CidRange {
        start: 2393026721,
        end: 2393026814,
        cid: 7406,
    },
    CidRange {
        start: 2393026977,
        end: 2393027070,
        cid: 7500,
    },
    CidRange {
        start: 2393027233,
        end: 2393027326,
        cid: 7594,
    },
    CidRange {
        start: 2393027489,
        end: 2393027582,
        cid: 7688,
    },
    CidRange {
        start: 2393027745,
        end: 2393027838,
        cid: 7782,
    },
    CidRange {
        start: 2393028001,
        end: 2393028094,
        cid: 7876,
    },
    CidRange {
        start: 2393028257,
        end: 2393028350,
        cid: 7970,
    },
    CidRange {
        start: 2393028513,
        end: 2393028606,
        cid: 8064,
    },
    CidRange {
        start: 2393028769,
        end: 2393028862,
        cid: 8158,
    },
    CidRange {
        start: 2393029025,
        end: 2393029118,
        cid: 8252,
    },
    CidRange {
        start: 2393029281,
        end: 2393029374,
        cid: 8346,
    },
    CidRange {
        start: 2393029537,
        end: 2393029630,
        cid: 8440,
    },
    CidRange {
        start: 2393029793,
        end: 2393029886,
        cid: 8534,
    },
    CidRange {
        start: 2393030049,
        end: 2393030142,
        cid: 8628,
    },
    CidRange {
        start: 2393030305,
        end: 2393030398,
        cid: 8722,
    },
    CidRange {
        start: 2393030561,
        end: 2393030654,
        cid: 8816,
    },
    CidRange {
        start: 2393030817,
        end: 2393030910,
        cid: 8910,
    },
    CidRange {
        start: 2393031073,
        end: 2393031166,
        cid: 9004,
    },
    CidRange {
        start: 2393031329,
        end: 2393031422,
        cid: 9098,
    },
    CidRange {
        start: 2393031585,
        end: 2393031678,
        cid: 9192,
    },
    CidRange {
        start: 2393031841,
        end: 2393031934,
        cid: 9286,
    },
    CidRange {
        start: 2393032097,
        end: 2393032190,
        cid: 9380,
    },
    CidRange {
        start: 2393032353,
        end: 2393032446,
        cid: 9474,
    },
    CidRange {
        start: 2393032609,
        end: 2393032702,
        cid: 9568,
    },
    CidRange {
        start: 2393032865,
        end: 2393032958,
        cid: 9662,
    },
    CidRange {
        start: 2393033121,
        end: 2393033214,
        cid: 9756,
    },
    CidRange {
        start: 2393033377,
        end: 2393033470,
        cid: 9850,
    },
    CidRange {
        start: 2393033633,
        end: 2393033726,
        cid: 9944,
    },
    CidRange {
        start: 2393033889,
        end: 2393033982,
        cid: 10038,
    },
    CidRange {
        start: 2393034145,
        end: 2393034238,
        cid: 10132,
    },
    CidRange {
        start: 2393034401,
        end: 2393034494,
        cid: 10226,
    },
    CidRange {
        start: 2393034657,
        end: 2393034750,
        cid: 10320,
    },
    CidRange {
        start: 2393034913,
        end: 2393035006,
        cid: 10414,
    },
    CidRange {
        start: 2393035169,
        end: 2393035262,
        cid: 10508,
    },
    CidRange {
        start: 2393035425,
        end: 2393035518,
        cid: 10602,
    },
    CidRange {
        start: 2393035681,
        end: 2393035774,
        cid: 10696,
    },
    CidRange {
        start: 2393035937,
        end: 2393036030,
        cid: 10790,
    },
    CidRange {
        start: 2393036193,
        end: 2393036286,
        cid: 10884,
    },
    CidRange {
        start: 2393036449,
        end: 2393036542,
        cid: 10978,
    },
    CidRange {
        start: 2393036705,
        end: 2393036798,
        cid: 11072,
    },
    CidRange {
        start: 2393036961,
        end: 2393037054,
        cid: 11166,
    },
    CidRange {
        start: 2393037217,
        end: 2393037310,
        cid: 11260,
    },
    CidRange {
        start: 2393037473,
        end: 2393037566,
        cid: 11354,
    },
    CidRange {
        start: 2393037729,
        end: 2393037822,
        cid: 11448,
    },
    CidRange {
        start: 2393037985,
        end: 2393038078,
        cid: 11542,
    },
    CidRange {
        start: 2393038241,
        end: 2393038334,
        cid: 11636,
    },
    CidRange {
        start: 2393038497,
        end: 2393038590,
        cid: 11730,
    },
    CidRange {
        start: 2393038753,
        end: 2393038846,
        cid: 11824,
    },
    CidRange {
        start: 2393039009,
        end: 2393039102,
        cid: 11918,
    },
    CidRange {
        start: 2393039265,
        end: 2393039358,
        cid: 12012,
    },
    CidRange {
        start: 2393039521,
        end: 2393039614,
        cid: 12106,
    },
    CidRange {
        start: 2393039777,
        end: 2393039870,
        cid: 12200,
    },
    CidRange {
        start: 2393040033,
        end: 2393040126,
        cid: 12294,
    },
    CidRange {
        start: 2393040289,
        end: 2393040382,
        cid: 12388,
    },
    CidRange {
        start: 2393040545,
        end: 2393040638,
        cid: 12482,
    },
    CidRange {
        start: 2393040801,
        end: 2393040894,
        cid: 12576,
    },
    CidRange {
        start: 2393041057,
        end: 2393041150,
        cid: 12670,
    },
    CidRange {
        start: 2393041313,
        end: 2393041406,
        cid: 12764,
    },
    CidRange {
        start: 2393041569,
        end: 2393041662,
        cid: 12858,
    },
    CidRange {
        start: 2393041825,
        end: 2393041918,
        cid: 12952,
    },
    CidRange {
        start: 2393042081,
        end: 2393042174,
        cid: 13046,
    },
    CidRange {
        start: 2393042337,
        end: 2393042430,
        cid: 13140,
    },
    CidRange {
        start: 2393042593,
        end: 2393042686,
        cid: 13234,
    },
    CidRange {
        start: 2393042849,
        end: 2393042942,
        cid: 13328,
    },
    CidRange {
        start: 2393043105,
        end: 2393043198,
        cid: 13422,
    },
    CidRange {
        start: 2393043361,
        end: 2393043454,
        cid: 13516,
    },
    CidRange {
        start: 2393043617,
        end: 2393043652,
        cid: 13610,
    },
    CidRange {
        start: 41377,
        end: 41470,
        cid: 99,
    },
    CidRange {
        start: 41633,
        end: 41726,
        cid: 193,
    },
    CidRange {
        start: 41889,
        end: 41934,
        cid: 287,
    },
    CidRange {
        start: 42145,
        end: 42238,
        cid: 333,
    },
    CidRange {
        start: 42401,
        end: 42476,
        cid: 427,
    },
    CidRange {
        start: 42478,
        end: 42480,
        cid: 503,
    },
    CidRange {
        start: 42657,
        end: 42686,
        cid: 506,
    },
    CidRange {
        start: 42913,
        end: 42913,
        cid: 595,
    },
    CidRange {
        start: 42914,
        end: 42916,
        cid: 536,
    },
    CidRange {
        start: 42917,
        end: 42917,
        cid: 596,
    },
    CidRange {
        start: 42918,
        end: 42918,
        cid: 539,
    },
    CidRange {
        start: 42919,
        end: 42919,
        cid: 602,
    },
    CidRange {
        start: 42920,
        end: 42920,
        cid: 540,
    },
    CidRange {
        start: 42921,
        end: 42924,
        cid: 603,
    },
    CidRange {
        start: 42925,
        end: 42927,
        cid: 541,
    },
    CidRange {
        start: 42928,
        end: 42928,
        cid: 607,
    },
    CidRange {
        start: 42929,
        end: 42929,
        cid: 5998,
    },
    CidRange {
        start: 42930,
        end: 42930,
        cid: 608,
    },
    CidRange {
        start: 42931,
        end: 42931,
        cid: 610,
    },
    CidRange {
        start: 42932,
        end: 42932,
        cid: 544,
    },
    CidRange {
        start: 42933,
        end: 42933,
        cid: 611,
    },
    CidRange {
        start: 42934,
        end: 42934,
        cid: 5999,
    },
    CidRange {
        start: 42935,
        end: 42935,
        cid: 545,
    },
    CidRange {
        start: 42936,
        end: 42937,
        cid: 612,
    },
    CidRange {
        start: 42938,
        end: 42938,
        cid: 546,
    },
    CidRange {
        start: 42939,
        end: 42939,
        cid: 6000,
    },
    CidRange {
        start: 42940,
        end: 42940,
        cid: 547,
    },
    CidRange {
        start: 42941,
        end: 42941,
        cid: 614,
    },
    CidRange {
        start: 42942,
        end: 42942,
        cid: 633,
    },
    CidRange {
        start: 42943,
        end: 42943,
        cid: 6005,
    },
    CidRange {
        start: 42944,
        end: 42945,
        cid: 634,
    },
    CidRange {
        start: 42946,
        end: 42946,
        cid: 548,
    },
    CidRange {
        start: 42947,
        end: 42950,
        cid: 636,
    },
    CidRange {
        start: 42951,
        end: 42951,
        cid: 549,
    },
    CidRange {
        start: 42952,
        end: 42955,
        cid: 642,
    },
    CidRange {
        start: 42956,
        end: 42956,
        cid: 6006,
    },
    CidRange {
        start: 42957,
        end: 42957,
        cid: 646,
    },
    CidRange {
        start: 42958,
        end: 42958,
        cid: 550,
    },
    CidRange {
        start: 42959,
        end: 42960,
        cid: 648,
    },
    CidRange {
        start: 42961,
        end: 42962,
        cid: 652,
    },
    CidRange {
        start: 42963,
        end: 42965,
        cid: 551,
    },
    CidRange {
        start: 42966,
        end: 42968,
        cid: 654,
    },
    CidRange {
        start: 42969,
        end: 42970,
        cid: 554,
    },
    CidRange {
        start: 42971,
        end: 42971,
        cid: 6007,
    },
    CidRange {
        start: 42972,
        end: 42975,
        cid: 720,
    },
    CidRange {
        start: 42976,
        end: 42976,
        cid: 725,
    },
    CidRange {
        start: 42977,
        end: 42977,
        cid: 556,
    },
    CidRange {
        start: 42978,
        end: 42981,
        cid: 726,
    },
    CidRange {
        start: 42982,
        end: 42982,
        cid: 557,
    },
    CidRange {
        start: 42983,
        end: 42989,
        cid: 730,
    },
    CidRange {
        start: 42990,
        end: 42990,
        cid: 6026,
    },
    CidRange {
        start: 42991,
        end: 42994,
        cid: 737,
    },
    CidRange {
        start: 42995,
        end: 42995,
        cid: 6028,
    },
    CidRange {
        start: 42996,
        end: 43000,
        cid: 741,
    },
    CidRange {
        start: 43001,
        end: 43001,
        cid: 6029,
    },
    CidRange {
        start: 43002,
        end: 43005,
        cid: 746,
    },
    CidRange {
        start: 43006,
        end: 43006,
        cid: 854,
    },
    CidRange {
        start: 43169,
        end: 43174,
        cid: 855,
    },
    CidRange {
        start: 43175,
        end: 43175,
        cid: 862,
    },
    CidRange {
        start: 43176,
        end: 43176,
        cid: 866,
    },
    CidRange {
        start: 43177,
        end: 43178,
        cid: 558,
    },
    CidRange {
        start: 43179,
        end: 43186,
        cid: 867,
    },
    CidRange {
        start: 43187,
        end: 43187,
        cid: 6066,
    },
    CidRange {
        start: 43188,
        end: 43190,
        cid: 875,
    },
    CidRange {
        start: 43191,
        end: 43194,
        cid: 1014,
    },
    CidRange {
        start: 43195,
        end: 43195,
        cid: 6162,
    },
    CidRange {
        start: 43196,
        end: 43198,
        cid: 1018,
    },
    CidRange {
        start: 43199,
        end: 43203,
        cid: 1022,
    },
    CidRange {
        start: 43204,
        end: 43212,
        cid: 1029,
    },
    CidRange {
        start: 43213,
        end: 43213,
        cid: 6163,
    },
    CidRange {
        start: 43214,
        end: 43214,
        cid: 6168,
    },
    CidRange {
        start: 43215,
        end: 43218,
        cid: 1039,
    },
    CidRange {
        start: 43219,
        end: 43219,
        cid: 6169,
    },
    CidRange {
        start: 43220,
        end: 43225,
        cid: 1288,
    },
    CidRange {
        start: 43226,
        end: 43226,
        cid: 6375,
    },
    CidRange {
        start: 43227,
        end: 43234,
        cid: 1294,
    },
    CidRange {
        start: 43235,
        end: 43235,
        cid: 560,
    },
    CidRange {
        start: 43236,
        end: 43236,
        cid: 1307,
    },
    CidRange {
        start: 43237,
        end: 43239,
        cid: 1312,
    },
    CidRange {
        start: 43240,
        end: 43243,
        cid: 1686,
    },
    CidRange {
        start: 43244,
        end: 43244,
        cid: 561,
    },
    CidRange {
        start: 43245,
        end: 43248,
        cid: 1695,
    },
    CidRange {
        start: 43249,
        end: 43259,
        cid: 2086,
    },
    CidRange {
        start: 43260,
        end: 43262,
        cid: 2549,
    },
    CidRange {
        start: 43425,
        end: 43425,
        cid: 7731,
    },
    CidRange {
        start: 43426,
        end: 43426,
        cid: 2552,
    },
    CidRange {
        start: 43427,
        end: 43427,
        cid: 7732,
    },
    CidRange {
        start: 43428,
        end: 43429,
        cid: 2553,
    },
    CidRange {
        start: 43430,
        end: 43435,
        cid: 3041,
    },
    CidRange {
        start: 43436,
        end: 43438,
        cid: 3515,
    },
    CidRange {
        start: 43439,
        end: 43439,
        cid: 9056,
    },
    CidRange {
        start: 43440,
        end: 43440,
        cid: 9746,
    },
    CidRange {
        start: 43441,
        end: 43443,
        cid: 3963,
    },
    CidRange {
        start: 43444,
        end: 43445,
        cid: 4352,
    },
    CidRange {
        start: 43446,
        end: 43446,
        cid: 4745,
    },
    CidRange {
        start: 43447,
        end: 43448,
        cid: 5042,
    },
    CidRange {
        start: 43449,
        end: 43449,
        cid: 12045,
    },
    CidRange {
        start: 49825,
        end: 49857,
        cid: 562,
    },
    CidRange {
        start: 50337,
        end: 50430,
        cid: 595,
    },
    CidRange {
        start: 50593,
        end: 50686,
        cid: 689,
    },
    CidRange {
        start: 50849,
        end: 50942,
        cid: 783,
    },
    CidRange {
        start: 51105,
        end: 51198,
        cid: 877,
    },
    CidRange {
        start: 51361,
        end: 51454,
        cid: 971,
    },
    CidRange {
        start: 51617,
        end: 51710,
        cid: 1065,
    },
    CidRange {
        start: 51873,
        end: 51966,
        cid: 1159,
    },
    CidRange {
        start: 52129,
        end: 52222,
        cid: 1253,
    },
    CidRange {
        start: 52385,
        end: 52478,
        cid: 1347,
    },
    CidRange {
        start: 52641,
        end: 52734,
        cid: 1441,
    },
    CidRange {
        start: 52897,
        end: 52990,
        cid: 1535,
    },
    CidRange {
        start: 53153,
        end: 53246,
        cid: 1629,
    },
    CidRange {
        start: 53409,
        end: 53502,
        cid: 1723,
    },
    CidRange {
        start: 53665,
        end: 53758,
        cid: 1817,
    },
    CidRange {
        start: 53921,
        end: 54014,
        cid: 1911,
    },
    CidRange {
        start: 54177,
        end: 54270,
        cid: 2005,
    },
    CidRange {
        start: 54433,
        end: 54526,
        cid: 2099,
    },
    CidRange {
        start: 54689,
        end: 54782,
        cid: 2193,
    },
    CidRange {
        start: 54945,
        end: 55038,
        cid: 2287,
    },
    CidRange {
        start: 55201,
        end: 55294,
        cid: 2381,
    },
    CidRange {
        start: 55457,
        end: 55550,
        cid: 2475,
    },
    CidRange {
        start: 55713,
        end: 55806,
        cid: 2569,
    },
    CidRange {
        start: 55969,
        end: 56062,
        cid: 2663,
    },
    CidRange {
        start: 56225,
        end: 56318,
        cid: 2757,
    },
    CidRange {
        start: 56481,
        end: 56574,
        cid: 2851,
    },
    CidRange {
        start: 56737,
        end: 56830,
        cid: 2945,
    },
    CidRange {
        start: 56993,
        end: 57086,
        cid: 3039,
    },
    CidRange {
        start: 57249,
        end: 57342,
        cid: 3133,
    },
    CidRange {
        start: 57505,
        end: 57598,
        cid: 3227,
    },
    CidRange {
        start: 57761,
        end: 57854,
        cid: 3321,
    },
    CidRange {
        start: 58017,
        end: 58110,
        cid: 3415,
    },
    CidRange {
        start: 58273,
        end: 58366,
        cid: 3509,
    },
    CidRange {
        start: 58529,
        end: 58622,
        cid: 3603,
    },
    CidRange {
        start: 58785,
        end: 58878,
        cid: 3697,
    },
    CidRange {
        start: 59041,
        end: 59134,
        cid: 3791,
    },
    CidRange {
        start: 59297,
        end: 59390,
        cid: 3885,
    },
    CidRange {
        start: 59553,
        end: 59646,
        cid: 3979,
    },
    CidRange {
        start: 59809,
        end: 59902,
        cid: 4073,
    },
    CidRange {
        start: 60065,
        end: 60158,
        cid: 4167,
    },
    CidRange {
        start: 60321,
        end: 60414,
        cid: 4261,
    },
    CidRange {
        start: 60577,
        end: 60670,
        cid: 4355,
    },
    CidRange {
        start: 60833,
        end: 60926,
        cid: 4449,
    },
    CidRange {
        start: 61089,
        end: 61182,
        cid: 4543,
    },
    CidRange {
        start: 61345,
        end: 61438,
        cid: 4637,
    },
    CidRange {
        start: 61601,
        end: 61694,
        cid: 4731,
    },
    CidRange {
        start: 61857,
        end: 61950,
        cid: 4825,
    },
    CidRange {
        start: 62113,
        end: 62206,
        cid: 4919,
    },
    CidRange {
        start: 62369,
        end: 62462,
        cid: 5013,
    },
    CidRange {
        start: 62625,
        end: 62718,
        cid: 5107,
    },
    CidRange {
        start: 62881,
        end: 62974,
        cid: 5201,
    },
    CidRange {
        start: 63137,
        end: 63230,
        cid: 5295,
    },
    CidRange {
        start: 63393,
        end: 63486,
        cid: 5389,
    },
    CidRange {
        start: 63649,
        end: 63742,
        cid: 5483,
    },
    CidRange {
        start: 63905,
        end: 63998,
        cid: 5577,
    },
    CidRange {
        start: 64161,
        end: 64254,
        cid: 5671,
    },
    CidRange {
        start: 64417,
        end: 64510,
        cid: 5765,
    },
    CidRange {
        start: 64673,
        end: 64766,
        cid: 5859,
    },
    CidRange {
        start: 64929,
        end: 64971,
        cid: 5953,
    },
];

const CID_RANGE_V: [CidRange; 441] = [
    CidRange {
        start: 32,
        end: 126,
        cid: 13648,
    },
    CidRange {
        start: 2392957345,
        end: 2392957355,
        cid: 99,
    },
    CidRange {
        start: 2392957356,
        end: 2392957356,
        cid: 13646,
    },
    CidRange {
        start: 2392957357,
        end: 2392957370,
        cid: 111,
    },
    CidRange {
        start: 2392957371,
        end: 2392957371,
        cid: 124,
    },
    CidRange {
        start: 2392957372,
        end: 2392957372,
        cid: 126,
    },
    CidRange {
        start: 2392957373,
        end: 2392957373,
        cid: 126,
    },
    CidRange {
        start: 2392957374,
        end: 2392957375,
        cid: 130,
    },
    CidRange {
        start: 2392957376,
        end: 2392957377,
        cid: 130,
    },
    CidRange {
        start: 2392957378,
        end: 2392957379,
        cid: 134,
    },
    CidRange {
        start: 2392957380,
        end: 2392957381,
        cid: 134,
    },
    CidRange {
        start: 2392957382,
        end: 2392957383,
        cid: 138,
    },
    CidRange {
        start: 2392957384,
        end: 2392957385,
        cid: 138,
    },
    CidRange {
        start: 2392957386,
        end: 2392957387,
        cid: 142,
    },
    CidRange {
        start: 2392957388,
        end: 2392957389,
        cid: 142,
    },
    CidRange {
        start: 2392957390,
        end: 2392957391,
        cid: 146,
    },
    CidRange {
        start: 2392957392,
        end: 2392957393,
        cid: 146,
    },
    CidRange {
        start: 2392957394,
        end: 2392957395,
        cid: 150,
    },
    CidRange {
        start: 2392957396,
        end: 2392957397,
        cid: 150,
    },
    CidRange {
        start: 2392957398,
        end: 2392957399,
        cid: 154,
    },
    CidRange {
        start: 2392957400,
        end: 2392957401,
        cid: 154,
    },
    CidRange {
        start: 2392957402,
        end: 2392957403,
        cid: 158,
    },
    CidRange {
        start: 2392957404,
        end: 2392957438,
        cid: 158,
    },
    CidRange {
        start: 2392957601,
        end: 2392957635,
        cid: 193,
    },
    CidRange {
        start: 2392957636,
        end: 2392957636,
        cid: 13647,
    },
    CidRange {
        start: 2392957637,
        end: 2392957694,
        cid: 229,
    },
    CidRange {
        start: 2392957857,
        end: 2392957902,
        cid: 287,
    },
    CidRange {
        start: 2392958113,
        end: 2392958206,
        cid: 333,
    },
    CidRange {
        start: 2392958369,
        end: 2392958444,
        cid: 427,
    },
    CidRange {
        start: 2392958446,
        end: 2392958448,
        cid: 503,
    },
    CidRange {
        start: 2392958625,
        end: 2392958654,
        cid: 506,
    },
    CidRange {
        start: 2392958881,
        end: 2392958881,
        cid: 595,
    },
    CidRange {
        start: 2392958882,
        end: 2392958884,
        cid: 536,
    },
    CidRange {
        start: 2392958885,
        end: 2392958885,
        cid: 596,
    },
    CidRange {
        start: 2392958886,
        end: 2392958886,
        cid: 539,
    },
    CidRange {
        start: 2392958887,
        end: 2392958887,
        cid: 602,
    },
    CidRange {
        start: 2392958888,
        end: 2392958888,
        cid: 540,
    },
    CidRange {
        start: 2392958889,
        end: 2392958892,
        cid: 603,
    },
    CidRange {
        start: 2392958893,
        end: 2392958895,
        cid: 541,
    },
    CidRange {
        start: 2392958896,
        end: 2392958896,
        cid: 607,
    },
    CidRange {
        start: 2392958897,
        end: 2392958897,
        cid: 5998,
    },
    CidRange {
        start: 2392958898,
        end: 2392958898,
        cid: 608,
    },
    CidRange {
        start: 2392958899,
        end: 2392958899,
        cid: 610,
    },
    CidRange {
        start: 2392958900,
        end: 2392958900,
        cid: 544,
    },
    CidRange {
        start: 2392958901,
        end: 2392958901,
        cid: 611,
    },
    CidRange {
        start: 2392958902,
        end: 2392958902,
        cid: 5999,
    },
    CidRange {
        start: 2392958903,
        end: 2392958903,
        cid: 545,
    },
    CidRange {
        start: 2392958904,
        end: 2392958905,
        cid: 612,
    },
    CidRange {
        start: 2392958906,
        end: 2392958906,
        cid: 546,
    },
    CidRange {
        start: 2392958907,
        end: 2392958907,
        cid: 6000,
    },
    CidRange {
        start: 2392958908,
        end: 2392958908,
        cid: 547,
    },
    CidRange {
        start: 2392958909,
        end: 2392958909,
        cid: 614,
    },
    CidRange {
        start: 2392958910,
        end: 2392958910,
        cid: 633,
    },
    CidRange {
        start: 2392958911,
        end: 2392958911,
        cid: 6005,
    },
    CidRange {
        start: 2392958912,
        end: 2392958913,
        cid: 634,
    },
    CidRange {
        start: 2392958914,
        end: 2392958914,
        cid: 548,
    },
    CidRange {
        start: 2392958915,
        end: 2392958918,
        cid: 636,
    },
    CidRange {
        start: 2392958919,
        end: 2392958919,
        cid: 549,
    },
    CidRange {
        start: 2392958920,
        end: 2392958923,
        cid: 642,
    },
    CidRange {
        start: 2392958924,
        end: 2392958924,
        cid: 6006,
    },
    CidRange {
        start: 2392958925,
        end: 2392958925,
        cid: 646,
    },
    CidRange {
        start: 2392958926,
        end: 2392958926,
        cid: 550,
    },
    CidRange {
        start: 2392958927,
        end: 2392958928,
        cid: 648,
    },
    CidRange {
        start: 2392958929,
        end: 2392958930,
        cid: 652,
    },
    CidRange {
        start: 2392958931,
        end: 2392958933,
        cid: 551,
    },
    CidRange {
        start: 2392958934,
        end: 2392958936,
        cid: 654,
    },
    CidRange {
        start: 2392958937,
        end: 2392958938,
        cid: 554,
    },
    CidRange {
        start: 2392958939,
        end: 2392958939,
        cid: 6007,
    },
    CidRange {
        start: 2392958940,
        end: 2392958943,
        cid: 720,
    },
    CidRange {
        start: 2392958944,
        end: 2392958944,
        cid: 725,
    },
    CidRange {
        start: 2392958945,
        end: 2392958945,
        cid: 556,
    },
    CidRange {
        start: 2392958946,
        end: 2392958949,
        cid: 726,
    },
    CidRange {
        start: 2392958950,
        end: 2392958950,
        cid: 557,
    },
    CidRange {
        start: 2392958951,
        end: 2392958957,
        cid: 730,
    },
    CidRange {
        start: 2392958958,
        end: 2392958958,
        cid: 6026,
    },
    CidRange {
        start: 2392958959,
        end: 2392958962,
        cid: 737,
    },
    CidRange {
        start: 2392958963,
        end: 2392958963,
        cid: 6028,
    },
    CidRange {
        start: 2392958964,
        end: 2392958968,
        cid: 741,
    },
    CidRange {
        start: 2392958969,
        end: 2392958969,
        cid: 6029,
    },
    CidRange {
        start: 2392958970,
        end: 2392958973,
        cid: 746,
    },
    CidRange {
        start: 2392958974,
        end: 2392958974,
        cid: 854,
    },
    CidRange {
        start: 2392959137,
        end: 2392959142,
        cid: 855,
    },
    CidRange {
        start: 2392959143,
        end: 2392959143,
        cid: 862,
    },
    CidRange {
        start: 2392959144,
        end: 2392959144,
        cid: 866,
    },
    CidRange {
        start: 2392959145,
        end: 2392959146,
        cid: 558,
    },
    CidRange {
        start: 2392959147,
        end: 2392959154,
        cid: 867,
    },
    CidRange {
        start: 2392959155,
        end: 2392959155,
        cid: 6066,
    },
    CidRange {
        start: 2392959156,
        end: 2392959158,
        cid: 875,
    },
    CidRange {
        start: 2392959159,
        end: 2392959162,
        cid: 1014,
    },
    CidRange {
        start: 2392959163,
        end: 2392959163,
        cid: 6162,
    },
    CidRange {
        start: 2392959164,
        end: 2392959166,
        cid: 1018,
    },
    CidRange {
        start: 2392959167,
        end: 2392959171,
        cid: 1022,
    },
    CidRange {
        start: 2392959172,
        end: 2392959180,
        cid: 1029,
    },
    CidRange {
        start: 2392959181,
        end: 2392959181,
        cid: 6163,
    },
    CidRange {
        start: 2392959182,
        end: 2392959182,
        cid: 6168,
    },
    CidRange {
        start: 2392959183,
        end: 2392959186,
        cid: 1039,
    },
    CidRange {
        start: 2392959187,
        end: 2392959187,
        cid: 6169,
    },
    CidRange {
        start: 2392959188,
        end: 2392959193,
        cid: 1288,
    },
    CidRange {
        start: 2392959194,
        end: 2392959194,
        cid: 6375,
    },
    CidRange {
        start: 2392959195,
        end: 2392959202,
        cid: 1294,
    },
    CidRange {
        start: 2392959203,
        end: 2392959203,
        cid: 560,
    },
    CidRange {
        start: 2392959204,
        end: 2392959204,
        cid: 1307,
    },
    CidRange {
        start: 2392959205,
        end: 2392959207,
        cid: 1312,
    },
    CidRange {
        start: 2392959208,
        end: 2392959211,
        cid: 1686,
    },
    CidRange {
        start: 2392959212,
        end: 2392959212,
        cid: 561,
    },
    CidRange {
        start: 2392959213,
        end: 2392959216,
        cid: 1695,
    },
    CidRange {
        start: 2392959217,
        end: 2392959227,
        cid: 2086,
    },
    CidRange {
        start: 2392959228,
        end: 2392959230,
        cid: 2549,
    },
    CidRange {
        start: 2392959393,
        end: 2392959393,
        cid: 7731,
    },
    CidRange {
        start: 2392959394,
        end: 2392959394,
        cid: 2552,
    },
    CidRange {
        start: 2392959395,
        end: 2392959395,
        cid: 7732,
    },
    CidRange {
        start: 2392959396,
        end: 2392959397,
        cid: 2553,
    },
    CidRange {
        start: 2392959398,
        end: 2392959403,
        cid: 3041,
    },
    CidRange {
        start: 2392959404,
        end: 2392959406,
        cid: 3515,
    },
    CidRange {
        start: 2392959407,
        end: 2392959407,
        cid: 9056,
    },
    CidRange {
        start: 2392959408,
        end: 2392959408,
        cid: 9746,
    },
    CidRange {
        start: 2392959409,
        end: 2392959411,
        cid: 3963,
    },
    CidRange {
        start: 2392959412,
        end: 2392959413,
        cid: 4352,
    },
    CidRange {
        start: 2392959414,
        end: 2392959414,
        cid: 4745,
    },
    CidRange {
        start: 2392959415,
        end: 2392959416,
        cid: 5042,
    },
    CidRange {
        start: 2392959417,
        end: 2392959417,
        cid: 12045,
    },
    CidRange {
        start: 2392965793,
        end: 2392965825,
        cid: 562,
    },
    CidRange {
        start: 2392966305,
        end: 2392966398,
        cid: 595,
    },
    CidRange {
        start: 2392966561,
        end: 2392966654,
        cid: 689,
    },
    CidRange {
        start: 2392966817,
        end: 2392966910,
        cid: 783,
    },
    CidRange {
        start: 2392967073,
        end: 2392967166,
        cid: 877,
    },
    CidRange {
        start: 2392967329,
        end: 2392967422,
        cid: 971,
    },
    CidRange {
        start: 2392967585,
        end: 2392967678,
        cid: 1065,
    },
    CidRange {
        start: 2392967841,
        end: 2392967934,
        cid: 1159,
    },
    CidRange {
        start: 2392968097,
        end: 2392968190,
        cid: 1253,
    },
    CidRange {
        start: 2392968353,
        end: 2392968446,
        cid: 1347,
    },
    CidRange {
        start: 2392968609,
        end: 2392968702,
        cid: 1441,
    },
    CidRange {
        start: 2392968865,
        end: 2392968958,
        cid: 1535,
    },
    CidRange {
        start: 2392969121,
        end: 2392969214,
        cid: 1629,
    },
    CidRange {
        start: 2392969377,
        end: 2392969470,
        cid: 1723,
    },
    CidRange {
        start: 2392969633,
        end: 2392969726,
        cid: 1817,
    },
    CidRange {
        start: 2392969889,
        end: 2392969982,
        cid: 1911,
    },
    CidRange {
        start: 2392970145,
        end: 2392970238,
        cid: 2005,
    },
    CidRange {
        start: 2392970401,
        end: 2392970494,
        cid: 2099,
    },
    CidRange {
        start: 2392970657,
        end: 2392970750,
        cid: 2193,
    },
    CidRange {
        start: 2392970913,
        end: 2392971006,
        cid: 2287,
    },
    CidRange {
        start: 2392971169,
        end: 2392971262,
        cid: 2381,
    },
    CidRange {
        start: 2392971425,
        end: 2392971518,
        cid: 2475,
    },
    CidRange {
        start: 2392971681,
        end: 2392971774,
        cid: 2569,
    },
    CidRange {
        start: 2392971937,
        end: 2392972030,
        cid: 2663,
    },
    CidRange {
        start: 2392972193,
        end: 2392972286,
        cid: 2757,
    },
    CidRange {
        start: 2392972449,
        end: 2392972542,
        cid: 2851,
    },
    CidRange {
        start: 2392972705,
        end: 2392972798,
        cid: 2945,
    },
    CidRange {
        start: 2392972961,
        end: 2392973054,
        cid: 3039,
    },
    CidRange {
        start: 2392973217,
        end: 2392973310,
        cid: 3133,
    },
    CidRange {
        start: 2392973473,
        end: 2392973566,
        cid: 3227,
    },
    CidRange {
        start: 2392973729,
        end: 2392973822,
        cid: 3321,
    },
    CidRange {
        start: 2392973985,
        end: 2392974078,
        cid: 3415,
    },
    CidRange {
        start: 2392974241,
        end: 2392974334,
        cid: 3509,
    },
    CidRange {
        start: 2392974497,
        end: 2392974590,
        cid: 3603,
    },
    CidRange {
        start: 2392974753,
        end: 2392974846,
        cid: 3697,
    },
    CidRange {
        start: 2392975009,
        end: 2392975102,
        cid: 3791,
    },
    CidRange {
        start: 2392975265,
        end: 2392975358,
        cid: 3885,
    },
    CidRange {
        start: 2392975521,
        end: 2392975614,
        cid: 3979,
    },
    CidRange {
        start: 2392975777,
        end: 2392975870,
        cid: 4073,
    },
    CidRange {
        start: 2392976033,
        end: 2392976126,
        cid: 4167,
    },
    CidRange {
        start: 2392976289,
        end: 2392976382,
        cid: 4261,
    },
    CidRange {
        start: 2392976545,
        end: 2392976638,
        cid: 4355,
    },
    CidRange {
        start: 2392976801,
        end: 2392976894,
        cid: 4449,
    },
    CidRange {
        start: 2392977057,
        end: 2392977150,
        cid: 4543,
    },
    CidRange {
        start: 2392977313,
        end: 2392977406,
        cid: 4637,
    },
    CidRange {
        start: 2392977569,
        end: 2392977662,
        cid: 4731,
    },
    CidRange {
        start: 2392977825,
        end: 2392977918,
        cid: 4825,
    },
    CidRange {
        start: 2392978081,
        end: 2392978174,
        cid: 4919,
    },
    CidRange {
        start: 2392978337,
        end: 2392978430,
        cid: 5013,
    },
    CidRange {
        start: 2392978593,
        end: 2392978686,
        cid: 5107,
    },
    CidRange {
        start: 2392978849,
        end: 2392978942,
        cid: 5201,
    },
    CidRange {
        start: 2392979105,
        end: 2392979198,
        cid: 5295,
    },
    CidRange {
        start: 2392979361,
        end: 2392979454,
        cid: 5389,
    },
    CidRange {
        start: 2392979617,
        end: 2392979710,
        cid: 5483,
    },
    CidRange {
        start: 2392979873,
        end: 2392979966,
        cid: 5577,
    },
    CidRange {
        start: 2392980129,
        end: 2392980222,
        cid: 5671,
    },
    CidRange {
        start: 2392980385,
        end: 2392980478,
        cid: 5765,
    },
    CidRange {
        start: 2392980641,
        end: 2392980734,
        cid: 5859,
    },
    CidRange {
        start: 2392980897,
        end: 2392980939,
        cid: 5953,
    },
    CidRange {
        start: 2393022881,
        end: 2393022974,
        cid: 5996,
    },
    CidRange {
        start: 2393023137,
        end: 2393023230,
        cid: 6090,
    },
    CidRange {
        start: 2393023393,
        end: 2393023486,
        cid: 6184,
    },
    CidRange {
        start: 2393023649,
        end: 2393023742,
        cid: 6278,
    },
    CidRange {
        start: 2393023905,
        end: 2393023998,
        cid: 6372,
    },
    CidRange {
        start: 2393024161,
        end: 2393024254,
        cid: 6466,
    },
    CidRange {
        start: 2393024417,
        end: 2393024510,
        cid: 6560,
    },
    CidRange {
        start: 2393024673,
        end: 2393024766,
        cid: 6654,
    },
    CidRange {
        start: 2393024929,
        end: 2393025022,
        cid: 6748,
    },
    CidRange {
        start: 2393025185,
        end: 2393025278,
        cid: 6842,
    },
    CidRange {
        start: 2393025441,
        end: 2393025534,
        cid: 6936,
    },
    CidRange {
        start: 2393025697,
        end: 2393025790,
        cid: 7030,
    },
    CidRange {
        start: 2393025953,
        end: 2393026046,
        cid: 7124,
    },
    CidRange {
        start: 2393026209,
        end: 2393026302,
        cid: 7218,
    },
    CidRange {
        start: 2393026465,
        end: 2393026558,
        cid: 7312,
    },
    CidRange {
        start: 2393026721,
        end: 2393026814,
        cid: 7406,
    },
    CidRange {
        start: 2393026977,
        end: 2393027070,
        cid: 7500,
    },
    CidRange {
        start: 2393027233,
        end: 2393027326,
        cid: 7594,
    },
    CidRange {
        start: 2393027489,
        end: 2393027582,
        cid: 7688,
    },
    CidRange {
        start: 2393027745,
        end: 2393027838,
        cid: 7782,
    },
    CidRange {
        start: 2393028001,
        end: 2393028094,
        cid: 7876,
    },
    CidRange {
        start: 2393028257,
        end: 2393028350,
        cid: 7970,
    },
    CidRange {
        start: 2393028513,
        end: 2393028606,
        cid: 8064,
    },
    CidRange {
        start: 2393028769,
        end: 2393028862,
        cid: 8158,
    },
    CidRange {
        start: 2393029025,
        end: 2393029118,
        cid: 8252,
    },
    CidRange {
        start: 2393029281,
        end: 2393029374,
        cid: 8346,
    },
    CidRange {
        start: 2393029537,
        end: 2393029630,
        cid: 8440,
    },
    CidRange {
        start: 2393029793,
        end: 2393029886,
        cid: 8534,
    },
    CidRange {
        start: 2393030049,
        end: 2393030142,
        cid: 8628,
    },
    CidRange {
        start: 2393030305,
        end: 2393030398,
        cid: 8722,
    },
    CidRange {
        start: 2393030561,
        end: 2393030654,
        cid: 8816,
    },
    CidRange {
        start: 2393030817,
        end: 2393030910,
        cid: 8910,
    },
    CidRange {
        start: 2393031073,
        end: 2393031166,
        cid: 9004,
    },
    CidRange {
        start: 2393031329,
        end: 2393031422,
        cid: 9098,
    },
    CidRange {
        start: 2393031585,
        end: 2393031678,
        cid: 9192,
    },
    CidRange {
        start: 2393031841,
        end: 2393031934,
        cid: 9286,
    },
    CidRange {
        start: 2393032097,
        end: 2393032190,
        cid: 9380,
    },
    CidRange {
        start: 2393032353,
        end: 2393032446,
        cid: 9474,
    },
    CidRange {
        start: 2393032609,
        end: 2393032702,
        cid: 9568,
    },
    CidRange {
        start: 2393032865,
        end: 2393032958,
        cid: 9662,
    },
    CidRange {
        start: 2393033121,
        end: 2393033214,
        cid: 9756,
    },
    CidRange {
        start: 2393033377,
        end: 2393033470,
        cid: 9850,
    },
    CidRange {
        start: 2393033633,
        end: 2393033726,
        cid: 9944,
    },
    CidRange {
        start: 2393033889,
        end: 2393033982,
        cid: 10038,
    },
    CidRange {
        start: 2393034145,
        end: 2393034238,
        cid: 10132,
    },
    CidRange {
        start: 2393034401,
        end: 2393034494,
        cid: 10226,
    },
    CidRange {
        start: 2393034657,
        end: 2393034750,
        cid: 10320,
    },
    CidRange {
        start: 2393034913,
        end: 2393035006,
        cid: 10414,
    },
    CidRange {
        start: 2393035169,
        end: 2393035262,
        cid: 10508,
    },
    CidRange {
        start: 2393035425,
        end: 2393035518,
        cid: 10602,
    },
    CidRange {
        start: 2393035681,
        end: 2393035774,
        cid: 10696,
    },
    CidRange {
        start: 2393035937,
        end: 2393036030,
        cid: 10790,
    },
    CidRange {
        start: 2393036193,
        end: 2393036286,
        cid: 10884,
    },
    CidRange {
        start: 2393036449,
        end: 2393036542,
        cid: 10978,
    },
    CidRange {
        start: 2393036705,
        end: 2393036798,
        cid: 11072,
    },
    CidRange {
        start: 2393036961,
        end: 2393037054,
        cid: 11166,
    },
    CidRange {
        start: 2393037217,
        end: 2393037310,
        cid: 11260,
    },
    CidRange {
        start: 2393037473,
        end: 2393037566,
        cid: 11354,
    },
    CidRange {
        start: 2393037729,
        end: 2393037822,
        cid: 11448,
    },
    CidRange {
        start: 2393037985,
        end: 2393038078,
        cid: 11542,
    },
    CidRange {
        start: 2393038241,
        end: 2393038334,
        cid: 11636,
    },
    CidRange {
        start: 2393038497,
        end: 2393038590,
        cid: 11730,
    },
    CidRange {
        start: 2393038753,
        end: 2393038846,
        cid: 11824,
    },
    CidRange {
        start: 2393039009,
        end: 2393039102,
        cid: 11918,
    },
    CidRange {
        start: 2393039265,
        end: 2393039358,
        cid: 12012,
    },
    CidRange {
        start: 2393039521,
        end: 2393039614,
        cid: 12106,
    },
    CidRange {
        start: 2393039777,
        end: 2393039870,
        cid: 12200,
    },
    CidRange {
        start: 2393040033,
        end: 2393040126,
        cid: 12294,
    },
    CidRange {
        start: 2393040289,
        end: 2393040382,
        cid: 12388,
    },
    CidRange {
        start: 2393040545,
        end: 2393040638,
        cid: 12482,
    },
    CidRange {
        start: 2393040801,
        end: 2393040894,
        cid: 12576,
    },
    CidRange {
        start: 2393041057,
        end: 2393041150,
        cid: 12670,
    },
    CidRange {
        start: 2393041313,
        end: 2393041406,
        cid: 12764,
    },
    CidRange {
        start: 2393041569,
        end: 2393041662,
        cid: 12858,
    },
    CidRange {
        start: 2393041825,
        end: 2393041918,
        cid: 12952,
    },
    CidRange {
        start: 2393042081,
        end: 2393042174,
        cid: 13046,
    },
    CidRange {
        start: 2393042337,
        end: 2393042430,
        cid: 13140,
    },
    CidRange {
        start: 2393042593,
        end: 2393042686,
        cid: 13234,
    },
    CidRange {
        start: 2393042849,
        end: 2393042942,
        cid: 13328,
    },
    CidRange {
        start: 2393043105,
        end: 2393043198,
        cid: 13422,
    },
    CidRange {
        start: 2393043361,
        end: 2393043454,
        cid: 13516,
    },
    CidRange {
        start: 2393043617,
        end: 2393043652,
        cid: 13610,
    },
    CidRange {
        start: 41377,
        end: 41387,
        cid: 99,
    },
    CidRange {
        start: 41388,
        end: 41388,
        cid: 13646,
    },
    CidRange {
        start: 41389,
        end: 41402,
        cid: 111,
    },
    CidRange {
        start: 41403,
        end: 41403,
        cid: 124,
    },
    CidRange {
        start: 41404,
        end: 41404,
        cid: 126,
    },
    CidRange {
        start: 41405,
        end: 41405,
        cid: 126,
    },
    CidRange {
        start: 41406,
        end: 41407,
        cid: 130,
    },
    CidRange {
        start: 41408,
        end: 41409,
        cid: 130,
    },
    CidRange {
        start: 41410,
        end: 41411,
        cid: 134,
    },
    CidRange {
        start: 41412,
        end: 41413,
        cid: 134,
    },
    CidRange {
        start: 41414,
        end: 41415,
        cid: 138,
    },
    CidRange {
        start: 41416,
        end: 41417,
        cid: 138,
    },
    CidRange {
        start: 41418,
        end: 41419,
        cid: 142,
    },
    CidRange {
        start: 41420,
        end: 41421,
        cid: 142,
    },
    CidRange {
        start: 41422,
        end: 41423,
        cid: 146,
    },
    CidRange {
        start: 41424,
        end: 41425,
        cid: 146,
    },
    CidRange {
        start: 41426,
        end: 41427,
        cid: 150,
    },
    CidRange {
        start: 41428,
        end: 41429,
        cid: 150,
    },
    CidRange {
        start: 41430,
        end: 41431,
        cid: 154,
    },
    CidRange {
        start: 41432,
        end: 41433,
        cid: 154,
    },
    CidRange {
        start: 41434,
        end: 41435,
        cid: 158,
    },
    CidRange {
        start: 41436,
        end: 41470,
        cid: 158,
    },
    CidRange {
        start: 41633,
        end: 41667,
        cid: 193,
    },
    CidRange {
        start: 41668,
        end: 41668,
        cid: 13647,
    },
    CidRange {
        start: 41669,
        end: 41726,
        cid: 229,
    },
    CidRange {
        start: 41889,
        end: 41934,
        cid: 287,
    },
    CidRange {
        start: 42145,
        end: 42238,
        cid: 333,
    },
    CidRange {
        start: 42401,
        end: 42476,
        cid: 427,
    },
    CidRange {
        start: 42478,
        end: 42480,
        cid: 503,
    },
    CidRange {
        start: 42657,
        end: 42686,
        cid: 506,
    },
    CidRange {
        start: 42913,
        end: 42913,
        cid: 595,
    },
    CidRange {
        start: 42914,
        end: 42916,
        cid: 536,
    },
    CidRange {
        start: 42917,
        end: 42917,
        cid: 596,
    },
    CidRange {
        start: 42918,
        end: 42918,
        cid: 539,
    },
    CidRange {
        start: 42919,
        end: 42919,
        cid: 602,
    },
    CidRange {
        start: 42920,
        end: 42920,
        cid: 540,
    },
    CidRange {
        start: 42921,
        end: 42924,
        cid: 603,
    },
    CidRange {
        start: 42925,
        end: 42927,
        cid: 541,
    },
    CidRange {
        start: 42928,
        end: 42928,
        cid: 607,
    },
    CidRange {
        start: 42929,
        end: 42929,
        cid: 5998,
    },
    CidRange {
        start: 42930,
        end: 42930,
        cid: 608,
    },
    CidRange {
        start: 42931,
        end: 42931,
        cid: 610,
    },
    CidRange {
        start: 42932,
        end: 42932,
        cid: 544,
    },
    CidRange {
        start: 42933,
        end: 42933,
        cid: 611,
    },
    CidRange {
        start: 42934,
        end: 42934,
        cid: 5999,
    },
    CidRange {
        start: 42935,
        end: 42935,
        cid: 545,
    },
    CidRange {
        start: 42936,
        end: 42937,
        cid: 612,
    },
    CidRange {
        start: 42938,
        end: 42938,
        cid: 546,
    },
    CidRange {
        start: 42939,
        end: 42939,
        cid: 6000,
    },
    CidRange {
        start: 42940,
        end: 42940,
        cid: 547,
    },
    CidRange {
        start: 42941,
        end: 42941,
        cid: 614,
    },
    CidRange {
        start: 42942,
        end: 42942,
        cid: 633,
    },
    CidRange {
        start: 42943,
        end: 42943,
        cid: 6005,
    },
    CidRange {
        start: 42944,
        end: 42945,
        cid: 634,
    },
    CidRange {
        start: 42946,
        end: 42946,
        cid: 548,
    },
    CidRange {
        start: 42947,
        end: 42950,
        cid: 636,
    },
    CidRange {
        start: 42951,
        end: 42951,
        cid: 549,
    },
    CidRange {
        start: 42952,
        end: 42955,
        cid: 642,
    },
    CidRange {
        start: 42956,
        end: 42956,
        cid: 6006,
    },
    CidRange {
        start: 42957,
        end: 42957,
        cid: 646,
    },
    CidRange {
        start: 42958,
        end: 42958,
        cid: 550,
    },
    CidRange {
        start: 42959,
        end: 42960,
        cid: 648,
    },
    CidRange {
        start: 42961,
        end: 42962,
        cid: 652,
    },
    CidRange {
        start: 42963,
        end: 42965,
        cid: 551,
    },
    CidRange {
        start: 42966,
        end: 42968,
        cid: 654,
    },
    CidRange {
        start: 42969,
        end: 42970,
        cid: 554,
    },
    CidRange {
        start: 42971,
        end: 42971,
        cid: 6007,
    },
    CidRange {
        start: 42972,
        end: 42975,
        cid: 720,
    },
    CidRange {
        start: 42976,
        end: 42976,
        cid: 725,
    },
    CidRange {
        start: 42977,
        end: 42977,
        cid: 556,
    },
    CidRange {
        start: 42978,
        end: 42981,
        cid: 726,
    },
    CidRange {
        start: 42982,
        end: 42982,
        cid: 557,
    },
    CidRange {
        start: 42983,
        end: 42989,
        cid: 730,
    },
    CidRange {
        start: 42990,
        end: 42990,
        cid: 6026,
    },
    CidRange {
        start: 42991,
        end: 42994,
        cid: 737,
    },
    CidRange {
        start: 42995,
        end: 42995,
        cid: 6028,
    },
    CidRange {
        start: 42996,
        end: 43000,
        cid: 741,
    },
    CidRange {
        start: 43001,
        end: 43001,
        cid: 6029,
    },
    CidRange {
        start: 43002,
        end: 43005,
        cid: 746,
    },
    CidRange {
        start: 43006,
        end: 43006,
        cid: 854,
    },
    CidRange {
        start: 43169,
        end: 43174,
        cid: 855,
    },
    CidRange {
        start: 43175,
        end: 43175,
        cid: 862,
    },
    CidRange {
        start: 43176,
        end: 43176,
        cid: 866,
    },
    CidRange {
        start: 43177,
        end: 43178,
        cid: 558,
    },
    CidRange {
        start: 43179,
        end: 43186,
        cid: 867,
    },
    CidRange {
        start: 43187,
        end: 43187,
        cid: 6066,
    },
    CidRange {
        start: 43188,
        end: 43190,
        cid: 875,
    },
    CidRange {
        start: 43191,
        end: 43194,
        cid: 1014,
    },
    CidRange {
        start: 43195,
        end: 43195,
        cid: 6162,
    },
    CidRange {
        start: 43196,
        end: 43198,
        cid: 1018,
    },
    CidRange {
        start: 43199,
        end: 43203,
        cid: 1022,
    },
    CidRange {
        start: 43204,
        end: 43212,
        cid: 1029,
    },
    CidRange {
        start: 43213,
        end: 43213,
        cid: 6163,
    },
    CidRange {
        start: 43214,
        end: 43214,
        cid: 6168,
    },
    CidRange {
        start: 43215,
        end: 43218,
        cid: 1039,
    },
    CidRange {
        start: 43219,
        end: 43219,
        cid: 6169,
    },
    CidRange {
        start: 43220,
        end: 43225,
        cid: 1288,
    },
    CidRange {
        start: 43226,
        end: 43226,
        cid: 6375,
    },
    CidRange {
        start: 43227,
        end: 43234,
        cid: 1294,
    },
    CidRange {
        start: 43235,
        end: 43235,
        cid: 560,
    },
    CidRange {
        start: 43236,
        end: 43236,
        cid: 1307,
    },
    CidRange {
        start: 43237,
        end: 43239,
        cid: 1312,
    },
    CidRange {
        start: 43240,
        end: 43243,
        cid: 1686,
    },
    CidRange {
        start: 43244,
        end: 43244,
        cid: 561,
    },
    CidRange {
        start: 43245,
        end: 43248,
        cid: 1695,
    },
    CidRange {
        start: 43249,
        end: 43259,
        cid: 2086,
    },
    CidRange {
        start: 43260,
        end: 43262,
        cid: 2549,
    },
    CidRange {
        start: 43425,
        end: 43425,
        cid: 7731,
    },
    CidRange {
        start: 43426,
        end: 43426,
        cid: 2552,
    },
    CidRange {
        start: 43427,
        end: 43427,
        cid: 7732,
    },
    CidRange {
        start: 43428,
        end: 43429,
        cid: 2553,
    },
    CidRange {
        start: 43430,
        end: 43435,
        cid: 3041,
    },
    CidRange {
        start: 43436,
        end: 43438,
        cid: 3515,
    },
    CidRange {
        start: 43439,
        end: 43439,
        cid: 9056,
    },
    CidRange {
        start: 43440,
        end: 43440,
        cid: 9746,
    },
    CidRange {
        start: 43441,
        end: 43443,
        cid: 3963,
    },
    CidRange {
        start: 43444,
        end: 43445,
        cid: 4352,
    },
    CidRange {
        start: 43446,
        end: 43446,
        cid: 4745,
    },
    CidRange {
        start: 43447,
        end: 43448,
        cid: 5042,
    },
    CidRange {
        start: 43449,
        end: 43449,
        cid: 12045,
    },
    CidRange {
        start: 49825,
        end: 49857,
        cid: 562,
    },
    CidRange {
        start: 50337,
        end: 50430,
        cid: 595,
    },
    CidRange {
        start: 50593,
        end: 50686,
        cid: 689,
    },
    CidRange {
        start: 50849,
        end: 50942,
        cid: 783,
    },
    CidRange {
        start: 51105,
        end: 51198,
        cid: 877,
    },
    CidRange {
        start: 51361,
        end: 51454,
        cid: 971,
    },
    CidRange {
        start: 51617,
        end: 51710,
        cid: 1065,
    },
    CidRange {
        start: 51873,
        end: 51966,
        cid: 1159,
    },
    CidRange {
        start: 52129,
        end: 52222,
        cid: 1253,
    },
    CidRange {
        start: 52385,
        end: 52478,
        cid: 1347,
    },
    CidRange {
        start: 52641,
        end: 52734,
        cid: 1441,
    },
    CidRange {
        start: 52897,
        end: 52990,
        cid: 1535,
    },
    CidRange {
        start: 53153,
        end: 53246,
        cid: 1629,
    },
    CidRange {
        start: 53409,
        end: 53502,
        cid: 1723,
    },
    CidRange {
        start: 53665,
        end: 53758,
        cid: 1817,
    },
    CidRange {
        start: 53921,
        end: 54014,
        cid: 1911,
    },
    CidRange {
        start: 54177,
        end: 54270,
        cid: 2005,
    },
    CidRange {
        start: 54433,
        end: 54526,
        cid: 2099,
    },
    CidRange {
        start: 54689,
        end: 54782,
        cid: 2193,
    },
    CidRange {
        start: 54945,
        end: 55038,
        cid: 2287,
    },
    CidRange {
        start: 55201,
        end: 55294,
        cid: 2381,
    },
    CidRange {
        start: 55457,
        end: 55550,
        cid: 2475,
    },
    CidRange {
        start: 55713,
        end: 55806,
        cid: 2569,
    },
    CidRange {
        start: 55969,
        end: 56062,
        cid: 2663,
    },
    CidRange {
        start: 56225,
        end: 56318,
        cid: 2757,
    },
    CidRange {
        start: 56481,
        end: 56574,
        cid: 2851,
    },
    CidRange {
        start: 56737,
        end: 56830,
        cid: 2945,
    },
    CidRange {
        start: 56993,
        end: 57086,
        cid: 3039,
    },
    CidRange {
        start: 57249,
        end: 57342,
        cid: 3133,
    },
    CidRange {
        start: 57505,
        end: 57598,
        cid: 3227,
    },
    CidRange {
        start: 57761,
        end: 57854,
        cid: 3321,
    },
    CidRange {
        start: 58017,
        end: 58110,
        cid: 3415,
    },
    CidRange {
        start: 58273,
        end: 58366,
        cid: 3509,
    },
    CidRange {
        start: 58529,
        end: 58622,
        cid: 3603,
    },
    CidRange {
        start: 58785,
        end: 58878,
        cid: 3697,
    },
    CidRange {
        start: 59041,
        end: 59134,
        cid: 3791,
    },
    CidRange {
        start: 59297,
        end: 59390,
        cid: 3885,
    },
    CidRange {
        start: 59553,
        end: 59646,
        cid: 3979,
    },
    CidRange {
        start: 59809,
        end: 59902,
        cid: 4073,
    },
    CidRange {
        start: 60065,
        end: 60158,
        cid: 4167,
    },
    CidRange {
        start: 60321,
        end: 60414,
        cid: 4261,
    },
    CidRange {
        start: 60577,
        end: 60670,
        cid: 4355,
    },
    CidRange {
        start: 60833,
        end: 60926,
        cid: 4449,
    },
    CidRange {
        start: 61089,
        end: 61182,
        cid: 4543,
    },
    CidRange {
        start: 61345,
        end: 61438,
        cid: 4637,
    },
    CidRange {
        start: 61601,
        end: 61694,
        cid: 4731,
    },
    CidRange {
        start: 61857,
        end: 61950,
        cid: 4825,
    },
    CidRange {
        start: 62113,
        end: 62206,
        cid: 4919,
    },
    CidRange {
        start: 62369,
        end: 62462,
        cid: 5013,
    },
    CidRange {
        start: 62625,
        end: 62718,
        cid: 5107,
    },
    CidRange {
        start: 62881,
        end: 62974,
        cid: 5201,
    },
    CidRange {
        start: 63137,
        end: 63230,
        cid: 5295,
    },
    CidRange {
        start: 63393,
        end: 63486,
        cid: 5389,
    },
    CidRange {
        start: 63649,
        end: 63742,
        cid: 5483,
    },
    CidRange {
        start: 63905,
        end: 63998,
        cid: 5577,
    },
    CidRange {
        start: 64161,
        end: 64254,
        cid: 5671,
    },
    CidRange {
        start: 64417,
        end: 64510,
        cid: 5765,
    },
    CidRange {
        start: 64673,
        end: 64766,
        cid: 5859,
    },
    CidRange {
        start: 64929,
        end: 64971,
        cid: 5953,
    },
];

pub const CNS_EUC_H: CMap = CMap {
    name: Cow::Borrowed(b"CNS-EUC-H"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(CNS_1),
        supplement: 0,
    },
    writing_mode: CMapWritingMode::Horizontal,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: NO_CID_CHARS,
    cid_range: Cow::Borrowed(&CID_RANGE_H),
    base_font_chars: NO_BASE_FONT_CHARS,
};

pub const CNS_EUC_V: CMap = CMap {
    name: Cow::Borrowed(b"CNS-EUC-V"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(CNS_1),
        supplement: 0,
    },
    writing_mode: CMapWritingMode::Vertical,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: NO_CID_CHARS,
    cid_range: Cow::Borrowed(&CID_RANGE_V),
    base_font_chars: NO_BASE_FONT_CHARS,
};
