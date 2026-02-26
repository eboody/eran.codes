<!-- Generated from rustdoc HTML: field/trait.Value.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/field/trait.Value.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/field/trait.Value.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/field/trait.Value.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/field/trait.Value.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/field/trait.Value.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/field/trait.Value.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/field/trait.Value.html)
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

## Value

[](../index.md)

## [tracing](../index.md)0.1.44

## Value

### Required Methods

  * record

### Implementations on Foreign Types

  * &'a T
  * &'a mut T
  * &dyn Valuable
  * Arguments<'_>
  * Box<T>
  * NonZero<i8>
  * NonZero<i16>
  * NonZero<i32>
  * NonZero<i64>
  * NonZero<i128>
  * NonZero<isize>
  * NonZero<u8>
  * NonZero<u16>
  * NonZero<u32>
  * NonZero<u64>
  * NonZero<u128>
  * NonZero<usize>
  * Option<T>
  * String
  * Value<'_>
  * Wrapping<T>
  * [u8]
  * bool
  * dyn Error
  * dyn Error + Send
  * dyn Error + Sync
  * dyn Error + Sync + Send
  * f32
  * f64
  * i8
  * i16
  * i32
  * i64
  * i128
  * isize
  * str
  * u8
  * u16
  * u32
  * u64
  * u128
  * usize

### Trait Implementations

  * Debug
  * Display

### Implementors

## [In tracing::field](index.md)

[tracing](../index.md)::[field](index.md)

# Trait Value Copy item path

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#350)
    
    
    pub trait Value: Sealed {
        // Required method
        fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"));
    }

Expand description

A field value of an erased type.

Implementors of `Value` may call the appropriate typed recording methods on the [visitor](trait.Visit.md "trait tracing::field::Visit") passed to their `record` method in order to indicate how their data should be recorded.

## Required Methods§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#352)

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

Visits this value with the given `Visitor`.

## Trait Implementations§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#680)§

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for dyn [Value](../trait.Value.md "trait tracing::Value")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#681)§

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#709)§

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for dyn [Value](../trait.Value.md "trait tracing::Value")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#710)§

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

## Implementations on Foreign Types§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#777)§

### impl [Value](../trait.Value.md "trait tracing::Value") for &dyn [Valuable](https://docs.rs/valuable/0.1.1/x86_64-unknown-linux-gnu/valuable/valuable/trait.Valuable.html "trait valuable::valuable::Valuable")

Available on **`tracing_unstable` and crate feature `valuable`** only.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#778)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#766)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [Value](https://docs.rs/valuable/0.1.1/x86_64-unknown-linux-gnu/valuable/value/enum.Value.html "enum valuable::value::Value")<'_>

Available on **`tracing_unstable` and crate feature `valuable`** only.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#767)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#571)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#572)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#674)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#675)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#655)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [Arguments](https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html "struct core::fmt::Arguments")<'_>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#656)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#579)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)]

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#580)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#590)§

### impl [Value](../trait.Value.md "trait tracing::Value") for dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error")

Available on **crate feature`std`** only.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#591)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#601)§

### impl [Value](../trait.Value.md "trait tracing::Value") for dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") \+ [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")

Available on **crate feature`std`** only.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#602)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#612)§

### impl [Value](../trait.Value.md "trait tracing::Value") for dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") \+ [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")

Available on **crate feature`std`** only.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#613)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#623)§

### impl [Value](../trait.Value.md "trait tracing::Value") for dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") \+ [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") \+ [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")

Available on **crate feature`std`** only.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#624)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#631-633)§

### impl<'a, T> [Value](../trait.Value.md "trait tracing::Value") for [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Value](../trait.Value.md "trait tracing::Value") \+ 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#635)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#642-644)§

### impl<'a, T> [Value](../trait.Value.md "trait tracing::Value") for [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Value](../trait.Value.md "trait tracing::Value") \+ 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#646)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#791)§

### impl<T> [Value](../trait.Value.md "trait tracing::Value") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [Value](../trait.Value.md "trait tracing::Value"),

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#792)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#663-665)§

### impl<T> [Value](../trait.Value.md "trait tracing::Value") for [Box](https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html "struct alloc::boxed::Box")<T>

where T: [Value](../trait.Value.md "trait tracing::Value") \+ ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#668)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#563)§

### impl<T> [Value](../trait.Value.md "trait tracing::Value") for [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<T>

where T: [Value](../trait.Value.md "trait tracing::Value"),

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#564)§

#### fn record(&self, key: &[Field](struct.Field.md "struct tracing::field::Field"), visitor: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

## Implementors§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#784)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [Empty](struct.Empty.md "struct tracing::field::Empty")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#744-746)§

### impl<T> [Value](../trait.Value.md "trait tracing::Value") for [DebugValue](struct.DebugValue.md "struct tracing::field::DebugValue")<T>

where T: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#719-721)§

### impl<T> [Value](../trait.Value.md "trait tracing::Value") for [DisplayValue](struct.DisplayValue.md "struct tracing::field::DisplayValue")<T>

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),
