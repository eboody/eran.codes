<!-- Generated from rustdoc HTML: field/trait.Visit.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/field/trait.Visit.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/field/trait.Visit.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/field/trait.Visit.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/field/trait.Visit.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/field/trait.Visit.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/field/trait.Visit.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/field/trait.Visit.html)
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

## Visit

[](../index.md)

## [tracing](../index.md)0.1.44

## Visit

### Sections

  * Examples

### Required Methods

  * record_debug

### Provided Methods

  * record_bool
  * record_bytes
  * record_error
  * record_f64
  * record_i64
  * record_i128
  * record_str
  * record_u64
  * record_u128
  * record_value

### Implementations on Foreign Types

  * DebugMap<'_, '_>
  * DebugStruct<'_, '_>

### Implementors

## [In tracing::field](index.md)

[tracing](../index.md)::[field](index.md)

# Trait Visit Copy item path

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#275)
    
    
    pub trait Visit {
        // Required method
        fn record_debug(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: &dyn [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"));
    
        // Provided methods
        fn record_value(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [Value](https://docs.rs/valuable/0.1.1/x86_64-unknown-linux-gnu/valuable/value/enum.Value.html "enum valuable::value::Value")<'_>) { ... }
        fn record_f64(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) { ... }
        fn record_i64(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) { ... }
        fn record_u64(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) { ... }
        fn record_i128(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)) { ... }
        fn record_u128(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)) { ... }
        fn record_bool(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) { ... }
        fn record_str(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) { ... }
        fn record_bytes(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: &[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)]) { ... }
        fn record_error(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: &(dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + 'static)) { ... }
    }

Expand description

Visits typed values.

An instance of `Visit` (“a visitor”) represents the logic necessary to record field values of various types. When an implementor of [`Value`](../trait.Value.md "trait tracing::Value") is [recorded](../trait.Value.md#tymethod.record "method tracing::Value::record"), it calls the appropriate method on the provided visitor to indicate the type that value should be recorded as.

When a [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") implementation [records an `Event`](../trait.Subscriber.md#tymethod.event "method tracing::Subscriber::event") or a [set of `Value`s added to a `Span`](../trait.Subscriber.md#tymethod.record "method tracing::Subscriber::record"), it can pass an `&mut Visit` to the `record` method on the provided [`ValueSet`](struct.ValueSet.md "struct tracing::field::ValueSet") or [`Event`](../struct.Event.md "struct tracing::Event"). This visitor will then be used to record all the field-value pairs present on that `Event` or `ValueSet`.

## §Examples

A simple visitor that writes to a string might be implemented like so:
    
    
    use std::fmt::{self, Write};
    use tracing::field::{Value, Visit, Field};
    pub struct StringVisitor<'a> {
        string: &'a mut String,
    }
    
    impl<'a> Visit for StringVisitor<'a> {
        fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
            write!(self.string, "{} = {:?}; ", field.name(), value).unwrap();
        }
    }

This visitor will format each recorded value using `fmt::Debug`, and append the field name and formatted value to the provided string, regardless of the type of the recorded value. When all the values have been recorded, the `StringVisitor` may be dropped, allowing the string to be printed or stored in some other data structure.

The `Visit` trait provides default implementations for `record_i64`, `record_u64`, `record_bool`, `record_str`, and `record_error`, which simply forward the recorded value to `record_debug`. Thus, `record_debug` is the only method which a `Visit` implementation _must_ implement. However, visitors may override the default implementations of these functions in order to implement type-specific behavior.

Additionally, when a visitor receives a value of a type it does not care about, it is free to ignore those values completely. For example, a visitor which only records numeric data might look like this:
    
    
    pub struct SumVisitor {
        sum: i64,
    }
    
    impl Visit for SumVisitor {
        fn record_i64(&mut self, _field: &Field, value: i64) {
           self.sum += value;
        }
    
        fn record_u64(&mut self, _field: &Field, value: u64) {
            self.sum += value as i64;
        }
    
        fn record_debug(&mut self, _field: &Field, _value: &dyn fmt::Debug) {
            // Do nothing
        }
    }

This visitor (which is probably not particularly useful) keeps a running sum of all the numeric values it records, and ignores all other values. A more practical example of recording typed values is presented in `examples/counters.rs`, which demonstrates a very simple metrics system implemented using `tracing`.
    
    
    **Note** : The record_error trait method is only
    available when the Rust standard library is present, as it requires the
    std::error::Error trait.
    

## Required Methods§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#340)

#### fn record_debug(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: &dyn [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"))

Visit a value implementing `fmt::Debug`.

## Provided Methods§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#281)

#### fn record_value(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [Value](https://docs.rs/valuable/0.1.1/x86_64-unknown-linux-gnu/valuable/value/enum.Value.html "enum valuable::value::Value")<'_>)

Available on **`tracing_unstable` and crate feature `valuable`** only.

Visits an arbitrary type implementing the [`valuable`](https://docs.rs/valuable) crate’s `Valuable` trait.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#286)

#### fn record_f64(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Visit a double-precision floating point value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#291)

#### fn record_i64(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Visit a signed 64-bit integer value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#296)

#### fn record_u64(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Visit an unsigned 64-bit integer value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#301)

#### fn record_i128(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html))

Visit a signed 128-bit integer value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#306)

#### fn record_u128(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html))

Visit an unsigned 128-bit integer value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#311)

#### fn record_bool(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

Visit a boolean value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#316)

#### fn record_str(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Visit a string value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#321)

#### fn record_bytes(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: &[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)])

Visit a byte slice.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#335)

#### fn record_error(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: &(dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") \+ 'static))

Available on **crate feature`std`** only.

Records a type implementing `Error`.
    
    
    **Note** : This is only enabled when the Rust standard library is
    present.
    

## Implementations on Foreign Types§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#425)§

### impl [Visit](trait.Visit.md "trait tracing::field::Visit") for [DebugMap](https://doc.rust-lang.org/nightly/core/fmt/builders/struct.DebugMap.html "struct core::fmt::builders::DebugMap")<'_, '_>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#426)§

#### fn record_debug(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: &dyn [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#419)§

### impl [Visit](trait.Visit.md "trait tracing::field::Visit") for [DebugStruct](https://doc.rust-lang.org/nightly/core/fmt/builders/struct.DebugStruct.html "struct core::fmt::builders::DebugStruct")<'_, '_>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#420)§

#### fn record_debug(&mut self, field: &[Field](struct.Field.md "struct tracing::field::Field"), value: &dyn [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"))

## Implementors§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#431-433)§

### impl<F> [Visit](trait.Visit.md "trait tracing::field::Visit") for F

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&[Field](struct.Field.md "struct tracing::field::Field"), &dyn [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug")),
