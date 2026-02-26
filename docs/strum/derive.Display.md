<!-- Generated from docs.rs rustdoc HTML: https://docs.rs/strum/latest/strum/derive.Display.html -->
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

## [Display][13]

## [strum][36]0.28.0

[strum][36]

# Derive Macro Display Copy item path

[Source][37]
``` 
#[derive(Display)]
{
    // Attributes available to this derive:
    #[strum]
}

```

Expand description

Converts enum variants to strings.

Deriving `Display` on an enum prints out the given enum. This enables you to perform round trip style conversions from enum into string and back again for unit style variants. `Display` choose which serialization to used based on the following criteria:

  1. If there is a `to_string` property, this value will be used. There can only be one per variant.

  2. Of the various `serialize` properties, the value with the longest length is chosen. If that behavior isn’t desired, you should use `to_string`.

  3. The name of the variant will be used if there are no `serialize` or `to_string` attributes.

  4. If the enum has a `strum(prefix = "some_value_")`, every variant will have that prefix prepended to the serialization.

  5. If the enum has a `strum(suffix = "_another_value")`, every variant will have that suffix appended to the serialization.

  6. Enums with fields support string interpolation. Note this means the variant will not “round trip” if you then deserialize the string.
     ``` 
     #[derive(strum_macros::Display)]
     pub enum Color {
         #[strum(to_string = "saturation is {sat}")]
         Red { sat: usize },
         #[strum(to_string = "hue is {1}, saturation is {0}")]
         Blue(usize, usize),
     }
     ```



``` 
// You need to bring the ToString trait into scope to use it
use std::string::ToString;
use strum_macros::Display;

#[derive(Display, Debug)]
enum Color {
    #[strum(serialize = "redred")]
    Red,
    Green {
        range: usize,
    },
    Blue(usize),
    Yellow,
    #[strum(to_string = "purple with {sat} saturation")]
    Purple {
        sat: usize,
    },
}

// uses the serialize string for Display
let red = Color::Red;
assert_eq!(String::from("redred"), format!("{}", red));
// by default the variants Name
let yellow = Color::Yellow;
assert_eq!(String::from("Yellow"), yellow.to_string());
// or for string formatting
assert_eq!(
   "blue: Blue green: Green",
   format!(
       "blue: {} green: {}",
       Color::Blue(10),
       Color::Green { range: 42 }
   )
);
// you can also use named fields in message
let purple = Color::Purple { sat: 10 };
assert_eq!(String::from("purple with 10 saturation"), purple.to_string());
```

   [1]: https://docs.rs/
   [2]: derive.Display.html# (Helpful macros for working with enums and strings)
   [3]: https://docs.rs/strum/0.28.0/strum/derive.Display.html (Get a link to this specific version)
   [4]: https://docs.rs/crate/strum/latest (See strum in docs.rs)
   [5]: https://spdx.org/licenses/MIT
   [6]: https://github.com/Peternator7/strum
   [7]: https://crates.io/crates/strum (See strum in crates.io)
   [8]: https://docs.rs/crate/strum/latest/source/ (Browse source of strum-0.28.0)
   [9]: https://crates.io/users/Peternator7
   [10]: https://docs.rs/phf/^0.13/
   [11]: https://docs.rs/strum_macros/^0.28/
   [12]: https://docs.rs/crate/strum/latest
   [13]: derive.Display.html#
   [14]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-apple-darwin/strum/derive.Display.html
   [15]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-unknown-linux-gnu/strum/derive.Display.html
   [16]: https://docs.rs/crate/strum/latest/target-redirect/i686-pc-windows-msvc/strum/derive.Display.html
   [17]: https://docs.rs/crate/strum/latest/target-redirect/x86_64-pc-windows-msvc/strum/derive.Display.html
   [18]: https://docs.rs/crate/strum/latest/target-redirect/strum/derive.Display.html
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
   [35]: derive.Display.html#main-content
   [36]: index.html
   [37]: https://docs.rs/strum_macros/0.28.0/x86_64-unknown-linux-gnu/src/strum_macros/lib.rs.html#463

