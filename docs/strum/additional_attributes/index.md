<!-- Generated from docs.rs rustdoc HTML: https://docs.rs/strum/latest/strum/additional_attributes/index.html -->
<!-- Crawl timestamp: 2026-02-26T18:13:59Z -->

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

## [Module additional_attributes][13]

## [strum][36]0.28.0

## [Module additional_attributes][13]

### [Sections][13]

  * [Documentation for Additional Attributes][37]
    * [Attributes on Enums][38]
    * [Attributes on Variants][39]



## [In crate strum][36]

[strum][36]

# Module additional_attributes Copy item path

[Source][40]

Expand description

## [§][41]Documentation for Additional Attributes

### [§][42]Attributes on Enums

Strum supports several custom attributes to modify the generated code. At the enum level, the following attributes are supported:

  * `#[strum(serialize_all = "case_style")]` attribute can be used to change the case used when serializing to and deserializing from strings. This feature is enabled by [withoutboats/heck][43] and supported case styles are:

    * `camelCase`
    * `PascalCase`
    * `kebab-case`
    * `snake_case`
    * `SCREAMING_SNAKE_CASE`
    * `SCREAMING-KEBAB-CASE`
    * `lowercase`
    * `UPPERCASE`
    * `title_case`
    * `mixed_case`
    * `Train-Case`
``` 
use strum_macros;
 
#[derive(Debug, Eq, PartialEq, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
enum Brightness {
    DarkBlack,
    Dim {
        glow: usize,
    },
    #[strum(serialize = "bright")]
    BrightWhite,
}
 
assert_eq!(
    String::from("dark_black"),
    Brightness::DarkBlack.to_string().as_ref()
);
assert_eq!(
    String::from("dim"),
    Brightness::Dim { glow: 0 }.to_string().as_ref()
);
assert_eq!(
    String::from("bright"),
    Brightness::BrightWhite.to_string().as_ref()
);
```

  * You can also apply the `#[strum(ascii_case_insensitive)]` attribute to the enum, and this has the same effect of applying it to every variant.




### [§][44]Attributes on Variants

Custom attributes are applied to a variant by adding `#[strum(parameter="value")]` to the variant.

  * `serialize="..."`: Changes the text that `FromStr()` looks for when parsing a string. This attribute can be applied multiple times to an element and the enum variant will be parsed if any of them match.

  * `to_string="..."`: Similar to `serialize`. This value will be included when using `FromStr()`. More importantly, this specifies what text to use when calling `variant.to_string()` with the `Display` derivation, or when calling `variant.as_ref()` with `AsRefStr`.

  * `default`: Applied to a single variant of an enum. The variant must be a Tuple-like variant with a single piece of data that can be created from a `&str` i.e. `T: From<&str>`. The generated code will now return the variant with the input string captured as shown below instead of failing.
    ``` 
    // Replaces this:
    _ => Err(strum::ParseError::VariantNotFound)
    // With this in generated code:
    default => Ok(Variant(default.into()))
    ```

The plugin will fail if the data doesn’t implement From<&str>. You can only have one `default` on your enum.

  * `transparent`: Signals that the inner field’s implementation should be used, instead of generating one for this variant. Only applicable to enum variants with a single field. Compatible with the `AsRefStr`, `Display` and `IntoStaticStr` derive macros. Note that `IntoStaticStr` has a few restrictions, the value must be `'static` and `const_into_str` is not supported in combination with `transparent` b/c transparent relies on a call on `From::from(variant)`.

  * `disabled`: removes variant from generated code.

  * `ascii_case_insensitive`: makes the comparison to this variant case insensitive (ASCII only). If the whole enum is marked `ascii_case_insensitive`, you can specify `ascii_case_insensitive = false` to disable case insensitivity on this variant.

  * `message=".."`: Adds a message to enum variant. This is used in conjunction with the `EnumMessage` trait to associate a message with a variant. If `detailed_message` is not provided, then `message` will also be returned when `get_detailed_message` is called.

  * `detailed_message=".."`: Adds a more detailed message to a variant. If this value is omitted, then `message` will be used in it’s place.

  * Structured documentation, as in `/// ...`: If using `EnumMessage`, is accessible via get_documentation().

  * `props(key="value")`: Enables associating additional information with a given variant.




   [1]: https://docs.rs/
   [2]: index.html# (Helpful macros for working with enums and strings)
   [3]: https://docs.rs/strum/0.28.0/strum/additional_attributes/ (Get a link to this specific version)
   [4]: https://docs.rs/crate/strum/latest (See strum in docs.rs)
   [5]: https://spdx.org/licenses/MIT
   [6]: https://github.com/Peternator7/strum
   [7]: https://crates.io/crates/strum (See strum in crates.io)
   [8]: https://docs.rs/crate/strum/latest/source/ (Browse source of strum-0.28.0)
   [9]: https://crates.io/users/Peternator7
   [10]: https://docs.rs/phf/^0.13/
   [11]: https://docs.rs/strum_macros/^0.28/
   [12]: https://docs.rs/crate/strum/latest
   [13]: index.html#
   [14]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-apple-darwin/strum/additional_attributes/
   [15]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-unknown-linux-gnu/strum/additional_attributes/
   [16]: https://docs.rs/crate/strum/latest/target-redirect/i686-pc-windows-msvc/strum/additional_attributes/
   [17]: https://docs.rs/crate/strum/latest/target-redirect/x86_64-pc-windows-msvc/strum/additional_attributes/
   [18]: https://docs.rs/crate/strum/latest/target-redirect/strum/additional_attributes/
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
   [35]: index.html#main-content
   [36]: ../index.html
   [37]: index.html#documentation-for-additional-attributes (Documentation for Additional Attributes)
   [38]: index.html#attributes-on-enums (Attributes on Enums)
   [39]: index.html#attributes-on-variants (Attributes on Variants)
   [40]: https://docs.rs/strum/latest/src/strum/additional_attributes.rs.html#1-98
   [41]: index.html#documentation-for-additional-attributes
   [42]: index.html#attributes-on-enums
   [43]: https://github.com/withoutboats/heck
   [44]: index.html#attributes-on-variants

