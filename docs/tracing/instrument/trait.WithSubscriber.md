<!-- Generated from rustdoc HTML: instrument/trait.WithSubscriber.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/instrument/trait.WithSubscriber.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/instrument/trait.WithSubscriber.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/instrument/trait.WithSubscriber.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/instrument/trait.WithSubscriber.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/instrument/trait.WithSubscriber.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/instrument/trait.WithSubscriber.html)
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

## WithSubscriber

[](../index.md)

## [tracing](../index.md)0.1.44

## WithSubscriber

### Provided Methods

  * with_current_subscriber
  * with_subscriber

### Dyn Compatibility

### Implementors

## [In tracing::instrument](index.md)

[tracing](../index.md)::[instrument](index.md)

# Trait WithSubscriber Copy item path

[Source](../../src/tracing/instrument.rs.html#136-234)
    
    
    pub trait WithSubscriber: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") {
        // Provided methods
        fn with_subscriber<S>(self, subscriber: S) -> [WithDispatch](struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ
           where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")> { ... }
        fn with_current_subscriber(self) -> [WithDispatch](struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ { ... }
    }

Available on **crate feature`std`** only.

Expand description

Extension trait allowing futures to be instrumented with a `tracing` [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber").

## Provided Methods§

[Source](../../src/tracing/instrument.rs.html#176-184)

#### fn with_subscriber<S>(self, subscriber: S) -> [WithDispatch](struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")>,

Attaches the provided [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper.

The attached [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") will be set as the [default](../dispatcher/index.md#setting-the-default-subscriber "mod tracing::dispatcher") when the returned [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html "trait core::future::future::Future") is polled.

##### §Examples
    
    
    use tracing::instrument::WithSubscriber;
    
    // Set the default `Subscriber`
    let _default = tracing::subscriber::set_default(MySubscriber::default());
    
    tracing::info!("this event will be recorded by the default `Subscriber`");
    
    // Create a different `Subscriber` and attach it to a future.
    let other_subscriber = MyOtherSubscriber::default();
    let future = async {
        tracing::info!("this event will be recorded by the other `Subscriber`");
        // ...
    };
    
    future
        // Attach the other `Subscriber` to the future before awaiting it
        .with_subscriber(other_subscriber)
        .await;
    
    // Once the future has completed, we return to the default `Subscriber`.
    tracing::info!("this event will be recorded by the default `Subscriber`");

[Source](../../src/tracing/instrument.rs.html#228-233)

#### fn with_current_subscriber(self) -> [WithDispatch](struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ

Attaches the current [default](../dispatcher/index.md#setting-the-default-subscriber "mod tracing::dispatcher") [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper.

The attached `Subscriber` will be set as the [default](../dispatcher/index.md#setting-the-default-subscriber "mod tracing::dispatcher") when the returned [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html "trait core::future::future::Future") is polled.

This can be used to propagate the current dispatcher context when spawning a new future that may run on a different thread.

##### §Examples
    
    
    use tracing::instrument::WithSubscriber;
    
    // Using `set_default` (rather than `set_global_default`) sets the
    // default `Subscriber` for *this* thread only.
    let _default = tracing::subscriber::set_default(MySubscriber::default());
    
    let future = async {
        // ...
    };
    
    // If a multi-threaded async runtime is in use, this spawned task may
    // run on a different thread, in a different default `Subscriber`'s context.
    tokio::spawn(future);
    
    // However, calling `with_current_subscriber` on the future before
    // spawning it, ensures that the current thread's default `Subscriber` is
    // propagated to the spawned task, regardless of where it executes:
    tokio::spawn(future.with_current_subscriber());

## Dyn Compatibility§

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementors§

[Source](../../src/tracing/instrument.rs.html#393)§

### impl<T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized")> [WithSubscriber](trait.WithSubscriber.md "trait tracing::instrument::WithSubscriber") for T
