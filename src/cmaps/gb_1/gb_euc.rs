use std::borrow::Cow;

use crate::cmaps::cmap::{
    CMap, CMapWritingMode, CidRange, Codespace, CodespaceRange, ADOBE_REGISTRY, NO_BASE_FONT_CHARS,
    NO_CID_CHARS,
};
use crate::font::font::CidSystemInfo;

use super::GB_1;

const CODE_SPACE: [CodespaceRange; 2] = [
    [0..=0, 0..=0, 0..=0, 0..=128],
    [0..=0, 0..=0, 161..=254, 161..=254],
];

const CID_RANGE_H: [CidRange; 90] = [
    CidRange {
        start: 32,
        end: 32,
        cid: 7716,
    },
    CidRange {
        start: 33,
        end: 126,
        cid: 814,
    },
    CidRange {
        start: 41377,
        end: 41470,
        cid: 96,
    },
    CidRange {
        start: 41649,
        end: 41698,
        cid: 190,
    },
    CidRange {
        start: 41701,
        end: 41710,
        cid: 240,
    },
    CidRange {
        start: 41713,
        end: 41724,
        cid: 250,
    },
    CidRange {
        start: 41889,
        end: 41982,
        cid: 262,
    },
    CidRange {
        start: 42145,
        end: 42227,
        cid: 356,
    },
    CidRange {
        start: 42401,
        end: 42486,
        cid: 439,
    },
    CidRange {
        start: 42657,
        end: 42680,
        cid: 525,
    },
    CidRange {
        start: 42689,
        end: 42741,
        cid: 549,
    },
    CidRange {
        start: 42913,
        end: 42945,
        cid: 602,
    },
    CidRange {
        start: 42961,
        end: 42993,
        cid: 635,
    },
    CidRange {
        start: 43169,
        end: 43200,
        cid: 668,
    },
    CidRange {
        start: 43205,
        end: 43242,
        cid: 700,
    },
    CidRange {
        start: 43428,
        end: 43503,
        cid: 738,
    },
    CidRange {
        start: 43681,
        end: 43774,
        cid: 814,
    },
    CidRange {
        start: 43937,
        end: 43968,
        cid: 908,
    },
    CidRange {
        start: 45217,
        end: 45310,
        cid: 940,
    },
    CidRange {
        start: 45473,
        end: 45566,
        cid: 1034,
    },
    CidRange {
        start: 45729,
        end: 45822,
        cid: 1128,
    },
    CidRange {
        start: 45985,
        end: 46078,
        cid: 1222,
    },
    CidRange {
        start: 46241,
        end: 46334,
        cid: 1316,
    },
    CidRange {
        start: 46497,
        end: 46590,
        cid: 1410,
    },
    CidRange {
        start: 46753,
        end: 46846,
        cid: 1504,
    },
    CidRange {
        start: 47009,
        end: 47102,
        cid: 1598,
    },
    CidRange {
        start: 47265,
        end: 47358,
        cid: 1692,
    },
    CidRange {
        start: 47521,
        end: 47614,
        cid: 1786,
    },
    CidRange {
        start: 47777,
        end: 47870,
        cid: 1880,
    },
    CidRange {
        start: 48033,
        end: 48126,
        cid: 1974,
    },
    CidRange {
        start: 48289,
        end: 48382,
        cid: 2068,
    },
    CidRange {
        start: 48545,
        end: 48638,
        cid: 2162,
    },
    CidRange {
        start: 48801,
        end: 48894,
        cid: 2256,
    },
    CidRange {
        start: 49057,
        end: 49150,
        cid: 2350,
    },
    CidRange {
        start: 49313,
        end: 49406,
        cid: 2444,
    },
    CidRange {
        start: 49569,
        end: 49662,
        cid: 2538,
    },
    CidRange {
        start: 49825,
        end: 49918,
        cid: 2632,
    },
    CidRange {
        start: 50081,
        end: 50174,
        cid: 2726,
    },
    CidRange {
        start: 50337,
        end: 50430,
        cid: 2820,
    },
    CidRange {
        start: 50593,
        end: 50686,
        cid: 2914,
    },
    CidRange {
        start: 50849,
        end: 50942,
        cid: 3008,
    },
    CidRange {
        start: 51105,
        end: 51198,
        cid: 3102,
    },
    CidRange {
        start: 51361,
        end: 51454,
        cid: 3196,
    },
    CidRange {
        start: 51617,
        end: 51710,
        cid: 3290,
    },
    CidRange {
        start: 51873,
        end: 51966,
        cid: 3384,
    },
    CidRange {
        start: 52129,
        end: 52222,
        cid: 3478,
    },
    CidRange {
        start: 52385,
        end: 52478,
        cid: 3572,
    },
    CidRange {
        start: 52641,
        end: 52734,
        cid: 3666,
    },
    CidRange {
        start: 52897,
        end: 52990,
        cid: 3760,
    },
    CidRange {
        start: 53153,
        end: 53246,
        cid: 3854,
    },
    CidRange {
        start: 53409,
        end: 53502,
        cid: 3948,
    },
    CidRange {
        start: 53665,
        end: 53758,
        cid: 4042,
    },
    CidRange {
        start: 53921,
        end: 54014,
        cid: 4136,
    },
    CidRange {
        start: 54177,
        end: 54270,
        cid: 4230,
    },
    CidRange {
        start: 54433,
        end: 54526,
        cid: 4324,
    },
    CidRange {
        start: 54689,
        end: 54782,
        cid: 4418,
    },
    CidRange {
        start: 54945,
        end: 55038,
        cid: 4512,
    },
    CidRange {
        start: 55201,
        end: 55289,
        cid: 4606,
    },
    CidRange {
        start: 55457,
        end: 55550,
        cid: 4695,
    },
    CidRange {
        start: 55713,
        end: 55806,
        cid: 4789,
    },
    CidRange {
        start: 55969,
        end: 56062,
        cid: 4883,
    },
    CidRange {
        start: 56225,
        end: 56318,
        cid: 4977,
    },
    CidRange {
        start: 56481,
        end: 56574,
        cid: 5071,
    },
    CidRange {
        start: 56737,
        end: 56830,
        cid: 5165,
    },
    CidRange {
        start: 56993,
        end: 57086,
        cid: 5259,
    },
    CidRange {
        start: 57249,
        end: 57342,
        cid: 5353,
    },
    CidRange {
        start: 57505,
        end: 57598,
        cid: 5447,
    },
    CidRange {
        start: 57761,
        end: 57854,
        cid: 5541,
    },
    CidRange {
        start: 58017,
        end: 58110,
        cid: 5635,
    },
    CidRange {
        start: 58273,
        end: 58366,
        cid: 5729,
    },
    CidRange {
        start: 58529,
        end: 58622,
        cid: 5823,
    },
    CidRange {
        start: 58785,
        end: 58878,
        cid: 5917,
    },
    CidRange {
        start: 59041,
        end: 59134,
        cid: 6011,
    },
    CidRange {
        start: 59297,
        end: 59390,
        cid: 6105,
    },
    CidRange {
        start: 59553,
        end: 59646,
        cid: 6199,
    },
    CidRange {
        start: 59809,
        end: 59902,
        cid: 6293,
    },
    CidRange {
        start: 60065,
        end: 60158,
        cid: 6387,
    },
    CidRange {
        start: 60321,
        end: 60414,
        cid: 6481,
    },
    CidRange {
        start: 60577,
        end: 60670,
        cid: 6575,
    },
    CidRange {
        start: 60833,
        end: 60926,
        cid: 6669,
    },
    CidRange {
        start: 61089,
        end: 61182,
        cid: 6763,
    },
    CidRange {
        start: 61345,
        end: 61438,
        cid: 6857,
    },
    CidRange {
        start: 61601,
        end: 61694,
        cid: 6951,
    },
    CidRange {
        start: 61857,
        end: 61950,
        cid: 7045,
    },
    CidRange {
        start: 62113,
        end: 62206,
        cid: 7139,
    },
    CidRange {
        start: 62369,
        end: 62462,
        cid: 7233,
    },
    CidRange {
        start: 62625,
        end: 62718,
        cid: 7327,
    },
    CidRange {
        start: 62881,
        end: 62974,
        cid: 7421,
    },
    CidRange {
        start: 63137,
        end: 63230,
        cid: 7515,
    },
    CidRange {
        start: 63393,
        end: 63486,
        cid: 7609,
    },
];

const CID_RANGE_V: [CidRange; 20] = [
    CidRange {
        start: 41378,
        end: 41378,
        cid: 575,
    },
    CidRange {
        start: 41379,
        end: 41379,
        cid: 574,
    },
    CidRange {
        start: 41386,
        end: 41386,
        cid: 598,
    },
    CidRange {
        start: 41387,
        end: 41388,
        cid: 7704,
    },
    CidRange {
        start: 41389,
        end: 41389,
        cid: 599,
    },
    CidRange {
        start: 41394,
        end: 41407,
        cid: 582,
    },
    CidRange {
        start: 41470,
        end: 41470,
        cid: 7706,
    },
    CidRange {
        start: 41889,
        end: 41889,
        cid: 578,
    },
    CidRange {
        start: 41896,
        end: 41897,
        cid: 580,
    },
    CidRange {
        start: 41900,
        end: 41900,
        cid: 573,
    },
    CidRange {
        start: 41902,
        end: 41902,
        cid: 7707,
    },
    CidRange {
        start: 41914,
        end: 41915,
        cid: 576,
    },
    CidRange {
        start: 41917,
        end: 41917,
        cid: 7708,
    },
    CidRange {
        start: 41919,
        end: 41919,
        cid: 579,
    },
    CidRange {
        start: 41947,
        end: 41947,
        cid: 7709,
    },
    CidRange {
        start: 41949,
        end: 41949,
        cid: 7710,
    },
    CidRange {
        start: 41951,
        end: 41951,
        cid: 600,
    },
    CidRange {
        start: 41979,
        end: 41979,
        cid: 596,
    },
    CidRange {
        start: 41981,
        end: 41981,
        cid: 597,
    },
    CidRange {
        start: 41982,
        end: 41982,
        cid: 7711,
    },
];

pub const GB_EUC_H: CMap = CMap {
    name: Cow::Borrowed(b"GB-EUC-H"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(GB_1),
        supplement: 0,
    },
    writing_mode: CMapWritingMode::Horizontal,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: NO_CID_CHARS,
    cid_range: Cow::Borrowed(&CID_RANGE_H),
    base_font_chars: NO_BASE_FONT_CHARS,
};

pub const GB_EUC_V: CMap = CMap {
    name: Cow::Borrowed(b"GB-EUC-V"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(GB_1),
        supplement: 0,
    },
    writing_mode: CMapWritingMode::Vertical,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: NO_CID_CHARS,
    cid_range: Cow::Borrowed(&CID_RANGE_V),
    base_font_chars: NO_BASE_FONT_CHARS,
};
