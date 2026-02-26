<!-- Generated from rustdoc HTML: subscriber/index.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/subscriber/index.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/subscriber/ "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/subscriber/)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/subscriber/)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/subscriber/)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/subscriber/)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/subscriber/)
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

## Module subscriber

[](../index.md)

## [tracing](../index.md)0.1.44

## Module subscriber

### Module Items

  * Structs
  * Traits
  * Functions

## [In crate tracing](../index.md)

[tracing](../index.md)

# Module subscriber Copy item path

[Source](../../src/tracing/subscriber.rs.html#1-64)

Expand description

Collects and records trace data.

## Structs§

[DefaultGuard](struct.DefaultGuard.md "struct tracing::subscriber::DefaultGuard")`std`
    A guard that resets the current default dispatcher to the prior default dispatcher when dropped.
[Interest](struct.Interest.md "struct tracing::subscriber::Interest")
    Indicates a [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber")’s interest in a particular callsite.
[NoSubscriber](struct.NoSubscriber.md "struct tracing::subscriber::NoSubscriber")
    A no-op [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber").
[SetGlobalDefaultError](struct.SetGlobalDefaultError.md "struct tracing::subscriber::SetGlobalDefaultError")
    Returned if setting the global dispatcher fails.

## Traits§

[Subscriber](trait.Subscriber.md "trait tracing::subscriber::Subscriber")
    Trait representing the functions required to collect trace data.

## Functions§

[set_default](fn.set_default.md "fn tracing::subscriber::set_default")`std`
    Sets the [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") as the default for the current thread for the duration of the lifetime of the returned [`DefaultGuard`](../dispatcher/struct.DefaultGuard.md "struct tracing::dispatcher::DefaultGuard").
[set_global_default](fn.set_global_default.md "fn tracing::subscriber::set_global_default")
    Sets this subscriber as the global default for the duration of the entire program. Will be used as a fallback if no thread-local subscriber has been set in a thread (using `with_default`.)
[with_default](fn.with_default.md "fn tracing::subscriber::with_default")`std`
    Sets this [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") as the default for the current thread for the duration of a closure.
