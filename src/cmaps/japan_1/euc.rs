use std::borrow::Cow;

use crate::cmaps::cmap::{
    CMap, CMapWritingMode, CidRange, Codespace, CodespaceRange, ADOBE_REGISTRY, NO_BASE_FONT_CHARS,
    NO_CID_CHARS,
};
use crate::font::cid_font::CidSystemInfo;

use super::JAPAN_1;

const CODE_SPACE: [CodespaceRange; 3] = [
    [0..=0, 0..=0, 0..=0, 0..=128],
    [0..=0, 0..=0, 142..=142, 160..=223],
    [0..=0, 0..=0, 161..=254, 161..=254],
];

const CID_RANGE_H: [CidRange; 120] = [
    CidRange {
        start: 32,
        end: 126,
        cid: 231,
    },
    CidRange {
        start: 36512,
        end: 36575,
        cid: 326,
    },
    CidRange {
        start: 41377,
        end: 41470,
        cid: 633,
    },
    CidRange {
        start: 41633,
        end: 41646,
        cid: 727,
    },
    CidRange {
        start: 41658,
        end: 41665,
        cid: 741,
    },
    CidRange {
        start: 41674,
        end: 41680,
        cid: 749,
    },
    CidRange {
        start: 41692,
        end: 41706,
        cid: 756,
    },
    CidRange {
        start: 41714,
        end: 41721,
        cid: 771,
    },
    CidRange {
        start: 41726,
        end: 41726,
        cid: 779,
    },
    CidRange {
        start: 41904,
        end: 41913,
        cid: 780,
    },
    CidRange {
        start: 41921,
        end: 41946,
        cid: 790,
    },
    CidRange {
        start: 41953,
        end: 41978,
        cid: 816,
    },
    CidRange {
        start: 42145,
        end: 42227,
        cid: 842,
    },
    CidRange {
        start: 42401,
        end: 42486,
        cid: 925,
    },
    CidRange {
        start: 42657,
        end: 42680,
        cid: 1011,
    },
    CidRange {
        start: 42689,
        end: 42712,
        cid: 1035,
    },
    CidRange {
        start: 42913,
        end: 42945,
        cid: 1059,
    },
    CidRange {
        start: 42961,
        end: 42993,
        cid: 1092,
    },
    CidRange {
        start: 43169,
        end: 43169,
        cid: 7479,
    },
    CidRange {
        start: 43170,
        end: 43170,
        cid: 7481,
    },
    CidRange {
        start: 43171,
        end: 43171,
        cid: 7491,
    },
    CidRange {
        start: 43172,
        end: 43172,
        cid: 7495,
    },
    CidRange {
        start: 43173,
        end: 43173,
        cid: 7503,
    },
    CidRange {
        start: 43174,
        end: 43174,
        cid: 7499,
    },
    CidRange {
        start: 43175,
        end: 43175,
        cid: 7507,
    },
    CidRange {
        start: 43176,
        end: 43176,
        cid: 7523,
    },
    CidRange {
        start: 43177,
        end: 43177,
        cid: 7515,
    },
    CidRange {
        start: 43178,
        end: 43178,
        cid: 7531,
    },
    CidRange {
        start: 43179,
        end: 43179,
        cid: 7539,
    },
    CidRange {
        start: 43180,
        end: 43180,
        cid: 7480,
    },
    CidRange {
        start: 43181,
        end: 43181,
        cid: 7482,
    },
    CidRange {
        start: 43182,
        end: 43182,
        cid: 7494,
    },
    CidRange {
        start: 43183,
        end: 43183,
        cid: 7498,
    },
    CidRange {
        start: 43184,
        end: 43184,
        cid: 7506,
    },
    CidRange {
        start: 43185,
        end: 43185,
        cid: 7502,
    },
    CidRange {
        start: 43186,
        end: 43186,
        cid: 7514,
    },
    CidRange {
        start: 43187,
        end: 43187,
        cid: 7530,
    },
    CidRange {
        start: 43188,
        end: 43188,
        cid: 7522,
    },
    CidRange {
        start: 43189,
        end: 43189,
        cid: 7538,
    },
    CidRange {
        start: 43190,
        end: 43190,
        cid: 7554,
    },
    CidRange {
        start: 43191,
        end: 43191,
        cid: 7511,
    },
    CidRange {
        start: 43192,
        end: 43192,
        cid: 7526,
    },
    CidRange {
        start: 43193,
        end: 43193,
        cid: 7519,
    },
    CidRange {
        start: 43194,
        end: 43194,
        cid: 7534,
    },
    CidRange {
        start: 43195,
        end: 43195,
        cid: 7542,
    },
    CidRange {
        start: 43196,
        end: 43196,
        cid: 7508,
    },
    CidRange {
        start: 43197,
        end: 43197,
        cid: 7527,
    },
    CidRange {
        start: 43198,
        end: 43198,
        cid: 7516,
    },
    CidRange {
        start: 43199,
        end: 43199,
        cid: 7535,
    },
    CidRange {
        start: 43200,
        end: 43200,
        cid: 7545,
    },
    CidRange {
        start: 45217,
        end: 45310,
        cid: 1125,
    },
    CidRange {
        start: 45473,
        end: 45566,
        cid: 1219,
    },
    CidRange {
        start: 45729,
        end: 45822,
        cid: 1313,
    },
    CidRange {
        start: 45985,
        end: 46078,
        cid: 1407,
    },
    CidRange {
        start: 46241,
        end: 46334,
        cid: 1501,
    },
    CidRange {
        start: 46497,
        end: 46590,
        cid: 1595,
    },
    CidRange {
        start: 46753,
        end: 46846,
        cid: 1689,
    },
    CidRange {
        start: 47009,
        end: 47102,
        cid: 1783,
    },
    CidRange {
        start: 47265,
        end: 47358,
        cid: 1877,
    },
    CidRange {
        start: 47521,
        end: 47614,
        cid: 1971,
    },
    CidRange {
        start: 47777,
        end: 47870,
        cid: 2065,
    },
    CidRange {
        start: 48033,
        end: 48126,
        cid: 2159,
    },
    CidRange {
        start: 48289,
        end: 48382,
        cid: 2253,
    },
    CidRange {
        start: 48545,
        end: 48638,
        cid: 2347,
    },
    CidRange {
        start: 48801,
        end: 48894,
        cid: 2441,
    },
    CidRange {
        start: 49057,
        end: 49150,
        cid: 2535,
    },
    CidRange {
        start: 49313,
        end: 49406,
        cid: 2629,
    },
    CidRange {
        start: 49569,
        end: 49662,
        cid: 2723,
    },
    CidRange {
        start: 49825,
        end: 49918,
        cid: 2817,
    },
    CidRange {
        start: 50081,
        end: 50174,
        cid: 2911,
    },
    CidRange {
        start: 50337,
        end: 50430,
        cid: 3005,
    },
    CidRange {
        start: 50593,
        end: 50686,
        cid: 3099,
    },
    CidRange {
        start: 50849,
        end: 50942,
        cid: 3193,
    },
    CidRange {
        start: 51105,
        end: 51198,
        cid: 3287,
    },
    CidRange {
        start: 51361,
        end: 51454,
        cid: 3381,
    },
    CidRange {
        start: 51617,
        end: 51710,
        cid: 3475,
    },
    CidRange {
        start: 51873,
        end: 51966,
        cid: 3569,
    },
    CidRange {
        start: 52129,
        end: 52222,
        cid: 3663,
    },
    CidRange {
        start: 52385,
        end: 52478,
        cid: 3757,
    },
    CidRange {
        start: 52641,
        end: 52734,
        cid: 3851,
    },
    CidRange {
        start: 52897,
        end: 52990,
        cid: 3945,
    },
    CidRange {
        start: 53153,
        end: 53203,
        cid: 4039,
    },
    CidRange {
        start: 53409,
        end: 53502,
        cid: 4090,
    },
    CidRange {
        start: 53665,
        end: 53758,
        cid: 4184,
    },
    CidRange {
        start: 53921,
        end: 54014,
        cid: 4278,
    },
    CidRange {
        start: 54177,
        end: 54270,
        cid: 4372,
    },
    CidRange {
        start: 54433,
        end: 54526,
        cid: 4466,
    },
    CidRange {
        start: 54689,
        end: 54782,
        cid: 4560,
    },
    CidRange {
        start: 54945,
        end: 55038,
        cid: 4654,
    },
    CidRange {
        start: 55201,
        end: 55294,
        cid: 4748,
    },
    CidRange {
        start: 55457,
        end: 55550,
        cid: 4842,
    },
    CidRange {
        start: 55713,
        end: 55806,
        cid: 4936,
    },
    CidRange {
        start: 55969,
        end: 56062,
        cid: 5030,
    },
    CidRange {
        start: 56225,
        end: 56318,
        cid: 5124,
    },
    CidRange {
        start: 56481,
        end: 56574,
        cid: 5218,
    },
    CidRange {
        start: 56737,
        end: 56830,
        cid: 5312,
    },
    CidRange {
        start: 56993,
        end: 57086,
        cid: 5406,
    },
    CidRange {
        start: 57249,
        end: 57342,
        cid: 5500,
    },
    CidRange {
        start: 57505,
        end: 57598,
        cid: 5594,
    },
    CidRange {
        start: 57761,
        end: 57854,
        cid: 5688,
    },
    CidRange {
        start: 58017,
        end: 58110,
        cid: 5782,
    },
    CidRange {
        start: 58273,
        end: 58366,
        cid: 5876,
    },
    CidRange {
        start: 58529,
        end: 58622,
        cid: 5970,
    },
    CidRange {
        start: 58785,
        end: 58878,
        cid: 6064,
    },
    CidRange {
        start: 59041,
        end: 59134,
        cid: 6158,
    },
    CidRange {
        start: 59297,
        end: 59390,
        cid: 6252,
    },
    CidRange {
        start: 59553,
        end: 59646,
        cid: 6346,
    },
    CidRange {
        start: 59809,
        end: 59902,
        cid: 6440,
    },
    CidRange {
        start: 60065,
        end: 60158,
        cid: 6534,
    },
    CidRange {
        start: 60321,
        end: 60414,
        cid: 6628,
    },
    CidRange {
        start: 60577,
        end: 60670,
        cid: 6722,
    },
    CidRange {
        start: 60833,
        end: 60926,
        cid: 6816,
    },
    CidRange {
        start: 61089,
        end: 61182,
        cid: 6910,
    },
    CidRange {
        start: 61345,
        end: 61438,
        cid: 7004,
    },
    CidRange {
        start: 61601,
        end: 61694,
        cid: 7098,
    },
    CidRange {
        start: 61857,
        end: 61950,
        cid: 7192,
    },
    CidRange {
        start: 62113,
        end: 62206,
        cid: 7286,
    },
    CidRange {
        start: 62369,
        end: 62462,
        cid: 7380,
    },
    CidRange {
        start: 62625,
        end: 62628,
        cid: 7474,
    },
    CidRange {
        start: 62629,
        end: 62630,
        cid: 8284,
    },
];

const CID_RANGE_V: [CidRange; 27] = [
    CidRange {
        start: 41378,
        end: 41379,
        cid: 7887,
    },
    CidRange {
        start: 41393,
        end: 41394,
        cid: 7889,
    },
    CidRange {
        start: 41404,
        end: 41406,
        cid: 7891,
    },
    CidRange {
        start: 41409,
        end: 41413,
        cid: 7894,
    },
    CidRange {
        start: 41418,
        end: 41435,
        cid: 7899,
    },
    CidRange {
        start: 41441,
        end: 41441,
        cid: 7917,
    },
    CidRange {
        start: 42145,
        end: 42145,
        cid: 7918,
    },
    CidRange {
        start: 42147,
        end: 42147,
        cid: 7919,
    },
    CidRange {
        start: 42149,
        end: 42149,
        cid: 7920,
    },
    CidRange {
        start: 42151,
        end: 42151,
        cid: 7921,
    },
    CidRange {
        start: 42153,
        end: 42153,
        cid: 7922,
    },
    CidRange {
        start: 42179,
        end: 42179,
        cid: 7923,
    },
    CidRange {
        start: 42211,
        end: 42211,
        cid: 7924,
    },
    CidRange {
        start: 42213,
        end: 42213,
        cid: 7925,
    },
    CidRange {
        start: 42215,
        end: 42215,
        cid: 7926,
    },
    CidRange {
        start: 42222,
        end: 42222,
        cid: 7927,
    },
    CidRange {
        start: 42401,
        end: 42401,
        cid: 7928,
    },
    CidRange {
        start: 42403,
        end: 42403,
        cid: 7929,
    },
    CidRange {
        start: 42405,
        end: 42405,
        cid: 7930,
    },
    CidRange {
        start: 42407,
        end: 42407,
        cid: 7931,
    },
    CidRange {
        start: 42409,
        end: 42409,
        cid: 7932,
    },
    CidRange {
        start: 42435,
        end: 42435,
        cid: 7933,
    },
    CidRange {
        start: 42467,
        end: 42467,
        cid: 7934,
    },
    CidRange {
        start: 42469,
        end: 42469,
        cid: 7935,
    },
    CidRange {
        start: 42471,
        end: 42471,
        cid: 7936,
    },
    CidRange {
        start: 42478,
        end: 42478,
        cid: 7937,
    },
    CidRange {
        start: 42485,
        end: 42486,
        cid: 7938,
    },
];

pub const EUC_H: CMap = CMap {
    name: Cow::Borrowed(b"EUC-H"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(JAPAN_1),
        supplement: 1,
    },
    writing_mode: CMapWritingMode::Horizontal,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: NO_CID_CHARS,
    cid_range: Cow::Borrowed(&CID_RANGE_H),
    base_font_chars: NO_BASE_FONT_CHARS,
};

pub const EUC_V: CMap = CMap {
    name: Cow::Borrowed(b"EUC-V"),
    cid_system_info: CidSystemInfo {
        registry: Cow::Borrowed(ADOBE_REGISTRY),
        ordering: Cow::Borrowed(JAPAN_1),
        supplement: 1,
    },
    writing_mode: CMapWritingMode::Vertical,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: NO_CID_CHARS,
    cid_range: Cow::Borrowed(&CID_RANGE_V),
    base_font_chars: NO_BASE_FONT_CHARS,
};
