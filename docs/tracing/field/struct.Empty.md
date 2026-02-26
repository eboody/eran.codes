<!-- Generated from rustdoc HTML: field/struct.Empty.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/field/struct.Empty.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/field/struct.Empty.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/field/struct.Empty.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/field/struct.Empty.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/field/struct.Empty.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/field/struct.Empty.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/field/struct.Empty.html)
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

## Empty

[](../index.md)

## [tracing](../index.md)0.1.44

## Empty

### Trait Implementations

  * Debug
  * Eq
  * PartialEq
  * StructuralPartialEq
  * Value

### Auto Trait Implementations

  * Freeze
  * RefUnwindSafe
  * Send
  * Sync
  * Unpin
  * UnwindSafe

### Blanket Implementations

  * Any
  * Borrow<T>
  * BorrowMut<T>
  * From<T>
  * Instrument
  * Into<U>
  * TryFrom<U>
  * TryInto<U>
  * WithSubscriber

## [In tracing::field](index.md)

[tracing](../index.md)::[field](index.md)

# Struct Empty Copy item path

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#146)
    
    
    pub struct Empty;

Expand description

An empty field.

This can be used to indicate that the value of a field is not currently present but will be recorded later.

When a field’s value is `Empty`. it will not be recorded.

## Trait Implementations§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#145)§

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Empty](struct.Empty.md "struct tracing::field::Empty")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#145)§

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#145)§

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Empty](struct.Empty.md "struct tracing::field::Empty")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#145)§

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Empty](struct.Empty.md "struct tracing::field::Empty")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0§

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#784)§

### impl [Value](../trait.Value.md "trait tracing::Value") for [Empty](struct.Empty.md "struct tracing::field::Empty")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#786)§

#### fn [record](../trait.Value.md#tymethod.record)(&self, _: &[Field](struct.Field.md "struct tracing::field::Field"), _: &mut dyn [Visit](trait.Visit.md "trait tracing::field::Visit"))

Visits this value with the given `Visitor`.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#145)§

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [Empty](struct.Empty.md "struct tracing::field::Empty")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#145)§

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [Empty](struct.Empty.md "struct tracing::field::Empty")

## Auto Trait Implementations§

§

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Empty](struct.Empty.md "struct tracing::field::Empty")

§

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Empty](struct.Empty.md "struct tracing::field::Empty")

§

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Empty](struct.Empty.md "struct tracing::field::Empty")

§

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Empty](struct.Empty.md "struct tracing::field::Empty")

§

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Empty](struct.Empty.md "struct tracing::field::Empty")

§

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Empty](struct.Empty.md "struct tracing::field::Empty")

## Blanket Implementations§

§

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

§

#### fn [type_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

§

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

§

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

§

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

§

#### fn [borrow_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

§

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

§

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](../../src/tracing/instrument.rs.html#325)§

### impl<T> [Instrument](../trait.Instrument.md "trait tracing::Instrument") for T

[Source](../../src/tracing/instrument.rs.html#86-91)§

#### fn [instrument](../trait.Instrument.md#method.instrument)(self, span: [Span](../struct.Span.md "struct tracing::Span")) -> [Instrumented](../instrument/struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ

Instruments this type with the provided [`Span`](../struct.Span.md "struct tracing::Span"), returning an `Instrumented` wrapper. [Read more](../trait.Instrument.md#method.instrument)

[Source](../../src/tracing/instrument.rs.html#128-130)§

#### fn [in_current_span](../trait.Instrument.md#method.in_current_span)(self) -> [Instrumented](../instrument/struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ

Instruments this type with the [current](../struct.Span.md#method.current "associated function tracing::Span::current") [`Span`](../struct.Span.md "struct tracing::Span"), returning an `Instrumented` wrapper. [Read more](../trait.Instrument.md#method.in_current_span)

§

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

§

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

§

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

§

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

§

#### fn [try_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")>

Performs the conversion.

§

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

§

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

§

#### fn [try_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")>

Performs the conversion.

[Source](../../src/tracing/instrument.rs.html#393)§

### impl<T> [WithSubscriber](../instrument/trait.WithSubscriber.md "trait tracing::instrument::WithSubscriber") for T

[Source](../../src/tracing/instrument.rs.html#176-184)§

#### fn [with_subscriber](../instrument/trait.WithSubscriber.md#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")>,

Available on **crate feature`std`** only.

Attaches the provided [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](../instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper. [Read more](../instrument/trait.WithSubscriber.md#method.with_subscriber)

[Source](../../src/tracing/instrument.rs.html#228-233)§

#### fn [with_current_subscriber](../instrument/trait.WithSubscriber.md#method.with_current_subscriber)(self) -> [WithDispatch](../instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ

Available on **crate feature`std`** only.

Attaches the current [default](../dispatcher/index.md#setting-the-default-subscriber "mod tracing::dispatcher") [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](../instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper. [Read more](../instrument/trait.WithSubscriber.md#method.with_current_subscriber)
