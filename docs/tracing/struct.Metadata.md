<!-- Generated from rustdoc HTML: struct.Metadata.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/struct.Metadata.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/struct.Metadata.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/struct.Metadata.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/struct.Metadata.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/struct.Metadata.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/struct.Metadata.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/struct.Metadata.html)
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

## Metadata

[](index.md)

## [tracing](index.md)0.1.44

## Metadata

### Sections

  * Equality

### Methods

  * callsite
  * fields
  * file
  * is_event
  * is_span
  * level
  * line
  * module_path
  * name
  * new
  * target

### Trait Implementations

  * Debug
  * Eq
  * PartialEq

### Auto Trait Implementations

  * !RefUnwindSafe
  * !UnwindSafe
  * Freeze
  * Send
  * Sync
  * Unpin

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

## [In crate tracing](index.md)

[tracing](index.md)

# Struct Metadata Copy item path

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#57)
    
    
    pub struct Metadata<'a> { /* private fields */ }

Expand description

Metadata describing a [span](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/span/index.html "mod tracing_core::span") or [event](event/index.md "mod tracing::event").

All spans and events have the following metadata:

  * A [name](struct.Metadata.md#method.name "method tracing::Metadata::name"), represented as a static string.
  * A [target](struct.Metadata.md#method.target "method tracing::Metadata::target"), a string that categorizes part of the system where the span or event occurred. The `tracing` macros default to using the module path where the span or event originated as the target, but it may be overridden.
  * A [verbosity level](struct.Metadata.md#method.level "method tracing::Metadata::level"). This determines how verbose a given span or event is, and allows enabling or disabling more verbose diagnostics situationally. See the documentation for the [`Level`](struct.Level.md "struct tracing::Level") type for details.
  * The names of the [fields](struct.Metadata.md#method.fields "method tracing::Metadata::fields") defined by the span or event.
  * Whether the metadata corresponds to a span or event.

In addition, the following optional metadata describing the source code location where the span or event originated _may_ be provided:

  * The [file name](struct.Metadata.md#method.file "method tracing::Metadata::file")
  * The [line number](struct.Metadata.md#method.line "method tracing::Metadata::line")
  * The [module path](struct.Metadata.md#method.module_path "method tracing::Metadata::module_path")

Metadata is used by [`Subscriber`](trait.Subscriber.md "trait tracing::Subscriber")s when filtering spans and events, and it may also be used as part of their data payload.

When created by the `event!` or `span!` macro, the metadata describing a particular event or span is constructed statically and exists as a single static instance. Thus, the overhead of creating the metadata is _significantly_ lower than that of creating the actual span. Therefore, filtering is based on metadata, rather than on the constructed span.

### §Equality

In well-behaved applications, two `Metadata` with equal [callsite identifiers](struct.Metadata.md#method.callsite "method tracing::Metadata::callsite") will be equal in all other ways (i.e., have the same `name`, `target`, etc.). Consequently, in release builds, [`Metadata::eq`](struct.Metadata.md#method.eq "method tracing::Metadata::eq") _only_ checks that its arguments have equal callsites. However, the equality of `Metadata`’s other fields is checked in debug builds.

## Implementations§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#249)§

### impl<'a> [Metadata](struct.Metadata.md "struct tracing::Metadata")<'a>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#252-261)

#### pub const fn new( name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), target: &'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), level: [Level](struct.Level.md "struct tracing::Level"), file: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>, line: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)>, module_path: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>, fields: [FieldSet](field/struct.FieldSet.md "struct tracing::field::FieldSet"), kind: [Kind](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/metadata/struct.Kind.html "struct tracing_core::metadata::Kind"), ) -> [Metadata](struct.Metadata.md "struct tracing::Metadata")<'a>

Construct new metadata for a span or event, with a name, target, level, field names, and optional source code location.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#276)

#### pub fn fields(&self) -> &[FieldSet](field/struct.FieldSet.md "struct tracing::field::FieldSet")

Returns the names of the fields on the described span or event.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#281)

#### pub fn level(&self) -> &[Level](struct.Level.md "struct tracing::Level")

Returns the level of verbosity of the described span or event.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#286)

#### pub fn name(&self) -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the name of the span.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#295)

#### pub fn target(&self) -> &'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string describing the part of the system where the span or event that this metadata describes occurred.

Typically, this is the module path, but alternate targets may be set when spans or events are constructed.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#301)

#### pub fn module_path(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>

Returns the path to the Rust module where the span occurred, or `None` if the module path is unknown.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#307)

#### pub fn file(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>

Returns the name of the source code file where the span occurred, or `None` if the file is unknown

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#313)

#### pub fn line(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)>

Returns the line number in the source code file where the span occurred, or `None` if the line number is unknown.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#320)

#### pub fn callsite(&self) -> [Identifier](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/callsite/struct.Identifier.html "struct tracing_core::callsite::Identifier")

Returns an opaque `Identifier` that uniquely identifies the callsite this `Metadata` originated from.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#325)

#### pub fn is_event(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the callsite kind is `Event`.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#330)

#### pub fn is_span(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Return true if the callsite kind is `Span`.

## Trait Implementations§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#343)§

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Metadata](struct.Metadata.md "struct tracing::Metadata")<'_>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#344)§

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#453)§

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Metadata](struct.Metadata.md "struct tracing::Metadata")<'_>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#455)§

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Metadata](struct.Metadata.md "struct tracing::Metadata")<'_>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0§

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#451)§

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [Metadata](struct.Metadata.md "struct tracing::Metadata")<'_>

## Auto Trait Implementations§

§

### impl<'a> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Metadata](struct.Metadata.md "struct tracing::Metadata")<'a>

§

### impl<'a> ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Metadata](struct.Metadata.md "struct tracing::Metadata")<'a>

§

### impl<'a> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Metadata](struct.Metadata.md "struct tracing::Metadata")<'a>

§

### impl<'a> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Metadata](struct.Metadata.md "struct tracing::Metadata")<'a>

§

### impl<'a> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Metadata](struct.Metadata.md "struct tracing::Metadata")<'a>

§

### impl<'a> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Metadata](struct.Metadata.md "struct tracing::Metadata")<'a>

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

[Source](../src/tracing/instrument.rs.html#325)§

### impl<T> [Instrument](trait.Instrument.md "trait tracing::Instrument") for T

[Source](../src/tracing/instrument.rs.html#86-91)§

#### fn [instrument](trait.Instrument.md#method.instrument)(self, span: [Span](struct.Span.md "struct tracing::Span")) -> [Instrumented](instrument/struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ

Instruments this type with the provided [`Span`](struct.Span.md "struct tracing::Span"), returning an `Instrumented` wrapper. [Read more](trait.Instrument.md#method.instrument)

[Source](../src/tracing/instrument.rs.html#128-130)§

#### fn [in_current_span](trait.Instrument.md#method.in_current_span)(self) -> [Instrumented](instrument/struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ

Instruments this type with the [current](struct.Span.md#method.current "associated function tracing::Span::current") [`Span`](struct.Span.md "struct tracing::Span"), returning an `Instrumented` wrapper. [Read more](trait.Instrument.md#method.in_current_span)

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

[Source](../src/tracing/instrument.rs.html#393)§

### impl<T> [WithSubscriber](instrument/trait.WithSubscriber.md "trait tracing::instrument::WithSubscriber") for T

[Source](../src/tracing/instrument.rs.html#176-184)§

#### fn [with_subscriber](instrument/trait.WithSubscriber.md#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](struct.Dispatch.md "struct tracing::Dispatch")>,

Available on **crate feature`std`** only.

Attaches the provided [`Subscriber`](trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper. [Read more](instrument/trait.WithSubscriber.md#method.with_subscriber)

[Source](../src/tracing/instrument.rs.html#228-233)§

#### fn [with_current_subscriber](instrument/trait.WithSubscriber.md#method.with_current_subscriber)(self) -> [WithDispatch](instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ

Available on **crate feature`std`** only.

Attaches the current [default](dispatcher/index.md#setting-the-default-subscriber "mod tracing::dispatcher") [`Subscriber`](trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper. [Read more](instrument/trait.WithSubscriber.md#method.with_current_subscriber)
