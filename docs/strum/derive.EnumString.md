<!-- Generated from docs.rs rustdoc HTML: https://docs.rs/strum/latest/strum/derive.EnumString.html -->
<!-- Crawl timestamp: 2026-02-26T18:14:00Z -->

[ Docs.rs ][1]

  * [ strum-0.28.0 ][2]

    * strum 0.28.0 
    * [ Permalink ][3]
    * [ Docs.rs crate page ][4]
    * [MIT][5]

    * Links
    * [ Homepage ][6]
    * [ Repository ][6]
    * [ crates.io ][7]
    * [ Source ][8]

    * Owners
    * [ Peternator7 ][9]

    * Dependencies
    *       * [ phf ^0.13 _normal_ _optional_ ][10]
      * [ strum_macros ^0.28 _normal_ _optional_ ][11]

    * Versions
    *     * [ **41.18%** of the crate is documented ][12]

  * [ Platform ][13]
    * [aarch64-apple-darwin][14]
    * [aarch64-unknown-linux-gnu][15]
    * [i686-pc-windows-msvc][16]
    * [x86_64-pc-windows-msvc][17]
    * [x86_64-unknown-linux-gnu][18]
  * [ Feature flags ][19]



  * [docs.rs][13]
    * [ About docs.rs][20]
    * [ Badges][21]
    * [ Builds][22]
    * [ Metadata][23]
    * [ Shorthand URLs][24]
    * [ Download][25]
    * [ Rustdoc JSON][26]
    * [ Build queue][27]
    * [ Privacy policy][28]


  * [Rust][13]
    * [Rust website][29]
    * [The Book][30]
    * [Standard Library API Reference][31]
    * [Rust by Example][32]
    * [The Cargo Guide][33]
    * [Clippy Documentation][34]



[Skip to main content][35]

## [EnumString][13]

## [strum][36]0.28.0

## [EnumString][13]

### [Sections][13]

  * [Infallible Parsing][37]
  * [Custom Error Types][38]
  * [Example how to use `EnumString`][39]



## [In crate strum][36]

[strum][36]

# Derive Macro EnumString Copy item path

[Source][40]
``` 
#[derive(EnumString)]
{
    // Attributes available to this derive:
    #[strum]
}

```

Expand description

Converts strings to enum variants based on their name.

auto-derives `std::str::FromStr` on the enum. Each variant of the enum will match on its own name. This can be overridden using `serialize="DifferentName"` or `to_string="DifferentName"` on the attribute as shown below. Multiple deserializations can be added to the same variant. If the variant contains additional data, they will be set to their default values upon deserialization.

The `default` attribute can be applied to a tuple variant with a single data parameter. When a match isn’t found, the given variant will be returned and the input string will be captured in the parameter.

Note that the implementation of `FromStr` by default only matches on the name of the variant. There is an option to match on different case conversions through the `#[strum(serialize_all = "snake_case")]` type attribute.

See the [Additional Attributes][41] Section for more information on using this feature.

If you have a large enum, you may want to consider using the `use_phf` attribute here. PHF (Perfect Hash Functions) use a hash lookup instead of a linear search that may perform faster for large enums. Note: as with all optimizations, you should test this for your specific usecase rather than just assume it will be faster. With SIMD + pipelining, linear string search (aka memcmp) can be very fast for enums with a surprisingly large number of enum variants.

## [§][42]Infallible Parsing

If the enum has a `#[strum(default)]` variant and no `parse_err_ty` is set, parsing is infallible: `From<&str>` is derived instead of `TryFrom<&str>`, which allows calling `MyEnum::from("string")` directly.

## [§][43]Custom Error Types

The default error type is `strum::ParseError`. This can be overridden by applying both the `parse_err_ty` and `parse_err_fn` attributes at the type level. `parse_err_fn` should be a function that accepts an `&str` and returns the type `parse_err_ty`. See [this test case][44] for an example. When `parse_err_ty` is set, `TryFrom<&str>` is always derived, even if the enum has a `#[strum(default)]` variant.

## [§][45]Example how to use `EnumString`
``` 
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
enum Color {
    Red,
    // The Default value will be inserted into range if we match "Green".
    Green {
        range: usize,
    },

    // We can match on multiple different patterns.
    #[strum(serialize = "blue", serialize = "b")]
    Blue(usize),

    // Notice that we can disable certain variants from being found
    #[strum(disabled)]
    Yellow,

    // We can make the comparison case insensitive (however Unicode is not supported at the moment)
    #[strum(ascii_case_insensitive)]
    Black,
}

/*
//The generated code will look like:
impl std::str::FromStr for Color {
    type Err = ::strum::ParseError;

    fn from_str(s: &str) -> ::core::result::Result<Color, Self::Err> {
        match s {
            "Red" => ::core::result::Result::Ok(Color::Red),
            "Green" => ::core::result::Result::Ok(Color::Green { range:Default::default() }),
            "blue" => ::core::result::Result::Ok(Color::Blue(Default::default())),
            "b" => ::core::result::Result::Ok(Color::Blue(Default::default())),
            s if s.eq_ignore_ascii_case("Black") => ::core::result::Result::Ok(Color::Black),
            _ => ::core::result::Result::Err(::strum::ParseError::VariantNotFound),
        }
    }
}
*/

// simple from string
let color_variant = Color::from_str("Red").unwrap();
assert_eq!(Color::Red, color_variant);
// short version works too
let color_variant = Color::from_str("b").unwrap();
assert_eq!(Color::Blue(0), color_variant);
// was disabled for parsing = returns parse-error
let color_variant = Color::from_str("Yellow");
assert!(color_variant.is_err());
// however the variant is still normally usable
println!("{:?}", Color::Yellow);
let color_variant = Color::from_str("bLACk").unwrap();
assert_eq!(Color::Black, color_variant);
```

   [1]: https://docs.rs/
   [2]: derive.EnumString.html# (Helpful macros for working with enums and strings)
   [3]: https://docs.rs/strum/0.28.0/strum/derive.EnumString.html (Get a link to this specific version)
   [4]: https://docs.rs/crate/strum/latest (See strum in docs.rs)
   [5]: https://spdx.org/licenses/MIT
   [6]: https://github.com/Peternator7/strum
   [7]: https://crates.io/crates/strum (See strum in crates.io)
   [8]: https://docs.rs/crate/strum/latest/source/ (Browse source of strum-0.28.0)
   [9]: https://crates.io/users/Peternator7
   [10]: https://docs.rs/phf/^0.13/
   [11]: https://docs.rs/strum_macros/^0.28/
   [12]: https://docs.rs/crate/strum/latest
   [13]: derive.EnumString.html#
   [14]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-apple-darwin/strum/derive.EnumString.html
   [15]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-unknown-linux-gnu/strum/derive.EnumString.html
   [16]: https://docs.rs/crate/strum/latest/target-redirect/i686-pc-windows-msvc/strum/derive.EnumString.html
   [17]: https://docs.rs/crate/strum/latest/target-redirect/x86_64-pc-windows-msvc/strum/derive.EnumString.html
   [18]: https://docs.rs/crate/strum/latest/target-redirect/strum/derive.EnumString.html
   [19]: https://docs.rs/crate/strum/latest/features (Browse available feature flags of strum-0.28.0)
   [20]: https://docs.rs/about
   [21]: https://docs.rs/about/badges
   [22]: https://docs.rs/about/builds
   [23]: https://docs.rs/about/metadata
   [24]: https://docs.rs/about/redirections
   [25]: https://docs.rs/about/download
   [26]: https://docs.rs/about/rustdoc-json
   [27]: https://docs.rs/releases/queue
   [28]: https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs
   [29]: https://www.rust-lang.org/
   [30]: https://doc.rust-lang.org/book/
   [31]: https://doc.rust-lang.org/std/
   [32]: https://doc.rust-lang.org/rust-by-example/
   [33]: https://doc.rust-lang.org/cargo/guide/
   [34]: https://doc.rust-lang.org/nightly/clippy
   [35]: derive.EnumString.html#main-content
   [36]: index.html
   [37]: derive.EnumString.html#infallible-parsing (Infallible Parsing)
   [38]: derive.EnumString.html#custom-error-types (Custom Error Types)
   [39]: derive.EnumString.html#example-how-to-use-enumstring (Example how to use `EnumString`)
   [40]: https://docs.rs/strum_macros/0.28.0/x86_64-unknown-linux-gnu/src/strum_macros/lib.rs.html#134
   [41]: additional_attributes/index.html
   [42]: derive.EnumString.html#infallible-parsing
   [43]: derive.EnumString.html#custom-error-types
   [44]: https://github.com/Peternator7/strum/blob/9db3c4dc9b6f585aeb9f5f15f9cc18b6cf4fd780/strum_tests/tests/from_str.rs#L233
   [45]: derive.EnumString.html#example-how-to-use-enumstring

