<!-- Generated from rustdoc HTML: macro.event.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/macro.event.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/macro.event.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/macro.event.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/macro.event.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/macro.event.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/macro.event.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/macro.event.html)
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

## event

[](index.md)

## [tracing](index.md)0.1.44

## event

### Sections

  * Examples

## [In crate tracing](index.md)

[tracing](index.md)

# Macro event Copy item path

[Source](../src/tracing/macros.rs.html#615-1053)
    
    
    macro_rules! event {
        (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
        (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
        (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
        (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => { ... };
        (name: $name:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
        (name: $name:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
        (name: $name:expr, target: $target:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
        (name: $name:expr, target: $target:expr, $lvl:expr, $($arg:tt)+) => { ... };
        (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
        (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
        (target: $target:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
        (target: $target:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => { ... };
        (name: $name:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
        (name: $name:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
        (name: $name:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
        (name: $name:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => { ... };
        (name: $name:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
        (name: $name:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
        (name: $name:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
        (name: $name:expr, $lvl:expr, $($arg:tt)+ ) => { ... };
        (target: $target:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
        (target: $target:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
        (target: $target:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
        (target: $target:expr, $lvl:expr, $($arg:tt)+ ) => { ... };
        (parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
        (parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($field:tt)*) => { ... };
        (parent: $parent:expr, $lvl:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
        (parent: $parent:expr, $lvl:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
        (parent: $parent:expr, $lvl:expr, $($k:ident).+, $($field:tt)*) => { ... };
        (parent: $parent:expr, $lvl:expr, %$($k:ident).+, $($field:tt)*) => { ... };
        (parent: $parent:expr, $lvl:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
        (parent: $parent:expr, $lvl:expr, $($arg:tt)+ ) => { ... };
        ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
        ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
        ($lvl:expr, $($k:ident).+ = $($field:tt)*) => { ... };
        ($lvl:expr, $($k:ident).+, $($field:tt)*) => { ... };
        ($lvl:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
        ($lvl:expr, %$($k:ident).+, $($field:tt)*) => { ... };
        ($lvl:expr, ?$($k:ident).+) => { ... };
        ($lvl:expr, %$($k:ident).+) => { ... };
        ($lvl:expr, $($k:ident).+) => { ... };
        ( $lvl:expr, $($arg:tt)+ ) => { ... };
    }

Expand description

Constructs a new `Event`.

The event macro is invoked with a `Level` and up to 32 key-value fields. Optionally, a format string and arguments may follow the fields; this will be used to construct an implicit field named “message”.

See [the top-level documentation](index.md#using-the-macros "mod tracing") for details on the syntax accepted by this macro.

## §Examples
    
    
    use tracing::{event, Level};
    
    let data = (42, "forty-two");
    let private_data = "private";
    let error = "a bad error";
    
    event!(Level::ERROR, %error, "Received error");
    event!(
        target: "app_events",
        Level::WARN,
        private_data,
        ?data,
        "App warning: {}",
        error
    );
    event!(name: "answer", Level::INFO, the_answer = data.0);
    event!(Level::INFO, the_answer = data.0);
