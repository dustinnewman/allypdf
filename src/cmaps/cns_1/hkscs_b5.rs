use std::borrow::Cow;

use crate::cmaps::NO_CID_CHARS;
use crate::font::cmap::{CMap, CMapWritingMode, CidChar, CidRange, Codespace, CodespaceRange};
use crate::font::font::CidSystemInfo;

const CODE_SPACE: [CodespaceRange; 2] = [
    [0..=0, 0..=0, 0..=0, 0..=128],
    [0..=0, 0..=0, 135..=254, 64..=254],
];

const CID_CHARS_H: [CidChar; 545] = [
    CidChar {
        char: 35139,
        cid: 17718,
    },
    CidChar {
        char: 35148,
        cid: 17723,
    },
    CidChar {
        char: 35149,
        cid: 17347,
    },
    CidChar {
        char: 35153,
        cid: 17306,
    },
    CidChar {
        char: 35238,
        cid: 17314,
    },
    CidChar {
        char: 35243,
        cid: 17388,
    },
    CidChar {
        char: 35244,
        cid: 17777,
    },
    CidChar {
        char: 35245,
        cid: 17387,
    },
    CidChar {
        char: 35246,
        cid: 17778,
    },
    CidChar {
        char: 35279,
        cid: 17340,
    },
    CidChar {
        char: 35289,
        cid: 17308,
    },
    CidChar {
        char: 35290,
        cid: 17815,
    },
    CidChar {
        char: 35291,
        cid: 17310,
    },
    CidChar {
        char: 35292,
        cid: 17816,
    },
    CidChar {
        char: 35293,
        cid: 17311,
    },
    CidChar {
        char: 35297,
        cid: 17313,
    },
    CidChar {
        char: 35298,
        cid: 17820,
    },
    CidChar {
        char: 35299,
        cid: 17315,
    },
    CidChar {
        char: 35322,
        cid: 17321,
    },
    CidChar {
        char: 35392,
        cid: 17845,
    },
    CidChar {
        char: 35393,
        cid: 17161,
    },
    CidChar {
        char: 35405,
        cid: 17846,
    },
    CidChar {
        char: 35418,
        cid: 17847,
    },
    CidChar {
        char: 35422,
        cid: 17848,
    },
    CidChar {
        char: 35441,
        cid: 17849,
    },
    CidChar {
        char: 35446,
        cid: 17214,
    },
    CidChar {
        char: 35447,
        cid: 17850,
    },
    CidChar {
        char: 35450,
        cid: 17851,
    },
    CidChar {
        char: 35451,
        cid: 17219,
    },
    CidChar {
        char: 35452,
        cid: 17852,
    },
    CidChar {
        char: 35453,
        cid: 17221,
    },
    CidChar {
        char: 35454,
        cid: 17853,
    },
    CidChar {
        char: 35496,
        cid: 17854,
    },
    CidChar {
        char: 35510,
        cid: 17855,
    },
    CidChar {
        char: 35511,
        cid: 17245,
    },
    CidChar {
        char: 35512,
        cid: 17856,
    },
    CidChar {
        char: 35513,
        cid: 17247,
    },
    CidChar {
        char: 35532,
        cid: 17857,
    },
    CidChar {
        char: 35558,
        cid: 17860,
    },
    CidChar {
        char: 35559,
        cid: 17371,
    },
    CidChar {
        char: 35648,
        cid: 17883,
    },
    CidChar {
        char: 35653,
        cid: 17294,
    },
    CidChar {
        char: 35654,
        cid: 17886,
    },
    CidChar {
        char: 35655,
        cid: 17295,
    },
    CidChar {
        char: 35656,
        cid: 17887,
    },
    CidChar {
        char: 35657,
        cid: 17296,
    },
    CidChar {
        char: 35658,
        cid: 17888,
    },
    CidChar {
        char: 35659,
        cid: 17297,
    },
    CidChar {
        char: 35660,
        cid: 17889,
    },
    CidChar {
        char: 35672,
        cid: 17303,
    },
    CidChar {
        char: 35673,
        cid: 17896,
    },
    CidChar {
        char: 35674,
        cid: 17304,
    },
    CidChar {
        char: 35675,
        cid: 17348,
    },
    CidChar {
        char: 35681,
        cid: 17319,
    },
    CidChar {
        char: 35688,
        cid: 17324,
    },
    CidChar {
        char: 35938,
        cid: 19022,
    },
    CidChar {
        char: 36059,
        cid: 19023,
    },
    CidChar {
        char: 36060,
        cid: 18964,
    },
    CidChar {
        char: 36160,
        cid: 19057,
    },
    CidChar {
        char: 36194,
        cid: 17338,
    },
    CidChar {
        char: 36200,
        cid: 17339,
    },
    CidChar {
        char: 36201,
        cid: 17312,
    },
    CidChar {
        char: 36202,
        cid: 17341,
    },
    CidChar {
        char: 36206,
        cid: 17342,
    },
    CidChar {
        char: 36214,
        cid: 17343,
    },
    CidChar {
        char: 36218,
        cid: 17344,
    },
    CidChar {
        char: 36219,
        cid: 17981,
    },
    CidChar {
        char: 36220,
        cid: 17345,
    },
    CidChar {
        char: 36261,
        cid: 17346,
    },
    CidChar {
        char: 36264,
        cid: 17337,
    },
    CidChar {
        char: 36265,
        cid: 17325,
    },
    CidChar {
        char: 36278,
        cid: 17351,
    },
    CidChar {
        char: 36291,
        cid: 17352,
    },
    CidChar {
        char: 36346,
        cid: 17401,
    },
    CidChar {
        char: 36421,
        cid: 18072,
    },
    CidChar {
        char: 36457,
        cid: 4181,
    },
    CidChar {
        char: 36458,
        cid: 14164,
    },
    CidChar {
        char: 36463,
        cid: 11752,
    },
    CidChar {
        char: 36470,
        cid: 18075,
    },
    CidChar {
        char: 36475,
        cid: 18076,
    },
    CidChar {
        char: 36478,
        cid: 289,
    },
    CidChar {
        char: 36518,
        cid: 18077,
    },
    CidChar {
        char: 36523,
        cid: 4203,
    },
    CidChar {
        char: 36532,
        cid: 4902,
    },
    CidChar {
        char: 36536,
        cid: 18078,
    },
    CidChar {
        char: 36553,
        cid: 18079,
    },
    CidChar {
        char: 36557,
        cid: 1643,
    },
    CidChar {
        char: 36560,
        cid: 4910,
    },
    CidChar {
        char: 36581,
        cid: 18080,
    },
    CidChar {
        char: 36591,
        cid: 18081,
    },
    CidChar {
        char: 36598,
        cid: 18082,
    },
    CidChar {
        char: 36695,
        cid: 3381,
    },
    CidChar {
        char: 36696,
        cid: 14296,
    },
    CidChar {
        char: 36697,
        cid: 18083,
    },
    CidChar {
        char: 36703,
        cid: 18084,
    },
    CidChar {
        char: 36711,
        cid: 18085,
    },
    CidChar {
        char: 36712,
        cid: 14309,
    },
    CidChar {
        char: 36713,
        cid: 10178,
    },
    CidChar {
        char: 36718,
        cid: 8877,
    },
    CidChar {
        char: 36729,
        cid: 18086,
    },
    CidChar {
        char: 36784,
        cid: 18087,
    },
    CidChar {
        char: 36805,
        cid: 18088,
    },
    CidChar {
        char: 36806,
        cid: 14368,
    },
    CidChar {
        char: 36807,
        cid: 18089,
    },
    CidChar {
        char: 36810,
        cid: 18090,
    },
    CidChar {
        char: 36811,
        cid: 4940,
    },
    CidChar {
        char: 36812,
        cid: 16377,
    },
    CidChar {
        char: 36826,
        cid: 18091,
    },
    CidChar {
        char: 36835,
        cid: 18092,
    },
    CidChar {
        char: 36860,
        cid: 18093,
    },
    CidChar {
        char: 36861,
        cid: 14420,
    },
    CidChar {
        char: 36862,
        cid: 2510,
    },
    CidChar {
        char: 36949,
        cid: 18094,
    },
    CidChar {
        char: 36973,
        cid: 3015,
    },
    CidChar {
        char: 36974,
        cid: 14466,
    },
    CidChar {
        char: 36975,
        cid: 18098,
    },
    CidChar {
        char: 36986,
        cid: 14057,
    },
    CidChar {
        char: 37030,
        cid: 18099,
    },
    CidChar {
        char: 37048,
        cid: 18100,
    },
    CidChar {
        char: 37084,
        cid: 5009,
    },
    CidChar {
        char: 37105,
        cid: 5796,
    },
    CidChar {
        char: 37221,
        cid: 18101,
    },
    CidChar {
        char: 37230,
        cid: 18102,
    },
    CidChar {
        char: 37246,
        cid: 18103,
    },
    CidChar {
        char: 37281,
        cid: 14633,
    },
    CidChar {
        char: 37282,
        cid: 18104,
    },
    CidChar {
        char: 37311,
        cid: 12402,
    },
    CidChar {
        char: 37320,
        cid: 18105,
    },
    CidChar {
        char: 37444,
        cid: 14728,
    },
    CidChar {
        char: 37476,
        cid: 18106,
    },
    CidChar {
        char: 37485,
        cid: 18107,
    },
    CidChar {
        char: 37553,
        cid: 284,
    },
    CidChar {
        char: 37554,
        cid: 283,
    },
    CidChar {
        char: 37576,
        cid: 16300,
    },
    CidChar {
        char: 37585,
        cid: 10620,
    },
    CidChar {
        char: 37605,
        cid: 18108,
    },
    CidChar {
        char: 37618,
        cid: 18109,
    },
    CidChar {
        char: 37736,
        cid: 18110,
    },
    CidChar {
        char: 37802,
        cid: 18111,
    },
    CidChar {
        char: 37826,
        cid: 18112,
    },
    CidChar {
        char: 37861,
        cid: 18113,
    },
    CidChar {
        char: 37864,
        cid: 18114,
    },
    CidChar {
        char: 37867,
        cid: 18115,
    },
    CidChar {
        char: 37958,
        cid: 18116,
    },
    CidChar {
        char: 37959,
        cid: 7430,
    },
    CidChar {
        char: 38009,
        cid: 18117,
    },
    CidChar {
        char: 38090,
        cid: 10657,
    },
    CidChar {
        char: 38091,
        cid: 18118,
    },
    CidChar {
        char: 38221,
        cid: 18119,
    },
    CidChar {
        char: 38234,
        cid: 18120,
    },
    CidChar {
        char: 38239,
        cid: 18121,
    },
    CidChar {
        char: 38342,
        cid: 18122,
    },
    CidChar {
        char: 38361,
        cid: 6171,
    },
    CidChar {
        char: 38468,
        cid: 15919,
    },
    CidChar {
        char: 38481,
        cid: 18123,
    },
    CidChar {
        char: 38506,
        cid: 18124,
    },
    CidChar {
        char: 38612,
        cid: 18125,
    },
    CidChar {
        char: 38637,
        cid: 15478,
    },
    CidChar {
        char: 38652,
        cid: 11044,
    },
    CidChar {
        char: 39023,
        cid: 18128,
    },
    CidChar {
        char: 39031,
        cid: 15705,
    },
    CidChar {
        char: 39034,
        cid: 15706,
    },
    CidChar {
        char: 39075,
        cid: 15707,
    },
    CidChar {
        char: 39087,
        cid: 15708,
    },
    CidChar {
        char: 39092,
        cid: 17354,
    },
    CidChar {
        char: 39093,
        cid: 18154,
    },
    CidChar {
        char: 39094,
        cid: 15709,
    },
    CidChar {
        char: 39095,
        cid: 18155,
    },
    CidChar {
        char: 39096,
        cid: 17356,
    },
    CidChar {
        char: 39097,
        cid: 15710,
    },
    CidChar {
        char: 39098,
        cid: 18156,
    },
    CidChar {
        char: 39099,
        cid: 17402,
    },
    CidChar {
        char: 39100,
        cid: 18157,
    },
    CidChar {
        char: 39106,
        cid: 15713,
    },
    CidChar {
        char: 39107,
        cid: 18161,
    },
    CidChar {
        char: 39108,
        cid: 15714,
    },
    CidChar {
        char: 39109,
        cid: 18162,
    },
    CidChar {
        char: 39122,
        cid: 17357,
    },
    CidChar {
        char: 39130,
        cid: 18178,
    },
    CidChar {
        char: 39131,
        cid: 17361,
    },
    CidChar {
        char: 39135,
        cid: 17364,
    },
    CidChar {
        char: 39139,
        cid: 15717,
    },
    CidChar {
        char: 39143,
        cid: 15718,
    },
    CidChar {
        char: 39149,
        cid: 15719,
    },
    CidChar {
        char: 39152,
        cid: 15720,
    },
    CidChar {
        char: 39153,
        cid: 18195,
    },
    CidChar {
        char: 39154,
        cid: 15721,
    },
    CidChar {
        char: 39155,
        cid: 18196,
    },
    CidChar {
        char: 39164,
        cid: 15722,
    },
    CidChar {
        char: 39165,
        cid: 18203,
    },
    CidChar {
        char: 39166,
        cid: 17367,
    },
    CidChar {
        char: 39234,
        cid: 17404,
    },
    CidChar {
        char: 39235,
        cid: 15723,
    },
    CidChar {
        char: 39236,
        cid: 18206,
    },
    CidChar {
        char: 39237,
        cid: 15724,
    },
    CidChar {
        char: 39238,
        cid: 18207,
    },
    CidChar {
        char: 39239,
        cid: 17368,
    },
    CidChar {
        char: 39247,
        cid: 15725,
    },
    CidChar {
        char: 39252,
        cid: 17369,
    },
    CidChar {
        char: 39260,
        cid: 17370,
    },
    CidChar {
        char: 39268,
        cid: 17372,
    },
    CidChar {
        char: 39274,
        cid: 15726,
    },
    CidChar {
        char: 39278,
        cid: 15727,
    },
    CidChar {
        char: 39285,
        cid: 15728,
    },
    CidChar {
        char: 39288,
        cid: 15729,
    },
    CidChar {
        char: 39329,
        cid: 18255,
    },
    CidChar {
        char: 39330,
        cid: 15730,
    },
    CidChar {
        char: 39331,
        cid: 18256,
    },
    CidChar {
        char: 39332,
        cid: 17349,
    },
    CidChar {
        char: 39333,
        cid: 18257,
    },
    CidChar {
        char: 39334,
        cid: 17350,
    },
    CidChar {
        char: 39342,
        cid: 15731,
    },
    CidChar {
        char: 39346,
        cid: 17374,
    },
    CidChar {
        char: 39350,
        cid: 15732,
    },
    CidChar {
        char: 39354,
        cid: 15733,
    },
    CidChar {
        char: 39370,
        cid: 17376,
    },
    CidChar {
        char: 39373,
        cid: 17378,
    },
    CidChar {
        char: 39379,
        cid: 17379,
    },
    CidChar {
        char: 39382,
        cid: 17381,
    },
    CidChar {
        char: 39391,
        cid: 17375,
    },
    CidChar {
        char: 39394,
        cid: 15734,
    },
    CidChar {
        char: 39395,
        cid: 18308,
    },
    CidChar {
        char: 39396,
        cid: 17323,
    },
    CidChar {
        char: 39397,
        cid: 18309,
    },
    CidChar {
        char: 39398,
        cid: 17383,
    },
    CidChar {
        char: 39399,
        cid: 18310,
    },
    CidChar {
        char: 39400,
        cid: 17385,
    },
    CidChar {
        char: 39407,
        cid: 17405,
    },
    CidChar {
        char: 39412,
        cid: 15735,
    },
    CidChar {
        char: 39498,
        cid: 15736,
    },
    CidChar {
        char: 39499,
        cid: 18341,
    },
    CidChar {
        char: 39500,
        cid: 15737,
    },
    CidChar {
        char: 39513,
        cid: 15738,
    },
    CidChar {
        char: 39519,
        cid: 17327,
    },
    CidChar {
        char: 39520,
        cid: 18359,
    },
    CidChar {
        char: 39521,
        cid: 15739,
    },
    CidChar {
        char: 39526,
        cid: 17389,
    },
    CidChar {
        char: 39527,
        cid: 18364,
    },
    CidChar {
        char: 39528,
        cid: 15740,
    },
    CidChar {
        char: 39529,
        cid: 17390,
    },
    CidChar {
        char: 39530,
        cid: 18365,
    },
    CidChar {
        char: 39531,
        cid: 17407,
    },
    CidChar {
        char: 39539,
        cid: 15741,
    },
    CidChar {
        char: 39540,
        cid: 18373,
    },
    CidChar {
        char: 39541,
        cid: 17393,
    },
    CidChar {
        char: 39550,
        cid: 15742,
    },
    CidChar {
        char: 39587,
        cid: 17395,
    },
    CidChar {
        char: 39588,
        cid: 18384,
    },
    CidChar {
        char: 39589,
        cid: 17394,
    },
    CidChar {
        char: 39593,
        cid: 17400,
    },
    CidChar {
        char: 39594,
        cid: 17396,
    },
    CidChar {
        char: 39602,
        cid: 15743,
    },
    CidChar {
        char: 39607,
        cid: 15744,
    },
    CidChar {
        char: 39608,
        cid: 18399,
    },
    CidChar {
        char: 39609,
        cid: 15745,
    },
    CidChar {
        char: 39610,
        cid: 18400,
    },
    CidChar {
        char: 39611,
        cid: 15746,
    },
    CidChar {
        char: 39612,
        cid: 18401,
    },
    CidChar {
        char: 39613,
        cid: 17335,
    },
    CidChar {
        char: 39623,
        cid: 15747,
    },
    CidChar {
        char: 39632,
        cid: 15748,
    },
    CidChar {
        char: 39633,
        cid: 18419,
    },
    CidChar {
        char: 39634,
        cid: 15749,
    },
    CidChar {
        char: 39650,
        cid: 15753,
    },
    CidChar {
        char: 39651,
        cid: 18432,
    },
    CidChar {
        char: 39652,
        cid: 15754,
    },
    CidChar {
        char: 39656,
        cid: 15755,
    },
    CidChar {
        char: 39657,
        cid: 17328,
    },
    CidChar {
        char: 39662,
        cid: 17330,
    },
    CidChar {
        char: 39666,
        cid: 15756,
    },
    CidChar {
        char: 39670,
        cid: 15757,
    },
    CidChar {
        char: 39675,
        cid: 15758,
    },
    CidChar {
        char: 39750,
        cid: 15759,
    },
    CidChar {
        char: 39754,
        cid: 15760,
    },
    CidChar {
        char: 39764,
        cid: 15762,
    },
    CidChar {
        char: 39768,
        cid: 15763,
    },
    CidChar {
        char: 39769,
        cid: 18474,
    },
    CidChar {
        char: 39770,
        cid: 15764,
    },
    CidChar {
        char: 39771,
        cid: 18475,
    },
    CidChar {
        char: 39772,
        cid: 15765,
    },
    CidChar {
        char: 39773,
        cid: 18476,
    },
    CidChar {
        char: 39776,
        cid: 18477,
    },
    CidChar {
        char: 39798,
        cid: 12112,
    },
    CidChar {
        char: 39799,
        cid: 15773,
    },
    CidChar {
        char: 39800,
        cid: 5925,
    },
    CidChar {
        char: 39803,
        cid: 13037,
    },
    CidChar {
        char: 39804,
        cid: 15776,
    },
    CidChar {
        char: 39805,
        cid: 18496,
    },
    CidChar {
        char: 39806,
        cid: 15777,
    },
    CidChar {
        char: 39841,
        cid: 15778,
    },
    CidChar {
        char: 39842,
        cid: 18497,
    },
    CidChar {
        char: 39851,
        cid: 18500,
    },
    CidChar {
        char: 39852,
        cid: 15785,
    },
    CidChar {
        char: 39855,
        cid: 15786,
    },
    CidChar {
        char: 39870,
        cid: 15795,
    },
    CidChar {
        char: 39871,
        cid: 18509,
    },
    CidChar {
        char: 39878,
        cid: 10969,
    },
    CidChar {
        char: 39882,
        cid: 15803,
    },
    CidChar {
        char: 39883,
        cid: 18513,
    },
    CidChar {
        char: 39884,
        cid: 15804,
    },
    CidChar {
        char: 39885,
        cid: 18514,
    },
    CidChar {
        char: 39886,
        cid: 17360,
    },
    CidChar {
        char: 39887,
        cid: 18515,
    },
    CidChar {
        char: 39890,
        cid: 18516,
    },
    CidChar {
        char: 39891,
        cid: 15807,
    },
    CidChar {
        char: 39892,
        cid: 18517,
    },
    CidChar {
        char: 39893,
        cid: 15808,
    },
    CidChar {
        char: 39901,
        cid: 15812,
    },
    CidChar {
        char: 39902,
        cid: 7188,
    },
    CidChar {
        char: 39903,
        cid: 15813,
    },
    CidChar {
        char: 39904,
        cid: 18522,
    },
    CidChar {
        char: 39905,
        cid: 15814,
    },
    CidChar {
        char: 39906,
        cid: 18523,
    },
    CidChar {
        char: 39907,
        cid: 15815,
    },
    CidChar {
        char: 39911,
        cid: 15816,
    },
    CidChar {
        char: 39912,
        cid: 18527,
    },
    CidChar {
        char: 39916,
        cid: 16890,
    },
    CidChar {
        char: 39917,
        cid: 18528,
    },
    CidChar {
        char: 39923,
        cid: 15823,
    },
    CidChar {
        char: 39926,
        cid: 4841,
    },
    CidChar {
        char: 39927,
        cid: 18534,
    },
    CidChar {
        char: 39930,
        cid: 18535,
    },
    CidChar {
        char: 40002,
        cid: 11438,
    },
    CidChar {
        char: 40003,
        cid: 18538,
    },
    CidChar {
        char: 40007,
        cid: 18539,
    },
    CidChar {
        char: 40008,
        cid: 15835,
    },
    CidChar {
        char: 40009,
        cid: 18540,
    },
    CidChar {
        char: 40010,
        cid: 15836,
    },
    CidChar {
        char: 40019,
        cid: 6756,
    },
    CidChar {
        char: 40020,
        cid: 18543,
    },
    CidChar {
        char: 40021,
        cid: 15844,
    },
    CidChar {
        char: 40022,
        cid: 18544,
    },
    CidChar {
        char: 40028,
        cid: 18545,
    },
    CidChar {
        char: 40029,
        cid: 15850,
    },
    CidChar {
        char: 40032,
        cid: 15851,
    },
    CidChar {
        char: 40033,
        cid: 18548,
    },
    CidChar {
        char: 40034,
        cid: 5124,
    },
    CidChar {
        char: 40035,
        cid: 18549,
    },
    CidChar {
        char: 40039,
        cid: 18550,
    },
    CidChar {
        char: 40040,
        cid: 8996,
    },
    CidChar {
        char: 40041,
        cid: 18551,
    },
    CidChar {
        char: 40042,
        cid: 15857,
    },
    CidChar {
        char: 40043,
        cid: 13418,
    },
    CidChar {
        char: 40044,
        cid: 18552,
    },
    CidChar {
        char: 40045,
        cid: 15858,
    },
    CidChar {
        char: 40046,
        cid: 18553,
    },
    CidChar {
        char: 40055,
        cid: 8849,
    },
    CidChar {
        char: 40056,
        cid: 18556,
    },
    CidChar {
        char: 40057,
        cid: 15866,
    },
    CidChar {
        char: 40058,
        cid: 18557,
    },
    CidChar {
        char: 40061,
        cid: 18558,
    },
    CidChar {
        char: 40062,
        cid: 15869,
    },
    CidChar {
        char: 40101,
        cid: 15872,
    },
    CidChar {
        char: 40106,
        cid: 18563,
    },
    CidChar {
        char: 40107,
        cid: 15875,
    },
    CidChar {
        char: 40108,
        cid: 18564,
    },
    CidChar {
        char: 40123,
        cid: 18567,
    },
    CidChar {
        char: 40124,
        cid: 6023,
    },
    CidChar {
        char: 40125,
        cid: 2399,
    },
    CidChar {
        char: 40142,
        cid: 18571,
    },
    CidChar {
        char: 40143,
        cid: 15903,
    },
    CidChar {
        char: 40144,
        cid: 7833,
    },
    CidChar {
        char: 40155,
        cid: 18576,
    },
    CidChar {
        char: 40166,
        cid: 18577,
    },
    CidChar {
        char: 40170,
        cid: 18578,
    },
    CidChar {
        char: 40173,
        cid: 18579,
    },
    CidChar {
        char: 40189,
        cid: 15938,
    },
    CidChar {
        char: 40190,
        cid: 18583,
    },
    CidChar {
        char: 40256,
        cid: 17384,
    },
    CidChar {
        char: 40262,
        cid: 15939,
    },
    CidChar {
        char: 40265,
        cid: 15940,
    },
    CidChar {
        char: 40270,
        cid: 18593,
    },
    CidChar {
        char: 40271,
        cid: 15944,
    },
    CidChar {
        char: 40272,
        cid: 18594,
    },
    CidChar {
        char: 40273,
        cid: 15945,
    },
    CidChar {
        char: 40277,
        cid: 15946,
    },
    CidChar {
        char: 40278,
        cid: 18598,
    },
    CidChar {
        char: 40279,
        cid: 9665,
    },
    CidChar {
        char: 40282,
        cid: 3340,
    },
    CidChar {
        char: 40289,
        cid: 17353,
    },
    CidChar {
        char: 40290,
        cid: 15948,
    },
    CidChar {
        char: 40291,
        cid: 18607,
    },
    CidChar {
        char: 40292,
        cid: 15949,
    },
    CidChar {
        char: 40312,
        cid: 17397,
    },
    CidChar {
        char: 40313,
        cid: 15950,
    },
    CidChar {
        char: 40318,
        cid: 15951,
    },
    CidChar {
        char: 40361,
        cid: 18635,
    },
    CidChar {
        char: 40362,
        cid: 15956,
    },
    CidChar {
        char: 40363,
        cid: 18636,
    },
    CidChar {
        char: 40368,
        cid: 15960,
    },
    CidChar {
        char: 40371,
        cid: 15961,
    },
    CidChar {
        char: 40372,
        cid: 18641,
    },
    CidChar {
        char: 40373,
        cid: 15962,
    },
    CidChar {
        char: 40374,
        cid: 18642,
    },
    CidChar {
        char: 40375,
        cid: 15963,
    },
    CidChar {
        char: 40382,
        cid: 18647,
    },
    CidChar {
        char: 40387,
        cid: 15968,
    },
    CidChar {
        char: 40388,
        cid: 1510,
    },
    CidChar {
        char: 40393,
        cid: 18652,
    },
    CidChar {
        char: 40394,
        cid: 15972,
    },
    CidChar {
        char: 40402,
        cid: 18655,
    },
    CidChar {
        char: 40444,
        cid: 18660,
    },
    CidChar {
        char: 40515,
        cid: 18661,
    },
    CidChar {
        char: 40543,
        cid: 18662,
    },
    CidChar {
        char: 40547,
        cid: 18663,
    },
    CidChar {
        char: 40552,
        cid: 16054,
    },
    CidChar {
        char: 40553,
        cid: 18666,
    },
    CidChar {
        char: 40554,
        cid: 16055,
    },
    CidChar {
        char: 40561,
        cid: 16056,
    },
    CidChar {
        char: 40562,
        cid: 18673,
    },
    CidChar {
        char: 40563,
        cid: 16057,
    },
    CidChar {
        char: 40569,
        cid: 18677,
    },
    CidChar {
        char: 40570,
        cid: 16060,
    },
    CidChar {
        char: 40571,
        cid: 18678,
    },
    CidChar {
        char: 40572,
        cid: 16061,
    },
    CidChar {
        char: 40573,
        cid: 18679,
    },
    CidChar {
        char: 40574,
        cid: 16062,
    },
    CidChar {
        char: 40611,
        cid: 18680,
    },
    CidChar {
        char: 40617,
        cid: 1832,
    },
    CidChar {
        char: 40618,
        cid: 16069,
    },
    CidChar {
        char: 40619,
        cid: 18683,
    },
    CidChar {
        char: 40622,
        cid: 18684,
    },
    CidChar {
        char: 40628,
        cid: 16075,
    },
    CidChar {
        char: 40629,
        cid: 18687,
    },
    CidChar {
        char: 40632,
        cid: 18688,
    },
    CidChar {
        char: 40633,
        cid: 16078,
    },
    CidChar {
        char: 40636,
        cid: 16079,
    },
    CidChar {
        char: 40646,
        cid: 18696,
    },
    CidChar {
        char: 40653,
        cid: 16089,
    },
    CidChar {
        char: 40658,
        cid: 18701,
    },
    CidChar {
        char: 40659,
        cid: 16092,
    },
    CidChar {
        char: 40687,
        cid: 9398,
    },
    CidChar {
        char: 40690,
        cid: 18706,
    },
    CidChar {
        char: 40699,
        cid: 18710,
    },
    CidChar {
        char: 40700,
        cid: 16124,
    },
    CidChar {
        char: 40701,
        cid: 6150,
    },
    CidChar {
        char: 40702,
        cid: 16126,
    },
    CidChar {
        char: 40771,
        cid: 18711,
    },
    CidChar {
        char: 40776,
        cid: 18712,
    },
    CidChar {
        char: 40800,
        cid: 13585,
    },
    CidChar {
        char: 40806,
        cid: 14661,
    },
    CidChar {
        char: 40816,
        cid: 18717,
    },
    CidChar {
        char: 40885,
        cid: 18718,
    },
    CidChar {
        char: 40891,
        cid: 18719,
    },
    CidChar {
        char: 40895,
        cid: 18720,
    },
    CidChar {
        char: 40896,
        cid: 16212,
    },
    CidChar {
        char: 40897,
        cid: 18721,
    },
    CidChar {
        char: 40907,
        cid: 3970,
    },
    CidChar {
        char: 40908,
        cid: 18722,
    },
    CidChar {
        char: 40916,
        cid: 18723,
    },
    CidChar {
        char: 40920,
        cid: 14910,
    },
    CidChar {
        char: 40932,
        cid: 18724,
    },
    CidChar {
        char: 40953,
        cid: 18725,
    },
    CidChar {
        char: 41024,
        cid: 18726,
    },
    CidChar {
        char: 41031,
        cid: 18727,
    },
    CidChar {
        char: 41045,
        cid: 18728,
    },
    CidChar {
        char: 41059,
        cid: 14353,
    },
    CidChar {
        char: 41069,
        cid: 18729,
    },
    CidChar {
        char: 41079,
        cid: 1522,
    },
    CidChar {
        char: 41083,
        cid: 18730,
    },
    CidChar {
        char: 41121,
        cid: 16330,
    },
    CidChar {
        char: 41122,
        cid: 18731,
    },
    CidChar {
        char: 41127,
        cid: 18732,
    },
    CidChar {
        char: 41157,
        cid: 18733,
    },
    CidChar {
        char: 41168,
        cid: 18734,
    },
    CidChar {
        char: 41173,
        cid: 15086,
    },
    CidChar {
        char: 41183,
        cid: 9341,
    },
    CidChar {
        char: 41187,
        cid: 18735,
    },
    CidChar {
        char: 41188,
        cid: 15049,
    },
    CidChar {
        char: 41198,
        cid: 17332,
    },
    CidChar {
        char: 41202,
        cid: 17336,
    },
    CidChar {
        char: 41462,
        cid: 248,
    },
    CidChar {
        char: 41463,
        cid: 247,
    },
    CidChar {
        char: 44286,
        cid: 2431,
    },
    CidChar {
        char: 48722,
        cid: 4308,
    },
    CidChar {
        char: 49867,
        cid: 5221,
    },
    CidChar {
        char: 50105,
        cid: 5551,
    },
    CidChar {
        char: 50106,
        cid: 5550,
    },
    CidChar {
        char: 50262,
        cid: 5495,
    },
    CidChar {
        char: 50900,
        cid: 558,
    },
    CidChar {
        char: 50902,
        cid: 560,
    },
    CidChar {
        char: 51424,
        cid: 18846,
    },
    CidChar {
        char: 51433,
        cid: 18847,
    },
    CidChar {
        char: 51441,
        cid: 18848,
    },
    CidChar {
        char: 51530,
        cid: 628,
    },
    CidChar {
        char: 51646,
        cid: 6039,
    },
    CidChar {
        char: 51959,
        cid: 6134,
    },
    CidChar {
        char: 54988,
        cid: 8788,
    },
    CidChar {
        char: 55162,
        cid: 8889,
    },
    CidChar {
        char: 56031,
        cid: 8142,
    },
    CidChar {
        char: 56828,
        cid: 9089,
    },
    CidChar {
        char: 60401,
        cid: 10926,
    },
    CidChar {
        char: 60638,
        cid: 11073,
    },
    CidChar {
        char: 61163,
        cid: 12308,
    },
    CidChar {
        char: 61526,
        cid: 11719,
    },
    CidChar {
        char: 61643,
        cid: 11361,
    },
    CidChar {
        char: 61803,
        cid: 12640,
    },
    CidChar {
        char: 62056,
        cid: 12783,
    },
    CidChar {
        char: 62645,
        cid: 12526,
    },
    CidChar {
        char: 63075,
        cid: 12900,
    },
    CidChar {
        char: 63940,
        cid: 13585,
    },
    CidChar {
        char: 63941,
        cid: 13629,
    },
    CidChar {
        char: 63942,
        cid: 13641,
    },
    CidChar {
        char: 64095,
        cid: 2106,
    },
    CidChar {
        char: 64102,
        cid: 2557,
    },
    CidChar {
        char: 64189,
        cid: 781,
    },
    CidChar {
        char: 64197,
        cid: 363,
    },
    CidChar {
        char: 64213,
        cid: 2144,
    },
    CidChar {
        char: 64328,
        cid: 16002,
    },
    CidChar {
        char: 64339,
        cid: 18760,
    },
    CidChar {
        char: 64366,
        cid: 18761,
    },
    CidChar {
        char: 64419,
        cid: 18762,
    },
    CidChar {
        char: 64440,
        cid: 3107,
    },
    CidChar {
        char: 64447,
        cid: 18763,
    },
    CidChar {
        char: 64461,
        cid: 18764,
    },
    CidChar {
        char: 64499,
        cid: 6116,
    },
    CidChar {
        char: 64505,
        cid: 16014,
    },
    CidChar {
        char: 64586,
        cid: 18765,
    },
    CidChar {
        char: 64591,
        cid: 8495,
    },
    CidChar {
        char: 64594,
        cid: 18766,
    },
    CidChar {
        char: 64611,
        cid: 18767,
    },
    CidChar {
        char: 64620,
        cid: 16385,
    },
    CidChar {
        char: 64621,
        cid: 18768,
    },
    CidChar {
        char: 64629,
        cid: 18769,
    },
    CidChar {
        char: 64697,
        cid: 4447,
    },
    CidChar {
        char: 64716,
        cid: 18772,
    },
    CidChar {
        char: 64738,
        cid: 3193,
    },
    CidChar {
        char: 64739,
        cid: 18773,
    },
    CidChar {
        char: 64750,
        cid: 18774,
    },
    CidChar {
        char: 64753,
        cid: 1219,
    },
    CidChar {
        char: 64841,
        cid: 18775,
    },
    CidChar {
        char: 64874,
        cid: 18776,
    },
    CidChar {
        char: 64951,
        cid: 6333,
    },
    CidChar {
        char: 64952,
        cid: 3261,
    },
    CidChar {
        char: 64955,
        cid: 3237,
    },
    CidChar {
        char: 64995,
        cid: 18777,
    },
    CidChar {
        char: 65009,
        cid: 3278,
    },
    CidChar {
        char: 65010,
        cid: 18778,
    },
    CidChar {
        char: 65106,
        cid: 15728,
    },
    CidChar {
        char: 65133,
        cid: 18779,
    },
    CidChar {
        char: 65134,
        cid: 17055,
    },
    CidChar {
        char: 65135,
        cid: 3716,
    },
    CidChar {
        char: 65144,
        cid: 18780,
    },
    CidChar {
        char: 65194,
        cid: 288,
    },
    CidChar {
        char: 65245,
        cid: 7080,
    },
];

const CID_RANGE_H: [CidRange; 667] = [
    CidRange {
        start: 32,
        end: 126,
        cid: 1,
    },
    CidRange {
        start: 34624,
        end: 34661,
        cid: 18965,
    },
    CidRange {
        start: 34663,
        end: 34681,
        cid: 19003,
    },
    CidRange {
        start: 34682,
        end: 34686,
        cid: 19088,
    },
    CidRange {
        start: 34721,
        end: 34783,
        cid: 19093,
    },
    CidRange {
        start: 34880,
        end: 34901,
        cid: 17609,
    },
    CidRange {
        start: 34902,
        end: 34942,
        cid: 18785,
    },
    CidRange {
        start: 34977,
        end: 34984,
        cid: 18826,
    },
    CidRange {
        start: 34985,
        end: 34986,
        cid: 18844,
    },
    CidRange {
        start: 35136,
        end: 35137,
        cid: 17716,
    },
    CidRange {
        start: 35142,
        end: 35145,
        cid: 17719,
    },
    CidRange {
        start: 35150,
        end: 35152,
        cid: 17724,
    },
    CidRange {
        start: 35154,
        end: 35198,
        cid: 17727,
    },
    CidRange {
        start: 35233,
        end: 35237,
        cid: 17772,
    },
    CidRange {
        start: 35248,
        end: 35250,
        cid: 17779,
    },
    CidRange {
        start: 35253,
        end: 35263,
        cid: 17782,
    },
    CidRange {
        start: 35265,
        end: 35267,
        cid: 17793,
    },
    CidRange {
        start: 35269,
        end: 35278,
        cid: 17796,
    },
    CidRange {
        start: 35280,
        end: 35288,
        cid: 17806,
    },
    CidRange {
        start: 35294,
        end: 35296,
        cid: 17817,
    },
    CidRange {
        start: 35300,
        end: 35305,
        cid: 17821,
    },
    CidRange {
        start: 35306,
        end: 35307,
        cid: 17317,
    },
    CidRange {
        start: 35308,
        end: 35321,
        cid: 17827,
    },
    CidRange {
        start: 35323,
        end: 35326,
        cid: 17841,
    },
    CidRange {
        start: 35395,
        end: 35404,
        cid: 17163,
    },
    CidRange {
        start: 35406,
        end: 35417,
        cid: 17174,
    },
    CidRange {
        start: 35419,
        end: 35421,
        cid: 17187,
    },
    CidRange {
        start: 35423,
        end: 35426,
        cid: 17191,
    },
    CidRange {
        start: 35428,
        end: 35440,
        cid: 17196,
    },
    CidRange {
        start: 35442,
        end: 35444,
        cid: 17210,
    },
    CidRange {
        start: 35448,
        end: 35449,
        cid: 17216,
    },
    CidRange {
        start: 35489,
        end: 35495,
        cid: 17223,
    },
    CidRange {
        start: 35497,
        end: 35498,
        cid: 17231,
    },
    CidRange {
        start: 35500,
        end: 35504,
        cid: 17234,
    },
    CidRange {
        start: 35506,
        end: 35509,
        cid: 17240,
    },
    CidRange {
        start: 35515,
        end: 35527,
        cid: 17249,
    },
    CidRange {
        start: 35529,
        end: 35531,
        cid: 17263,
    },
    CidRange {
        start: 35534,
        end: 35541,
        cid: 17268,
    },
    CidRange {
        start: 35542,
        end: 35543,
        cid: 17858,
    },
    CidRange {
        start: 35544,
        end: 35548,
        cid: 17278,
    },
    CidRange {
        start: 35551,
        end: 35557,
        cid: 17285,
    },
    CidRange {
        start: 35560,
        end: 35572,
        cid: 17861,
    },
    CidRange {
        start: 35574,
        end: 35582,
        cid: 17874,
    },
    CidRange {
        start: 35649,
        end: 35650,
        cid: 17292,
    },
    CidRange {
        start: 35651,
        end: 35652,
        cid: 17884,
    },
    CidRange {
        start: 35661,
        end: 35664,
        cid: 17298,
    },
    CidRange {
        start: 35665,
        end: 35667,
        cid: 17890,
    },
    CidRange {
        start: 35669,
        end: 35671,
        cid: 17893,
    },
    CidRange {
        start: 35676,
        end: 35680,
        cid: 17897,
    },
    CidRange {
        start: 35682,
        end: 35687,
        cid: 17902,
    },
    CidRange {
        start: 35689,
        end: 35710,
        cid: 17908,
    },
    CidRange {
        start: 35745,
        end: 35775,
        cid: 17930,
    },
    CidRange {
        start: 35776,
        end: 35804,
        cid: 17631,
    },
    CidRange {
        start: 35806,
        end: 35837,
        cid: 17660,
    },
    CidRange {
        start: 35904,
        end: 35937,
        cid: 18849,
    },
    CidRange {
        start: 35939,
        end: 35966,
        cid: 18883,
    },
    CidRange {
        start: 36001,
        end: 36005,
        cid: 18911,
    },
    CidRange {
        start: 36007,
        end: 36037,
        cid: 18916,
    },
    CidRange {
        start: 36041,
        end: 36044,
        cid: 18947,
    },
    CidRange {
        start: 36046,
        end: 36058,
        cid: 18951,
    },
    CidRange {
        start: 36061,
        end: 36068,
        cid: 19024,
    },
    CidRange {
        start: 36070,
        end: 36094,
        cid: 19032,
    },
    CidRange {
        start: 36162,
        end: 36191,
        cid: 19058,
    },
    CidRange {
        start: 36192,
        end: 36193,
        cid: 17961,
    },
    CidRange {
        start: 36195,
        end: 36199,
        cid: 17963,
    },
    CidRange {
        start: 36203,
        end: 36205,
        cid: 17968,
    },
    CidRange {
        start: 36207,
        end: 36213,
        cid: 17971,
    },
    CidRange {
        start: 36215,
        end: 36217,
        cid: 17978,
    },
    CidRange {
        start: 36221,
        end: 36222,
        cid: 17982,
    },
    CidRange {
        start: 36257,
        end: 36260,
        cid: 17984,
    },
    CidRange {
        start: 36262,
        end: 36263,
        cid: 17988,
    },
    CidRange {
        start: 36266,
        end: 36277,
        cid: 17990,
    },
    CidRange {
        start: 36279,
        end: 36290,
        cid: 18002,
    },
    CidRange {
        start: 36292,
        end: 36345,
        cid: 18014,
    },
    CidRange {
        start: 36347,
        end: 36350,
        cid: 18068,
    },
    CidRange {
        start: 36416,
        end: 36420,
        cid: 14123,
    },
    CidRange {
        start: 36422,
        end: 36456,
        cid: 14128,
    },
    CidRange {
        start: 36459,
        end: 36460,
        cid: 18073,
    },
    CidRange {
        start: 36461,
        end: 36462,
        cid: 14166,
    },
    CidRange {
        start: 36464,
        end: 36469,
        cid: 14169,
    },
    CidRange {
        start: 36471,
        end: 36474,
        cid: 14175,
    },
    CidRange {
        start: 36476,
        end: 36477,
        cid: 14180,
    },
    CidRange {
        start: 36513,
        end: 36517,
        cid: 14182,
    },
    CidRange {
        start: 36519,
        end: 36522,
        cid: 14187,
    },
    CidRange {
        start: 36524,
        end: 36531,
        cid: 14192,
    },
    CidRange {
        start: 36533,
        end: 36535,
        cid: 14201,
    },
    CidRange {
        start: 36537,
        end: 36552,
        cid: 14205,
    },
    CidRange {
        start: 36554,
        end: 36556,
        cid: 14221,
    },
    CidRange {
        start: 36558,
        end: 36559,
        cid: 14225,
    },
    CidRange {
        start: 36561,
        end: 36580,
        cid: 14228,
    },
    CidRange {
        start: 36582,
        end: 36590,
        cid: 14248,
    },
    CidRange {
        start: 36592,
        end: 36597,
        cid: 14257,
    },
    CidRange {
        start: 36599,
        end: 36606,
        cid: 14264,
    },
    CidRange {
        start: 36672,
        end: 36694,
        cid: 14272,
    },
    CidRange {
        start: 36698,
        end: 36702,
        cid: 14297,
    },
    CidRange {
        start: 36704,
        end: 36710,
        cid: 14302,
    },
    CidRange {
        start: 36714,
        end: 36717,
        cid: 14311,
    },
    CidRange {
        start: 36719,
        end: 36728,
        cid: 14316,
    },
    CidRange {
        start: 36730,
        end: 36734,
        cid: 14327,
    },
    CidRange {
        start: 36769,
        end: 36783,
        cid: 14332,
    },
    CidRange {
        start: 36785,
        end: 36804,
        cid: 14348,
    },
    CidRange {
        start: 36808,
        end: 36809,
        cid: 14369,
    },
    CidRange {
        start: 36813,
        end: 36825,
        cid: 14374,
    },
    CidRange {
        start: 36827,
        end: 36834,
        cid: 14387,
    },
    CidRange {
        start: 36836,
        end: 36859,
        cid: 14396,
    },
    CidRange {
        start: 36928,
        end: 36948,
        cid: 14422,
    },
    CidRange {
        start: 36950,
        end: 36955,
        cid: 14444,
    },
    CidRange {
        start: 36956,
        end: 36958,
        cid: 18095,
    },
    CidRange {
        start: 36959,
        end: 36972,
        cid: 14451,
    },
    CidRange {
        start: 36976,
        end: 36985,
        cid: 14467,
    },
    CidRange {
        start: 36987,
        end: 36990,
        cid: 14477,
    },
    CidRange {
        start: 37025,
        end: 37029,
        cid: 14481,
    },
    CidRange {
        start: 37031,
        end: 37047,
        cid: 14486,
    },
    CidRange {
        start: 37049,
        end: 37083,
        cid: 14503,
    },
    CidRange {
        start: 37085,
        end: 37104,
        cid: 14539,
    },
    CidRange {
        start: 37106,
        end: 37118,
        cid: 14560,
    },
    CidRange {
        start: 37184,
        end: 37220,
        cid: 14573,
    },
    CidRange {
        start: 37222,
        end: 37229,
        cid: 14610,
    },
    CidRange {
        start: 37231,
        end: 37245,
        cid: 14618,
    },
    CidRange {
        start: 37283,
        end: 37310,
        cid: 14634,
    },
    CidRange {
        start: 37312,
        end: 37319,
        cid: 14663,
    },
    CidRange {
        start: 37321,
        end: 37374,
        cid: 14672,
    },
    CidRange {
        start: 37440,
        end: 37443,
        cid: 14726,
    },
    CidRange {
        start: 37445,
        end: 37475,
        cid: 14731,
    },
    CidRange {
        start: 37477,
        end: 37484,
        cid: 14763,
    },
    CidRange {
        start: 37486,
        end: 37502,
        cid: 14772,
    },
    CidRange {
        start: 37537,
        end: 37550,
        cid: 14789,
    },
    CidRange {
        start: 37551,
        end: 37552,
        cid: 281,
    },
    CidRange {
        start: 37555,
        end: 37575,
        cid: 14803,
    },
    CidRange {
        start: 37577,
        end: 37584,
        cid: 14825,
    },
    CidRange {
        start: 37586,
        end: 37604,
        cid: 14834,
    },
    CidRange {
        start: 37606,
        end: 37617,
        cid: 14853,
    },
    CidRange {
        start: 37619,
        end: 37630,
        cid: 14865,
    },
    CidRange {
        start: 37696,
        end: 37735,
        cid: 14877,
    },
    CidRange {
        start: 37737,
        end: 37758,
        cid: 14917,
    },
    CidRange {
        start: 37793,
        end: 37801,
        cid: 14939,
    },
    CidRange {
        start: 37803,
        end: 37825,
        cid: 14948,
    },
    CidRange {
        start: 37827,
        end: 37860,
        cid: 14971,
    },
    CidRange {
        start: 37862,
        end: 37863,
        cid: 15005,
    },
    CidRange {
        start: 37865,
        end: 37866,
        cid: 15008,
    },
    CidRange {
        start: 37868,
        end: 37886,
        cid: 15010,
    },
    CidRange {
        start: 37952,
        end: 37957,
        cid: 15029,
    },
    CidRange {
        start: 37960,
        end: 38008,
        cid: 15036,
    },
    CidRange {
        start: 38010,
        end: 38014,
        cid: 15086,
    },
    CidRange {
        start: 38049,
        end: 38089,
        cid: 15091,
    },
    CidRange {
        start: 38092,
        end: 38142,
        cid: 15134,
    },
    CidRange {
        start: 38208,
        end: 38220,
        cid: 15185,
    },
    CidRange {
        start: 38222,
        end: 38233,
        cid: 15198,
    },
    CidRange {
        start: 38235,
        end: 38238,
        cid: 15210,
    },
    CidRange {
        start: 38240,
        end: 38270,
        cid: 15215,
    },
    CidRange {
        start: 38305,
        end: 38341,
        cid: 15246,
    },
    CidRange {
        start: 38343,
        end: 38360,
        cid: 15283,
    },
    CidRange {
        start: 38362,
        end: 38398,
        cid: 15302,
    },
    CidRange {
        start: 38464,
        end: 38467,
        cid: 15339,
    },
    CidRange {
        start: 38469,
        end: 38480,
        cid: 15344,
    },
    CidRange {
        start: 38482,
        end: 38505,
        cid: 15357,
    },
    CidRange {
        start: 38507,
        end: 38526,
        cid: 15382,
    },
    CidRange {
        start: 38561,
        end: 38611,
        cid: 15402,
    },
    CidRange {
        start: 38613,
        end: 38636,
        cid: 15453,
    },
    CidRange {
        start: 38638,
        end: 38651,
        cid: 15478,
    },
    CidRange {
        start: 38653,
        end: 38654,
        cid: 15493,
    },
    CidRange {
        start: 38720,
        end: 38782,
        cid: 15495,
    },
    CidRange {
        start: 38817,
        end: 38910,
        cid: 15558,
    },
    CidRange {
        start: 38976,
        end: 38979,
        cid: 15652,
    },
    CidRange {
        start: 38980,
        end: 38981,
        cid: 18126,
    },
    CidRange {
        start: 38982,
        end: 39022,
        cid: 15658,
    },
    CidRange {
        start: 39024,
        end: 39028,
        cid: 15700,
    },
    CidRange {
        start: 39029,
        end: 39030,
        cid: 18129,
    },
    CidRange {
        start: 39032,
        end: 39033,
        cid: 18131,
    },
    CidRange {
        start: 39035,
        end: 39038,
        cid: 18133,
    },
    CidRange {
        start: 39073,
        end: 39074,
        cid: 18137,
    },
    CidRange {
        start: 39076,
        end: 39086,
        cid: 18139,
    },
    CidRange {
        start: 39088,
        end: 39091,
        cid: 18150,
    },
    CidRange {
        start: 39101,
        end: 39102,
        cid: 15711,
    },
    CidRange {
        start: 39103,
        end: 39105,
        cid: 18158,
    },
    CidRange {
        start: 39110,
        end: 39111,
        cid: 15715,
    },
    CidRange {
        start: 39112,
        end: 39121,
        cid: 18163,
    },
    CidRange {
        start: 39123,
        end: 39127,
        cid: 18173,
    },
    CidRange {
        start: 39128,
        end: 39129,
        cid: 17358,
    },
    CidRange {
        start: 39132,
        end: 39134,
        cid: 18179,
    },
    CidRange {
        start: 39136,
        end: 39138,
        cid: 18182,
    },
    CidRange {
        start: 39140,
        end: 39142,
        cid: 18185,
    },
    CidRange {
        start: 39144,
        end: 39148,
        cid: 18188,
    },
    CidRange {
        start: 39150,
        end: 39151,
        cid: 18193,
    },
    CidRange {
        start: 39156,
        end: 39157,
        cid: 17365,
    },
    CidRange {
        start: 39158,
        end: 39163,
        cid: 18197,
    },
    CidRange {
        start: 39232,
        end: 39233,
        cid: 18204,
    },
    CidRange {
        start: 39240,
        end: 39246,
        cid: 18208,
    },
    CidRange {
        start: 39248,
        end: 39251,
        cid: 18215,
    },
    CidRange {
        start: 39253,
        end: 39259,
        cid: 18219,
    },
    CidRange {
        start: 39261,
        end: 39267,
        cid: 18226,
    },
    CidRange {
        start: 39269,
        end: 39273,
        cid: 18233,
    },
    CidRange {
        start: 39275,
        end: 39277,
        cid: 18238,
    },
    CidRange {
        start: 39279,
        end: 39284,
        cid: 18241,
    },
    CidRange {
        start: 39286,
        end: 39287,
        cid: 18247,
    },
    CidRange {
        start: 39289,
        end: 39294,
        cid: 18249,
    },
    CidRange {
        start: 39335,
        end: 39341,
        cid: 18258,
    },
    CidRange {
        start: 39343,
        end: 39345,
        cid: 18265,
    },
    CidRange {
        start: 39347,
        end: 39349,
        cid: 18268,
    },
    CidRange {
        start: 39351,
        end: 39353,
        cid: 18271,
    },
    CidRange {
        start: 39355,
        end: 39369,
        cid: 18274,
    },
    CidRange {
        start: 39371,
        end: 39372,
        cid: 18289,
    },
    CidRange {
        start: 39374,
        end: 39378,
        cid: 18291,
    },
    CidRange {
        start: 39380,
        end: 39381,
        cid: 18296,
    },
    CidRange {
        start: 39383,
        end: 39390,
        cid: 18298,
    },
    CidRange {
        start: 39392,
        end: 39393,
        cid: 18306,
    },
    CidRange {
        start: 39401,
        end: 39406,
        cid: 18311,
    },
    CidRange {
        start: 39408,
        end: 39411,
        cid: 18317,
    },
    CidRange {
        start: 39413,
        end: 39422,
        cid: 18321,
    },
    CidRange {
        start: 39488,
        end: 39497,
        cid: 18331,
    },
    CidRange {
        start: 39501,
        end: 39512,
        cid: 18342,
    },
    CidRange {
        start: 39514,
        end: 39518,
        cid: 18354,
    },
    CidRange {
        start: 39522,
        end: 39525,
        cid: 18360,
    },
    CidRange {
        start: 39532,
        end: 39538,
        cid: 18366,
    },
    CidRange {
        start: 39542,
        end: 39549,
        cid: 18374,
    },
    CidRange {
        start: 39585,
        end: 39586,
        cid: 18382,
    },
    CidRange {
        start: 39590,
        end: 39592,
        cid: 18385,
    },
    CidRange {
        start: 39595,
        end: 39601,
        cid: 18388,
    },
    CidRange {
        start: 39603,
        end: 39606,
        cid: 18395,
    },
    CidRange {
        start: 39614,
        end: 39622,
        cid: 18402,
    },
    CidRange {
        start: 39624,
        end: 39631,
        cid: 18411,
    },
    CidRange {
        start: 39635,
        end: 39640,
        cid: 18420,
    },
    CidRange {
        start: 39641,
        end: 39643,
        cid: 15750,
    },
    CidRange {
        start: 39644,
        end: 39649,
        cid: 18426,
    },
    CidRange {
        start: 39653,
        end: 39655,
        cid: 18433,
    },
    CidRange {
        start: 39658,
        end: 39661,
        cid: 18436,
    },
    CidRange {
        start: 39663,
        end: 39665,
        cid: 18440,
    },
    CidRange {
        start: 39667,
        end: 39669,
        cid: 18443,
    },
    CidRange {
        start: 39671,
        end: 39674,
        cid: 18446,
    },
    CidRange {
        start: 39676,
        end: 39678,
        cid: 18450,
    },
    CidRange {
        start: 39744,
        end: 39749,
        cid: 18453,
    },
    CidRange {
        start: 39751,
        end: 39753,
        cid: 18459,
    },
    CidRange {
        start: 39755,
        end: 39763,
        cid: 18462,
    },
    CidRange {
        start: 39765,
        end: 39767,
        cid: 18471,
    },
    CidRange {
        start: 39774,
        end: 39775,
        cid: 15766,
    },
    CidRange {
        start: 39778,
        end: 39791,
        cid: 18478,
    },
    CidRange {
        start: 39792,
        end: 39795,
        cid: 15768,
    },
    CidRange {
        start: 39796,
        end: 39797,
        cid: 18492,
    },
    CidRange {
        start: 39801,
        end: 39802,
        cid: 18494,
    },
    CidRange {
        start: 39843,
        end: 39844,
        cid: 15779,
    },
    CidRange {
        start: 39845,
        end: 39846,
        cid: 18498,
    },
    CidRange {
        start: 39847,
        end: 39850,
        cid: 15781,
    },
    CidRange {
        start: 39853,
        end: 39854,
        cid: 18501,
    },
    CidRange {
        start: 39856,
        end: 39857,
        cid: 18503,
    },
    CidRange {
        start: 39858,
        end: 39865,
        cid: 15787,
    },
    CidRange {
        start: 39866,
        end: 39869,
        cid: 18505,
    },
    CidRange {
        start: 39872,
        end: 39877,
        cid: 15796,
    },
    CidRange {
        start: 39879,
        end: 39881,
        cid: 18510,
    },
    CidRange {
        start: 39888,
        end: 39889,
        cid: 15805,
    },
    CidRange {
        start: 39894,
        end: 39895,
        cid: 18518,
    },
    CidRange {
        start: 39896,
        end: 39898,
        cid: 15809,
    },
    CidRange {
        start: 39899,
        end: 39900,
        cid: 18520,
    },
    CidRange {
        start: 39908,
        end: 39910,
        cid: 18524,
    },
    CidRange {
        start: 39913,
        end: 39915,
        cid: 15817,
    },
    CidRange {
        start: 39918,
        end: 39919,
        cid: 15821,
    },
    CidRange {
        start: 39920,
        end: 39922,
        cid: 18529,
    },
    CidRange {
        start: 39924,
        end: 39925,
        cid: 18532,
    },
    CidRange {
        start: 39928,
        end: 39929,
        cid: 15825,
    },
    CidRange {
        start: 39931,
        end: 39932,
        cid: 15827,
    },
    CidRange {
        start: 39933,
        end: 39934,
        cid: 18536,
    },
    CidRange {
        start: 40000,
        end: 40001,
        cid: 15829,
    },
    CidRange {
        start: 40004,
        end: 40006,
        cid: 15832,
    },
    CidRange {
        start: 40011,
        end: 40012,
        cid: 18541,
    },
    CidRange {
        start: 40013,
        end: 40018,
        cid: 15837,
    },
    CidRange {
        start: 40023,
        end: 40027,
        cid: 15845,
    },
    CidRange {
        start: 40030,
        end: 40031,
        cid: 18546,
    },
    CidRange {
        start: 40036,
        end: 40038,
        cid: 15853,
    },
    CidRange {
        start: 40047,
        end: 40050,
        cid: 15859,
    },
    CidRange {
        start: 40051,
        end: 40052,
        cid: 18554,
    },
    CidRange {
        start: 40053,
        end: 40054,
        cid: 15863,
    },
    CidRange {
        start: 40059,
        end: 40060,
        cid: 15867,
    },
    CidRange {
        start: 40097,
        end: 40098,
        cid: 15870,
    },
    CidRange {
        start: 40099,
        end: 40100,
        cid: 18559,
    },
    CidRange {
        start: 40102,
        end: 40103,
        cid: 18561,
    },
    CidRange {
        start: 40104,
        end: 40105,
        cid: 15873,
    },
    CidRange {
        start: 40109,
        end: 40110,
        cid: 15876,
    },
    CidRange {
        start: 40111,
        end: 40112,
        cid: 18565,
    },
    CidRange {
        start: 40113,
        end: 40122,
        cid: 15878,
    },
    CidRange {
        start: 40126,
        end: 40130,
        cid: 15890,
    },
    CidRange {
        start: 40131,
        end: 40133,
        cid: 18568,
    },
    CidRange {
        start: 40134,
        end: 40141,
        cid: 15895,
    },
    CidRange {
        start: 40145,
        end: 40147,
        cid: 15905,
    },
    CidRange {
        start: 40148,
        end: 40151,
        cid: 18572,
    },
    CidRange {
        start: 40152,
        end: 40154,
        cid: 15908,
    },
    CidRange {
        start: 40156,
        end: 40165,
        cid: 15911,
    },
    CidRange {
        start: 40167,
        end: 40169,
        cid: 15921,
    },
    CidRange {
        start: 40171,
        end: 40172,
        cid: 15924,
    },
    CidRange {
        start: 40174,
        end: 40185,
        cid: 15926,
    },
    CidRange {
        start: 40186,
        end: 40188,
        cid: 18580,
    },
    CidRange {
        start: 40257,
        end: 40261,
        cid: 18584,
    },
    CidRange {
        start: 40263,
        end: 40264,
        cid: 18589,
    },
    CidRange {
        start: 40266,
        end: 40267,
        cid: 18591,
    },
    CidRange {
        start: 40268,
        end: 40269,
        cid: 15942,
    },
    CidRange {
        start: 40274,
        end: 40276,
        cid: 18595,
    },
    CidRange {
        start: 40280,
        end: 40281,
        cid: 18599,
    },
    CidRange {
        start: 40283,
        end: 40288,
        cid: 18601,
    },
    CidRange {
        start: 40293,
        end: 40311,
        cid: 18608,
    },
    CidRange {
        start: 40314,
        end: 40317,
        cid: 18627,
    },
    CidRange {
        start: 40353,
        end: 40356,
        cid: 18631,
    },
    CidRange {
        start: 40357,
        end: 40360,
        cid: 15952,
    },
    CidRange {
        start: 40364,
        end: 40365,
        cid: 15957,
    },
    CidRange {
        start: 40366,
        end: 40367,
        cid: 18637,
    },
    CidRange {
        start: 40369,
        end: 40370,
        cid: 18639,
    },
    CidRange {
        start: 40376,
        end: 40379,
        cid: 18643,
    },
    CidRange {
        start: 40380,
        end: 40381,
        cid: 15964,
    },
    CidRange {
        start: 40383,
        end: 40384,
        cid: 15966,
    },
    CidRange {
        start: 40385,
        end: 40386,
        cid: 18648,
    },
    CidRange {
        start: 40389,
        end: 40390,
        cid: 18650,
    },
    CidRange {
        start: 40391,
        end: 40392,
        cid: 15970,
    },
    CidRange {
        start: 40395,
        end: 40396,
        cid: 18653,
    },
    CidRange {
        start: 40397,
        end: 40401,
        cid: 15973,
    },
    CidRange {
        start: 40403,
        end: 40405,
        cid: 15978,
    },
    CidRange {
        start: 40406,
        end: 40409,
        cid: 18656,
    },
    CidRange {
        start: 40410,
        end: 40443,
        cid: 15981,
    },
    CidRange {
        start: 40445,
        end: 40446,
        cid: 16015,
    },
    CidRange {
        start: 40512,
        end: 40514,
        cid: 16017,
    },
    CidRange {
        start: 40516,
        end: 40542,
        cid: 16021,
    },
    CidRange {
        start: 40544,
        end: 40546,
        cid: 16049,
    },
    CidRange {
        start: 40548,
        end: 40549,
        cid: 16052,
    },
    CidRange {
        start: 40550,
        end: 40551,
        cid: 18664,
    },
    CidRange {
        start: 40555,
        end: 40560,
        cid: 18667,
    },
    CidRange {
        start: 40564,
        end: 40566,
        cid: 18674,
    },
    CidRange {
        start: 40567,
        end: 40568,
        cid: 16058,
    },
    CidRange {
        start: 40609,
        end: 40610,
        cid: 16063,
    },
    CidRange {
        start: 40612,
        end: 40614,
        cid: 16065,
    },
    CidRange {
        start: 40615,
        end: 40616,
        cid: 18681,
    },
    CidRange {
        start: 40620,
        end: 40621,
        cid: 16070,
    },
    CidRange {
        start: 40623,
        end: 40625,
        cid: 16072,
    },
    CidRange {
        start: 40626,
        end: 40627,
        cid: 18685,
    },
    CidRange {
        start: 40630,
        end: 40631,
        cid: 16076,
    },
    CidRange {
        start: 40634,
        end: 40635,
        cid: 18689,
    },
    CidRange {
        start: 40637,
        end: 40638,
        cid: 18691,
    },
    CidRange {
        start: 40639,
        end: 40640,
        cid: 16080,
    },
    CidRange {
        start: 40641,
        end: 40643,
        cid: 18693,
    },
    CidRange {
        start: 40644,
        end: 40645,
        cid: 16082,
    },
    CidRange {
        start: 40647,
        end: 40650,
        cid: 16084,
    },
    CidRange {
        start: 40651,
        end: 40652,
        cid: 18697,
    },
    CidRange {
        start: 40654,
        end: 40655,
        cid: 18699,
    },
    CidRange {
        start: 40656,
        end: 40657,
        cid: 16090,
    },
    CidRange {
        start: 40660,
        end: 40661,
        cid: 18702,
    },
    CidRange {
        start: 40662,
        end: 40663,
        cid: 16093,
    },
    CidRange {
        start: 40664,
        end: 40665,
        cid: 18704,
    },
    CidRange {
        start: 40666,
        end: 40686,
        cid: 16095,
    },
    CidRange {
        start: 40688,
        end: 40689,
        cid: 16117,
    },
    CidRange {
        start: 40691,
        end: 40693,
        cid: 16119,
    },
    CidRange {
        start: 40694,
        end: 40696,
        cid: 18707,
    },
    CidRange {
        start: 40697,
        end: 40698,
        cid: 16122,
    },
    CidRange {
        start: 40768,
        end: 40770,
        cid: 16127,
    },
    CidRange {
        start: 40772,
        end: 40775,
        cid: 16130,
    },
    CidRange {
        start: 40777,
        end: 40778,
        cid: 16134,
    },
    CidRange {
        start: 40779,
        end: 40780,
        cid: 18713,
    },
    CidRange {
        start: 40781,
        end: 40799,
        cid: 16136,
    },
    CidRange {
        start: 40801,
        end: 40805,
        cid: 16156,
    },
    CidRange {
        start: 40807,
        end: 40808,
        cid: 18715,
    },
    CidRange {
        start: 40809,
        end: 40815,
        cid: 16163,
    },
    CidRange {
        start: 40817,
        end: 40830,
        cid: 16170,
    },
    CidRange {
        start: 40865,
        end: 40884,
        cid: 16184,
    },
    CidRange {
        start: 40886,
        end: 40890,
        cid: 16204,
    },
    CidRange {
        start: 40892,
        end: 40894,
        cid: 16209,
    },
    CidRange {
        start: 40898,
        end: 40906,
        cid: 16213,
    },
    CidRange {
        start: 40909,
        end: 40915,
        cid: 16224,
    },
    CidRange {
        start: 40917,
        end: 40919,
        cid: 16232,
    },
    CidRange {
        start: 40921,
        end: 40931,
        cid: 16236,
    },
    CidRange {
        start: 40933,
        end: 40952,
        cid: 16247,
    },
    CidRange {
        start: 40954,
        end: 40958,
        cid: 16267,
    },
    CidRange {
        start: 41025,
        end: 41030,
        cid: 16272,
    },
    CidRange {
        start: 41032,
        end: 41044,
        cid: 16278,
    },
    CidRange {
        start: 41046,
        end: 41058,
        cid: 16291,
    },
    CidRange {
        start: 41060,
        end: 41068,
        cid: 16305,
    },
    CidRange {
        start: 41070,
        end: 41078,
        cid: 16314,
    },
    CidRange {
        start: 41080,
        end: 41082,
        cid: 16324,
    },
    CidRange {
        start: 41084,
        end: 41086,
        cid: 16327,
    },
    CidRange {
        start: 41123,
        end: 41126,
        cid: 16331,
    },
    CidRange {
        start: 41128,
        end: 41156,
        cid: 16335,
    },
    CidRange {
        start: 41158,
        end: 41167,
        cid: 16364,
    },
    CidRange {
        start: 41169,
        end: 41172,
        cid: 16374,
    },
    CidRange {
        start: 41174,
        end: 41182,
        cid: 16379,
    },
    CidRange {
        start: 41184,
        end: 41186,
        cid: 16389,
    },
    CidRange {
        start: 41189,
        end: 41190,
        cid: 16393,
    },
    CidRange {
        start: 41191,
        end: 41197,
        cid: 18736,
    },
    CidRange {
        start: 41199,
        end: 41201,
        cid: 18743,
    },
    CidRange {
        start: 41203,
        end: 41214,
        cid: 18746,
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
        start: 50849,
        end: 50878,
        cid: 506,
    },
    CidRange {
        start: 50879,
        end: 50894,
        cid: 537,
    },
    CidRange {
        start: 50896,
        end: 50898,
        cid: 554,
    },
    CidRange {
        start: 50904,
        end: 50909,
        cid: 13747,
    },
    CidRange {
        start: 50912,
        end: 50942,
        cid: 13754,
    },
    CidRange {
        start: 51008,
        end: 51070,
        cid: 13785,
    },
    CidRange {
        start: 51105,
        end: 51198,
        cid: 13848,
    },
    CidRange {
        start: 51264,
        end: 51326,
        cid: 13942,
    },
    CidRange {
        start: 51361,
        end: 51364,
        cid: 14005,
    },
    CidRange {
        start: 51405,
        end: 51411,
        cid: 14049,
    },
    CidRange {
        start: 51412,
        end: 51414,
        cid: 17606,
    },
    CidRange {
        start: 51415,
        end: 51423,
        cid: 17692,
    },
    CidRange {
        start: 51425,
        end: 51432,
        cid: 17701,
    },
    CidRange {
        start: 51434,
        end: 51440,
        cid: 17709,
    },
    CidRange {
        start: 51445,
        end: 51454,
        cid: 18834,
    },
    CidRange {
        start: 51520,
        end: 51529,
        cid: 5996,
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
        start: 63958,
        end: 63998,
        cid: 14056,
    },
    CidRange {
        start: 64064,
        end: 64094,
        cid: 16395,
    },
    CidRange {
        start: 64096,
        end: 64101,
        cid: 16427,
    },
    CidRange {
        start: 64103,
        end: 64126,
        cid: 16434,
    },
    CidRange {
        start: 64161,
        end: 64168,
        cid: 16458,
    },
    CidRange {
        start: 64169,
        end: 64170,
        cid: 18758,
    },
    CidRange {
        start: 64171,
        end: 64188,
        cid: 16468,
    },
    CidRange {
        start: 64190,
        end: 64196,
        cid: 16487,
    },
    CidRange {
        start: 64198,
        end: 64212,
        cid: 16495,
    },
    CidRange {
        start: 64214,
        end: 64254,
        cid: 16511,
    },
    CidRange {
        start: 64320,
        end: 64327,
        cid: 16552,
    },
    CidRange {
        start: 64329,
        end: 64338,
        cid: 16561,
    },
    CidRange {
        start: 64340,
        end: 64365,
        cid: 16572,
    },
    CidRange {
        start: 64367,
        end: 64382,
        cid: 16599,
    },
    CidRange {
        start: 64417,
        end: 64418,
        cid: 16615,
    },
    CidRange {
        start: 64420,
        end: 64439,
        cid: 16618,
    },
    CidRange {
        start: 64441,
        end: 64446,
        cid: 16639,
    },
    CidRange {
        start: 64448,
        end: 64460,
        cid: 16645,
    },
    CidRange {
        start: 64462,
        end: 64498,
        cid: 16658,
    },
    CidRange {
        start: 64500,
        end: 64504,
        cid: 16696,
    },
    CidRange {
        start: 64506,
        end: 64510,
        cid: 16702,
    },
    CidRange {
        start: 64576,
        end: 64585,
        cid: 16707,
    },
    CidRange {
        start: 64587,
        end: 64590,
        cid: 16717,
    },
    CidRange {
        start: 64592,
        end: 64593,
        cid: 16721,
    },
    CidRange {
        start: 64595,
        end: 64610,
        cid: 16723,
    },
    CidRange {
        start: 64612,
        end: 64619,
        cid: 16739,
    },
    CidRange {
        start: 64622,
        end: 64628,
        cid: 16749,
    },
    CidRange {
        start: 64630,
        end: 64638,
        cid: 16756,
    },
    CidRange {
        start: 64673,
        end: 64696,
        cid: 16765,
    },
    CidRange {
        start: 64698,
        end: 64699,
        cid: 16789,
    },
    CidRange {
        start: 64700,
        end: 64701,
        cid: 18770,
    },
    CidRange {
        start: 64702,
        end: 64715,
        cid: 16792,
    },
    CidRange {
        start: 64717,
        end: 64737,
        cid: 16807,
    },
    CidRange {
        start: 64740,
        end: 64749,
        cid: 16829,
    },
    CidRange {
        start: 64751,
        end: 64752,
        cid: 16839,
    },
    CidRange {
        start: 64754,
        end: 64766,
        cid: 16842,
    },
    CidRange {
        start: 64832,
        end: 64840,
        cid: 16855,
    },
    CidRange {
        start: 64842,
        end: 64873,
        cid: 16864,
    },
    CidRange {
        start: 64875,
        end: 64894,
        cid: 16897,
    },
    CidRange {
        start: 64929,
        end: 64950,
        cid: 16917,
    },
    CidRange {
        start: 64953,
        end: 64954,
        cid: 16941,
    },
    CidRange {
        start: 64956,
        end: 64994,
        cid: 16944,
    },
    CidRange {
        start: 64996,
        end: 65008,
        cid: 16984,
    },
    CidRange {
        start: 65011,
        end: 65022,
        cid: 16998,
    },
    CidRange {
        start: 65088,
        end: 65105,
        cid: 17010,
    },
    CidRange {
        start: 65107,
        end: 65132,
        cid: 17029,
    },
    CidRange {
        start: 65136,
        end: 65143,
        cid: 17057,
    },
    CidRange {
        start: 65145,
        end: 65150,
        cid: 17065,
    },
    CidRange {
        start: 65185,
        end: 65193,
        cid: 17071,
    },
    CidRange {
        start: 65195,
        end: 65244,
        cid: 17080,
    },
    CidRange {
        start: 65246,
        end: 65247,
        cid: 18781,
    },
    CidRange {
        start: 65248,
        end: 65260,
        cid: 17131,
    },
    CidRange {
        start: 65261,
        end: 65262,
        cid: 18783,
    },
    CidRange {
        start: 65263,
        end: 65278,
        cid: 17144,
    },
];

const CID_RANGE_V: [CidRange; 13] = [
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
    CidRange {
        start: 50916,
        end: 50917,
        cid: 14097,
    },
];

pub const HKSCS_B5_H: CMap = CMap {
    name: b"HKscs-B5-H",
    cid_system_info: CidSystemInfo {
        registry: b"Adobe",
        ordering: b"CNS1",
        supplement: 6,
    },
    writing_mode: CMapWritingMode::Horizontal,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: Cow::Borrowed(&CID_CHARS_H),
    cid_range: Cow::Borrowed(&CID_RANGE_H),
};

pub const HKSCS_B5_V: CMap = CMap {
    name: b"HKscs-B5-V",
    cid_system_info: CidSystemInfo {
        registry: b"Adobe",
        ordering: b"CNS1",
        supplement: 6,
    },
    writing_mode: CMapWritingMode::Vertical,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: Cow::Borrowed(&NO_CID_CHARS),
    cid_range: Cow::Borrowed(&CID_RANGE_V),
};
