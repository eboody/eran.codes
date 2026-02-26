<!-- Generated from rustdoc HTML: field/fn.display.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/field/fn.display.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/field/fn.display.html "Get a link to this specific version")
    * [ Docs.rs crate page ](/crate/tracing/latest "See tracing in docs.rs")
    * [MIT](https://spdx.org/licenses/MIT)

    * Links
    * [ Homepage ](https://tokio.rs)
    * [ Repository ](https://github.com/tokio-rs/tracing)
    * [ crates.io ](https://crates.io/crates/tracing "See tracing in crates.io")
    * [ Source ](/crate/tracing/latest/source/ "Browse source of tracing-0.1.44")

    * Owners
    * [ carllerche ](https://crates.io/users/carllerche)
    * [ hawkw ](https://crates.io/users/hawkw)
    * [ github:tokio-rs:publish-tracing ](https://crates.io/teams/github:tokio-rs:publish-tracing)

    * Dependencies
    *       * [ log ^0.4.17 _normal_ _optional_ ](/log/^0.4.17/)
      * [ pin-project-lite ^0.2.9 _normal_ ](/pin-project-lite/^0.2.9/)
      * [ tracing-attributes ^0.1.31 _normal_ _optional_ ](/tracing-attributes/^0.1.31/)
      * [ tracing-core ^0.1.36 _normal_ ](/tracing-core/^0.1.36/)
      * [ criterion ^0.3.6 _dev_ ](/criterion/^0.3.6/)
      * [ futures ^0.3.21 _dev_ ](/futures/^0.3.21/)
      * [ log ^0.4.17 _dev_ ](/log/^0.4.17/)
      * [ wasm-bindgen-test ^0.3.38 _dev_ ](/wasm-bindgen-test/^0.3.38/)

    * Versions
    *     * [ **100%** of the crate is documented ](/crate/tracing/latest)

  * Platform
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/field/fn.display.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/field/fn.display.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/field/fn.display.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/field/fn.display.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/field/fn.display.html)
  * [ Feature flags ](/crate/tracing/latest/features "Browse available feature flags of tracing-0.1.44")

  * docs.rs
    * [ About docs.rs](/about)
    * [ Badges](/about/badges)
    * [ Builds](/about/builds)
    * [ Metadata](/about/metadata)
    * [ Shorthand URLs](/about/redirections)
    * [ Download](/about/download)
    * [ Rustdoc JSON](/about/rustdoc-json)
    * [ Build queue](/releases/queue)
    * [ Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

  * Rust
    * [Rust website](https://www.rust-lang.org/)
    * [The Book](https://doc.rust-lang.org/book/)
    * [Standard Library API Reference](https://doc.rust-lang.org/std/)
    * [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
    * [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
    * [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

## display

[](../index.md)

## [tracing](../index.md)0.1.44

## [In tracing::field](index.md)

[tracing](../index.md)::[field](index.md)

# Function display Copy item path

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#368-370)
    
    
    pub fn display<T>(t: T) -> [DisplayValue](struct.DisplayValue.md "struct tracing::field::DisplayValue")<T>
    
    where
        T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Expand description

Wraps a type implementing `fmt::Display` as a `Value` that can be recorded using its `Display` implementation.
