use std::borrow::Cow;

use crate::cmaps::NO_CID_CHARS;
use crate::font::cmap::{CMap, CMapWritingMode, CidChar, CidRange, Codespace, CodespaceRange};
use crate::font::font::CidSystemInfo;

const CODE_SPACE: [CodespaceRange; 0] = [];

const CID_RANGE_H: [CidRange; 1] = [CidRange {
    start: 32,
    end: 126,
    cid: 1,
}];

const CID_CHARS_V: [CidChar; 6] = [
    CidChar {
        char: 41291,
        cid: 13646,
    },
    CidChar {
        char: 41292,
        cid: 109,
    },
    CidChar {
        char: 41302,
        cid: 312,
    },
    CidChar {
        char: 41304,
        cid: 122,
    },
    CidChar {
        char: 41306,
        cid: 13743,
    },
    CidChar {
        char: 41308,
        cid: 13745,
    },
];

const CID_RANGE_V: [CidRange; 12] = [
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
        start: 41341,
        end: 41342,
        cid: 130,
    },
    CidRange {
        start: 41377,
        end: 41378,
        cid: 134,
    },
    CidRange {
        start: 41379,
        end: 41380,
        cid: 138,
    },
    CidRange {
        start: 50916,
        end: 50917,
        cid: 14097,
    },
];

pub const ETENMS_B5_H: CMap = CMap {
    name: b"ETenms-B5-H",
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

pub const ETENMS_B5_V: CMap = CMap {
    name: b"ETenms-B5-V",
    cid_system_info: CidSystemInfo {
        registry: b"Adobe",
        ordering: b"CNS1",
        supplement: 0,
    },
    writing_mode: CMapWritingMode::Vertical,
    codespace: Codespace::from(Cow::Borrowed(&CODE_SPACE)),
    cid_chars: Cow::Borrowed(&CID_CHARS_V),
    cid_range: Cow::Borrowed(&CID_RANGE_V),
};