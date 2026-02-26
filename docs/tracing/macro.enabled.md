<!-- Generated from rustdoc HTML: macro.enabled.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/macro.enabled.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/macro.enabled.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/macro.enabled.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/macro.enabled.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/macro.enabled.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/macro.enabled.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/macro.enabled.html)
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

## enabled

[](index.md)

## [tracing](index.md)0.1.44

## enabled

### Sections

  * Usage
  * Examples
  * Alternatives

## [In crate tracing](index.md)

[tracing](index.md)

# Macro enabled Copy item path

[Source](../src/tracing/macros.rs.html#1214-1294)
    
    
    macro_rules! enabled {
        (kind: $kind:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
        (kind: $kind:expr, target: $target:expr, $lvl:expr ) => { ... };
        (target: $target:expr, $lvl:expr ) => { ... };
        (kind: $kind:expr, target: $target:expr, $lvl:expr, $($field:tt)*) => { ... };
        (target: $target:expr, $lvl:expr, $($field:tt)*) => { ... };
        (kind: $kind:expr, $lvl:expr, $($field:tt)*) => { ... };
        (kind: $kind:expr, $lvl:expr) => { ... };
        ($lvl:expr) => { ... };
        ($lvl:expr, $($field:tt)*) => { ... };
    }

Expand description

Checks whether a span or event is [enabled](trait.Subscriber.md#tymethod.enabled "method tracing::Subscriber::enabled") based on the provided [metadata](struct.Metadata.md "struct tracing::Metadata").

This macro is a specialized tool: it is intended to be used prior to an expensive computation required _just_ for that event, but _cannot_ be done as part of an argument to that event, such as when multiple events are emitted (e.g., iterating over a collection and emitting an event for each item).

## §Usage

[Subscribers](trait.Subscriber.md "trait tracing::Subscriber") can make filtering decisions based all the data included in a span or event’s [`Metadata`](struct.Metadata.md "struct tracing::Metadata"). This means that it is possible for `enabled!` to return a _false positive_ (indicating that something would be enabled when it actually would not be) or a _false negative_ (indicating that something would be disabled when it would actually be enabled).

This occurs when a subscriber is using a _more specific_ filter than the metadata provided to the `enabled!` macro. Some situations that can result in false positives or false negatives include:

  * If a subscriber is using a filter which may enable a span or event based on field names, but `enabled!` is invoked without listing field names, `enabled!` may return a false negative if a specific field name would cause the subscriber to enable something that would otherwise be disabled.
  * If a subscriber is using a filter which enables or disables specific events by file path and line number, a particular event may be enabled/disabled even if an `enabled!` invocation with the same level, target, and fields indicated otherwise.
  * The subscriber can choose to enable _only_ spans or _only_ events, which `enabled` will not reflect.

`enabled!()` requires a [level](struct.Level.md "struct tracing::Level") argument, an optional `target:` argument, and an optional set of field names. If the fields are not provided, they are considered to be unknown. `enabled!` attempts to match the syntax of `event!()` as closely as possible, which can be seen in the examples below.

## §Examples

If the current subscriber is interested in recording `DEBUG`-level spans and events in the current file and module path, this will evaluate to true:
    
    
    use tracing::{enabled, Level};
    
    if enabled!(Level::DEBUG) {
        // some expensive work...
    }

If the current subscriber is interested in recording spans and events in the current file and module path, with the target “my_crate”, and at the level `DEBUG`, this will evaluate to true:
    
    
    if enabled!(target: "my_crate", Level::DEBUG) {
        // some expensive work...
    }

If the current subscriber is interested in recording spans and events in the current file and module path, with the target “my_crate”, at the level `DEBUG`, and with a field named “hello”, this will evaluate to true:
    
    
    if enabled!(target: "my_crate", Level::DEBUG, hello) {
        // some expensive work...
    }

## §Alternatives

`enabled!` queries subscribers with [`Metadata`](struct.Metadata.md "struct tracing::Metadata") where [`is_event`](struct.Metadata.md#method.is_event "method tracing::Metadata::is_event") and [`is_span`](struct.Metadata.md#method.is_span "method tracing::Metadata::is_span") both return `false`. Alternatively, use [`event_enabled!`](macro.event_enabled.md "macro tracing::event_enabled") or [`span_enabled!`](macro.span_enabled.md "macro tracing::span_enabled") to ensure one of these returns true.
