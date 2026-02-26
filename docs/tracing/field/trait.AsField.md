<!-- Generated from rustdoc HTML: field/trait.AsField.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/field/trait.AsField.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/field/trait.AsField.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/field/trait.AsField.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/field/trait.AsField.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/field/trait.AsField.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/field/trait.AsField.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/field/trait.AsField.html)
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

## AsField

[](../index.md)

## [tracing](../index.md)0.1.44

## AsField

### Required Methods

  * as_field

### Implementations on Foreign Types

  * str

### Implementors

## [In tracing::field](index.md)

[tracing](../index.md)::[field](index.md)

# Trait AsField Copy item path

[Source](../../src/tracing/field.rs.html#129-135)
    
    
    pub trait AsField: Sealed {
        // Required method
        fn as_field(&self, metadata: &[Metadata](../struct.Metadata.md "struct tracing::Metadata")<'_>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Field](struct.Field.md "struct tracing::field::Field")>;
    }

Expand description

Trait implemented to allow a type to be used as a field key.
    
    
    **Note** : Although this is implemented for both the
    [Field](struct.Field.md) type _and_ any
    type that can be borrowed as an &str, only Field
    allows _O_(1) access.
    Indexing a field with a string results in an iterative search that performs
    string comparisons. Thus, if possible, once the key for a field is known, it
    should be used whenever possible.
    

## Required Methods§

[Source](../../src/tracing/field.rs.html#134)

#### fn as_field(&self, metadata: &[Metadata](../struct.Metadata.md "struct tracing::Metadata")<'_>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Field](struct.Field.md "struct tracing::field::Field")>

Attempts to convert `&self` into a `Field` with the specified `metadata`.

If `metadata` defines this field, then the field is returned. Otherwise, this returns `None`.

## Implementations on Foreign Types§

[Source](../../src/tracing/field.rs.html#161-166)§

### impl [AsField](trait.AsField.md "trait tracing::field::AsField") for [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](../../src/tracing/field.rs.html#163-165)§

#### fn as_field(&self, metadata: &[Metadata](../struct.Metadata.md "struct tracing::Metadata")<'_>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Field](struct.Field.md "struct tracing::field::Field")>

## Implementors§

[Source](../../src/tracing/field.rs.html#150-159)§

### impl [AsField](trait.AsField.md "trait tracing::field::AsField") for &[Field](struct.Field.md "struct tracing::field::Field")

[Source](../../src/tracing/field.rs.html#139-148)§

### impl [AsField](trait.AsField.md "trait tracing::field::AsField") for [Field](struct.Field.md "struct tracing::field::Field")
