<!-- Generated from rustdoc HTML: macro.trace.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/macro.trace.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/macro.trace.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/macro.trace.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/macro.trace.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/macro.trace.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/macro.trace.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/macro.trace.html)
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

## trace

[](index.md)

## [tracing](index.md)0.1.44

## trace

### Sections

  * Examples

## [In crate tracing](index.md)

[tracing](index.md)

# Macro trace Copy item path

[Source](../src/tracing/macros.rs.html#1333-1583)
    
    
    macro_rules! trace {
        (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
        (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
        (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
        (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
        (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
        (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
        (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
        (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
        (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
        (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => { ... };
        (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
        (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
        (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
        (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
        (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
        (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
        (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
        (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
        (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
        (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
        (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
        (name: $name:expr, $($k:ident).+ $($field:tt)* ) => { ... };
        (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
        (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
        (name: $name:expr, $($arg:tt)+ ) => { ... };
        (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
        (target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
        (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
        (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
        (target: $target:expr, $($arg:tt)+ ) => { ... };
        (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => { ... };
        (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => { ... };
        (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
        (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
        (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => { ... };
        (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
        (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => { ... };
        (parent: $parent:expr, $($arg:tt)+) => { ... };
        ({ $($field:tt)+ }, $($arg:tt)+ ) => { ... };
        ($($k:ident).+ = $($field:tt)*) => { ... };
        (?$($k:ident).+ = $($field:tt)*) => { ... };
        (%$($k:ident).+ = $($field:tt)*) => { ... };
        ($($k:ident).+, $($field:tt)*) => { ... };
        (?$($k:ident).+, $($field:tt)*) => { ... };
        (%$($k:ident).+, $($field:tt)*) => { ... };
        (?$($k:ident).+) => { ... };
        (%$($k:ident).+) => { ... };
        ($($k:ident).+) => { ... };
        ($($arg:tt)+) => { ... };
    }

Expand description

Constructs an event at the trace level.

This functions similarly to the [`event!`](macro.event.md "macro tracing::event") macro. See [the top-level documentation](index.md#using-the-macros "mod tracing") for details on the syntax accepted by this macro.

## §Examples
    
    
    use tracing::trace;
    let pos = Position { x: 3.234, y: -1.223 };
    let origin_dist = pos.dist(Position::ORIGIN);
    
    trace!(position = ?pos, ?origin_dist);
    trace!(
        target: "app_events",
        position = ?pos,
        "x is {} and y is {}",
        if pos.x >= 0.0 { "positive" } else { "negative" },
        if pos.y >= 0.0 { "positive" } else { "negative" }
    );
    trace!(name: "completed", position = ?pos);
