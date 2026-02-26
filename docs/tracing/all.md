<!-- Generated from rustdoc HTML: all.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/all.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/all.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/all.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/all.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/all.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/all.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/all.html)
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

## All

[](index.md)

## [tracing](index.md)0.1.44

### Crate Items

  * Macros
  * Structs
  * Constants
  * Traits
  * Functions
  * Attribute Macros

# List of all items

### Structs

  * [Dispatch](struct.Dispatch.md)
  * [Event](struct.Event.md)
  * [Level](struct.Level.md)
  * [Metadata](struct.Metadata.md)
  * [Span](struct.Span.md)
  * [dispatcher::DefaultGuard](dispatcher/struct.DefaultGuard.md)
  * [dispatcher::Dispatch](dispatcher/struct.Dispatch.md)
  * [dispatcher::SetGlobalDefaultError](dispatcher/struct.SetGlobalDefaultError.md)
  * [dispatcher::WeakDispatch](dispatcher/struct.WeakDispatch.md)
  * [event::Event](event/struct.Event.md)
  * [field::DebugValue](field/struct.DebugValue.md)
  * [field::DisplayValue](field/struct.DisplayValue.md)
  * [field::Empty](field/struct.Empty.md)
  * [field::Field](field/struct.Field.md)
  * [field::FieldSet](field/struct.FieldSet.md)
  * [field::Iter](field/struct.Iter.md)
  * [field::ValueSet](field/struct.ValueSet.md)
  * [instrument::Instrumented](instrument/struct.Instrumented.md)
  * [instrument::WithDispatch](instrument/struct.WithDispatch.md)
  * [level_filters::LevelFilter](level_filters/struct.LevelFilter.md)
  * [level_filters::ParseLevelFilterError](level_filters/struct.ParseLevelFilterError.md)
  * [span::Attributes](span/struct.Attributes.md)
  * [span::Entered](span/struct.Entered.md)
  * [span::EnteredSpan](span/struct.EnteredSpan.md)
  * [span::Id](span/struct.Id.md)
  * [span::Record](span/struct.Record.md)
  * [span::Span](span/struct.Span.md)
  * [subscriber::DefaultGuard](subscriber/struct.DefaultGuard.md)
  * [subscriber::Interest](subscriber/struct.Interest.md)
  * [subscriber::NoSubscriber](subscriber/struct.NoSubscriber.md)
  * [subscriber::SetGlobalDefaultError](subscriber/struct.SetGlobalDefaultError.md)

### Traits

  * [Instrument](trait.Instrument.md)
  * [Subscriber](trait.Subscriber.md)
  * [Value](trait.Value.md)
  * [field::AsField](field/trait.AsField.md)
  * [field::Value](field/trait.Value.md)
  * [field::Visit](field/trait.Visit.md)
  * [instrument::Instrument](instrument/trait.Instrument.md)
  * [instrument::WithSubscriber](instrument/trait.WithSubscriber.md)
  * [span::AsId](span/trait.AsId.md)
  * [subscriber::Subscriber](subscriber/trait.Subscriber.md)

### Macros

  * [debug](macro.debug.md)
  * [debug_span](macro.debug_span.md)
  * [enabled](macro.enabled.md)
  * [error](macro.error.md)
  * [error_span](macro.error_span.md)
  * [event](macro.event.md)
  * [event_enabled](macro.event_enabled.md)
  * [info](macro.info.md)
  * [info_span](macro.info_span.md)
  * [record_all](macro.record_all.md)
  * [span](macro.span.md)
  * [span_enabled](macro.span_enabled.md)
  * [trace](macro.trace.md)
  * [trace_span](macro.trace_span.md)
  * [warn](macro.warn.md)
  * [warn_span](macro.warn_span.md)

### Attribute Macros

  * [instrument](attr.instrument.md)

### Functions

  * [dispatcher::get_default](dispatcher/fn.get_default.md)
  * [dispatcher::set_default](dispatcher/fn.set_default.md)
  * [dispatcher::set_global_default](dispatcher/fn.set_global_default.md)
  * [dispatcher::with_default](dispatcher/fn.with_default.md)
  * [field::debug](field/fn.debug.md)
  * [field::display](field/fn.display.md)
  * [field::valuable](field/fn.valuable.md)
  * [subscriber::set_default](subscriber/fn.set_default.md)
  * [subscriber::set_global_default](subscriber/fn.set_global_default.md)
  * [subscriber::with_default](subscriber/fn.with_default.md)

### Constants

  * [level_filters::STATIC_MAX_LEVEL](level_filters/constant.STATIC_MAX_LEVEL.md)
