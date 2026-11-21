# tygr-uri

The URI syntax of [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986) as
[tygr](https://crates.io/crates/tygr) types: each rule of its Appendix A is a
type that parses and prints its text.

```rust
use tygr_uri::tygr::{Parse, Print};
use tygr_uri::{HierPart, Uri};

let uri = Uri::parse("foo://example.com:8042/over/there?name=ferret#nose").unwrap();
assert_eq!(uri.scheme.as_str(), "foo");
let HierPart::Authority { authority, path } = &uri.hier_part else { panic!() };
assert_eq!(authority.prefixed.print(), "example.com:8042");
assert_eq!(path.print(), "/over/there");
```

tygr tries alternatives in the order they are written, so four rules are
written differently from the RFC's ABNF, each matching the same text:

- `dec-octet`: longest alternative first.
- `path`: `path-rootless` first.
- `IPv6address`: `[ h16 *n( ":" h16 ) ]` for `[ *n( h16 ":" ) h16 ]`, so each
  `":"` goes with the `h16` after it, and the repetition stops at `"::"`.
- `host`: `IP-literal / reg-name`, as every `IPv4address` is also a `reg-name`
  (hence the type `Ipv4OrRegName`). Whether a host is an IPv4 address (Section
  3.2.2) is a matter of meaning, not structure: `Ipv4Address::try_from` decides.

IPv6 zone IDs are not accepted: RFC 9844 reverted RFC 6874, which added them.
Reference resolution and normalization are not covered.

The tests check the examples of RFC 3986, RFC 4291 and RFC 5952.

## License

Licensed under either of
[Apache License, Version 2.0](https://github.com/LuKuangChen/tygr-uri/blob/main/LICENSE-APACHE)
or [MIT license](https://github.com/LuKuangChen/tygr-uri/blob/main/LICENSE-MIT)
at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
