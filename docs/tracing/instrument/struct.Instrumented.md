<!-- Generated from rustdoc HTML: instrument/struct.Instrumented.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/instrument/struct.Instrumented.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/instrument/struct.Instrumented.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/instrument/struct.Instrumented.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/instrument/struct.Instrumented.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/instrument/struct.Instrumented.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/instrument/struct.Instrumented.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/instrument/struct.Instrumented.html)
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

## Instrumented

[](../index.md)

## [tracing](../index.md)0.1.44

## Instrumented

### Methods

  * inner
  * inner_mut
  * inner_pin_mut
  * inner_pin_ref
  * into_inner
  * span
  * span_mut

### Trait Implementations

  * Clone
  * Debug
  * Drop
  * Future
  * Unpin

### Auto Trait Implementations

  * !RefUnwindSafe
  * !UnwindSafe
  * Freeze
  * Send
  * Sync

### Blanket Implementations

  * Any
  * Borrow<T>
  * BorrowMut<T>
  * CloneToUninit
  * From<T>
  * Instrument
  * Into<U>
  * IntoFuture
  * ToOwned
  * TryFrom<U>
  * TryInto<U>
  * WithSubscriber

## [In tracing::instrument](index.md)

[tracing](../index.md)::[instrument](index.md)

# Struct Instrumented Copy item path

[Source](../../src/tracing/instrument.rs.html#254-288)
    
    
    pub struct Instrumented<T> { /* private fields */ }

Expand description

A [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html "trait core::future::future::Future") that has been instrumented with a `tracing` [`Span`](../struct.Span.md "struct tracing::Span").

This type is returned by the [`Instrument`](../trait.Instrument.md "trait tracing::Instrument") extension trait. See that trait’s documentation for details.

## Implementations§

[Source](../../src/tracing/instrument.rs.html#327-374)§

### impl<T> [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<T>

[Source](../../src/tracing/instrument.rs.html#329-331)

#### pub fn span(&self) -> &[Span](../struct.Span.md "struct tracing::Span")

Borrows the `Span` that this type is instrumented by.

[Source](../../src/tracing/instrument.rs.html#334-336)

#### pub fn span_mut(&mut self) -> &mut [Span](../struct.Span.md "struct tracing::Span")

Mutably borrows the `Span` that this type is instrumented by.

[Source](../../src/tracing/instrument.rs.html#339-341)

#### pub fn inner(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Borrows the wrapped type.

[Source](../../src/tracing/instrument.rs.html#344-346)

#### pub fn inner_mut(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows the wrapped type.

[Source](../../src/tracing/instrument.rs.html#349-351)

#### pub fn inner_pin_ref(self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&Self>) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)>

Get a pinned reference to the wrapped type.

[Source](../../src/tracing/instrument.rs.html#354-356)

#### pub fn inner_pin_mut(self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)>

Get a pinned mutable reference to the wrapped type.

[Source](../../src/tracing/instrument.rs.html#361-373)

#### pub fn into_inner(self) -> T

Consumes the `Instrumented`, returning the wrapped type.

Note that this drops the span.

## Trait Implementations§

[Source](../../src/tracing/instrument.rs.html#264)§

### impl<T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone")> [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<T>

[Source](../../src/tracing/instrument.rs.html#264)§

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<T> ⓘ

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0§

#### fn [clone_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](../../src/tracing/instrument.rs.html#264)§

### impl<T: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug")> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<T>

[Source](../../src/tracing/instrument.rs.html#264)§

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](../../src/tracing/instrument.rs.html#254-288)§

### impl<T> [Drop](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html "trait core::ops::drop::Drop") for [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<T>

[Source](../../src/tracing/instrument.rs.html#254-288)§

#### fn [drop](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

[Source](../../src/tracing/instrument.rs.html#315-323)§

### impl<T: [Future](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html "trait core::future::future::Future")> [Future](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html "trait core::future::future::Future") for [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<T>

[Source](../../src/tracing/instrument.rs.html#316)§

#### type [Output](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output) = <T as [Future](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html "trait core::future::future::Future")>::[Output](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output "type core::future::future::Future::Output")

The type of value produced on completion.

[Source](../../src/tracing/instrument.rs.html#318-322)§

#### fn [poll](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#tymethod.poll)(self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<Self::[Output](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output "type core::future::future::Future::Output")>

Attempts to resolve the future to a final value, registering the current task for wakeup if the value is not yet available. [Read more](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#tymethod.poll)

[Source](../../src/tracing/instrument.rs.html#254-288)§

### impl<'__pin, T> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<T>

where PinnedFieldsOf<__Origin<'__pin, T>>: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

## Auto Trait Implementations§

§

### impl<T> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<T>

where T: [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze"),

§

### impl<T> ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<T>

§

### impl<T> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

§

### impl<T> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<T>

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

§

### impl<T> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<T>

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

#### fn [instrument](../trait.Instrument.md#method.instrument)(self, span: [Span](../struct.Span.md "struct tracing::Span")) -> [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ

Instruments this type with the provided [`Span`](../struct.Span.md "struct tracing::Span"), returning an `Instrumented` wrapper. [Read more](../trait.Instrument.md#method.instrument)

[Source](../../src/tracing/instrument.rs.html#128-130)§

#### fn [in_current_span](../trait.Instrument.md#method.in_current_span)(self) -> [Instrumented](struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ

Instruments this type with the [current](../struct.Span.md#method.current "associated function tracing::Span::current") [`Span`](../struct.Span.md "struct tracing::Span"), returning an `Instrumented` wrapper. [Read more](../trait.Instrument.md#method.in_current_span)

§

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

§

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

§

### impl<F> [IntoFuture](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html "trait core::future::into_future::IntoFuture") for F

where F: [Future](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html "trait core::future::future::Future"),

§

#### type [Output](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.Output) = <F as [Future](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html "trait core::future::future::Future")>::[Output](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output "type core::future::future::Future::Output")

The output that the future will produce on completion.

§

#### type [IntoFuture](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture) = F

Which kind of future are we turning this into?

§

#### fn [into_future](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#tymethod.into_future)(self) -> <F as [IntoFuture](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html "trait core::future::into_future::IntoFuture")>::[IntoFuture](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture "type core::future::into_future::IntoFuture::IntoFuture")

Creates a future from a value. [Read more](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#tymethod.into_future)

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

### impl<T> [WithSubscriber](trait.WithSubscriber.md "trait tracing::instrument::WithSubscriber") for T

[Source](../../src/tracing/instrument.rs.html#176-184)§

#### fn [with_subscriber](trait.WithSubscriber.md#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")>,

Available on **crate feature`std`** only.

Attaches the provided [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper. [Read more](trait.WithSubscriber.md#method.with_subscriber)

[Source](../../src/tracing/instrument.rs.html#228-233)§

#### fn [with_current_subscriber](trait.WithSubscriber.md#method.with_current_subscriber)(self) -> [WithDispatch](struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ

Available on **crate feature`std`** only.

Attaches the current [default](../dispatcher/index.md#setting-the-default-subscriber "mod tracing::dispatcher") [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper. [Read more](trait.WithSubscriber.md#method.with_current_subscriber)
