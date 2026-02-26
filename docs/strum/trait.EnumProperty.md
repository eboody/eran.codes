<!-- Generated from docs.rs rustdoc HTML: https://docs.rs/strum/latest/strum/trait.EnumProperty.html -->
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

## [EnumProperty][13]

## [strum][36]0.28.0

## [EnumProperty][13]

### [Sections][13]

  * [Example][37]



### [Required Methods][38]

  * [get_bool][39]
  * [get_int][40]
  * [get_str][41]



### [Implementors][42]

## [In crate strum][36]

[strum][36]

# Trait EnumProperty Copy item path

[Source][43]
``` 
pub trait EnumProperty {
    // Required methods
    fn [get_str][44](&self, prop: &[str][45]) -> [Option][46]<&'static [str][45]>;
    fn [get_int][47](&self, _prop: &[str][45]) -> [Option][46]<[i64][48]>;
    fn [get_bool][49](&self, _prop: &[str][45]) -> [Option][46]<[bool][50]>;
}
```

Expand description

`EnumProperty` is a trait that makes it possible to store additional information with enum variants. This trait is designed to be used with the macro of the same name in the `strum_macros` crate. Currently, the string, integer and bool literals are supported in attributes.

## [§][51]Example
``` 
// You need to bring the type into scope to use it!!!
use strum::EnumProperty;

#[derive(PartialEq, Eq, Debug, EnumProperty)]
enum Class {
    #[strum(props(Teacher="Ms.Frizzle", Room="201", students=16, mandatory=true))]
    History,
    #[strum(props(Teacher="Mr.Smith"))]
    #[strum(props(Room="103", students=10))]
    Mathematics,
    #[strum(props(Time="2:30", mandatory=true))]
    Science,
}

let history = Class::History;
assert_eq!("Ms.Frizzle", history.get_str("Teacher").unwrap());
assert_eq!(16, history.get_int("students").unwrap());
assert!(history.get_bool("mandatory").unwrap());
```

## Required Methods[§][38]

[Source][52]

#### fn [get_str][44](&self, prop: &[str][45]) -> [Option][46]<&'static [str][45]>

[Source][53]

#### fn [get_int][47](&self, _prop: &[str][45]) -> [Option][46]<[i64][48]>

[Source][54]

#### fn [get_bool][49](&self, _prop: &[str][45]) -> [Option][46]<[bool][50]>

## Implementors[§][42]

   [1]: https://docs.rs/
   [2]: trait.EnumProperty.html# (Helpful macros for working with enums and strings)
   [3]: https://docs.rs/strum/0.28.0/strum/trait.EnumProperty.html (Get a link to this specific version)
   [4]: https://docs.rs/crate/strum/latest (See strum in docs.rs)
   [5]: https://spdx.org/licenses/MIT
   [6]: https://github.com/Peternator7/strum
   [7]: https://crates.io/crates/strum (See strum in crates.io)
   [8]: https://docs.rs/crate/strum/latest/source/ (Browse source of strum-0.28.0)
   [9]: https://crates.io/users/Peternator7
   [10]: https://docs.rs/phf/^0.13/
   [11]: https://docs.rs/strum_macros/^0.28/
   [12]: https://docs.rs/crate/strum/latest
   [13]: trait.EnumProperty.html#
   [14]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-apple-darwin/strum/trait.EnumProperty.html
   [15]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-unknown-linux-gnu/strum/trait.EnumProperty.html
   [16]: https://docs.rs/crate/strum/latest/target-redirect/i686-pc-windows-msvc/strum/trait.EnumProperty.html
   [17]: https://docs.rs/crate/strum/latest/target-redirect/x86_64-pc-windows-msvc/strum/trait.EnumProperty.html
   [18]: https://docs.rs/crate/strum/latest/target-redirect/strum/trait.EnumProperty.html
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
   [35]: trait.EnumProperty.html#main-content
   [36]: index.html
   [37]: trait.EnumProperty.html#example (Example)
   [38]: trait.EnumProperty.html#required-methods
   [39]: trait.EnumProperty.html#tymethod.get_bool (get_bool)
   [40]: trait.EnumProperty.html#tymethod.get_int (get_int)
   [41]: trait.EnumProperty.html#tymethod.get_str (get_str)
   [42]: trait.EnumProperty.html#implementors
   [43]: https://docs.rs/strum/latest/src/strum/lib.rs.html#183-187
   [44]: trait.EnumProperty.html#tymethod.get_str
   [45]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [46]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [47]: trait.EnumProperty.html#tymethod.get_int
   [48]: https://doc.rust-lang.org/nightly/std/primitive.i64.html
   [49]: trait.EnumProperty.html#tymethod.get_bool
   [50]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [51]: trait.EnumProperty.html#example
   [52]: https://docs.rs/strum/latest/src/strum/lib.rs.html#184
   [53]: https://docs.rs/strum/latest/src/strum/lib.rs.html#185
   [54]: https://docs.rs/strum/latest/src/strum/lib.rs.html#186

