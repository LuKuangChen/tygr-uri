//! The examples of RFC 3986, by section, with the rule each one matches;
//! and the IPv6 address forms of RFC 4291 (Section 2.2) and RFC 5952.

use tygr_uri::tygr::{Parse, Print};
use tygr_uri::*;

/// Asserts that each text matches its rule: `section Rule: text…;`.
macro_rules! assert_match {
    ($($section:literal $Rule:ident: $($text:literal)+;)*) => {$($(
        assert!($Rule::check($text).is_ok(), "{} {}: {:?}", $section, stringify!($Rule), $text);
    )+)*};
}

/// Asserts that no text matches its rule: `section Rule: text…;`.
macro_rules! assert_no_match {
    ($($section:literal $Rule:ident: $($text:literal)+;)*) => {$($(
        assert!($Rule::check($text).is_err(), "{} {}: {:?}", $section, stringify!($Rule), $text);
    )+)*};
}

#[test]
fn examples_that_match() {
    assert_match! {
        "1.1" Uri: "http://localhost/" "file:///etc/hosts";
        "1.1.2" Uri: "ftp://ftp.is.co.za/rfc/rfc1808.txt" "http://www.ietf.org/rfc/rfc2396.txt"
            "ldap://[2001:db8::7]/c=GB?objectClass?one" "mailto:John.Doe@example.com"
            "news:comp.infosystems.www.servers.unix" "tel:+1-816-555-1212"
            "telnet://192.0.2.16:80/" "urn:oasis:names:specification:docbook:dtd:xml:4.1.2";
        "1.1.2" Ipv4Address: "192.0.2.16";
        "1.2.3" Uri: "scheme:." "scheme:..";
        "2.1" PctEncoded: "%20";
        "2.3" PctEncoded: "%41" "%5A" "%61" "%7A" "%30" "%39" "%2D" "%2E" "%5F" "%7E";
        "2.4" PctEncoded: "%25";
        "2.5" Segment: "Laguna%20Beach" "%D3%81%87%A4%95%81@%C2%85%81%83%88" "A" "%C3%80"
            "%E3%82%A2";
        "3" Uri: "foo://example.com:8042/over/there?name=ferret#nose"
            "urn:example:animal:ferret:nose";
        "3.1" Scheme: "http" "HTTP";
        "3.2.1" Userinfo: "user:password";
        "3.3" Uri: "mailto:fred@example.com" "foo://info.example.com?fred";
        "3.3" Segment: "name;v=1.1" "name,1.1";
        "4.2" Uri: "this:that";
        "4.2" Path: "this:that";
        "4.2" RelativeRef: "./this:that";
        "4.5" RelativeRef: "www.w3.org/Addressing/";
        "5.2.4" Path: "/a/b/c/./../../g" "/a/g" "mid/content=5/../6" "mid/6";
        "5.4" Uri: "http://a/b/c/d;p?q" "g:h" "http:g" "http://a/b/c/g" "http://a/b/c/g/"
            "http://a/g" "http://g" "http://a/b/c/d;p?y" "http://a/b/c/g?y" "http://a/b/c/d;p?q#s"
            "http://a/b/c/g#s" "http://a/b/c/g?y#s" "http://a/b/c/;x" "http://a/b/c/g;x"
            "http://a/b/c/g;x?y#s" "http://a/b/c/" "http://a/b/" "http://a/b/g" "http://a/"
            "http://a/b/c/g." "http://a/b/c/.g" "http://a/b/c/g.." "http://a/b/c/..g"
            "http://a/b/c/g/h" "http://a/b/c/h" "http://a/b/c/g;x=1/y" "http://a/b/c/y"
            "http://a/b/c/g?y/./x" "http://a/b/c/g?y/../x" "http://a/b/c/g#s/./x"
            "http://a/b/c/g#s/../x";
        "5.4.1" RelativeRef: "g" "./g" "g/" "/g" "//g" "?y" "g?y" "#s" "g#s" "g?y#s" ";x" "g;x"
            "g;x?y#s" "" "." "./" ".." "../" "../g" "../.." "../../" "../../g";
        "5.4.2" RelativeRef: "../../../g" "../../../../g" "/./g" "/../g" "g." ".g" "g.." "..g"
            "./../g" "./g/." "g/./h" "g/../h" "g;x=1/./y" "g;x=1/../y" "g?y/./x" "g?y/../x"
            "g#s/./x" "g#s/../x";
        "6.2.2" Uri: "example://a/b/c/%7Bfoo%7D" "eXAMPLE://a/./b/../b/%63/%7bfoo%7d";
        "6.2.2.1" Uri: "HTTP://www.EXAMPLE.com/" "http://www.example.com/";
        "6.2.3" Uri: "http://example.com" "http://example.com/" "http://example.com:/"
            "http://example.com:80/" "http://example.com/?" "mailto:Joe@Example.COM"
            "mailto:Joe@example.com";
        "6.2.4" Uri: "http://example.com/data" "http://example.com/data/";
        "7.6" Uri: "ftp://cnn.example.com&story=breaking_news@10.0.0.1/top_story.htm";
        "7.6" Ipv4Address: "10.0.0.1";
        "B" Uri: "http://www.ics.uci.edu/pub/ietf/uri/#Related";
        "C" Uri: "http://example.com/" "http://www.w3.org/Addressing/" "ftp://foo.example.com/rfc/"
            "http://www.ics.uci.edu/pub/ietf/uri/historical.html#WARNING";
        "RFC 4291" Ipv6Address: "ABCD:EF01:2345:6789:ABCD:EF01:2345:6789"
            "2001:DB8:0:0:8:800:200C:417A" "2001:DB8::8:800:200C:417A" "FF01:0:0:0:0:0:0:101"
            "FF01::101" "0:0:0:0:0:0:0:1" "::1" "0:0:0:0:0:0:0:0" "::" "0:0:0:0:0:0:13.1.68.3"
            "0:0:0:0:0:FFFF:129.144.52.38" "::13.1.68.3" "::FFFF:129.144.52.38";
        "RFC 5952" Ipv6Address: "2001:db8:0:0:1:0:0:1" "2001:0db8:0:0:1:0:0:1" "2001:db8::1:0:0:1"
            "2001:db8::0:1:0:0:1" "2001:0db8::1:0:0:1" "2001:db8:0:0:1::1" "2001:db8:0000:0:1::1"
            "2001:DB8:0:0:1::1" "2001:db8:aaaa:bbbb:cccc:dddd:eeee:0001"
            "2001:db8:aaaa:bbbb:cccc:dddd:eeee:001" "2001:db8:aaaa:bbbb:cccc:dddd:eeee:01"
            "2001:db8:aaaa:bbbb:cccc:dddd:eeee:1" "2001:db8:0:0:0::1" "2001:db8:0:0::1"
            "2001:db8:0::1" "2001:db8::1" "2001:db8::aaaa:0:0:1" "2001:db8:0:0:aaaa::1";
    }
}

#[test]
fn examples_that_dont_match() {
    assert_no_match! {
        "4.2" RelativeRef: "this:that";
        "RFC 6874" Uri: "http://[fe80::a%25en1]";
    }
}

#[test]
fn hosts_that_only_begin_like_an_ipv4_address() {
    // Constructed from Section 3.2.2: such a host is a registered name.
    assert_match! {
        "3.2.2" Uri: "http://1.2.3.4.example/" "http://256.0.0.1/" "http://01.2.3.4/";
    }
    assert_no_match! {
        "3.2.2" Ipv4Address: "1.2.3.4.example" "256.0.0.1" "01.2.3.4";
    }
}

/// Examples with their components, as Section 3, Section 3.3 and Appendix B
/// give them: `URI | scheme | authority | path | query | fragment`.
const COMPONENTS: &str = "
foo://example.com:8042/over/there?name=ferret#nose | foo | example.com:8042 | /over/there | name=ferret | nose
urn:example:animal:ferret:nose | urn | <undefined> | example:animal:ferret:nose | <undefined> | <undefined>
mailto:fred@example.com | mailto | <undefined> | fred@example.com | <undefined> | <undefined>
foo://info.example.com?fred | foo | info.example.com |  | fred | <undefined>
http://www.ics.uci.edu/pub/ietf/uri/#Related | http | www.ics.uci.edu | /pub/ietf/uri/ | <undefined> | Related
";

fn components(uri: &str) -> Vec<String> {
    let uri = Uri::parse(uri).unwrap();
    let undefined = || "<undefined>".to_string();
    let (authority, path) = match &uri.hier_part {
        HierPart::Authority {
            authority, path, ..
        } => (authority.print(), path.print()),
        other => (undefined(), other.print()),
    };
    let query = uri
        .query
        .map_or_else(undefined, |query| query.as_str().into());
    let fragment = uri
        .fragment
        .map_or_else(undefined, |fragment| fragment.as_str().into());
    vec![uri.scheme.as_str().into(), authority, path, query, fragment]
}

#[test]
fn components_of_examples() {
    for line in COMPONENTS.lines().filter(|line| !line.is_empty()) {
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        assert_eq!(components(cells[0]), cells[1..], "{line}");
    }
    // 7.6: the host is 10.0.0.1, after the user information.
    let authority = Authority::parse("cnn.example.com&story=breaking_news@10.0.0.1").unwrap();
    assert_eq!(authority.host.print(), "10.0.0.1");
}
