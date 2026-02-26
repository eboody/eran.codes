<!-- Generated from rustdoc HTML: struct.Level.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/struct.Level.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/struct.Level.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/struct.Level.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/struct.Level.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/struct.Level.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/struct.Level.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/struct.Level.html)
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

## Level

[](index.md)

## [tracing](index.md)0.1.44

## Level

### Sections

  * Comparing Levels
  * Filtering
    * Examples

### Associated Constants

  * DEBUG
  * ERROR
  * INFO
  * TRACE
  * WARN

### Methods

  * as_str

### Trait Implementations

  * Clone
  * Copy
  * Debug
  * Display
  * Eq
  * From<Level>
  * FromStr
  * Hash
  * Ord
  * PartialEq
  * PartialEq<Level>
  * PartialEq<LevelFilter>
  * PartialOrd
  * PartialOrd<Level>
  * PartialOrd<LevelFilter>
  * StructuralPartialEq

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
  * ToString
  * TryFrom<U>
  * TryInto<U>
  * WithSubscriber

## [In crate tracing](index.md)

[tracing](index.md)

# Struct Level Copy item path

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#221)
    
    
    pub struct Level(/* private fields */);

Expand description

Describes the level of verbosity of a span or event.

## §Comparing Levels

`Level` implements the [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") and [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") traits, allowing two `Level`s to be compared to determine which is considered more or less verbose. Levels which are more verbose are considered “greater than” levels which are less verbose, with [`Level::ERROR`](struct.Level.md#associatedconstant.ERROR "associated constant tracing::Level::ERROR") considered the lowest, and [`Level::TRACE`](struct.Level.md#associatedconstant.TRACE "associated constant tracing::Level::TRACE") considered the highest.

For example:
    
    
    use tracing_core::Level;
    
    assert!(Level::TRACE > Level::DEBUG);
    assert!(Level::ERROR < Level::WARN);
    assert!(Level::INFO <= Level::DEBUG);
    assert_eq!(Level::TRACE, Level::TRACE);

## §Filtering

`Level`s are typically used to implement filtering that determines which spans and events are enabled. Depending on the use case, more or less verbose diagnostics may be desired. For example, when running in development, [`DEBUG`](struct.Level.md#associatedconstant.DEBUG "associated constant tracing::Level::DEBUG")-level traces may be enabled by default. When running in production, only [`INFO`](struct.Level.md#associatedconstant.INFO "associated constant tracing::Level::INFO")-level and lower traces might be enabled. Libraries may include very verbose diagnostics at the [`DEBUG`](struct.Level.md#associatedconstant.DEBUG "associated constant tracing::Level::DEBUG") and/or [`TRACE`](struct.Level.md#associatedconstant.TRACE "associated constant tracing::Level::TRACE") levels. Applications using those libraries typically chose to ignore those traces. However, when debugging an issue involving said libraries, it may be useful to temporarily enable the more verbose traces.

The [`LevelFilter`](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter") type is provided to enable filtering traces by verbosity. `Level`s can be compared against [`LevelFilter`](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")s, and [`LevelFilter`](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter") has a variant for each `Level`, which compares analogously to that level. In addition, [`LevelFilter`](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter") adds a [`LevelFilter::OFF`](level_filters/struct.LevelFilter.md#associatedconstant.OFF "associated constant tracing::level_filters::LevelFilter::OFF") variant, which is considered “less verbose” than every other `Level`. This is intended to allow filters to completely disable tracing in a particular context.

For example:
    
    
    use tracing_core::{Level, LevelFilter};
    
    assert!(LevelFilter::OFF < Level::TRACE);
    assert!(LevelFilter::TRACE > Level::DEBUG);
    assert!(LevelFilter::ERROR < Level::WARN);
    assert!(LevelFilter::INFO <= Level::DEBUG);
    assert!(LevelFilter::INFO >= Level::INFO);

### §Examples

Below is a simple example of how a [`Subscriber`](trait.Subscriber.md "trait tracing::Subscriber") could implement filtering through a [`LevelFilter`](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter"). When a span or event is recorded, the [`Subscriber::enabled`](trait.Subscriber.md#tymethod.enabled "method tracing::Subscriber::enabled") method compares the span or event’s `Level` against the configured [`LevelFilter`](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter"). The optional [`Subscriber::max_level_hint`](trait.Subscriber.md#method.max_level_hint "method tracing::Subscriber::max_level_hint") method can also be implemented to allow spans and events above a maximum verbosity level to be skipped more efficiently, often improving performance in short-lived programs.
    
    
    use tracing_core::{span, Event, Level, LevelFilter, Subscriber, Metadata};
    
    #[derive(Debug)]
    pub struct MySubscriber {
        /// The most verbose level that this subscriber will enable.
        max_level: LevelFilter,
    
        // ...
    }
    
    impl MySubscriber {
        /// Returns a new `MySubscriber` which will record spans and events up to
        /// `max_level`.
        pub fn with_max_level(max_level: LevelFilter) -> Self {
            Self {
                max_level,
                // ...
            }
        }
    }
    impl Subscriber for MySubscriber {
        fn enabled(&self, meta: &Metadata<'_>) -> bool {
            // A span or event is enabled if it is at or below the configured
            // maximum level.
            meta.level() <= &self.max_level
        }
    
        // This optional method returns the most verbose level that this
        // subscriber will enable. Although implementing this method is not
        // *required*, it permits additional optimizations when it is provided,
        // allowing spans and events above the max level to be skipped
        // more efficiently.
        fn max_level_hint(&self) -> Option<LevelFilter> {
            Some(self.max_level)
        }
    
        // Implement the rest of the subscriber...
        fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
            // ...
        }
    
        fn event(&self, event: &Event<'_>) {
            // ...
        }
    
        // ...
    }

It is worth noting that the `tracing-subscriber` crate provides [additional APIs](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html) for performing more sophisticated filtering, such as enabling different levels based on which module or crate a span or event is recorded in.

## Implementations§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#509)§

### impl [Level](struct.Level.md "struct tracing::Level")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#513)

#### pub const ERROR: [Level](struct.Level.md "struct tracing::Level")

The “error” level.

Designates very serious errors.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#517)

#### pub const WARN: [Level](struct.Level.md "struct tracing::Level")

The “warn” level.

Designates hazardous situations.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#521)

#### pub const INFO: [Level](struct.Level.md "struct tracing::Level")

The “info” level.

Designates useful information.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#525)

#### pub const DEBUG: [Level](struct.Level.md "struct tracing::Level")

The “debug” level.

Designates lower priority information.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#529)

#### pub const TRACE: [Level](struct.Level.md "struct tracing::Level")

The “trace” level.

Designates very low priority, often extremely verbose, information.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#534)

#### pub fn as_str(&self) -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the string representation of the `Level`.

This returns the same string as the `fmt::Display` implementation.

## Trait Implementations§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#220)§

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Level](struct.Level.md "struct tracing::Level")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#220)§

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Level](struct.Level.md "struct tracing::Level")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0§

#### fn [clone_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#220)§

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Level](struct.Level.md "struct tracing::Level")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#220)§

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#545)§

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [Level](struct.Level.md "struct tracing::Level")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#546)§

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#612)§

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Level](struct.Level.md "struct tracing::Level")> for [LevelFilter](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#614)§

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(level: [Level](struct.Level.md "struct tracing::Level")) -> [LevelFilter](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")

Converts to this type from the input type.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#561)§

### impl [FromStr](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr") for [Level](struct.Level.md "struct tracing::Level")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#562)§

#### type [Err](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err) = [ParseLevelError](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/metadata/struct.ParseLevelError.html "struct tracing_core::metadata::ParseLevelError")

The associated error which can be returned from parsing.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#563)§

#### fn [from_str](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)(s: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Level](struct.Level.md "struct tracing::Level"), [ParseLevelError](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/metadata/struct.ParseLevelError.html "struct tracing_core::metadata::ParseLevelError")>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#220)§

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [Level](struct.Level.md "struct tracing::Level")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#220)§

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<__H>(&self, state: [&mut __H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where __H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0§

#### fn [hash_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &[Self], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#945)§

### impl [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for [Level](struct.Level.md "struct tracing::Level")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#947)§

#### fn [cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)(&self, other: &[Level](struct.Level.md "struct tracing::Level")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0§

#### fn [max](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)(self, other: Self) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0§

#### fn [min](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)(self, other: Self) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0§

#### fn [clamp](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)(self, min: Self, max: Self) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#987)§

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[Level](struct.Level.md "struct tracing::Level")> for [LevelFilter](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#989)§

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Level](struct.Level.md "struct tracing::Level")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0§

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#911)§

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[LevelFilter](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")> for [Level](struct.Level.md "struct tracing::Level")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#913)§

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[LevelFilter](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0§

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#220)§

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Level](struct.Level.md "struct tracing::Level")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#220)§

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Level](struct.Level.md "struct tracing::Level")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0§

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#1028)§

### impl [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<[Level](struct.Level.md "struct tracing::Level")> for [LevelFilter](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#1030)§

#### fn [partial_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[Level](struct.Level.md "struct tracing::Level")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#1035)§

#### fn [lt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)(&self, other: &[Level](struct.Level.md "struct tracing::Level")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#1040)§

#### fn [le](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)(&self, other: &[Level](struct.Level.md "struct tracing::Level")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#1045)§

#### fn [gt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)(&self, other: &[Level](struct.Level.md "struct tracing::Level")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#1050)§

#### fn [ge](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)(&self, other: &[Level](struct.Level.md "struct tracing::Level")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#952)§

### impl [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<[LevelFilter](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")> for [Level](struct.Level.md "struct tracing::Level")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#954)§

#### fn [partial_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[LevelFilter](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#959)§

#### fn [lt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)(&self, other: &[LevelFilter](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#964)§

#### fn [le](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)(&self, other: &[LevelFilter](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#969)§

#### fn [gt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)(&self, other: &[LevelFilter](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#974)§

#### fn [ge](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)(&self, other: &[LevelFilter](level_filters/struct.LevelFilter.md "struct tracing::level_filters::LevelFilter")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#918)§

### impl [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") for [Level](struct.Level.md "struct tracing::Level")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#920)§

#### fn [partial_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[Level](struct.Level.md "struct tracing::Level")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#925)§

#### fn [lt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)(&self, other: &[Level](struct.Level.md "struct tracing::Level")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#930)§

#### fn [le](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)(&self, other: &[Level](struct.Level.md "struct tracing::Level")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#935)§

#### fn [gt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)(&self, other: &[Level](struct.Level.md "struct tracing::Level")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#940)§

#### fn [ge](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)(&self, other: &[Level](struct.Level.md "struct tracing::Level")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#220)§

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Level](struct.Level.md "struct tracing::Level")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#220)§

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [Level](struct.Level.md "struct tracing::Level")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/metadata.rs.html#220)§

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [Level](struct.Level.md "struct tracing::Level")

## Auto Trait Implementations§

§

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Level](struct.Level.md "struct tracing::Level")

§

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Level](struct.Level.md "struct tracing::Level")

§

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Level](struct.Level.md "struct tracing::Level")

§

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Level](struct.Level.md "struct tracing::Level")

§

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Level](struct.Level.md "struct tracing::Level")

§

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Level](struct.Level.md "struct tracing::Level")

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

[Source](../src/tracing/instrument.rs.html#325)§

### impl<T> [Instrument](trait.Instrument.md "trait tracing::Instrument") for T

[Source](../src/tracing/instrument.rs.html#86-91)§

#### fn [instrument](trait.Instrument.md#method.instrument)(self, span: [Span](struct.Span.md "struct tracing::Span")) -> [Instrumented](instrument/struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ

Instruments this type with the provided [`Span`](struct.Span.md "struct tracing::Span"), returning an `Instrumented` wrapper. [Read more](trait.Instrument.md#method.instrument)

[Source](../src/tracing/instrument.rs.html#128-130)§

#### fn [in_current_span](trait.Instrument.md#method.in_current_span)(self) -> [Instrumented](instrument/struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ

Instruments this type with the [current](struct.Span.md#method.current "associated function tracing::Span::current") [`Span`](struct.Span.md "struct tracing::Span"), returning an `Instrumented` wrapper. [Read more](trait.Instrument.md#method.in_current_span)

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

### impl<T> [ToString](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html "trait alloc::string::ToString") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") \+ ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

§

#### fn [to_string](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)(&self) -> [String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String")

Converts the given value to a `String`. [Read more](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)

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

[Source](../src/tracing/instrument.rs.html#393)§

### impl<T> [WithSubscriber](instrument/trait.WithSubscriber.md "trait tracing::instrument::WithSubscriber") for T

[Source](../src/tracing/instrument.rs.html#176-184)§

#### fn [with_subscriber](instrument/trait.WithSubscriber.md#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](struct.Dispatch.md "struct tracing::Dispatch")>,

Available on **crate feature`std`** only.

Attaches the provided [`Subscriber`](trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper. [Read more](instrument/trait.WithSubscriber.md#method.with_subscriber)

[Source](../src/tracing/instrument.rs.html#228-233)§

#### fn [with_current_subscriber](instrument/trait.WithSubscriber.md#method.with_current_subscriber)(self) -> [WithDispatch](instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ

Available on **crate feature`std`** only.

Attaches the current [default](dispatcher/index.md#setting-the-default-subscriber "mod tracing::dispatcher") [`Subscriber`](trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper. [Read more](instrument/trait.WithSubscriber.md#method.with_current_subscriber)
