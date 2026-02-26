<!-- Generated from rustdoc HTML: trait.Instrument.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/trait.Instrument.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/trait.Instrument.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/trait.Instrument.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/trait.Instrument.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/trait.Instrument.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/trait.Instrument.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/trait.Instrument.html)
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

## Instrument

[](index.md)

## [tracing](index.md)0.1.44

## Instrument

### Provided Methods

  * in_current_span
  * instrument

### Dyn Compatibility

### Implementors

## [In crate tracing](index.md)

[tracing](index.md)

# Trait Instrument Copy item path

[Source](../src/tracing/instrument.rs.html#20-131)
    
    
    pub trait Instrument: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") {
        // Provided methods
        fn instrument(self, span: [Span](struct.Span.md "struct tracing::Span")) -> [Instrumented](instrument/struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ { ... }
        fn in_current_span(self) -> [Instrumented](instrument/struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ { ... }
    }

Expand description

Attaches spans to a [`std::future::Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html "trait core::future::future::Future").

Extension trait allowing futures to be instrumented with a `tracing` [span](struct.Span.md "struct tracing::Span").

## Provided Methods§

[Source](../src/tracing/instrument.rs.html#86-91)

#### fn instrument(self, span: [Span](struct.Span.md "struct tracing::Span")) -> [Instrumented](instrument/struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ

Instruments this type with the provided [`Span`](struct.Span.md "struct tracing::Span"), returning an `Instrumented` wrapper.

The attached [`Span`](struct.Span.md "struct tracing::Span") will be [entered](struct.Span.md#method.enter "method tracing::Span::enter") every time the instrumented [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html "trait core::future::future::Future") is polled or [`Drop`](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html "trait core::ops::drop::Drop")ped.

##### §Examples

Instrumenting a future:
    
    
    use tracing::Instrument;
    
    let my_future = async {
        // ...
    };
    
    my_future
        .instrument(tracing::info_span!("my_future"))
        .await

The [`Span::or_current`](struct.Span.md#method.or_current "method tracing::Span::or_current") combinator can be used in combination with `instrument` to ensure that the [current span](struct.Span.md#method.current "associated function tracing::Span::current") is attached to the future if the span passed to `instrument` is [disabled](struct.Span.md#method.is_disabled "method tracing::Span::is_disabled"):
    
    
    use tracing::Instrument;
    
    let my_future = async {
        // ...
    };
    
    let outer_span = tracing::info_span!("outer").entered();
    
    // If the "my_future" span is enabled, then the spawned task will
    // be within both "my_future" *and* "outer", since "outer" is
    // "my_future"'s parent. However, if "my_future" is disabled,
    // the spawned task will *not* be in any span.
    tokio::spawn(
        my_future
            .instrument(tracing::debug_span!("my_future"))
    );
    
    // Using `Span::or_current` ensures the spawned task is instrumented
    // with the current span, if the new span passed to `instrument` is
    // not enabled. This means that if the "my_future"  span is disabled,
    // the spawned task will still be instrumented with the "outer" span:
    tokio::spawn(
       my_future
            .instrument(tracing::debug_span!("my_future").or_current())
    );

[Source](../src/tracing/instrument.rs.html#128-130)

#### fn in_current_span(self) -> [Instrumented](instrument/struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ

Instruments this type with the [current](struct.Span.md#method.current "associated function tracing::Span::current") [`Span`](struct.Span.md "struct tracing::Span"), returning an `Instrumented` wrapper.

The attached [`Span`](struct.Span.md "struct tracing::Span") will be [entered](struct.Span.md#method.enter "method tracing::Span::enter") every time the instrumented [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html "trait core::future::future::Future") is polled or [`Drop`](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html "trait core::ops::drop::Drop")ped.

This can be used to propagate the current span when spawning a new future.

##### §Examples
    
    
    use tracing::Instrument;
    
    let span = tracing::info_span!("my_span");
    let _enter = span.enter();
    
    // ...
    
    let future = async {
        tracing::debug!("this event will occur inside `my_span`");
        // ...
    };
    tokio::spawn(future.in_current_span());

## Dyn Compatibility§

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementors§

[Source](../src/tracing/instrument.rs.html#325)§

### impl<T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized")> [Instrument](trait.Instrument.md "trait tracing::Instrument") for T
