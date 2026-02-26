<!-- Generated from docs.rs rustdoc HTML: https://docs.rs/strum/latest/strum/trait.EnumMessage.html -->
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

## [EnumMessage][13]

## [strum][36]0.28.0

## [EnumMessage][13]

### [Sections][13]

  * [Example][37]



### [Required Methods][38]

  * [get_detailed_message][39]
  * [get_documentation][40]
  * [get_message][41]
  * [get_serializations][42]



### [Implementors][43]

## [In crate strum][36]

[strum][36]

# Trait EnumMessage Copy item path

[Source][44]
``` 
pub trait EnumMessage {
    // Required methods
    fn [get_message][45](&self) -> [Option][46]<&'static [str][47]>;
    fn [get_detailed_message][48](&self) -> [Option][46]<&'static [str][47]>;
    fn [get_documentation][49](&self) -> [Option][46]<&'static [str][47]>;
    fn [get_serializations][50](&self) -> &'static [&'static [str][47]];
}
```

Expand description

Associates additional pieces of information with an Enum. This can be autoimplemented by deriving `EnumMessage` and annotating your variants with `#[strum(message="...")]`.

## [§][51]Example
``` 
// You need to bring the type into scope to use it!!!
use strum::EnumMessage;

#[derive(PartialEq, Eq, Debug, EnumMessage)]
enum Pet {
    #[strum(message="I have a dog")]
    #[strum(detailed_message="My dog's name is Spots")]
    Dog,
    /// I am documented.
    #[strum(message="I don't have a cat")]
    Cat,
}

let my_pet = Pet::Dog;
assert_eq!("I have a dog", my_pet.get_message().unwrap());
```

## Required Methods[§][38]

[Source][52]

#### fn [get_message][45](&self) -> [Option][46]<&'static [str][47]>

[Source][53]

#### fn [get_detailed_message][48](&self) -> [Option][46]<&'static [str][47]>

[Source][54]

#### fn [get_documentation][49](&self) -> [Option][46]<&'static [str][47]>

Get the doc comment associated with a variant if it exists.

[Source][55]

#### fn [get_serializations][50](&self) -> &'static [&'static [str][47]]

## Implementors[§][43]

   [1]: https://docs.rs/
   [2]: trait.EnumMessage.html# (Helpful macros for working with enums and strings)
   [3]: https://docs.rs/strum/0.28.0/strum/trait.EnumMessage.html (Get a link to this specific version)
   [4]: https://docs.rs/crate/strum/latest (See strum in docs.rs)
   [5]: https://spdx.org/licenses/MIT
   [6]: https://github.com/Peternator7/strum
   [7]: https://crates.io/crates/strum (See strum in crates.io)
   [8]: https://docs.rs/crate/strum/latest/source/ (Browse source of strum-0.28.0)
   [9]: https://crates.io/users/Peternator7
   [10]: https://docs.rs/phf/^0.13/
   [11]: https://docs.rs/strum_macros/^0.28/
   [12]: https://docs.rs/crate/strum/latest
   [13]: trait.EnumMessage.html#
   [14]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-apple-darwin/strum/trait.EnumMessage.html
   [15]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-unknown-linux-gnu/strum/trait.EnumMessage.html
   [16]: https://docs.rs/crate/strum/latest/target-redirect/i686-pc-windows-msvc/strum/trait.EnumMessage.html
   [17]: https://docs.rs/crate/strum/latest/target-redirect/x86_64-pc-windows-msvc/strum/trait.EnumMessage.html
   [18]: https://docs.rs/crate/strum/latest/target-redirect/strum/trait.EnumMessage.html
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
   [35]: trait.EnumMessage.html#main-content
   [36]: index.html
   [37]: trait.EnumMessage.html#example (Example)
   [38]: trait.EnumMessage.html#required-methods
   [39]: trait.EnumMessage.html#tymethod.get_detailed_message (get_detailed_message)
   [40]: trait.EnumMessage.html#tymethod.get_documentation (get_documentation)
   [41]: trait.EnumMessage.html#tymethod.get_message (get_message)
   [42]: trait.EnumMessage.html#tymethod.get_serializations (get_serializations)
   [43]: trait.EnumMessage.html#implementors
   [44]: https://docs.rs/strum/latest/src/strum/lib.rs.html#146-153
   [45]: trait.EnumMessage.html#tymethod.get_message
   [46]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [47]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [48]: trait.EnumMessage.html#tymethod.get_detailed_message
   [49]: trait.EnumMessage.html#tymethod.get_documentation
   [50]: trait.EnumMessage.html#tymethod.get_serializations
   [51]: trait.EnumMessage.html#example
   [52]: https://docs.rs/strum/latest/src/strum/lib.rs.html#147
   [53]: https://docs.rs/strum/latest/src/strum/lib.rs.html#148
   [54]: https://docs.rs/strum/latest/src/strum/lib.rs.html#151
   [55]: https://docs.rs/strum/latest/src/strum/lib.rs.html#152

