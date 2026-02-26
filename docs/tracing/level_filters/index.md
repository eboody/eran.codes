<!-- Generated from rustdoc HTML: level_filters/index.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/level_filters/index.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/level_filters/ "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/level_filters/)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/level_filters/)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/level_filters/)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/level_filters/)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/level_filters/)
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

## Module level_filters

[](../index.md)

## [tracing](../index.md)0.1.44

## Module level_filters

### Sections

  * Compile time filters
    * Notes

### Module Items

  * Structs
  * Constants

## [In crate tracing](../index.md)

[tracing](../index.md)

# Module level_filters Copy item path

[Source](../../src/tracing/level_filters.rs.html#1-113)

Expand description

Trace verbosity level filtering.

## §Compile time filters

Trace verbosity levels can be statically disabled at compile time via Cargo features, similar to the [`log` crate](https://docs.rs/log/latest/log/#compile-time-filters). Trace instrumentation at disabled levels will be skipped and will not even be present in the resulting binary unless the verbosity level is specified dynamically. This level is configured separately for release and debug builds. The features are:

  * `max_level_off`
  * `max_level_error`
  * `max_level_warn`
  * `max_level_info`
  * `max_level_debug`
  * `max_level_trace`
  * `release_max_level_off`
  * `release_max_level_error`
  * `release_max_level_warn`
  * `release_max_level_info`
  * `release_max_level_debug`
  * `release_max_level_trace`

These features control the value of the `STATIC_MAX_LEVEL` constant. The instrumentation macros macros check this value before recording an event or constructing a span. By default, no levels are disabled.

For example, a crate can disable trace level instrumentation in debug builds and trace, debug, and info level instrumentation in release builds with the following configuration:
    
    
    [dependencies]
    tracing = { version = "0.1", features = ["max_level_debug", "release_max_level_warn"] }

### §Notes

Please note that `tracing`’s static max level features do _not_ control the [`log`](https://docs.rs/log/) records that may be emitted when [`tracing`’s “log” feature flag](../index.md#emitting-log-records) is enabled. This is to allow `tracing` to be disabled entirely at compile time while still emitting `log` records — such as when a library using `tracing` is used by an application using `log` that doesn’t want to generate any `tracing`-related code, but does want to collect `log` records.

This means that if the “log” feature is in use, some code may be generated for `log` records emitted by disabled `tracing` events. If this is not desirable, `log` records may be disabled separately using [`log`’s static max level features](https://docs.rs/log/latest/log/#compile-time-filters).

## Structs§

[LevelFilter](struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")
    A filter comparable to a verbosity [`Level`](../struct.Level.md "struct tracing::Level").
[ParseLevelFilterError](struct.ParseLevelFilterError.md "struct tracing::level_filters::ParseLevelFilterError")
    Indicates that a string could not be parsed to a valid level.

## Constants§

[STATIC_MAX_LEVEL](constant.STATIC_MAX_LEVEL.md "constant tracing::level_filters::STATIC_MAX_LEVEL")
    The statically configured maximum trace level.
