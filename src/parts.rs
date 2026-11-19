//! The parts that RFC 3986 writes inside its rules rather than as rules of
//! their own: value ranges, groups `( … )`, and alternatives.

use crate::{
    Alpha, DecOctet, Digit, H16, Ipv4Address, Ls32, Pchar, PctEncoded, SubDelims, Unreserved,
};
use tygr::{Grammar, StringEq, char_class};

char_class!(pub UpperCaseLetter, "%x41-5A", |ch| ch.is_ascii_uppercase());
char_class!(pub LowerCaseLetter, "%x61-7A", |ch| ch.is_ascii_lowercase());
char_class!(pub DecimalDigit, "%x30-39", |ch| ch.is_ascii_digit());
char_class!(pub DigitOneToNine, "%x31-39", |ch| matches!(ch, '1'..='9'));
char_class!(pub DigitZeroToFour, "%x30-34", |ch| matches!(ch, '0'..='4'));
char_class!(pub DigitZeroToFive, "%x30-35", |ch| matches!(ch, '0'..='5'));

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
#[tygr(inline)]
pub enum SchemeChar {
    Alpha(Alpha),
    Digit(Digit),
    PlusSign(StringEq!("+")),
    Hyphen(StringEq!("-")),
    Period(StringEq!(".")),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
#[tygr(inline)]
pub enum UserinfoChar {
    Unreserved(Unreserved),
    PctEncoded(PctEncoded),
    SubDelims(SubDelims),
    Colon(StringEq!(":")),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
#[tygr(inline)]
pub enum RegNameChar {
    Unreserved(Unreserved),
    PctEncoded(PctEncoded),
    SubDelims(SubDelims),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
#[tygr(inline)]
pub enum IpvFutureChar {
    Unreserved(Unreserved),
    SubDelims(SubDelims),
    Colon(StringEq!(":")),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
#[tygr(inline)]
pub enum SegmentNzNcChar {
    Unreserved(Unreserved),
    PctEncoded(PctEncoded),
    SubDelims(SubDelims),
    AtSign(StringEq!("@")),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
#[tygr(inline)]
pub enum QueryChar {
    Pchar(Pchar),
    Slash(StringEq!("/")),
    QuestionMark(StringEq!("?")),
}

pub(crate) type AtMost1<T> = Option<T>;
pub(crate) type AtMost2<T> = Option<(T, AtMost1<T>)>;
pub(crate) type AtMost3<T> = Option<(T, AtMost2<T>)>;
pub(crate) type AtMost4<T> = Option<(T, AtMost3<T>)>;
pub(crate) type AtMost5<T> = Option<(T, AtMost4<T>)>;
pub(crate) type AtMost6<T> = Option<(T, AtMost5<T>)>;

type H16Colon = (H16, StringEq!(":"));
type ColonH16 = (StringEq!(":"), H16);
pub(crate) type DotDecOctet = (StringEq!("."), DecOctet);

/// The alternatives of `IPv6address`, in the RFC's order. Before `"::"`, the
/// RFC's `[ *n( h16 ":" ) h16 ]` is written `[ h16 *n( ":" h16 ) ]`, which
/// matches the same text but stops at `"::"`.
#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
#[tygr(inline)]
#[rustfmt::skip]
pub enum Ipv6AddressAlternatives {
    Alt1((H16Colon, H16Colon, H16Colon, H16Colon, H16Colon, H16Colon), Ls32),
    Alt2(StringEq!("::"), (H16Colon, H16Colon, H16Colon, H16Colon, H16Colon), Ls32),
    Alt3(Option<H16>, StringEq!("::"), (H16Colon, H16Colon, H16Colon, H16Colon), Ls32),
    Alt4(Option<(H16, AtMost1<ColonH16>)>, StringEq!("::"), (H16Colon, H16Colon, H16Colon), Ls32),
    Alt5(Option<(H16, AtMost2<ColonH16>)>, StringEq!("::"), (H16Colon, H16Colon), Ls32),
    Alt6(Option<(H16, AtMost3<ColonH16>)>, StringEq!("::"), H16Colon, Ls32),
    Alt7(Option<(H16, AtMost4<ColonH16>)>, StringEq!("::"), Ls32),
    Alt8(Option<(H16, AtMost5<ColonH16>)>, StringEq!("::"), H16),
    Alt9(Option<(H16, AtMost6<ColonH16>)>, StringEq!("::")),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
#[tygr(inline)]
pub enum Ls32Alternatives {
    TwoH16(H16, StringEq!(":"), H16),
    Ipv4Address(Ipv4Address),
}

/// The alternatives of `dec-octet`, longest first: tygr takes the first that
/// matches, so `192` is `"1" 2DIGIT`, not `DIGIT` then `92`.
#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
#[tygr(inline)]
pub enum DecOctetAlternatives {
    From250To255(StringEq!("25"), DigitZeroToFive),
    From200To249(StringEq!("2"), DigitZeroToFour, Digit),
    From100To199(StringEq!("1"), Digit, Digit),
    From10To99(DigitOneToNine, Digit),
    From0To9(Digit),
}
