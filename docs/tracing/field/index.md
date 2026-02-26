<!-- Generated from rustdoc HTML: field/index.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/field/index.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/field/ "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/field/)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/field/)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/field/)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/field/)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/field/)
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

## Module field

[](../index.md)

## [tracing](../index.md)0.1.44

## Module field

### Sections

  * `Value`s and `Subscriber`s
  * Using `valuable`

### Module Items

  * Structs
  * Traits
  * Functions

## [In crate tracing](../index.md)

[tracing](../index.md)

# Module field Copy item path

[Source](../../src/tracing/field.rs.html#1-170)

Expand description

`Span` and `Event` key-value data.

Spans and events may be annotated with key-value data, referred to as _fields_. These fields consist of a mapping from a key (corresponding to a `&str` but represented internally as an array index) to a [`Value`](../trait.Value.md "trait tracing::Value").

## §`Value`s and `Subscriber`s

`Subscriber`s consume `Value`s as fields attached to [span](../span/index.md "mod tracing::span")s or [`Event`](../struct.Event.md "struct tracing::Event")s. The set of field keys on a given span or event is defined on its [`Metadata`](../struct.Metadata.md "struct tracing::Metadata"). When a span is created, it provides [`Attributes`](../span/struct.Attributes.md "struct tracing::span::Attributes") to the `Subscriber`’s [`new_span`](../trait.Subscriber.md#tymethod.new_span "method tracing::Subscriber::new_span") method, containing any fields whose values were provided when the span was created; and may call the `Subscriber`’s [`record`](../span/struct.Record.md "struct tracing::span::Record") method with additional [`Record`](../span/struct.Record.md "struct tracing::span::Record")s if values are added for more of its fields. Similarly, the [`Event`](../struct.Event.md "struct tracing::Event") type passed to the subscriber’s [`event`](../struct.Event.md "struct tracing::Event") method will contain any fields attached to each event.

`tracing` represents values as either one of a set of Rust primitives (`i64`, `u64`, `f64`, `bool`, and `&str`) or using a `fmt::Display` or `fmt::Debug` implementation. `Subscriber`s are provided these primitive value types as `dyn Value` trait objects.

These trait objects can be formatted using `fmt::Debug`, but may also be recorded as typed data by calling the [`Value::record`](../trait.Value.md#tymethod.record "method tracing::Value::record") method on these trait objects with a _visitor_ implementing the [`Visit`](trait.Visit.md "trait tracing::field::Visit") trait. This trait represents the behavior used to record values of various types. For example, an implementation of `Visit` might record integers by incrementing counters for their field names rather than printing them.

## §Using `valuable`

`tracing`’s [`Value`](../trait.Value.md "trait tracing::Value") trait is intentionally minimalist: it supports only a small number of Rust primitives as typed values, and only permits recording user-defined types with their [`fmt::Debug`](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") or [`fmt::Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") implementations. However, there are some cases where it may be useful to record nested values (such as arrays, `Vec`s, or `HashMap`s containing values), or user-defined `struct` and `enum` types without having to format them as unstructured text.

To address `Value`’s limitations, `tracing` offers experimental support for the [`valuable`](https://crates.io/crates/valuable) crate, which provides object-safe inspection of structured values. User-defined types can implement the [`valuable::Valuable`](https://docs.rs/valuable/latest/valuable/trait.Valuable.html) trait, and be recorded as a `tracing` field by calling their [`as_value`](https://docs.rs/valuable/latest/valuable/trait.Valuable.html#tymethod.as_value) method. If the [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") also supports the `valuable` crate, it can then visit those types fields as structured values using `valuable`.
    
    
        **Note** : valuable support is an
        [unstable feature](../index.md#unstable-features). See
        the documentation on unstable features for details on how to enable it.
    

For example:

ⓘ
    
    
    // Derive `Valuable` for our types:
    use valuable::Valuable;
    
    #[derive(Clone, Debug, Valuable)]
    struct User {
        name: String,
        age: u32,
        address: Address,
    }
    
    #[derive(Clone, Debug, Valuable)]
    struct Address {
        country: String,
        city: String,
        street: String,
    }
    
    let user = User {
        name: "Arwen Undomiel".to_string(),
        age: 3000,
        address: Address {
            country: "Middle Earth".to_string(),
            city: "Rivendell".to_string(),
            street: "leafy lane".to_string(),
        },
    };
    
    // Recording `user` as a `valuable::Value` will allow the `tracing` subscriber
    // to traverse its fields as a nested, typed structure:
    tracing::info!(current_user = user.as_value());

Alternatively, the [`valuable()`](fn.valuable.md "fn tracing::field::valuable") function may be used to convert a type implementing [`Valuable`](https://crates.io/crates/valuable) into a `tracing` field value.

When the `valuable` feature is enabled, the [`Visit`](trait.Visit.md "trait tracing::field::Visit") trait will include an optional [`record_value`](trait.Visit.md#method.record_value "method tracing::field::Visit::record_value") method. `Visit` implementations that wish to record `valuable` values can implement this method with custom behavior. If a visitor does not implement `record_value`, the [`valuable::Value`](https://docs.rs/valuable/latest/valuable/enum.Value.html) will be forwarded to the visitor’s [`record_debug`](trait.Visit.md#tymethod.record_debug "method tracing::field::Visit::record_debug") method.

## Structs§

[DebugValue](struct.DebugValue.md "struct tracing::field::DebugValue")
    A `Value` which serializes as a string using `fmt::Debug`.
[DisplayValue](struct.DisplayValue.md "struct tracing::field::DisplayValue")
    A `Value` which serializes using `fmt::Display`.
[Empty](struct.Empty.md "struct tracing::field::Empty")
    An empty field.
[Field](struct.Field.md "struct tracing::field::Field")
    An opaque key allowing _O_(1) access to a field in a `Span`’s key-value data.
[FieldSet](struct.FieldSet.md "struct tracing::field::FieldSet")
    Describes the fields present on a span.
[Iter](struct.Iter.md "struct tracing::field::Iter")
    An iterator over a set of fields.
[ValueSet](struct.ValueSet.md "struct tracing::field::ValueSet")
    A set of fields and values for a span.

## Traits§

[AsField](trait.AsField.md "trait tracing::field::AsField")
    Trait implemented to allow a type to be used as a field key.
[Value](trait.Value.md "trait tracing::field::Value")
    A field value of an erased type.
[Visit](trait.Visit.md "trait tracing::field::Visit")
    Visits typed values.

## Functions§

[debug](fn.debug.md "fn tracing::field::debug")
    Wraps a type implementing `fmt::Debug` as a `Value` that can be recorded using its `Debug` implementation.
[display](fn.display.md "fn tracing::field::display")
    Wraps a type implementing `fmt::Display` as a `Value` that can be recorded using its `Display` implementation.
[valuable](fn.valuable.md "fn tracing::field::valuable")`tracing_unstable` and `valuable`
    Wraps a type implementing [`Valuable`](https://docs.rs/valuable/latest/valuable/trait.Valuable.html) as a `Value` that can be recorded using its `Valuable` implementation.
