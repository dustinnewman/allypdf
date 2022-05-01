use crate::font::cmap::{CidChar, CMap};

pub mod cns_1;
pub mod gb_1;
pub mod japan_1;
pub mod korea_1;

pub const NO_CID_CHARS: [CidChar; 0] = [];

pub fn try_predefined_cmap(name: &[u8]) -> Option<Box<CMap<'static>>> {
    let cmap = match name {
        b"GB-EUC-H" => gb_1::gb_euc::GB_EUC_H,
        b"GB-EUC-V" => gb_1::gb_euc::GB_EUC_V,
        b"GBpc-EUC-H" => gb_1::gbpc_euc::GB_1_GBPC_EUC_H,
        b"GBpc-EUC-V" => gb_1::gbpc_euc::GB_1_GBPC_EUC_V,
        b"GBK-EUC-H" => gb_1::gbk_euc::GBK_EUC_H,
        b"GBK-EUC-V" => gb_1::gbk_euc::GBK_EUC_V,
        b"GBKp-EUC-H" => gb_1::gbkp_euc::GBKP_EUC_H,
        b"GBKp-EUC-V" => gb_1::gbkp_euc::GBKP_EUC_V,
        b"GBK2K-H" => gb_1::gbk2k::GBK2K_H,
        b"GBK2K-V" => gb_1::gbk2k::GBK2K_V,
        b"UniGB-UCS2-H" => gb_1::unigb_ucs2::UNIGB_UCS2_H,
        b"UniGB-UCS2-V" => gb_1::unigb_ucs2::UNIGB_UCS2_V,
        b"UniGB-UTF16-H" => gb_1::unigb_utf16::UNIGB_UTF16_H,
        b"UniGB-UTF16-V" => gb_1::unigb_utf16::UNIGB_UTF16_V,
        b"B5pc-H" => cns_1::b5pc::B5PC_H,
        b"B5pc-V" => cns_1::b5pc::B5PC_V,
        b"HKscs-B5-H" => cns_1::hkscs_b5::HKSCS_B5_H,
        b"HKscs-B5-V" => cns_1::hkscs_b5::HKSCS_B5_V,
        b"ETen-B5-H" => cns_1::eten_b5::ETEN_B5_H,
        b"ETen-B5-V" => cns_1::eten_b5::ETEN_B5_V,
        b"ETenms-B5-H" => cns_1::etenms_b5::ETENMS_B5_H,
        b"ETenms-B5-V" => cns_1::etenms_b5::ETENMS_B5_V,
        b"CNS-EUC-H" => cns_1::cns_euc::CNS_EUC_H,
        b"CNS-EUC-V" => cns_1::cns_euc::CNS_EUC_V,
        b"UniCNS-UCS2-H" => cns_1::unicns_ucs2::UNICNS_UCS2_H,
        b"UniCNS-UCS2-V" => cns_1::unicns_ucs2::UNICNS_UCS2_V,
        b"UniCNS-UTF16-H" => cns_1::unicns_utf16::UNICNS_UTF16_H,
        b"UniCNS-UTF16-V" => cns_1::unicns_utf16::UNICNS_UTF16_V,
        b"83pv-RKSJ-H" => japan_1::jp_83pv_rksj::JAPAN_1_83PV_RKSJ_H,
        b"90ms-RKSJ-H" => japan_1::jp_90ms_rksj::JAPAN_1_90MS_RKSJ_H,
        b"90ms-RKSJ-V" => japan_1::jp_90ms_rksj::JAPAN_1_90MS_RKSJ_V,
        b"90msp-RKSJ-H" => japan_1::jp_90msp_rksj::JAPAN_1_90MSP_RKSJ_H,
        b"90msp-RKSJ-V" => japan_1::jp_90msp_rksj::JAPAN_1_90MSP_RKSJ_V,
        b"90pv-RKSJ-H" => japan_1::jp_90pv_rksj::JAPAN_1_90PV_RKSJ_H,
        b"Add-RKSJ-H" => japan_1::add_rksj::ADD_RKSJ_H,
        b"Add-RKSJ-V" => japan_1::add_rksj::ADD_RKSJ_V,
        b"EUC-H" => japan_1::euc::EUC_H,
        b"EUC-V" => japan_1::euc::EUC_V,
        b"Ext-RKSJ-H" => japan_1::ext_rksj::EXT_RKSJ_H,
        b"Ext-RKSJ-V" => japan_1::ext_rksj::EXT_RKSJ_V,
        b"H" => japan_1::h_v::H,
        b"V" => japan_1::h_v::V,
        b"UniJIS-UCS2-H" => japan_1::unijis_ucs2::JAPAN_1_UNIJIS_UCS2_H,
        b"UniJIS-UCS2-V" => japan_1::unijis_ucs2::JAPAN_1_UNIJIS_UCS2_V,
        b"UniJIS-UCS2-HW-H" => japan_1::unijis_ucs2_hw::UNIJIS_UCS2_HW_H,
        b"UniJIS-UCS2-HW-V" => japan_1::unijis_ucs2_hw::UNIJIS_UCS2_HW_V,
        b"UniJIS-UTF16-H" => japan_1::unijis_utf16::UNIJIS_UTF16_H,
        b"UniJIS-UTF16-V" => japan_1::unijis_utf16::UNIJIS_UTF16_V,
        b"KSC-EUC-H" => korea_1::ksc_euc::KSC_EUC_H,
        b"KSC-EUC-V" => korea_1::ksc_euc::KSC_EUC_V,
        b"KSCms-UHC-H" => korea_1::kscms_uhc::KSCMS_UHC_H,
        b"KSCms-UHC-V" => korea_1::kscms_uhc::KSCMS_UHC_V,
        b"KSCms-UHC-HW-H" => korea_1::kscms_uhc_hw::KSCMS_UHC_HW_H,
        b"KSCms-UHC-HW-V" => korea_1::kscms_uhc_hw::KSCMS_UHC_HW_V,
        b"KSCpc-EUC-H" => korea_1::kscpc_euc::KSCPC_EUC_H,
        b"UniKS-UCS2-H" => korea_1::uniks_ucs2::UNIKS_UCS2_H,
        b"UniKS-UCS2-V" => korea_1::uniks_ucs2::UNIKS_UCS2_V,
        b"UniKS-UTF16-H" => korea_1::uniks_utf16::UNIKS_UTF16_H,
        b"UniKS-UTF16-V" => korea_1::uniks_utf16::UNIKS_UTF16_V,
        _ => return None
    };
    Some(Box::new(cmap))
}
