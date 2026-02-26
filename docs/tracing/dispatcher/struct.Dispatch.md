<!-- Generated from rustdoc HTML: dispatcher/struct.Dispatch.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/dispatcher/struct.Dispatch.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/dispatcher/struct.Dispatch.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/dispatcher/struct.Dispatch.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/dispatcher/struct.Dispatch.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/dispatcher/struct.Dispatch.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/dispatcher/struct.Dispatch.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/dispatcher/struct.Dispatch.html)
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

## Dispatch

[](../index.md)

## [tracing](../index.md)0.1.44

## Dispatch

### Methods

  * clone_span
  * current_span
  * downcast_ref
  * downgrade
  * drop_span
  * enabled
  * enter
  * event
  * exit
  * is
  * new
  * new_span
  * none
  * record
  * record_follows_from
  * register_callsite
  * try_close

### Trait Implementations

  * Clone
  * Debug
  * Default
  * From<S>

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
  * CloneToUninit
  * From<T>
  * Instrument
  * Into<U>
  * ToOwned
  * TryFrom<U>
  * TryInto<U>
  * WithSubscriber

## [In tracing::dispatcher](index.md)

[tracing](../index.md)::[dispatcher](index.md)

# Struct Dispatch Copy item path

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#149)
    
    
    pub struct Dispatch { /* private fields */ }

Expand description

`Dispatch` trace data to a [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber").

## Implementations§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#460)§

### impl [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#463)

#### pub fn none() -> [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

Returns a new `Dispatch` that discards events and spans.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#472-474)

#### pub fn new<S>(subscriber: S) -> [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

where S: [Subscriber](../trait.Subscriber.md "trait tracing::Subscriber") \+ [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") \+ [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") \+ 'static,

Returns a `Dispatch` that forwards to the given [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber").

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#502)

#### pub fn downgrade(&self) -> [WeakDispatch](struct.WeakDispatch.md "struct tracing::dispatcher::WeakDispatch")

Creates a [`WeakDispatch`](struct.WeakDispatch.md "struct tracing::dispatcher::WeakDispatch") from this `Dispatch`.

A [`WeakDispatch`](struct.WeakDispatch.md "struct tracing::dispatcher::WeakDispatch") is similar to a [`Dispatch`](../struct.Dispatch.md "struct tracing::Dispatch"), but it does not prevent the underlying [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") from being dropped. Instead, it only permits access while other references to the `Subscriber` exist. This is equivalent to the standard library’s [`Arc::downgrade`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html#method.downgrade "associated function alloc::sync::Arc::downgrade") method, but for `Dispatch` rather than `Arc`.

The primary use for creating a [`WeakDispatch`](struct.WeakDispatch.md "struct tracing::dispatcher::WeakDispatch") is to allow a `Subscriber` to hold a cyclical reference to itself without creating a memory leak. See [here](../trait.Subscriber.md#avoiding-memory-leaks "trait tracing::Subscriber") for details.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#525)

#### pub fn register_callsite( &self, metadata: &'static [Metadata](../struct.Metadata.md "struct tracing::Metadata")<'static>, ) -> [Interest](../subscriber/struct.Interest.md "struct tracing::subscriber::Interest")

Registers a new callsite with this subscriber, returning whether or not the subscriber is interested in being notified about the callsite.

This calls the [`register_callsite`](../trait.Subscriber.md#method.register_callsite "method tracing::Subscriber::register_callsite") function on the [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") that this `Dispatch` forwards to.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#555)

#### pub fn new_span(&self, span: &[Attributes](../span/struct.Attributes.md "struct tracing::span::Attributes")<'_>) -> [Id](../span/struct.Id.md "struct tracing::span::Id")

Record the construction of a new span, returning a new [ID](../span/struct.Id.md "struct tracing::span::Id") for the span being constructed.

This calls the [`new_span`](../trait.Subscriber.md#tymethod.new_span "method tracing::Subscriber::new_span") function on the [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") that this `Dispatch` forwards to.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#567)

#### pub fn record(&self, span: &[Id](../span/struct.Id.md "struct tracing::span::Id"), values: &[Record](../span/struct.Record.md "struct tracing::span::Record")<'_>)

Record a set of values on a span.

This calls the [`record`](../trait.Subscriber.md#tymethod.record "method tracing::Subscriber::record") function on the [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") that this `Dispatch` forwards to.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#580)

#### pub fn record_follows_from(&self, span: &[Id](../span/struct.Id.md "struct tracing::span::Id"), follows: &[Id](../span/struct.Id.md "struct tracing::span::Id"))

Adds an indication that `span` follows from the span with the id `follows`.

This calls the [`record_follows_from`](../trait.Subscriber.md#tymethod.record_follows_from "method tracing::Subscriber::record_follows_from") function on the [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") that this `Dispatch` forwards to.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#594)

#### pub fn enabled(&self, metadata: &[Metadata](../struct.Metadata.md "struct tracing::Metadata")<'_>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if a span with the specified [metadata](../struct.Metadata.md "struct tracing::Metadata") would be recorded.

This calls the [`enabled`](../trait.Subscriber.md#tymethod.enabled "method tracing::Subscriber::enabled") function on the [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") that this `Dispatch` forwards to.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#607)

#### pub fn event(&self, event: &[Event](../struct.Event.md "struct tracing::Event")<'_>)

Records that an [`Event`](../struct.Event.md "struct tracing::Event") has occurred.

This calls the [`event`](../struct.Event.md "struct tracing::Event") function on the [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") that this `Dispatch` forwards to.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#621)

#### pub fn enter(&self, span: &[Id](../span/struct.Id.md "struct tracing::span::Id"))

Records that a span has been can_enter.

This calls the [`enter`](../trait.Subscriber.md#tymethod.enter "method tracing::Subscriber::enter") function on the [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") that this `Dispatch` forwards to.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#632)

#### pub fn exit(&self, span: &[Id](../span/struct.Id.md "struct tracing::span::Id"))

Records that a span has been exited.

This calls the [`exit`](../trait.Subscriber.md#tymethod.exit "method tracing::Subscriber::exit") function on the [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") that this `Dispatch` forwards to.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#651)

#### pub fn clone_span(&self, id: &[Id](../span/struct.Id.md "struct tracing::span::Id")) -> [Id](../span/struct.Id.md "struct tracing::span::Id")

Notifies the subscriber that a [span ID](../span/struct.Id.md "struct tracing::span::Id") has been cloned.

This function must only be called with span IDs that were returned by this `Dispatch`’s [`new_span`](../trait.Subscriber.md#tymethod.new_span "method tracing::Subscriber::new_span") function. The `tracing` crate upholds this guarantee and any other libraries implementing instrumentation APIs must as well.

This calls the [`clone_span`](../trait.Subscriber.md#method.clone_span "method tracing::Subscriber::clone_span") function on the `Subscriber` that this `Dispatch` forwards to.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#679)

#### pub fn drop_span(&self, id: [Id](../span/struct.Id.md "struct tracing::span::Id"))

👎Deprecated since 0.1.2: use `Dispatch::try_close` instead

Notifies the subscriber that a [span ID](../span/struct.Id.md "struct tracing::span::Id") has been dropped.

This function must only be called with span IDs that were returned by this `Dispatch`’s [`new_span`](../trait.Subscriber.md#tymethod.new_span "method tracing::Subscriber::new_span") function. The `tracing` crate upholds this guarantee and any other libraries implementing instrumentation APIs must as well.

This calls the [`drop_span`](../trait.Subscriber.md#method.drop_span "method tracing::Subscriber::drop_span") function on the [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") that this `Dispatch` forwards to.
    
    
        **Deprecated** : The 
        try_close method is functionally identical, but returns
        true if the span is now closed. It should be used
        instead of this method.
    

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#699)

#### pub fn try_close(&self, id: [Id](../span/struct.Id.md "struct tracing::span::Id")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Notifies the subscriber that a [span ID](../span/struct.Id.md "struct tracing::span::Id") has been dropped, and returns `true` if there are now 0 IDs referring to that span.

This function must only be called with span IDs that were returned by this `Dispatch`’s [`new_span`](../trait.Subscriber.md#tymethod.new_span "method tracing::Subscriber::new_span") function. The `tracing` crate upholds this guarantee and any other libraries implementing instrumentation APIs must as well.

This calls the [`try_close`](../trait.Subscriber.md#method.try_close "method tracing::Subscriber::try_close") function on the [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") that this `Dispatch` forwards to.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#710)

#### pub fn current_span(&self) -> [Current](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/span/struct.Current.html "struct tracing_core::span::Current")

Returns a type representing this subscriber’s view of the current span.

This calls the [`current`](../trait.Subscriber.md#method.current_span "method tracing::Subscriber::current_span") function on the `Subscriber` that this `Dispatch` forwards to.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#717)

#### pub fn is<T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Returns `true` if this `Dispatch` forwards to a `Subscriber` of type `T`.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#724)

#### pub fn downcast_ref<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)>

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Returns some reference to the `Subscriber` this `Dispatch` forwards to if it is of type `T`, or `None` if it isn’t.

## Trait Implementations§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#148)§

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#148)§

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0§

#### fn [clone_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#736)§

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#737)§

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#729)§

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#731)§

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

Returns the current default dispatcher

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#751-753)§

### impl<S> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<S> for [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

where S: [Subscriber](../trait.Subscriber.md "trait tracing::Subscriber") \+ [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") \+ [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") \+ 'static,

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#756)§

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(subscriber: S) -> [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

Converts to this type from the input type.

## Auto Trait Implementations§

§

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

§

### impl ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

§

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

§

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

§

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

§

### impl ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")

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

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

§

#### unsafe fn [clone_to_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [*mut ](https://doc.rust-lang.org/nightly/std/primitive.pointer.html)[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit` [#126799](https://github.com/tokio-rs/tracing/issues/126799))

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

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

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

§

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

§

#### fn [to_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

§

#### fn [clone_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

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

Attaches the current [default](index.md#setting-the-default-subscriber "mod tracing::dispatcher") [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](../instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper. [Read more](../instrument/trait.WithSubscriber.md#method.with_current_subscriber)
