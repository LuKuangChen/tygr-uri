#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub use tygr;

pub mod parts;

use parts::*;
use tygr::{Grammar, Parse, Prefix, Raw, StringEq, StringEqCI, Suffix, Vec1, Wrap};

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uri {
    pub scheme: Scheme,
    pub colon: StringEq!(":"),
    pub hier_part: HierPart,
    pub query: Option<Prefix<StringEq!("?"), Query>>,
    pub fragment: Option<Prefix<StringEq!("#"), Fragment>>,
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub enum HierPart {
    Authority {
        double_slash: StringEq!("//"),
        authority: Authority,
        path: PathAbempty,
    },
    Absolute(PathAbsolute),
    Rootless(PathRootless),
    Empty(PathEmpty),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UriReference {
    Uri(Uri),
    RelativeRef(RelativeRef),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbsoluteUri {
    pub scheme: Scheme,
    pub colon: StringEq!(":"),
    pub hier_part: HierPart,
    pub query: Option<Prefix<StringEq!("?"), Query>>,
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelativeRef {
    pub relative_part: RelativePart,
    pub query: Option<Prefix<StringEq!("?"), Query>>,
    pub fragment: Option<Prefix<StringEq!("#"), Fragment>>,
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelativePart {
    Authority {
        double_slash: StringEq!("//"),
        authority: Authority,
        path: PathAbempty,
    },
    Absolute(PathAbsolute),
    Noscheme(PathNoscheme),
    Empty(PathEmpty),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scheme(Raw<(Alpha, Vec<SchemeChar>)>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Authority {
    pub userinfo: Option<Suffix<Userinfo, StringEq!("@")>>,
    pub host: Host,
    pub port: Option<Prefix<StringEq!(":"), Port>>,
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Userinfo(Raw<Vec<UserinfoChar>>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Host {
    IpLiteral(IpLiteral),
    Ipv4OrRegName(Ipv4OrRegName),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Port(Raw<Vec<Digit>>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct IpLiteral(pub Wrap<StringEq!("["), IpLiteralAddress, StringEq!("]")>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
#[tygr(inline)]
pub enum IpLiteralAddress {
    Ipv6Address(Ipv6Address),
    IpvFuture(IpvFuture),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct IpvFuture(
    Raw<(
        StringEqCI!("v"),
        Vec1<Hexdig>,
        StringEq!("."),
        Vec1<IpvFutureChar>,
    )>,
);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ipv6Address(Raw<Ipv6AddressAlternatives>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct H16(Raw<(Hexdig, AtMost3<Hexdig>)>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ls32(Raw<Ls32Alternatives>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ipv4Address(Raw<(DecOctet, DotDecOctet, DotDecOctet, DotDecOctet)>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecOctet(Raw<DecOctetAlternatives>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ipv4OrRegName(Raw<Vec<RegNameChar>>);

/// An IPv4 address if the whole text matches `IPv4address` (RFC 3986,
/// Section 3.2.2); otherwise a registered name, given back.
///
/// ```
/// use tygr_uri::tygr::Parse;
/// use tygr_uri::{Ipv4Address, Ipv4OrRegName};
///
/// let address = Ipv4OrRegName::parse("192.0.2.16").unwrap();
/// assert_eq!(Ipv4Address::try_from(address).unwrap().as_str(), "192.0.2.16");
/// let name = Ipv4OrRegName::parse("1.2.3.4.example").unwrap();
/// assert_eq!(Ipv4Address::try_from(name).unwrap_err().as_str(), "1.2.3.4.example");
/// ```
impl TryFrom<Ipv4OrRegName> for Ipv4Address {
    type Error = Ipv4OrRegName;

    fn try_from(name: Ipv4OrRegName) -> Result<Ipv4Address, Ipv4OrRegName> {
        Ipv4Address::parse(name.as_str()).map_err(|_| name)
    }
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Path {
    Rootless(PathRootless),
    Abempty(PathAbempty),
    Absolute(PathAbsolute),
    Noscheme(PathNoscheme),
    Empty(PathEmpty),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathAbempty(pub Vec<Prefix<StringEq!("/"), Segment>>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathAbsolute(
    pub StringEq!("/"),
    pub Option<(SegmentNz, Vec<Prefix<StringEq!("/"), Segment>>)>,
);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathNoscheme(pub SegmentNzNc, pub Vec<Prefix<StringEq!("/"), Segment>>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathRootless(pub SegmentNz, pub Vec<Prefix<StringEq!("/"), Segment>>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathEmpty;

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Segment(Raw<Vec<Pchar>>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SegmentNz(Raw<Vec1<Pchar>>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SegmentNzNc(Raw<Vec1<SegmentNzNcChar>>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pchar {
    Unreserved(Unreserved),
    PctEncoded(PctEncoded),
    SubDelims(SubDelims),
    Colon(StringEq!(":")),
    AtSign(StringEq!("@")),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Query(Raw<Vec<QueryChar>>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fragment(Raw<Vec<QueryChar>>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PctEncoded(Raw<(StringEq!("%"), Hexdig, Hexdig)>);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Unreserved {
    Alpha(Alpha),
    Digit(Digit),
    Hyphen(StringEq!("-")),
    Period(StringEq!(".")),
    Underscore(StringEq!("_")),
    Tilde(StringEq!("~")),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Reserved {
    GenDelims(GenDelims),
    SubDelims(SubDelims),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenDelims {
    Colon(StringEq!(":")),
    Slash(StringEq!("/")),
    QuestionMark(StringEq!("?")),
    NumberSign(StringEq!("#")),
    LeftBracket(StringEq!("[")),
    RightBracket(StringEq!("]")),
    AtSign(StringEq!("@")),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubDelims {
    ExclamationMark(StringEq!("!")),
    DollarSign(StringEq!("$")),
    Ampersand(StringEq!("&")),
    Apostrophe(StringEq!("'")),
    LeftParenthesis(StringEq!("(")),
    RightParenthesis(StringEq!(")")),
    Asterisk(StringEq!("*")),
    PlusSign(StringEq!("+")),
    Comma(StringEq!(",")),
    Semicolon(StringEq!(";")),
    EqualsSign(StringEq!("=")),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Alpha {
    Upper(UpperCaseLetter),
    Lower(LowerCaseLetter),
}

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Digit(pub DecimalDigit);

#[derive(Grammar, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Hexdig {
    Digit(Digit),
    A(StringEqCI!("A")),
    B(StringEqCI!("B")),
    C(StringEqCI!("C")),
    D(StringEqCI!("D")),
    E(StringEqCI!("E")),
    F(StringEqCI!("F")),
}

macro_rules! text {
    ($($Rule:ident)*) => {$(
        impl $Rule {
            /// The text it matched.
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }
    )*};
}

text![Scheme Userinfo Port IpvFuture Ipv6Address H16 Ls32 Ipv4Address DecOctet Ipv4OrRegName
    Segment SegmentNz SegmentNzNc Query Fragment PctEncoded];
