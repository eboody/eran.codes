<!-- Generated from rustdoc HTML: subscriber/struct.Interest.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/subscriber/struct.Interest.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/subscriber/struct.Interest.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/subscriber/struct.Interest.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/subscriber/struct.Interest.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/subscriber/struct.Interest.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/subscriber/struct.Interest.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/subscriber/struct.Interest.html)
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

## Interest

[](../index.md)

## [tracing](../index.md)0.1.44

## Interest

### Methods

  * always
  * is_always
  * is_never
  * is_sometimes
  * never
  * sometimes

### Trait Implementations

  * Clone
  * Debug

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
  * CloneToUninit
  * From<T>
  * Instrument
  * Into<U>
  * ToOwned
  * TryFrom<U>
  * TryInto<U>
  * WithSubscriber

## [In tracing::subscriber](index.md)

[tracing](../index.md)::[subscriber](index.md)

# Struct Interest Copy item path

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#589)
    
    
    pub struct Interest(/* private fields */);

Expand description

Indicates a [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber")’s interest in a particular callsite.

`Subscriber`s return an `Interest` from their [`register_callsite`](../trait.Subscriber.md#method.register_callsite "method tracing::Subscriber::register_callsite") methods in order to determine whether that span should be enabled or disabled.

## Implementations§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#598)§

### impl [Interest](struct.Interest.md "struct tracing::subscriber::Interest")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#605)

#### pub fn never() -> [Interest](struct.Interest.md "struct tracing::subscriber::Interest")

Returns an `Interest` indicating that the subscriber is never interested in being notified about a callsite.

If all active subscribers are `never()` interested in a callsite, it will be completely disabled unless a new subscriber becomes active.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#617)

#### pub fn sometimes() -> [Interest](struct.Interest.md "struct tracing::subscriber::Interest")

Returns an `Interest` indicating the subscriber is sometimes interested in being notified about a callsite.

If all active subscribers are `sometimes` or `never` interested in a callsite, the currently active subscriber will be asked to filter that callsite every time it creates a span. This will be the case until a new subscriber expresses that it is `always` interested in the callsite.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#627)

#### pub fn always() -> [Interest](struct.Interest.md "struct tracing::subscriber::Interest")

Returns an `Interest` indicating the subscriber is always interested in being notified about a callsite.

If any subscriber expresses that it is `always()` interested in a given callsite, then the callsite will always be enabled.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#634)

#### pub fn is_never(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the subscriber is never interested in being notified about this callsite.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#641)

#### pub fn is_sometimes(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the subscriber is sometimes interested in being notified about this callsite.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#648)

#### pub fn is_always(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the subscriber is always interested in being notified about this callsite.

## Trait Implementations§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#588)§

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Interest](struct.Interest.md "struct tracing::subscriber::Interest")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#588)§

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Interest](struct.Interest.md "struct tracing::subscriber::Interest")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0§

#### fn [clone_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#588)§

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Interest](struct.Interest.md "struct tracing::subscriber::Interest")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#588)§

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations§

§

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Interest](struct.Interest.md "struct tracing::subscriber::Interest")

§

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Interest](struct.Interest.md "struct tracing::subscriber::Interest")

§

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Interest](struct.Interest.md "struct tracing::subscriber::Interest")

§

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Interest](struct.Interest.md "struct tracing::subscriber::Interest")

§

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Interest](struct.Interest.md "struct tracing::subscriber::Interest")

§

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Interest](struct.Interest.md "struct tracing::subscriber::Interest")

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

Attaches the current [default](../dispatcher/index.md#setting-the-default-subscriber "mod tracing::dispatcher") [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](../instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper. [Read more](../instrument/trait.WithSubscriber.md#method.with_current_subscriber)
