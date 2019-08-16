# A HTML entity encoding library for Rust

[![crates.io version][crate-shield]][crate] [![TravisCI build status][travis-shield]][travis] [![Docs][docs-shield]][docs] ![License][license-shield]


## Example usage

All example assume a `extern crate escaper;` and `use escaper::{relevant functions here};` is present.

### Encoding
`escaper::encode_minimal()` encodes an input string using a minimal set of HTML entities.

```rust
let title = "Cats & dogs";
let tag = format!("<title>{}</title>", encode_minimal(title));
assert_eq!(tag.as_slice(), "<title>Cats &amp; dogs</title>");
```

There is also a `escaper::encode_attribute()` function for encoding strings that are to be used
as html attribute values.

### Decoding

`escaper::decode_html()` decodes an encoded string, replacing HTML entities with the

corresponding characters. Named, hex, and decimal entities are supported. A `Result` value is
returned, with either the decoded string in `Ok`, or an error in `Err`.

```rust
let encoded = "Cats&#x20;&amp;&#32;dogs";
let decoded = match decode_html(encoded) {
  Err(reason) => panic!("Error {:?} at character {}", reason.kind, reason.position),
  Ok(s) => s
};
assert_eq!(decoded.as_slice(), "Cats & dogs");
```

### Avoiding allocations

Both the encoding and decoding functions are available in forms that take a `Writer` for output rather
than returning an `String`. These version can be used to avoid allocation and copying if the returned
`String` was just going to be written to a `Writer` anyway.


## LICENSE

MIT or Apache 2.0

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in pgp by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[travis-shield]: https://travis-ci.org/dignifiedquire/rust-escaper.png?branch=master&style=flat-squre
[travis]: https://travis-ci.org/dignifiedquire/rust-escaper
[docs-shield]: https://img.shields.io/badge/docs-online-blue.svg?style=flat-square
[docs]: https://docs.rs/crate/escaper/
[license-shield]: https://img.shields.io/badge/License-MIT%2FApache2.0-green.svg?style=flat-square
[crate-shield]: https://img.shields.io/crates/v/escaper.svg?style=flat-square
[crate]: https://crates.io/crates/escaper
