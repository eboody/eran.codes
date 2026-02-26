<!-- Generated from rustdoc HTML: dispatcher/index.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/dispatcher/index.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/dispatcher/ "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/dispatcher/)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/dispatcher/)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/dispatcher/)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/dispatcher/)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/dispatcher/)
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

## Module dispatcher

[](../index.md)

## [tracing](../index.md)0.1.44

## Module dispatcher

### Sections

  * Using the Trace Dispatcher
    * Setting the Default Subscriber
    * Accessing the Default Subscriber

### Module Items

  * Structs
  * Functions

## [In crate tracing](../index.md)

[tracing](../index.md)

# Module dispatcher Copy item path

[Source](../../src/tracing/dispatcher.rs.html#1-145)

Expand description

Dispatches trace events to [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber")s.

The _dispatcher_ is the component of the tracing system which is responsible for forwarding trace data from the instrumentation points that generate it to the subscriber that collects it.

## §Using the Trace Dispatcher

Every thread in a program using `tracing` has a _default subscriber_. When events occur, or spans are created, they are dispatched to the thread’s current subscriber.

### §Setting the Default Subscriber

By default, the current subscriber is an empty implementation that does nothing. To use a subscriber implementation, it must be set as the default. There are two methods for doing so: [`with_default`](fn.with_default.md "fn tracing::dispatcher::with_default") and [`set_global_default`](fn.set_global_default.md "fn tracing::dispatcher::set_global_default"). `with_default` sets the default subscriber for the duration of a scope, while `set_global_default` sets a default subscriber for the entire process.

To use either of these functions, we must first wrap our subscriber in a [`Dispatch`](../struct.Dispatch.md "struct tracing::Dispatch"), a cloneable, type-erased reference to a subscriber. For example:
    
    
    use dispatcher::Dispatch;
    
    let my_subscriber = FooSubscriber::new();
    let my_dispatch = Dispatch::new(my_subscriber);

Then, we can use [`with_default`](fn.with_default.md "fn tracing::dispatcher::with_default") to set our `Dispatch` as the default for the duration of a block:
    
    
    // no default subscriber
    
    dispatcher::with_default(&my_dispatch, || {
        // my_subscriber is the default
    });
    
    // no default subscriber again

It’s important to note that `with_default` will not propagate the current thread’s default subscriber to any threads spawned within the `with_default` block. To propagate the default subscriber to new threads, either use `with_default` from the new thread, or use `set_global_default`.

As an alternative to `with_default`, we can use [`set_global_default`](fn.set_global_default.md "fn tracing::dispatcher::set_global_default") to set a `Dispatch` as the default for all threads, for the lifetime of the program. For example:
    
    
    // no default subscriber
    
    dispatcher::set_global_default(my_dispatch)
        // `set_global_default` will return an error if the global default
        // subscriber has already been set.
        .expect("global default was already set!");
    
    // `my_subscriber` is now the default
    
    
    **Note** : The thread-local scoped dispatcher (with_default)
    requires the Rust standard library. no_std users should
    use [set_global_default](fn.set_global_default.md)
    instead.
    

### §Accessing the Default Subscriber

A thread’s current default subscriber can be accessed using the [`get_default`](fn.get_default.md "fn tracing::dispatcher::get_default") function, which executes a closure with a reference to the currently default `Dispatch`. This is used primarily by `tracing` instrumentation.

## Structs§

[DefaultGuard](struct.DefaultGuard.md "struct tracing::dispatcher::DefaultGuard")`std`
    A guard that resets the current default dispatcher to the prior default dispatcher when dropped.
[Dispatch](struct.Dispatch.md "struct tracing::dispatcher::Dispatch")
    `Dispatch` trace data to a [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber").
[SetGlobalDefaultError](struct.SetGlobalDefaultError.md "struct tracing::dispatcher::SetGlobalDefaultError")
    Returned if setting the global dispatcher fails.
[WeakDispatch](struct.WeakDispatch.md "struct tracing::dispatcher::WeakDispatch")
    `WeakDispatch` is a version of [`Dispatch`](../struct.Dispatch.md "struct tracing::Dispatch") that holds a non-owning reference to a [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber").

## Functions§

[get_default](fn.get_default.md "fn tracing::dispatcher::get_default")
    Executes a closure with a reference to this thread’s current [dispatcher](../struct.Dispatch.md "struct tracing::Dispatch").
[set_default](fn.set_default.md "fn tracing::dispatcher::set_default")`std`
    Sets the dispatch as the default dispatch for the duration of the lifetime of the returned DefaultGuard
[set_global_default](fn.set_global_default.md "fn tracing::dispatcher::set_global_default")
    Sets this dispatch as the global default for the duration of the entire program. Will be used as a fallback if no thread-local dispatch has been set in a thread (using `with_default`.)
[with_default](fn.with_default.md "fn tracing::dispatcher::with_default")`std`
    Sets this dispatch as the default for the duration of a closure.
