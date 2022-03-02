use std::convert::TryFrom;

use crate::{parser::parser::{Dictionary, Object, ObjectKind}, error::PdfError};

pub const ENCODING_SIZE: usize = 256;
pub struct Encoding<'a>([Option<&'a [u8]>; ENCODING_SIZE]);

const A_UPPER: &[u8] = b"A";
const AE_UPPER: &[u8] = b"AE";
const A_ACUTE_UPPER: &[u8] = b"Aacute";
const A_CIRCUMFLEX_UPPER: &[u8] = b"Acircumflex";
const A_DIERESIS_UPPER: &[u8] = b"Adieresis";
const A_GRAVE_UPPER: &[u8] = b"Agrave";
const A_RING_UPPER: &[u8] = b"Aring";
const A_TILDE_UPPER: &[u8] = b"Atilde";
const B_UPPER: &[u8] = b"B";
const C_UPPER: &[u8] = b"C";
const C_CEDILLA_UPPER: &[u8] = b"Ccedilla";
const D_UPPER: &[u8] = b"D";
const DELTA_UPPER: &[u8] = b"Delta";
const E_UPPER: &[u8] = b"E";
const E_ACUTE_UPPER: &[u8] = b"Eacute";
const E_CIRCUMFLEX_UPPER: &[u8] = b"Ecircumflex";
const E_DIERESIS_UPPER: &[u8] = b"Edieresis";
const E_GRAVE_UPPER: &[u8] = b"Egrave";
const ETH_UPPER: &[u8] = b"Eth";
const EURO: &[u8] = b"Euro";
const F_UPPER: &[u8] = b"F";
const G_UPPER: &[u8] = b"G";
const H_UPPER: &[u8] = b"H";
const I_UPPER: &[u8] = b"I";
const I_ACUTE_UPPER: &[u8] = b"Iacute";
const I_CIRCUMFLEX_UPPER: &[u8] = b"Icircumflex";
const I_DIERESIS_UPPER: &[u8] = b"Idieresis";
const I_GRAVE_UPPER: &[u8] = b"Igrave";
const J_UPPER: &[u8] = b"J";
const K_UPPER: &[u8] = b"K";
const L_UPPER: &[u8] = b"L";
const L_SLASH_UPPER: &[u8] = b"Lslash";
const M_UPPER: &[u8] = b"M";
const N_UPPER: &[u8] = b"N";
const N_TILDE_UPPER: &[u8] = b"Ntilde";
const O_UPPER: &[u8] = b"O";
const OE_UPPER: &[u8] = b"OE";
const O_ACUTE_UPPER: &[u8] = b"Oacute";
const O_CIRCUMFLEX_UPPER: &[u8] = b"Ocircumflex";
const O_DIERESIS_UPPER: &[u8] = b"Odieresis";
const O_GRAVE_UPPER: &[u8] = b"Ograve";
const OMEGA_UPPER: &[u8] = b"Omega";
const O_SLASH_UPPER: &[u8] = b"Oslash";
const O_TILDE_UPPER: &[u8] = b"Otilde";
const P_UPPER: &[u8] = b"P";
const Q_UPPER: &[u8] = b"Q";
const R_UPPER: &[u8] = b"R";
const S_UPPER: &[u8] = b"S";
const S_CARON_UPPER: &[u8] = b"Scaron";
const T_UPPER: &[u8] = b"T";
const THORN_UPPER: &[u8] = b"Thorn";
const U_UPPER: &[u8] = b"U";
const U_ACUTE_UPPER: &[u8] = b"Uacute";
const U_CIRCUMFLEX_UPPER: &[u8] = b"Ucircumflex";
const U_DIERESIS_UPPER: &[u8] = b"Udieresis";
const U_GRAVE_UPPER: &[u8] = b"Ugrave";
const V_UPPER: &[u8] = b"V";
const W_UPPER: &[u8] = b"W";
const X_UPPER: &[u8] = b"X";
const Y_UPPER: &[u8] = b"Y";
const Y_ACUTE_UPPER: &[u8] = b"Yacute";
const Y_DIERESIS_UPPER: &[u8] = b"Ydieresis";
const Z_UPPER: &[u8] = b"Z";
const Z_CARON_UPPER: &[u8] = b"Zcaron";
const A_LOWER: &[u8] = b"a";
const A_ACUTE_LOWER: &[u8] = b"aacute";
const A_CIRCUMFLEX_LOWER: &[u8] = b"acircumflex";
const ACUTE: &[u8] = b"acute";
const A_DIERESIS_LOWER: &[u8] = b"adieresis";
const AE_LOWER: &[u8] = b"ae";
const A_GRAVE_LOWER: &[u8] = b"agrave";
const AMPERSAND: &[u8] = b"ampersand";
const APPLE: &[u8] = b"apple";
const APPROX_EQUAL: &[u8] = b"approxequal";
const A_RING_LOWER: &[u8] = b"aring";
const ASCII_CIRCUM: &[u8] = b"asciicircum";
const ASCII_TILDE: &[u8] = b"asciitilde";
const ASTERISK: &[u8] = b"asterisk";
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
const BULLET: &[u8] = b"bullet";
const C_LOWER: &[u8] = b"c";
const CARON: &[u8] = b"caron";
const C_CEDILLA_LOWER: &[u8] = b"ccedilla";
const CEDILLA: &[u8] = b"cedilla";
const CENT: &[u8] = b"cent";
const CIRCUMFLEX: &[u8] = b"circumflex";
const COLON: &[u8] = b"colon";
const COMMA: &[u8] = b"comma";
const COPYRIGHT: &[u8] = b"copyright";
const CURRENCY: &[u8] = b"currency";
const D_LOWER: &[u8] = b"d";
const DAGGER: &[u8] = b"dagger";
const DAGGER_DOUBLE: &[u8] = b"daggerdbl";
const DEGREE: &[u8] = b"degree";
const DIERESIS: &[u8] = b"dieresis";
const DIVIDE: &[u8] = b"divide";
const DOLLAR: &[u8] = b" dollar";
const DOT_ACCENT: &[u8] = b"dotaccent";
const DOTLESS_I: &[u8] = b"dotlessi";
const E_LOWER: &[u8] = b"e";
const E_ACUTE_LOWER: &[u8] = b"eacute";
const E_CIRCUMFLEX_LOWER: &[u8] = b"ecircumflex";
const E_DIERESIS_LOWER: &[u8] = b"edieresis";
const E_GRAVE_LOWER: &[u8] = b"egrave";
const EIGHT: &[u8] = b"eight";
const ELLIPSIS: &[u8] = b"ellipsis";
const EM_DASH: &[u8] = b"emdash";
const EN_DASH: &[u8] = b"endash";
const EQUAL: &[u8] = b"EQUAL";
const ETH_LOWER: &[u8] = b"eth";
const EXCLAM: &[u8] = b"exclam";
const EXCLAM_DOWN: &[u8] = b"exclamdown";
const F_LOWER: &[u8] = b"f";
const FI: &[u8] = b"fi";
const FIVE: &[u8] = b"five";
const FL: &[u8] = b"fl";
const FLORIN: &[u8] = b"florin";
const FOUR: &[u8] = b"four";
const FRACTION: &[u8] = b"fraction";
const G_LOWER: &[u8] = b"g";
// Also ESZETT_LOWER
const GERMAN_DOUBLES: &[u8] = b"germandbls";
const GRAVE: &[u8] = b"grave";
const GREATER: &[u8] = b"greater";
const GREATER_EQUAL: &[u8] = b"greaterequal";
// Adobe messed up the names of guillemets. They still map to these misspellings
const GUILLEMET_LEFT: &[u8] = b"guillemotleft";
const GUILLEMET_RIGHT: &[u8] = b"guillemotright";
const GUILLEMET_SINGLE_LEFT: &[u8] = b"guilsinglleft";
const GUILLEMET_SINGLE_RIGHT: &[u8] = b"guilsinglright";
const H_LOWER: &[u8] = b"h";
const HUNGARUMLAUT: &[u8] = b"hungarumlaut";
const HYPHEN: &[u8] = b"hyphen";
const I_LOWER: &[u8] = b"i";
const I_ACUTE_LOWER: &[u8] = b"iacute";
const I_CIRCUMFLEX_LOWER: &[u8] = b"icircumflex";
const I_DIERESIS_LOWER: &[u8] = b"idieresis";
const I_GRAVE_LOWER: &[u8] = b"igrave";
const INFINITY: &[u8] = b"infinity";
const INTEGRAL: &[u8] = b"integral";
const J_LOWER: &[u8] = b"j";
const K_LOWER: &[u8] = b"k";
const L_LOWER: &[u8] = b"l";
const LESS: &[u8] = b"less";
const LESS_EQUAL: &[u8] = b"lessequal";
const LOGICAL_NOT: &[u8] = b"logicalnot";
const LOZENGE: &[u8] = b"lozenge";
const L_SLASH_LOWER: &[u8] = b"lslash";
const M_LOWER: &[u8] = b"m";
const MACRON: &[u8] = b"macron";
const MINUS: &[u8] = b"minus";
const MU_LOWER: &[u8] = b"mu";
const MULTIPLY: &[u8] = b"multiply";
const N_LOWER: &[u8] = b"n";
const NINE: &[u8] = b"nine";
const N_TILDE_LOWER: &[u8] = b"ntilde";
const NOT_EQUAL: &[u8] = b"notequal";
const NUMBER_SIGN: &[u8] = b"numbersign";
const O_LOWER: &[u8] = b"o";
const O_ACUTE_LOWER: &[u8] = b"oacute";
const O_CIRCUMFLEX_LOWER: &[u8] = b"ocircumflex";
const O_DIERESIS_LOWER: &[u8] = b"odieresis";
const OE_LOWER: &[u8] = b"oe";
const OGONEK: &[u8] = b"ogonek";
const O_GRAVE_LOWER: &[u8] = b"ograve";
const ONE: &[u8] = b"one";
const ONE_HALF: &[u8] = b"onehalf";
const ONE_QUARTER: &[u8] = b"onequarter";
const ONE_SUPERIOR: &[u8] = b"onesuperior";
const ORD_FEMININE: &[u8] = b"ordfeminine";
const ORD_MASCULINE: &[u8] = b"ordmasculine";
const O_SLASH_LOWER: &[u8] = b"oslash";
const O_TILDE_LOWER: &[u8] = b"otilde";
const P_LOWER: &[u8] = b"p";
const PARAGRAPH: &[u8] = b"paragraph";
const PAREN_LEFT: &[u8] = b"parenleft";
const PAREN_RIGHT: &[u8] = b"parenright";
const PARTIAL_DIFF: &[u8] = b"partialdiff";
const PERCENT: &[u8] = b"percent";
const PERIOD: &[u8] = b"period";
const PERIOD_CENTERED: &[u8] = b"periodcentered";
const PER_THOUSAND: &[u8] = b"perthousand";
const PI_LOWER: &[u8] = b"pi";
const PLUS: &[u8] = b"plus";
const PLUS_MINUS: &[u8] = b"plusminus";
const PRODUCT: &[u8] = b"product";
const Q_LOWER: &[u8] = b"q";
const QUESTION: &[u8] = b"question";
const QUESTION_DOWN: &[u8] = b"questiondown";
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
const S_LOWER: &[u8] = b"s";
const S_CARON_LOWER: &[u8] = b"scaron";
const SECTION: &[u8] = b"section";
const SEMI_COLON: &[u8] = b"semicolon";
const SEVEN: &[u8] = b"seven";
const SIX: &[u8] = b"six";
const SLASH: &[u8] = b"slash";
const SPACE: &[u8] = b"space";
const STERLING: &[u8] = b"sterling";
const SUMMATION: &[u8] = b"summation";
const T_LOWER: &[u8] = b"t";
const THORN_LOWER: &[u8] = b"thorn";
const THREE: &[u8] = b"three";
const THREE_QUARTERS: &[u8] = b"threequarters";
const THREE_SUPERIOR: &[u8] = b"threesuperior";
const TILDE: &[u8] = b"tilde";
const TRADEMARK: &[u8] = b"trademark";
const TWO: &[u8] = b"two";
const TWO_SUPERIOR: &[u8] = b"twosuperior";
const U_LOWER: &[u8] = b"u";
const U_ACUTE_LOWER: &[u8] = b"uacute";
const U_CIRCUMFLEX_LOWER: &[u8] = b"ucircumflex";
const U_DIERESIS_LOWER: &[u8] = b"udieresis";
const U_GRAVE_LOWER: &[u8] = b"ugrave";
const UNDERSCORE: &[u8] = b"underscore";
const V_LOWER: &[u8] = b"v";
const W_LOWER: &[u8] = b"w";
const X_LOWER: &[u8] = b"x";
const Y_LOWER: &[u8] = b"y";
const Y_ACUTE_LOWER: &[u8] = b"yacute";
const Y_DIERESIS_LOWER: &[u8] = b"ydieresis";
const YEN: &[u8] = b"yen";
const Z_LOWER: &[u8] = b"z";
const Z_CARON_LOWER: &[u8] = b"zcaron";
const ZERO: &[u8] = b"zero";

const BASE_ENCODING: &[u8] = b"BaseEncoding";
const MAC_ROMAN: &[u8] = b"MacRoman";
const WIN_ANSI: &[u8] = b"WinAnsi";
const MAC_EXPERT: &[u8] = b"MacExpert";

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

const MAC_ROMAN_ENCODING: Encoding<'static> = Encoding([
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

const WIN_ANSI_ENCODING: Encoding<'static> = Encoding([
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

impl<'a> Default for Encoding<'a> {
    fn default() -> Self {
        Self([None; ENCODING_SIZE])
    }
}

impl<'a> TryFrom<&'a Dictionary> for Encoding<'a> {
    type Error = PdfError;

    fn try_from(dict: &'a Dictionary) -> Result<Self, Self::Error> {
        let mut encoding = if let Some(Object { kind: ObjectKind::Name(name), .. }) = dict.get(BASE_ENCODING) {
            if name == MAC_ROMAN {
                MAC_ROMAN_ENCODING
            } else if name == WIN_ANSI {
                WIN_ANSI_ENCODING
            } else {
                Self::default()
            }
        } else {
            Self::default()
        };
        // TODO: Loop through `Differences` array
        Ok(encoding)
    }
}