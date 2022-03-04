use std::{
    convert::TryFrom,
    ops::{Deref, DerefMut},
};

use crate::{
    error::PdfError,
    parser::parser::{Dictionary, Object, ObjectKind, Name},
};

pub const ENCODING_SIZE: usize = 256;

#[derive(Debug)]
pub struct Encoding<'a>([Option<&'a [u8]>; ENCODING_SIZE]);

const A_UPPER: &[u8] = b"A";
const AE_UPPER: &[u8] = b"AE";
const AE_SMALL_UPPER: &[u8] = b"AEsmall";
const A_ACUTE_UPPER: &[u8] = b"Aacute";
const A_ACUTE_SMALL_UPPER: &[u8] = b"Aacutesmall";
const A_CIRCUMFLEX_UPPER: &[u8] = b"Acircumflex";
const A_CIRCUMFLEX_SMALL_UPPER: &[u8] = b"Acircumflexsmall";
const ACUTE_SMALL: &[u8] = b"Acutesmall";
const A_DIERESIS_UPPER: &[u8] = b"Adieresis";
const A_DIERESIS_SMALL_UPPER: &[u8] = b"Adieresissmall";
const A_GRAVE_UPPER: &[u8] = b"Agrave";
const A_GRAVE_SMALL_UPPER: &[u8] = b"Agravesmall";
const A_RING_UPPER: &[u8] = b"Aring";
const A_RING_SMALL_UPPER: &[u8] = b"Aringsmall";
const A_UPPER_SMALL: &[u8] = b"Asmall";
const A_TILDE_UPPER: &[u8] = b"Atilde";
const A_TILDE_SMALL_UPPER: &[u8] = b"Atildesmall";
const B_UPPER: &[u8] = b"B";
const B_UPPER_SMALL: &[u8] = b"Bsmall";
const BREVE_SMALL: &[u8] = b"Brevesmall";
const C_UPPER: &[u8] = b"C";
const CARON_SMALL: &[u8] = b"Caronsmall";
const C_CEDILLA_UPPER: &[u8] = b"Ccedilla";
const C_CEDILLA_SMALL_UPPER: &[u8] = b"Ccedillasmall";
const CEDILLA_SMALL: &[u8] = b"Cedillasmall";
const CIRCUMFLEX_SMALL: &[u8] = b"Circumflexsmall";
const C_UPPER_SMALL: &[u8] = b"Csmall";
const D_UPPER: &[u8] = b"D";
const DELTA_UPPER: &[u8] = b"Delta";
const DIERESIS_SMALL: &[u8] = b"Dieresissmall";
const DOT_ACCENT_SMALL: &[u8] = b"Dotaccentsmall";
const D_UPPER_SMALL: &[u8] = b"Dsmall";
const E_UPPER: &[u8] = b"E";
const E_ACUTE_UPPER: &[u8] = b"Eacute";
const E_ACUTE_SMALL_UPPER: &[u8] = b"Eacutesmall";
const E_CIRCUMFLEX_UPPER: &[u8] = b"Ecircumflex";
const E_CIRCUMFLEX_SMALL_UPPER: &[u8] = b"Ecircumflexsmall";
const E_DIERESIS_UPPER: &[u8] = b"Edieresis";
const E_DIERESIS_SMALL_UPPER: &[u8] = b"Edieresissmall";
const E_GRAVE_UPPER: &[u8] = b"Egrave";
const E_GRAVE_SMALL_UPPER: &[u8] = b"Egravesmall";
const E_UPPER_SMALL: &[u8] = b"Esmall";
const ETH_UPPER: &[u8] = b"Eth";
const ETH_SMALL_UPPER: &[u8] = b"Ethsmall";
const EURO: &[u8] = b"Euro";
const F_UPPER: &[u8] = b"F";
const F_UPPER_SMALL: &[u8] = b"Fsmall";
const G_UPPER: &[u8] = b"G";
const G_UPPER_SMALL: &[u8] = b"Gsmall";
const GRAVE_SMALL: &[u8] = b"Gravesmall";
const H_UPPER: &[u8] = b"H";
const H_UPPER_SMALL: &[u8] = b"Hsmall";
const HUNGARUMLAUT_SMALL: &[u8] = b"Hungarumlautsmall";
const I_UPPER: &[u8] = b"I";
const I_ACUTE_UPPER: &[u8] = b"Iacute";
const I_ACUTE_SMALL_UPPER: &[u8] = b"Iacutesmall";
const I_CIRCUMFLEX_UPPER: &[u8] = b"Icircumflex";
const I_CIRCUMFLEX_SMALL_UPPER: &[u8] = b"Icircumflexsmall";
const I_DIERESIS_UPPER: &[u8] = b"Idieresis";
const I_DIERESIS_SMALL_UPPER: &[u8] = b"Idieresissmall";
const I_GRAVE_UPPER: &[u8] = b"Igrave";
const I_GRAVE_SMALL_UPPER: &[u8] = b"Igravesmall";
const I_UPPER_SMALL: &[u8] = b"Ismall";
const J_UPPER: &[u8] = b"J";
const J_UPPER_SMALL: &[u8] = b"Jsmall";
const K_UPPER: &[u8] = b"K";
const K_UPPER_SMALL: &[u8] = b"Ksmall";
const L_UPPER: &[u8] = b"L";
const L_SLASH_UPPER: &[u8] = b"Lslash";
const L_SLASH_SMALL_UPPER: &[u8] = b"Lslashsmall";
const L_UPPER_SMALL: &[u8] = b"Lsmall";
const M_UPPER: &[u8] = b"M";
const MACRON_SMALL: &[u8] = b"Macronsmall";
const M_UPPER_SMALL: &[u8] = b"Msmall";
const N_UPPER: &[u8] = b"N";
const N_UPPER_SMALL: &[u8] = b"Nsmall";
const N_TILDE_UPPER: &[u8] = b"Ntilde";
const N_TILDE_SMALL_UPPER: &[u8] = b"Ntildesmall";
const O_UPPER: &[u8] = b"O";
const OE_UPPER: &[u8] = b"OE";
const OE_SMALL_UPPER: &[u8] = b"OEsmall";
const O_ACUTE_UPPER: &[u8] = b"Oacute";
const O_ACUTE_SMALL_UPPER: &[u8] = b"Oacutesmall";
const O_CIRCUMFLEX_UPPER: &[u8] = b"Ocircumflex";
const O_CIRCUMFLEX_SMALL_UPPER: &[u8] = b"Ocircumflexsmall";
const O_DIERESIS_UPPER: &[u8] = b"Odieresis";
const O_DIERESIS_SMALL_UPPER: &[u8] = b"Odieresissmall";
const OGONEK_SMALL: &[u8] = b"Ogoneksmall";
const O_GRAVE_UPPER: &[u8] = b"Ograve";
const O_GRAVE_SMALL_UPPER: &[u8] = b"Ogravesmall";
const OMEGA_UPPER: &[u8] = b"Omega";
const O_SLASH_UPPER: &[u8] = b"Oslash";
const O_SLASH_SMALL_UPPER: &[u8] = b"Oslashsmall";
const O_UPPER_SMALL: &[u8] = b"Osmall";
const O_TILDE_UPPER: &[u8] = b"Otilde";
const O_TILDE_SMALL_UPPER: &[u8] = b"Otildesmall";
const P_UPPER: &[u8] = b"P";
const P_UPPER_SMALL: &[u8] = b"Psmall";
const Q_UPPER: &[u8] = b"Q";
const Q_UPPER_SMALL: &[u8] = b"Qsmall";
const R_UPPER: &[u8] = b"R";
const R_UPPER_SMALL: &[u8] = b"Rsmall";
const RING_SMALL: &[u8] = b"Ringsmall";
const S_UPPER: &[u8] = b"S";
const S_UPPER_SMALL: &[u8] = b"Ssmall";
const S_CARON_UPPER: &[u8] = b"Scaron";
const S_CARON_SMALL_UPPER: &[u8] = b"Scaronsmall";
const T_UPPER: &[u8] = b"T";
const THORN_UPPER: &[u8] = b"Thorn";
const THORN_SMALL_UPPER: &[u8] = b"Thornsmall";
const TILDE_SMALL: &[u8] = b"Tildesmall";
const T_UPPER_SMALL: &[u8] = b"Tsmall";
const U_UPPER: &[u8] = b"U";
const U_ACUTE_UPPER: &[u8] = b"Uacute";
const U_ACUTE_SMALL_UPPER: &[u8] = b"Uacutesmall";
const U_CIRCUMFLEX_UPPER: &[u8] = b"Ucircumflex";
const U_CIRCUMFLEX_SMALL_UPPER: &[u8] = b"Ucircumflexsmall";
const U_DIERESIS_UPPER: &[u8] = b"Udieresis";
const U_DIERESIS_SMALL_UPPER: &[u8] = b"Udieresissmall";
const U_GRAVE_UPPER: &[u8] = b"Ugrave";
const U_GRAVE_SMALL_UPPER: &[u8] = b"Ugravesmall";
const U_UPPER_SMALL: &[u8] = b"Usmall";
const V_UPPER: &[u8] = b"V";
const V_UPPER_SMALL: &[u8] = b"Vsmall";
const W_UPPER: &[u8] = b"W";
const W_UPPER_SMALL: &[u8] = b"Wsmall";
const X_UPPER: &[u8] = b"X";
const X_UPPER_SMALL: &[u8] = b"Xsmall";
const Y_UPPER: &[u8] = b"Y";
const Y_UPPER_SMALL: &[u8] = b"Ysmall";
const Y_ACUTE_UPPER: &[u8] = b"Yacute";
const Y_ACUTE_SMALL_UPPER: &[u8] = b"Yacutesmall";
const Y_DIERESIS_UPPER: &[u8] = b"Ydieresis";
const Y_DIERESIS_SMALL_UPPER: &[u8] = b"Ydieresissmall";
const Z_UPPER: &[u8] = b"Z";
const Z_CARON_UPPER: &[u8] = b"Zcaron";
const Z_CARON_SMALL_UPPER: &[u8] = b"Zcaronsmall";
const Z_UPPER_SMALL: &[u8] = b"Zsmall";
const A_LOWER: &[u8] = b"a";
const A_ACUTE_LOWER: &[u8] = b"aacute";
const A_CIRCUMFLEX_LOWER: &[u8] = b"acircumflex";
const ACUTE: &[u8] = b"acute";
const A_DIERESIS_LOWER: &[u8] = b"adieresis";
const AE_LOWER: &[u8] = b"ae";
const A_GRAVE_LOWER: &[u8] = b"agrave";
const AMPERSAND: &[u8] = b"ampersand";
const AMPERSAND_SMALL: &[u8] = b"ampersandsmall";
const APPLE: &[u8] = b"apple";
const APPROX_EQUAL: &[u8] = b"approxequal";
const A_RING_LOWER: &[u8] = b"aring";
const ASCII_CIRCUM: &[u8] = b"asciicircum";
const ASCII_TILDE: &[u8] = b"asciitilde";
const ASTERISK: &[u8] = b"asterisk";
const A_SUPERIOR_LOWER: &[u8] = b"asuperior";
const AT: &[u8] = b"at";
const A_TILDE_LOWER: &[u8] = b"atilde";
const B_LOWER: &[u8] = b"b";
const BACKSLASH: &[u8] = b"backslash";
const BAR: &[u8] = b"bar";
const BRACE_LEFT: &[u8] = b"braceleft";
const BRACE_RIGHT: &[u8] = b"braceright";
const BRACKET_LEFT: &[u8] = b"bracketleft";
const BRACKET_RIGHT: &[u8] = b"bracketright";
const BREVE: &[u8] = b"breve";
const BROKEN_BAR: &[u8] = b"brokenbar";
const B_SUPERIOR_LOWER: &[u8] = b"bsuperior";
const BULLET: &[u8] = b"bullet";
const C_LOWER: &[u8] = b"c";
const CARON: &[u8] = b"caron";
const C_CEDILLA_LOWER: &[u8] = b"ccedilla";
const CEDILLA: &[u8] = b"cedilla";
const CENT: &[u8] = b"cent";
const CENT_INFERIOR: &[u8] = b"centinferior";
const CENT_OLD_STYLE: &[u8] = b"centoldstyle";
const CIRCUMFLEX: &[u8] = b"circumflex";
const COLON: &[u8] = b"colon";
const COLON_MONETARY: &[u8] = b"colonmonetary";
const COMMA: &[u8] = b"comma";
const COMMA_INFERIOR: &[u8] = b"commainferior";
const COMMA_SUPERIOR: &[u8] = b"commasuperior";
const COPYRIGHT: &[u8] = b"copyright";
const C_SUPERIOR_LOWER: &[u8] = b"csuperior";
const CURRENCY: &[u8] = b"currency";
const D_LOWER: &[u8] = b"d";
const DAGGER: &[u8] = b"dagger";
const DAGGER_DOUBLE: &[u8] = b"daggerdbl";
const DEGREE: &[u8] = b"degree";
const DIERESIS: &[u8] = b"dieresis";
const DIVIDE: &[u8] = b"divide";
const DOLLAR: &[u8] = b"dollar";
const DOLLAR_INFERIOR: &[u8] = b"dollarinferior";
const DOLLAR_OLD_STYLE: &[u8] = b"dollaroldstyle";
const DOLLAR_SUPERIOR: &[u8] = b"dollarsuperior";
const DOT_ACCENT: &[u8] = b"dotaccent";
const D_SUPERIOR_LOWER: &[u8] = b"dsuperior";
const DOTLESS_I: &[u8] = b"dotlessi";
const E_LOWER: &[u8] = b"e";
const E_ACUTE_LOWER: &[u8] = b"eacute";
const E_CIRCUMFLEX_LOWER: &[u8] = b"ecircumflex";
const E_DIERESIS_LOWER: &[u8] = b"edieresis";
const E_GRAVE_LOWER: &[u8] = b"egrave";
const EIGHT: &[u8] = b"eight";
const EIGHT_INFERIOR: &[u8] = b"eightinferior";
const EIGHT_OLD_STYLE: &[u8] = b"eightoldstyle";
const EIGHT_SUPERIOR: &[u8] = b"eightsuperior";
const ELLIPSIS: &[u8] = b"ellipsis";
const EM_DASH: &[u8] = b"emdash";
const EN_DASH: &[u8] = b"endash";
const EQUAL: &[u8] = b"equal";
const E_SUPERIOR_LOWER: &[u8] = b"esuperior";
const ETH_LOWER: &[u8] = b"eth";
const ETH_SMALL_LOWER: &[u8] = b"ethsmall";
const EXCLAM: &[u8] = b"exclam";
const EXCLAM_DOWN: &[u8] = b"exclamdown";
const EXCLAM_DOWN_SMALL: &[u8] = b"exclamdownsmall";
const EXCLAM_SMALL: &[u8] = b"exclamsmall";
const F_LOWER: &[u8] = b"f";
const FF: &[u8] = b"ff";
const FFI: &[u8] = b"ffi";
const FFL: &[u8] = b"ffl";
const FI: &[u8] = b"fi";
const FIGURE_DASH: &[u8] = b"figuredash";
const FIVE: &[u8] = b"five";
const FIVE_EIGHTHS: &[u8] = b"fiveeighths";
const FIVE_INFERIOR: &[u8] = b"fiveinferior";
const FIVE_OLD_STYLE: &[u8] = b"fiveoldstyle";
const FIVE_SUPERIOR: &[u8] = b"fivesuperior";
const FL: &[u8] = b"fl";
const FLORIN: &[u8] = b"florin";
const FOUR: &[u8] = b"four";
const FOUR_INFERIOR: &[u8] = b"fourinferior";
const FOUR_OLD_STYLE: &[u8] = b"fouroldstyle";
const FOUR_SUPERIOR: &[u8] = b"foursuperior";
const FRACTION: &[u8] = b"fraction";
const F_SUPERIOR_LOWER: &[u8] = b"fsuperior";
const G_LOWER: &[u8] = b"g";
// Also ESZETT_LOWER
const GERMAN_DOUBLES: &[u8] = b"germandbls";
const GRAVE: &[u8] = b"grave";
const GREATER: &[u8] = b"greater";
const GREATER_EQUAL: &[u8] = b"greaterequal";
const G_SUPERIOR_LOWER: &[u8] = b"gsuperior";
// Adobe messed up the names of guillemets. They still map to these misspellings
const GUILLEMET_LEFT: &[u8] = b"guillemotleft";
const GUILLEMET_RIGHT: &[u8] = b"guillemotright";
const GUILLEMET_SINGLE_LEFT: &[u8] = b"guilsinglleft";
const GUILLEMET_SINGLE_RIGHT: &[u8] = b"guilsinglright";
const H_LOWER: &[u8] = b"h";
const H_SUPERIOR_LOWER: &[u8] = b"hsuperior";
const HUNGARUMLAUT: &[u8] = b"hungarumlaut";
const HYPHEN: &[u8] = b"hyphen";
const HYPHEN_INFERIOR: &[u8] = b"hypheninferior";
const HYPHEN_SUPERIOR: &[u8] = b"hyphensuperior";
const I_LOWER: &[u8] = b"i";
const I_ACUTE_LOWER: &[u8] = b"iacute";
const I_CIRCUMFLEX_LOWER: &[u8] = b"icircumflex";
const I_DIERESIS_LOWER: &[u8] = b"idieresis";
const I_GRAVE_LOWER: &[u8] = b"igrave";
const INFINITY: &[u8] = b"infinity";
const INTEGRAL: &[u8] = b"integral";
const I_SUPERIOR_LOWER: &[u8] = b"isuperior";
const J_LOWER: &[u8] = b"j";
const J_SUPERIOR_LOWER: &[u8] = b"jsuperior";
const K_LOWER: &[u8] = b"k";
const K_SUPERIOR_LOWER: &[u8] = b"ksuperior";
const L_LOWER: &[u8] = b"l";
const LESS: &[u8] = b"less";
const LESS_EQUAL: &[u8] = b"lessequal";
const LOGICAL_NOT: &[u8] = b"logicalnot";
const L_SUPERIOR_LOWER: &[u8] = b"lsuperior";
const LOZENGE: &[u8] = b"lozenge";
const L_SLASH_LOWER: &[u8] = b"lslash";
const M_LOWER: &[u8] = b"m";
const MACRON: &[u8] = b"macron";
const MINUS: &[u8] = b"minus";
const MU_LOWER: &[u8] = b"mu";
const MULTIPLY: &[u8] = b"multiply";
const M_SUPERIOR_LOWER: &[u8] = b"msuperior";
const N_LOWER: &[u8] = b"n";
const NINE: &[u8] = b"nine";
const NINE_INFERIOR: &[u8] = b"nineinferior";
const NINE_OLD_STYLE: &[u8] = b"nineoldstyle";
const NINE_SUPERIOR: &[u8] = b"ninesuperior";
const N_TILDE_LOWER: &[u8] = b"ntilde";
const NOT_EQUAL: &[u8] = b"notequal";
const N_SUPERIOR_LOWER: &[u8] = b"nsuperior";
const NUMBER_SIGN: &[u8] = b"numbersign";
const O_LOWER: &[u8] = b"o";
const O_ACUTE_LOWER: &[u8] = b"oacute";
const O_CIRCUMFLEX_LOWER: &[u8] = b"ocircumflex";
const O_DIERESIS_LOWER: &[u8] = b"odieresis";
const OE_LOWER: &[u8] = b"oe";
const OGONEK: &[u8] = b"ogonek";
const O_GRAVE_LOWER: &[u8] = b"ograve";
const ONE: &[u8] = b"one";
const ONE_DOT_EN_LEADER: &[u8] = b"onedotenleader";
const ONE_EIGHTH: &[u8] = b"oneeighth";
const ONE_FITTED: &[u8] = b"onefitted";
const ONE_HALF: &[u8] = b"onehalf";
const ONE_INFERIOR: &[u8] = b"oneinferior";
const ONE_OLD_STYLE: &[u8] = b"oneoldstyle";
const ONE_QUARTER: &[u8] = b"onequarter";
const ONE_SUPERIOR: &[u8] = b"onesuperior";
const ONE_THIRD: &[u8] = b"onethird";
const ORD_FEMININE: &[u8] = b"ordfeminine";
const ORD_MASCULINE: &[u8] = b"ordmasculine";
const O_SLASH_LOWER: &[u8] = b"oslash";
const O_SUPERIOR_LOWER: &[u8] = b"osuperior";
const O_TILDE_LOWER: &[u8] = b"otilde";
const P_LOWER: &[u8] = b"p";
const PARAGRAPH: &[u8] = b"paragraph";
const PAREN_LEFT: &[u8] = b"parenleft";
const PAREN_LEFT_INFERIOR: &[u8] = b"parenleftinferior";
const PAREN_LEFT_SUPERIOR: &[u8] = b"parenleftsuperior";
const PAREN_RIGHT: &[u8] = b"parenright";
const PAREN_RIGHT_INFERIOR: &[u8] = b"parenrightinferior";
const PAREN_RIGHT_SUPERIOR: &[u8] = b"parenrightsuperior";
const PARTIAL_DIFF: &[u8] = b"partialdiff";
const PERCENT: &[u8] = b"percent";
const PERIOD: &[u8] = b"period";
const PERIOD_CENTERED: &[u8] = b"periodcentered";
const PERIOD_INFERIOR: &[u8] = b"periodinferior";
const PERIOD_SUPERIOR: &[u8] = b"periodsuperior";
const PER_THOUSAND: &[u8] = b"perthousand";
const PI_LOWER: &[u8] = b"pi";
const PLUS: &[u8] = b"plus";
const PLUS_MINUS: &[u8] = b"plusminus";
const PRODUCT: &[u8] = b"product";
const P_SUPERIOR_LOWER: &[u8] = b"psuperior";
const Q_LOWER: &[u8] = b"q";
const Q_SUPERIOR_LOWER: &[u8] = b"qsuperior";
const QUESTION: &[u8] = b"question";
const QUESTION_DOWN: &[u8] = b"questiondown";
const QUESTION_DOWN_SMALL: &[u8] = b"questiondownsmall";
const QUESTION_SMALL: &[u8] = b"questionsmall";
const QUOTE_DOUBLE: &[u8] = b"quotedbl";
const QUOTE_DOUBLE_BASE: &[u8] = b"quotedblbase";
const QUOTE_DOUBLE_LEFT: &[u8] = b"quotedblleft";
const QUOTE_DOUBLE_RIGHT: &[u8] = b"quotedblright";
const QUOTE_LEFT: &[u8] = b"quoteleft";
const QUOTE_RIGHT: &[u8] = b"quoteright";
const QUOTE_SINGLE_BASE: &[u8] = b"quotesinglbase";
const QUOTE_SINGLE: &[u8] = b"quotesingle";
const R_LOWER: &[u8] = b"r";
const RADICAL: &[u8] = b"radical";
const REGISTERED: &[u8] = b"registered";
const RING: &[u8] = b"ring";
const R_SUPERIOR_LOWER: &[u8] = b"rsuperior";
const RUPIAH: &[u8] = b"rupiah";
const S_LOWER: &[u8] = b"s";
const S_CARON_LOWER: &[u8] = b"scaron";
const SECTION: &[u8] = b"section";
const SEMI_COLON: &[u8] = b"semicolon";
const SEVEN: &[u8] = b"seven";
const SEVEN_EIGHTHS: &[u8] = b"seveneighths";
const SEVEN_INFERIOR: &[u8] = b"seveninferior";
const SEVEN_OLD_STYLE: &[u8] = b"sevenoldstyle";
const SEVEN_SUPERIOR: &[u8] = b"sevensuperior";
const SIX: &[u8] = b"six";
const SIX_INFERIOR: &[u8] = b"sixinferior";
const SIX_OLD_STYLE: &[u8] = b"sixoldstyle";
const SIX_SUPERIOR: &[u8] = b"sixsuperior";
const SLASH: &[u8] = b"slash";
const SPACE: &[u8] = b"space";
const S_SUPERIOR_LOWER: &[u8] = b"ssuperior";
const STERLING: &[u8] = b"sterling";
const SUMMATION: &[u8] = b"summation";
const T_LOWER: &[u8] = b"t";
const THORN_LOWER: &[u8] = b"thorn";
const THORN_SMALL_LOWER: &[u8] = b"thornsmall";
const THREE: &[u8] = b"three";
const THREE_EIGHTHS: &[u8] = b"threeeighths";
const THREE_OLD_STYLE: &[u8] = b"threeoldstyle";
const THREE_INFERIOR: &[u8] = b"threeinferior";
const THREE_QUARTERS: &[u8] = b"threequarters";
const THREE_QUARTERS_EM_DASH: &[u8] = b"threequartersemdash";
const THREE_SUPERIOR: &[u8] = b"threesuperior";
const TILDE: &[u8] = b"tilde";
const TRADEMARK: &[u8] = b"trademark";
const T_SUPERIOR_LOWER: &[u8] = b"tsuperior";
const TWO: &[u8] = b"two";
const TWO_INFERIOR: &[u8] = b"twoinferior";
const TWO_DOT_EN_LEADER: &[u8] = b"twodotenleader";
const TWO_OLD_STYLE: &[u8] = b"twooldstyle";
const TWO_SUPERIOR: &[u8] = b"twosuperior";
const TWO_THIRDS: &[u8] = b"twothirds";
const U_LOWER: &[u8] = b"u";
const U_ACUTE_LOWER: &[u8] = b"uacute";
const U_CIRCUMFLEX_LOWER: &[u8] = b"ucircumflex";
const U_DIERESIS_LOWER: &[u8] = b"udieresis";
const U_GRAVE_LOWER: &[u8] = b"ugrave";
const UNDERSCORE: &[u8] = b"underscore";
const U_SUPERIOR_LOWER: &[u8] = b"usuperior";
const V_LOWER: &[u8] = b"v";
const V_SUPERIOR_LOWER: &[u8] = b"vsuperior";
const W_LOWER: &[u8] = b"w";
const W_SUPERIOR_LOWER: &[u8] = b"wsuperior";
const X_LOWER: &[u8] = b"x";
const X_SUPERIOR_LOWER: &[u8] = b"xsuperior";
const Y_LOWER: &[u8] = b"y";
const Y_ACUTE_LOWER: &[u8] = b"yacute";
const Y_DIERESIS_LOWER: &[u8] = b"ydieresis";
const YEN: &[u8] = b"yen";
const Y_SUPERIOR_LOWER: &[u8] = b"ysuperior";
const Z_LOWER: &[u8] = b"z";
const Z_CARON_LOWER: &[u8] = b"zcaron";
const ZERO: &[u8] = b"zero";
const ZERO_INFERIOR: &[u8] = b"zeroinferior";
const ZERO_OLD_STYLE: &[u8] = b"zerooldstyle";
const ZERO_SUPERIOR: &[u8] = b"zerosuperior";
const Z_SUPERIOR_LOWER: &[u8] = b"zsuperior";

const BASE_ENCODING: &[u8] = b"BaseEncoding";
const DIFFERENCES: &[u8] = b"Differences";
const MAC_ROMAN: &[u8] = b"MacRomanEncoding";
const WIN_ANSI: &[u8] = b"WinAnsiEncoding";
const MAC_EXPERT: &[u8] = b"MacExpertEncoding";

pub enum DefaultBaseEncoding {
    MacRoman,
    MacExpert,
    WinAnsi,
}

const STANDARD_ENCODING: Encoding<'static> = Encoding([
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(SPACE),
    Some(EXCLAM),
    Some(QUOTE_DOUBLE),
    Some(NUMBER_SIGN),
    Some(DOLLAR),
    Some(PERCENT),
    Some(AMPERSAND),
    Some(QUOTE_RIGHT),
    Some(PAREN_LEFT),
    Some(PAREN_RIGHT),
    Some(ASTERISK),
    Some(PLUS),
    Some(COMMA),
    Some(HYPHEN),
    Some(PERIOD),
    Some(SLASH),
    Some(ZERO),
    Some(ONE),
    Some(TWO),
    Some(THREE),
    Some(FOUR),
    Some(FIVE),
    Some(SIX),
    Some(SEVEN),
    Some(EIGHT),
    Some(NINE),
    Some(COLON),
    Some(SEMI_COLON),
    Some(LESS),
    Some(EQUAL),
    Some(GREATER),
    Some(QUESTION),
    Some(AT),
    Some(A_UPPER),
    Some(B_UPPER),
    Some(C_UPPER),
    Some(D_UPPER),
    Some(E_UPPER),
    Some(F_UPPER),
    Some(G_UPPER),
    Some(H_UPPER),
    Some(I_UPPER),
    Some(J_UPPER),
    Some(K_UPPER),
    Some(L_UPPER),
    Some(M_UPPER),
    Some(N_UPPER),
    Some(O_UPPER),
    Some(P_UPPER),
    Some(Q_UPPER),
    Some(R_UPPER),
    Some(S_UPPER),
    Some(T_UPPER),
    Some(U_UPPER),
    Some(V_UPPER),
    Some(W_UPPER),
    Some(X_UPPER),
    Some(Y_UPPER),
    Some(Z_UPPER),
    Some(BRACKET_LEFT),
    Some(BACKSLASH),
    Some(BRACKET_RIGHT),
    Some(ASCII_CIRCUM),
    Some(UNDERSCORE),
    Some(QUOTE_LEFT),
    Some(A_LOWER),
    Some(B_LOWER),
    Some(C_LOWER),
    Some(D_LOWER),
    Some(E_LOWER),
    Some(F_LOWER),
    Some(G_LOWER),
    Some(H_LOWER),
    Some(I_LOWER),
    Some(J_LOWER),
    Some(K_LOWER),
    Some(L_LOWER),
    Some(M_LOWER),
    Some(N_LOWER),
    Some(O_LOWER),
    Some(P_LOWER),
    Some(Q_LOWER),
    Some(R_LOWER),
    Some(S_LOWER),
    Some(T_LOWER),
    Some(U_LOWER),
    Some(V_LOWER),
    Some(W_LOWER),
    Some(X_LOWER),
    Some(Y_LOWER),
    Some(Z_LOWER),
    Some(BRACE_LEFT),
    Some(BAR),
    Some(BRACE_RIGHT),
    Some(ASCII_TILDE),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(EXCLAM_DOWN),
    Some(CENT),
    Some(STERLING),
    Some(FRACTION),
    Some(YEN),
    Some(FLORIN),
    Some(SECTION),
    Some(CURRENCY),
    Some(QUOTE_SINGLE),
    Some(QUOTE_DOUBLE_LEFT),
    Some(GUILLEMET_LEFT),
    Some(GUILLEMET_SINGLE_LEFT),
    Some(GUILLEMET_SINGLE_RIGHT),
    Some(FI),
    Some(FL),
    None,
    Some(EN_DASH),
    Some(DAGGER),
    Some(DAGGER_DOUBLE),
    Some(PERIOD_CENTERED),
    None,
    Some(PARAGRAPH),
    Some(BULLET),
    Some(QUOTE_SINGLE_BASE),
    Some(QUOTE_DOUBLE_BASE),
    Some(QUOTE_DOUBLE_RIGHT),
    Some(GUILLEMET_RIGHT),
    Some(ELLIPSIS),
    Some(PER_THOUSAND),
    None,
    Some(QUESTION_DOWN),
    None,
    Some(GRAVE),
    Some(ACUTE),
    Some(CIRCUMFLEX),
    Some(TILDE),
    Some(MACRON),
    Some(BREVE),
    Some(DOT_ACCENT),
    Some(DIERESIS),
    None,
    Some(RING),
    Some(CEDILLA),
    None,
    Some(HUNGARUMLAUT),
    Some(OGONEK),
    Some(CARON),
    Some(EM_DASH),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(AE_UPPER),
    None,
    Some(ORD_FEMININE),
    None,
    None,
    None,
    None,
    Some(L_SLASH_UPPER),
    Some(O_SLASH_UPPER),
    Some(OE_UPPER),
    Some(ORD_MASCULINE),
    None,
    None,
    None,
    None,
    None,
    Some(AE_LOWER),
    None,
    None,
    None,
    Some(DOTLESS_I),
    None,
    None,
    Some(L_SLASH_LOWER),
    Some(O_SLASH_LOWER),
    Some(OE_LOWER),
    Some(GERMAN_DOUBLES),
    None,
    None,
    None,
    None,
]);

pub const MAC_ROMAN_ENCODING: Encoding<'static> = Encoding([
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(SPACE),
    Some(EXCLAM),
    Some(QUOTE_DOUBLE),
    Some(NUMBER_SIGN),
    Some(DOLLAR),
    Some(PERCENT),
    Some(AMPERSAND),
    Some(QUOTE_SINGLE),
    Some(PAREN_LEFT),
    Some(PAREN_RIGHT),
    Some(ASTERISK),
    Some(PLUS),
    Some(COMMA),
    Some(HYPHEN),
    Some(PERIOD),
    Some(SLASH),
    Some(ZERO),
    Some(ONE),
    Some(TWO),
    Some(THREE),
    Some(FOUR),
    Some(FIVE),
    Some(SIX),
    Some(SEVEN),
    Some(EIGHT),
    Some(NINE),
    Some(COLON),
    Some(SEMI_COLON),
    Some(LESS),
    Some(EQUAL),
    Some(GREATER),
    Some(QUESTION),
    Some(AT),
    Some(A_UPPER),
    Some(B_UPPER),
    Some(C_UPPER),
    Some(D_UPPER),
    Some(E_UPPER),
    Some(F_UPPER),
    Some(G_UPPER),
    Some(H_UPPER),
    Some(I_UPPER),
    Some(J_UPPER),
    Some(K_UPPER),
    Some(L_UPPER),
    Some(M_UPPER),
    Some(N_UPPER),
    Some(O_UPPER),
    Some(P_UPPER),
    Some(Q_UPPER),
    Some(R_UPPER),
    Some(S_UPPER),
    Some(T_UPPER),
    Some(U_UPPER),
    Some(V_UPPER),
    Some(W_UPPER),
    Some(X_UPPER),
    Some(Y_UPPER),
    Some(Z_UPPER),
    Some(BRACKET_LEFT),
    Some(BACKSLASH),
    Some(BRACKET_RIGHT),
    Some(ASCII_CIRCUM),
    Some(UNDERSCORE),
    Some(GRAVE),
    Some(A_LOWER),
    Some(B_LOWER),
    Some(C_LOWER),
    Some(D_LOWER),
    Some(E_LOWER),
    Some(F_LOWER),
    Some(G_LOWER),
    Some(H_LOWER),
    Some(I_LOWER),
    Some(J_LOWER),
    Some(K_LOWER),
    Some(L_LOWER),
    Some(M_LOWER),
    Some(N_LOWER),
    Some(O_LOWER),
    Some(P_LOWER),
    Some(Q_LOWER),
    Some(R_LOWER),
    Some(S_LOWER),
    Some(T_LOWER),
    Some(U_LOWER),
    Some(V_LOWER),
    Some(W_LOWER),
    Some(X_LOWER),
    Some(Y_LOWER),
    Some(Z_LOWER),
    Some(BRACE_LEFT),
    Some(BAR),
    Some(BRACE_RIGHT),
    Some(ASCII_TILDE),
    None,
    Some(A_DIERESIS_UPPER),
    Some(A_RING_UPPER),
    Some(C_CEDILLA_UPPER),
    Some(E_ACUTE_UPPER),
    Some(N_TILDE_UPPER),
    Some(O_DIERESIS_UPPER),
    Some(U_DIERESIS_UPPER),
    Some(A_ACUTE_LOWER),
    Some(A_GRAVE_LOWER),
    Some(A_CIRCUMFLEX_LOWER),
    Some(A_DIERESIS_LOWER),
    Some(A_TILDE_LOWER),
    Some(A_RING_LOWER),
    Some(C_CEDILLA_LOWER),
    Some(E_ACUTE_LOWER),
    Some(E_GRAVE_LOWER),
    Some(E_CIRCUMFLEX_LOWER),
    Some(E_DIERESIS_LOWER),
    Some(I_ACUTE_LOWER),
    Some(I_GRAVE_LOWER),
    Some(I_CIRCUMFLEX_LOWER),
    Some(I_DIERESIS_LOWER),
    Some(N_TILDE_LOWER),
    Some(O_ACUTE_LOWER),
    Some(O_GRAVE_LOWER),
    Some(O_CIRCUMFLEX_LOWER),
    Some(O_DIERESIS_LOWER),
    Some(O_TILDE_LOWER),
    Some(U_ACUTE_LOWER),
    Some(U_GRAVE_LOWER),
    Some(U_CIRCUMFLEX_LOWER),
    Some(U_DIERESIS_LOWER),
    Some(DAGGER),
    Some(DEGREE),
    Some(CENT),
    Some(STERLING),
    Some(SECTION),
    Some(BULLET),
    Some(PARAGRAPH),
    Some(GERMAN_DOUBLES),
    Some(REGISTERED),
    Some(COPYRIGHT),
    Some(TRADEMARK),
    Some(ACUTE),
    Some(DIERESIS),
    Some(NOT_EQUAL),
    Some(AE_UPPER),
    Some(O_SLASH_UPPER),
    Some(INFINITY),
    Some(PLUS_MINUS),
    Some(LESS_EQUAL),
    Some(GREATER_EQUAL),
    Some(YEN),
    Some(MU_LOWER),
    Some(PARTIAL_DIFF),
    Some(SUMMATION),
    Some(PRODUCT),
    Some(PI_LOWER),
    Some(INTEGRAL),
    Some(ORD_FEMININE),
    Some(ORD_MASCULINE),
    Some(OMEGA_UPPER),
    Some(AE_LOWER),
    Some(O_SLASH_LOWER),
    Some(QUESTION_DOWN),
    Some(EXCLAM_DOWN),
    Some(LOGICAL_NOT),
    Some(RADICAL),
    Some(FLORIN),
    Some(APPROX_EQUAL),
    Some(DELTA_UPPER),
    Some(GUILLEMET_LEFT),
    Some(GUILLEMET_RIGHT),
    Some(ELLIPSIS),
    Some(SPACE),
    Some(A_GRAVE_UPPER),
    Some(A_TILDE_UPPER),
    Some(O_TILDE_UPPER),
    Some(OE_UPPER),
    Some(OE_LOWER),
    Some(EN_DASH),
    Some(EM_DASH),
    Some(QUOTE_DOUBLE_LEFT),
    Some(QUOTE_DOUBLE_RIGHT),
    Some(QUOTE_LEFT),
    Some(QUOTE_RIGHT),
    Some(DIVIDE),
    Some(LOZENGE),
    Some(Y_DIERESIS_LOWER),
    Some(Y_DIERESIS_UPPER),
    Some(FRACTION),
    // After MacOS 8.5, the Mac Roman encoding mapped this code to "euro" rather
    // than currency, but the PDF spec says to use currency still.
    Some(CURRENCY),
    Some(GUILLEMET_SINGLE_LEFT),
    Some(GUILLEMET_SINGLE_RIGHT),
    Some(FI),
    Some(FL),
    Some(DAGGER_DOUBLE),
    Some(PERIOD_CENTERED),
    Some(QUOTE_SINGLE_BASE),
    Some(QUOTE_DOUBLE_BASE),
    Some(PER_THOUSAND),
    Some(A_CIRCUMFLEX_UPPER),
    Some(E_CIRCUMFLEX_UPPER),
    Some(A_ACUTE_UPPER),
    Some(E_DIERESIS_UPPER),
    Some(E_GRAVE_UPPER),
    Some(I_ACUTE_UPPER),
    Some(I_CIRCUMFLEX_UPPER),
    Some(I_DIERESIS_UPPER),
    Some(I_GRAVE_UPPER),
    Some(O_ACUTE_UPPER),
    Some(O_CIRCUMFLEX_UPPER),
    Some(APPLE),
    Some(O_GRAVE_UPPER),
    Some(U_ACUTE_UPPER),
    Some(U_CIRCUMFLEX_UPPER),
    Some(U_GRAVE_UPPER),
    Some(DOTLESS_I),
    Some(CIRCUMFLEX),
    Some(TILDE),
    Some(MACRON),
    Some(BREVE),
    Some(DOT_ACCENT),
    Some(RING),
    Some(CEDILLA),
    Some(HUNGARUMLAUT),
    Some(OGONEK),
    Some(CARON),
]);

pub const WIN_ANSI_ENCODING: Encoding<'static> = Encoding([
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(SPACE),
    Some(EXCLAM),
    Some(QUOTE_DOUBLE),
    Some(NUMBER_SIGN),
    Some(DOLLAR),
    Some(PERCENT),
    Some(AMPERSAND),
    Some(QUOTE_SINGLE),
    Some(PAREN_LEFT),
    Some(PAREN_RIGHT),
    Some(ASTERISK),
    Some(PLUS),
    Some(COMMA),
    Some(HYPHEN),
    Some(PERIOD),
    Some(SLASH),
    Some(ZERO),
    Some(ONE),
    Some(TWO),
    Some(THREE),
    Some(FOUR),
    Some(FIVE),
    Some(SIX),
    Some(SEVEN),
    Some(EIGHT),
    Some(NINE),
    Some(COLON),
    Some(SEMI_COLON),
    Some(LESS),
    Some(EQUAL),
    Some(GREATER),
    Some(QUESTION),
    Some(AT),
    Some(A_UPPER),
    Some(B_UPPER),
    Some(C_UPPER),
    Some(D_UPPER),
    Some(E_UPPER),
    Some(F_UPPER),
    Some(G_UPPER),
    Some(H_UPPER),
    Some(I_UPPER),
    Some(J_UPPER),
    Some(K_UPPER),
    Some(L_UPPER),
    Some(M_UPPER),
    Some(N_UPPER),
    Some(O_UPPER),
    Some(P_UPPER),
    Some(Q_UPPER),
    Some(R_UPPER),
    Some(S_UPPER),
    Some(T_UPPER),
    Some(U_UPPER),
    Some(V_UPPER),
    Some(W_UPPER),
    Some(X_UPPER),
    Some(Y_UPPER),
    Some(Z_UPPER),
    Some(BRACKET_LEFT),
    Some(BACKSLASH),
    Some(BRACKET_RIGHT),
    Some(ASCII_CIRCUM),
    Some(UNDERSCORE),
    Some(GRAVE),
    Some(A_LOWER),
    Some(B_LOWER),
    Some(C_LOWER),
    Some(D_LOWER),
    Some(E_LOWER),
    Some(F_LOWER),
    Some(G_LOWER),
    Some(H_LOWER),
    Some(I_LOWER),
    Some(J_LOWER),
    Some(K_LOWER),
    Some(L_LOWER),
    Some(M_LOWER),
    Some(N_LOWER),
    Some(O_LOWER),
    Some(P_LOWER),
    Some(Q_LOWER),
    Some(R_LOWER),
    Some(S_LOWER),
    Some(T_LOWER),
    Some(U_LOWER),
    Some(V_LOWER),
    Some(W_LOWER),
    Some(X_LOWER),
    Some(Y_LOWER),
    Some(Z_LOWER),
    Some(BRACE_LEFT),
    Some(BAR),
    Some(BRACE_RIGHT),
    Some(ASCII_TILDE),
    Some(BULLET),
    Some(EURO),
    Some(BULLET),
    Some(QUOTE_SINGLE_BASE),
    Some(FLORIN),
    Some(QUOTE_DOUBLE_BASE),
    Some(ELLIPSIS),
    Some(DAGGER),
    Some(DAGGER_DOUBLE),
    Some(CIRCUMFLEX),
    Some(PER_THOUSAND),
    Some(S_CARON_UPPER),
    Some(GUILLEMET_SINGLE_LEFT),
    Some(OE_UPPER),
    Some(BULLET),
    Some(Z_CARON_UPPER),
    Some(BULLET),
    Some(BULLET),
    Some(QUOTE_LEFT),
    Some(QUOTE_RIGHT),
    Some(QUOTE_DOUBLE_LEFT),
    Some(QUOTE_DOUBLE_RIGHT),
    Some(BULLET),
    Some(EN_DASH),
    Some(EM_DASH),
    Some(TILDE),
    Some(TRADEMARK),
    Some(S_CARON_LOWER),
    Some(GUILLEMET_SINGLE_LEFT),
    Some(OE_LOWER),
    Some(BULLET),
    Some(Z_CARON_LOWER),
    Some(Y_DIERESIS_UPPER),
    Some(SPACE),
    Some(EXCLAM_DOWN),
    Some(CENT),
    Some(STERLING),
    Some(CURRENCY),
    Some(YEN),
    Some(BROKEN_BAR),
    Some(SECTION),
    Some(DIERESIS),
    Some(COPYRIGHT),
    Some(ORD_FEMININE),
    Some(GUILLEMET_LEFT),
    Some(LOGICAL_NOT),
    Some(HYPHEN),
    Some(REGISTERED),
    Some(MACRON),
    Some(DEGREE),
    Some(PLUS_MINUS),
    Some(TWO_SUPERIOR),
    Some(THREE_SUPERIOR),
    Some(ACUTE),
    Some(MU_LOWER),
    Some(PARAGRAPH),
    Some(PERIOD_CENTERED),
    Some(CEDILLA),
    Some(ONE_SUPERIOR),
    Some(ORD_MASCULINE),
    Some(GUILLEMET_RIGHT),
    Some(ONE_QUARTER),
    Some(ONE_HALF),
    Some(THREE_QUARTERS),
    Some(QUESTION_DOWN),
    Some(A_GRAVE_UPPER),
    Some(A_ACUTE_UPPER),
    Some(A_CIRCUMFLEX_UPPER),
    Some(A_TILDE_UPPER),
    Some(A_DIERESIS_UPPER),
    Some(A_RING_UPPER),
    Some(AE_UPPER),
    Some(C_CEDILLA_UPPER),
    Some(E_GRAVE_UPPER),
    Some(E_ACUTE_UPPER),
    Some(E_CIRCUMFLEX_UPPER),
    Some(E_DIERESIS_UPPER),
    Some(I_GRAVE_UPPER),
    Some(I_ACUTE_UPPER),
    Some(I_CIRCUMFLEX_UPPER),
    Some(I_DIERESIS_UPPER),
    Some(ETH_UPPER),
    Some(N_TILDE_UPPER),
    Some(O_GRAVE_UPPER),
    Some(O_ACUTE_UPPER),
    Some(O_CIRCUMFLEX_UPPER),
    Some(O_TILDE_UPPER),
    Some(O_DIERESIS_UPPER),
    Some(MULTIPLY),
    Some(O_SLASH_UPPER),
    Some(U_GRAVE_UPPER),
    Some(U_ACUTE_UPPER),
    Some(U_CIRCUMFLEX_UPPER),
    Some(U_DIERESIS_UPPER),
    Some(Y_ACUTE_UPPER),
    Some(THORN_UPPER),
    Some(GERMAN_DOUBLES),
    Some(A_GRAVE_LOWER),
    Some(A_ACUTE_LOWER),
    Some(A_CIRCUMFLEX_LOWER),
    Some(A_TILDE_LOWER),
    Some(A_DIERESIS_LOWER),
    Some(A_RING_LOWER),
    Some(AE_LOWER),
    Some(C_CEDILLA_LOWER),
    Some(E_GRAVE_LOWER),
    Some(E_ACUTE_LOWER),
    Some(E_CIRCUMFLEX_LOWER),
    Some(E_DIERESIS_LOWER),
    Some(I_GRAVE_LOWER),
    Some(I_ACUTE_LOWER),
    Some(I_CIRCUMFLEX_LOWER),
    Some(I_DIERESIS_LOWER),
    Some(ETH_LOWER),
    Some(N_TILDE_LOWER),
    Some(O_GRAVE_LOWER),
    Some(O_ACUTE_LOWER),
    Some(O_CIRCUMFLEX_LOWER),
    Some(O_TILDE_LOWER),
    Some(O_DIERESIS_LOWER),
    Some(DIVIDE),
    Some(O_SLASH_LOWER),
    Some(U_GRAVE_LOWER),
    Some(U_ACUTE_LOWER),
    Some(U_CIRCUMFLEX_LOWER),
    Some(U_DIERESIS_LOWER),
    Some(Y_ACUTE_LOWER),
    Some(THORN_LOWER),
    Some(Y_DIERESIS_LOWER),
]);

pub const MAC_EXPERT_ENCODING: Encoding<'static> = Encoding([
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(SPACE),
    Some(EXCLAM_SMALL),
    Some(HUNGARUMLAUT_SMALL),
    Some(CENT_OLD_STYLE),
    Some(DOLLAR_OLD_STYLE),
    Some(DOLLAR_SUPERIOR),
    Some(AMPERSAND_SMALL),
    Some(ACUTE_SMALL),
    Some(PAREN_LEFT_SUPERIOR),
    Some(PAREN_RIGHT_SUPERIOR),
    Some(TWO_DOT_EN_LEADER),
    Some(ONE_DOT_EN_LEADER),
    Some(COMMA),
    Some(HYPHEN),
    Some(PERIOD),
    Some(FRACTION),
    Some(ZERO_OLD_STYLE),
    Some(ONE_OLD_STYLE),
    Some(TWO_OLD_STYLE),
    Some(THREE_OLD_STYLE),
    Some(FOUR_OLD_STYLE),
    Some(FIVE_OLD_STYLE),
    Some(SIX_OLD_STYLE),
    Some(SEVEN_OLD_STYLE),
    Some(EIGHT_OLD_STYLE),
    Some(NINE_OLD_STYLE),
    Some(COLON),
    Some(SEMI_COLON),
    None,
    Some(THREE_QUARTERS_EM_DASH),
    None,
    Some(QUESTION_SMALL),
    None,
    None,
    None,
    None,
    Some(ETH_SMALL_UPPER),
    None,
    None,
    Some(ONE_QUARTER),
    Some(ONE_HALF),
    Some(THREE_QUARTERS),
    Some(ONE_EIGHTH),
    Some(THREE_EIGHTHS),
    Some(FIVE_EIGHTHS),
    Some(SEVEN_EIGHTHS),
    Some(ONE_THIRD),
    Some(TWO_THIRDS),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(FF),
    Some(FI),
    Some(FL),
    Some(FFI),
    Some(FFL),
    Some(PAREN_LEFT_INFERIOR),
    None,
    Some(PAREN_RIGHT_INFERIOR),
    Some(CIRCUMFLEX_SMALL),
    Some(HYPHEN_INFERIOR),
    Some(GRAVE_SMALL),
    Some(A_UPPER_SMALL),
    Some(B_UPPER_SMALL),
    Some(C_UPPER_SMALL),
    Some(D_UPPER_SMALL),
    Some(E_UPPER_SMALL),
    Some(F_UPPER_SMALL),
    Some(G_UPPER_SMALL),
    Some(H_UPPER_SMALL),
    Some(I_UPPER_SMALL),
    Some(J_UPPER_SMALL),
    Some(K_UPPER_SMALL),
    Some(L_UPPER_SMALL),
    Some(M_UPPER_SMALL),
    Some(N_UPPER_SMALL),
    Some(O_UPPER_SMALL),
    Some(P_UPPER_SMALL),
    Some(Q_UPPER_SMALL),
    Some(R_UPPER_SMALL),
    Some(S_UPPER_SMALL),
    Some(T_UPPER_SMALL),
    Some(U_UPPER_SMALL),
    Some(V_UPPER_SMALL),
    Some(W_UPPER_SMALL),
    Some(X_UPPER_SMALL),
    Some(Y_UPPER_SMALL),
    Some(Z_UPPER_SMALL),
    Some(COLON_MONETARY),
    Some(ONE_FITTED),
    Some(RUPIAH),
    Some(TILDE_SMALL),
    None,
    None,
    Some(A_SUPERIOR_LOWER),
    Some(CENT_INFERIOR),
    None,
    None,
    None,
    None,
    Some(A_ACUTE_SMALL_UPPER),
    Some(A_GRAVE_SMALL_UPPER),
    Some(A_CIRCUMFLEX_SMALL_UPPER),
    Some(A_DIERESIS_SMALL_UPPER),
    Some(A_TILDE_SMALL_UPPER),
    Some(A_RING_SMALL_UPPER),
    Some(C_CEDILLA_SMALL_UPPER),
    Some(E_ACUTE_SMALL_UPPER),
    Some(E_GRAVE_SMALL_UPPER),
    Some(E_CIRCUMFLEX_SMALL_UPPER),
    Some(E_DIERESIS_SMALL_UPPER),
    Some(I_ACUTE_SMALL_UPPER),
    Some(I_GRAVE_SMALL_UPPER),
    Some(I_CIRCUMFLEX_SMALL_UPPER),
    Some(I_DIERESIS_SMALL_UPPER),
    Some(N_TILDE_SMALL_UPPER),
    Some(O_ACUTE_SMALL_UPPER),
    Some(O_GRAVE_SMALL_UPPER),
    Some(O_CIRCUMFLEX_SMALL_UPPER),
    Some(O_DIERESIS_SMALL_UPPER),
    Some(O_TILDE_SMALL_UPPER),
    Some(U_ACUTE_SMALL_UPPER),
    Some(U_GRAVE_SMALL_UPPER),
    Some(U_CIRCUMFLEX_SMALL_UPPER),
    Some(U_DIERESIS_SMALL_UPPER),
    None,
    Some(EIGHT_SUPERIOR),
    Some(FOUR_INFERIOR),
    Some(THREE_INFERIOR),
    Some(SIX_INFERIOR),
    Some(EIGHT_INFERIOR),
    Some(SEVEN_INFERIOR),
    Some(S_CARON_SMALL_UPPER),
    None,
    Some(CENT_INFERIOR),
    Some(TWO_INFERIOR),
    None,
    Some(DIERESIS_SMALL),
    None,
    Some(CARON_SMALL),
    Some(O_SUPERIOR_LOWER),
    Some(FIVE_INFERIOR),
    None,
    Some(COMMA_INFERIOR),
    Some(PERIOD_INFERIOR),
    Some(Y_ACUTE_SMALL_UPPER),
    None,
    Some(DOLLAR_INFERIOR),
    None,
    None,
    Some(THORN_SMALL_UPPER),
    None,
    Some(NINE_INFERIOR),
    Some(ZERO_INFERIOR),
    Some(Z_CARON_SMALL_UPPER),
    Some(AE_SMALL_UPPER),
    Some(O_SLASH_SMALL_UPPER),
    Some(QUESTION_DOWN_SMALL),
    Some(ONE_INFERIOR),
    Some(L_SLASH_SMALL_UPPER),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(CEDILLA_SMALL),
    None,
    None,
    None,
    None,
    None,
    Some(OE_SMALL_UPPER),
    Some(FIGURE_DASH),
    Some(HYPHEN_SUPERIOR),
    None,
    None,
    None,
    None,
    Some(EXCLAM_DOWN_SMALL),
    None,
    Some(Y_DIERESIS_SMALL_UPPER),
    None,
    Some(ONE_SUPERIOR),
    Some(TWO_SUPERIOR),
    Some(THREE_SUPERIOR),
    Some(FOUR_SUPERIOR),
    Some(FIVE_SUPERIOR),
    Some(SIX_SUPERIOR),
    Some(SEVEN_SUPERIOR),
    Some(NINE_SUPERIOR),
    Some(ZERO_SUPERIOR),
    None,
    Some(E_SUPERIOR_LOWER),
    Some(R_SUPERIOR_LOWER),
    Some(T_SUPERIOR_LOWER),
    None,
    None,
    Some(I_SUPERIOR_LOWER),
    Some(S_SUPERIOR_LOWER),
    Some(D_SUPERIOR_LOWER),
    None,
    None,
    None,
    None,
    None,
    Some(L_SUPERIOR_LOWER),
    Some(OGONEK_SMALL),
    Some(BREVE_SMALL),
    Some(MACRON_SMALL),
    Some(B_SUPERIOR_LOWER),
    Some(N_SUPERIOR_LOWER),
    Some(M_SUPERIOR_LOWER),
    Some(COMMA_SUPERIOR),
    Some(PERIOD_SUPERIOR),
    Some(DOT_ACCENT_SMALL),
    Some(RING_SMALL),
    None,
    None,
    None,
    None,
]);

impl<'a> Default for Encoding<'a> {
    fn default() -> Self {
        STANDARD_ENCODING
    }
}

impl<'a> Deref for Encoding<'a> {
    type Target = [Option<&'a [u8]>; ENCODING_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Encoding<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> TryFrom<&'a Name> for Encoding<'a> {
    type Error = PdfError;

    fn try_from(name: &'a Name) -> Result<Self, Self::Error> {
        if name == MAC_ROMAN {
            Ok(MAC_ROMAN_ENCODING)
        } else if name == WIN_ANSI {
            Ok(WIN_ANSI_ENCODING)
        } else if name == MAC_EXPERT {
            todo!()
        } else {
            Err(PdfError::Other {
                msg: "Could not convert dictionary to encoding.".to_string(),
            })
        }
    }
}

impl<'a> TryFrom<&'a Dictionary> for Encoding<'a> {
    type Error = PdfError;

    fn try_from(dict: &'a Dictionary) -> Result<Self, Self::Error> {
        let mut encoding = if let Some(Object {
            kind: ObjectKind::Name(name),
            ..
        }) = dict.get(BASE_ENCODING)
        {
            Self::try_from(name)?
        } else {
            Self::default()
        };
        if let Some(Object {
            kind: ObjectKind::Array(differences),
            ..
        }) = dict.get(DIFFERENCES)
        {
            let mut array_index = 0;
            for object in differences {
                match object {
                    Object {
                        kind: ObjectKind::Integer(i),
                        ..
                    } => array_index = *i as usize,
                    Object {
                        kind: ObjectKind::Name(name),
                        ..
                    } => {
                        encoding[array_index] = Some(name);
                        array_index += 1;
                    }
                    _ => continue,
                }
            }
        }
        Ok(encoding)
    }
}
