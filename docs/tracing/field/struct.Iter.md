<!-- Generated from rustdoc HTML: field/struct.Iter.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/field/struct.Iter.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/field/struct.Iter.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/field/struct.Iter.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/field/struct.Iter.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/field/struct.Iter.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/field/struct.Iter.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/field/struct.Iter.html)
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

## Iter

[](../index.md)

## [tracing](../index.md)0.1.44

## Iter

### Trait Implementations

  * Debug
  * Iterator

### Auto Trait Implementations

  * !RefUnwindSafe
  * !UnwindSafe
  * Freeze
  * Send
  * Sync
  * Unpin

### Blanket Implementations

  * Any
  * Borrow<T>
  * BorrowMut<T>
  * From<T>
  * Instrument
  * Into<U>
  * IntoIterator
  * TryFrom<U>
  * TryInto<U>
  * WithSubscriber

## [In tracing::field](index.md)

[tracing](../index.md)::[field](index.md)

# Struct Iter Copy item path

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#182)
    
    
    pub struct Iter { /* private fields */ }

Expand description

An iterator over a set of fields.

## Trait Implementations§

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#181)§

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Iter](struct.Iter.md "struct tracing::field::Iter")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#181)§

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#1037)§

### impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") for [Iter](struct.Iter.md "struct tracing::field::Iter")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#1038)§

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item) = [Field](struct.Field.md "struct tracing::field::Field")

The type of the elements being iterated over.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#1040)§

#### fn [next](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#tymethod.next)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Field](struct.Field.md "struct tracing::field::Field")>

Advances the iterator and returns the next value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#tymethod.next)

§

#### fn [next_chunk](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.next_chunk)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)>( &mut self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)], [IntoIter](https://doc.rust-lang.org/nightly/core/array/iter/struct.IntoIter.html "struct core::array::iter::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), N>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

🔬This is a nightly-only experimental API. (`iter_next_chunk` [#98326](https://github.com/tokio-rs/tracing/issues/98326))

Advances the iterator and returns an array containing the next `N` values. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.next_chunk)

1.0.0§

#### fn [size_hint](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)>)

Returns the bounds on the remaining length of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.size_hint)

1.0.0§

#### fn [count](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.count)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Consumes the iterator, counting the number of iterations and returning it. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.count)

1.0.0§

#### fn [last](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.last)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Consumes the iterator, returning the last element. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.last)

§

#### fn [advance_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.advance_by)(&mut self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)>>

🔬This is a nightly-only experimental API. (`iter_advance_by` [#77404](https://github.com/tokio-rs/tracing/issues/77404))

Advances the iterator by `n` elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.advance_by)

1.0.0§

#### fn [nth](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.nth)(&mut self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>

Returns the `n`th element of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.nth)

1.28.0§

#### fn [step_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.step_by)(self, step: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [StepBy](https://doc.rust-lang.org/nightly/core/iter/adapters/step_by/struct.StepBy.html "struct core::iter::adapters::step_by::StepBy")<Self> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an iterator starting at the same point, but stepping by the given amount at each iteration. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.step_by)

1.0.0§

#### fn [chain](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.chain)<U>(self, other: U) -> [Chain](https://doc.rust-lang.org/nightly/core/iter/adapters/chain/struct.Chain.html "struct core::iter::adapters::chain::Chain")<Self, <U as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), U: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>,

Takes two iterators and creates a new iterator over both in sequence. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.chain)

1.0.0§

#### fn [zip](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.zip)<U>(self, other: U) -> [Zip](https://doc.rust-lang.org/nightly/core/iter/adapters/zip/struct.Zip.html "struct core::iter::adapters::zip::Zip")<Self, <U as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), U: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

‘Zips up’ two iterators into a single iterator of pairs. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.zip)

§

#### fn [intersperse](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse)(self, separator: Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Intersperse](https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.Intersperse.html "struct core::iter::adapters::intersperse::Intersperse")<Self> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

🔬This is a nightly-only experimental API. (`iter_intersperse` [#79524](https://github.com/tokio-rs/tracing/issues/79524))

Creates a new iterator which places a copy of `separator` between adjacent items of the original iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse)

§

#### fn [intersperse_with](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse_with)<G>(self, separator: G) -> [IntersperseWith](https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.IntersperseWith.html "struct core::iter::adapters::intersperse::IntersperseWith")<Self, G> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), G: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")() -> Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"),

🔬This is a nightly-only experimental API. (`iter_intersperse` [#79524](https://github.com/tokio-rs/tracing/issues/79524))

Creates a new iterator which places an item generated by `separator` between adjacent items of the original iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse_with)

1.0.0§

#### fn [map](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map)<B, F>(self, f: F) -> [Map](https://doc.rust-lang.org/nightly/core/iter/adapters/map/struct.Map.html "struct core::iter::adapters::map::Map")<Self, F> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> B,

Takes a closure and creates an iterator which calls that closure on each element. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map)

1.21.0§

#### fn [for_each](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each)<F>(self, f: F)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")),

Calls a closure on each element of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each)

1.0.0§

#### fn [filter](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter)<P>(self, predicate: P) -> [Filter](https://doc.rust-lang.org/nightly/core/iter/adapters/filter/struct.Filter.html "struct core::iter::adapters::filter::Filter")<Self, P> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Creates an iterator which uses a closure to determine if an element should be yielded. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter)

1.0.0§

#### fn [filter_map](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter_map)<B, F>(self, f: F) -> [FilterMap](https://doc.rust-lang.org/nightly/core/iter/adapters/filter_map/struct.FilterMap.html "struct core::iter::adapters::filter_map::FilterMap")<Self, F> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

Creates an iterator that both filters and maps. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter_map)

1.0.0§

#### fn [enumerate](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.enumerate)(self) -> [Enumerate](https://doc.rust-lang.org/nightly/core/iter/adapters/enumerate/struct.Enumerate.html "struct core::iter::adapters::enumerate::Enumerate")<Self> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an iterator which gives the current iteration count as well as the next value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.enumerate)

1.0.0§

#### fn [peekable](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.peekable)(self) -> [Peekable](https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html "struct core::iter::adapters::peekable::Peekable")<Self> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an iterator which can use the [`peek`](https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html#method.peek "method core::iter::adapters::peekable::Peekable::peek") and [`peek_mut`](https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html#method.peek_mut "method core::iter::adapters::peekable::Peekable::peek_mut") methods to look at the next element of the iterator without consuming it. See their documentation for more information. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.peekable)

1.0.0§

#### fn [skip_while](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip_while)<P>(self, predicate: P) -> [SkipWhile](https://doc.rust-lang.org/nightly/core/iter/adapters/skip_while/struct.SkipWhile.html "struct core::iter::adapters::skip_while::SkipWhile")<Self, P> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Creates an iterator that [`skip`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip "method core::iter::traits::iterator::Iterator::skip")s elements based on a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip_while)

1.0.0§

#### fn [take_while](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take_while)<P>(self, predicate: P) -> [TakeWhile](https://doc.rust-lang.org/nightly/core/iter/adapters/take_while/struct.TakeWhile.html "struct core::iter::adapters::take_while::TakeWhile")<Self, P> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Creates an iterator that yields elements based on a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take_while)

1.57.0§

#### fn [map_while](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_while)<B, P>(self, predicate: P) -> [MapWhile](https://doc.rust-lang.org/nightly/core/iter/adapters/map_while/struct.MapWhile.html "struct core::iter::adapters::map_while::MapWhile")<Self, P> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

Creates an iterator that both yields elements based on a predicate and maps. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_while)

1.0.0§

#### fn [skip](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Skip](https://doc.rust-lang.org/nightly/core/iter/adapters/skip/struct.Skip.html "struct core::iter::adapters::skip::Skip")<Self> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an iterator that skips the first `n` elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip)

1.0.0§

#### fn [take](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Take](https://doc.rust-lang.org/nightly/core/iter/adapters/take/struct.Take.html "struct core::iter::adapters::take::Take")<Self> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an iterator that yields the first `n` elements, or fewer if the underlying iterator ends sooner. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take)

1.0.0§

#### fn [scan](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.scan)<St, B, F>(self, initial_state: St, f: F) -> [Scan](https://doc.rust-lang.org/nightly/core/iter/adapters/scan/struct.Scan.html "struct core::iter::adapters::scan::Scan")<Self, St, F> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&mut St](https://doc.rust-lang.org/nightly/std/primitive.reference.html), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

An iterator adapter which, like [`fold`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold "method core::iter::traits::iterator::Iterator::fold"), holds internal state, but unlike [`fold`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold "method core::iter::traits::iterator::Iterator::fold"), produces a new iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.scan)

1.0.0§

#### fn [flat_map](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flat_map)<U, F>(self, f: F) -> [FlatMap](https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.FlatMap.html "struct core::iter::adapters::flatten::FlatMap")<Self, U, F> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), U: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> U,

Creates an iterator that works like map, but flattens nested structure. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flat_map)

1.29.0§

#### fn [flatten](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flatten)(self) -> [Flatten](https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.Flatten.html "struct core::iter::adapters::flatten::Flatten")<Self> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Creates an iterator that flattens nested structure. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flatten)

§

#### fn [map_windows](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_windows)<F, R, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)>(self, f: F) -> [MapWindows](https://doc.rust-lang.org/nightly/core/iter/adapters/map_windows/struct.MapWindows.html "struct core::iter::adapters::map_windows::MapWindows")<Self, F, N> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&[Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)]) -> R,

🔬This is a nightly-only experimental API. (`iter_map_windows` [#87155](https://github.com/tokio-rs/tracing/issues/87155))

Calls the given function `f` for each contiguous window of size `N` over `self` and returns an iterator over the outputs of `f`. Like [`slice::windows()`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.windows "method slice::windows"), the windows during mapping overlap as well. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_windows)

1.0.0§

#### fn [fuse](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fuse)(self) -> [Fuse](https://doc.rust-lang.org/nightly/core/iter/adapters/fuse/struct.Fuse.html "struct core::iter::adapters::fuse::Fuse")<Self> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an iterator which ends after the first [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"). [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fuse)

1.0.0§

#### fn [inspect](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.inspect)<F>(self, f: F) -> [Inspect](https://doc.rust-lang.org/nightly/core/iter/adapters/inspect/struct.Inspect.html "struct core::iter::adapters::inspect::Inspect")<Self, F> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")),

Does something with each element of an iterator, passing the value on. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.inspect)

1.0.0§

#### fn [by_ref](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.by_ref)(&mut self) -> &mut Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates a “by reference” adapter for this instance of `Iterator`. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.by_ref)

1.0.0§

#### fn [collect](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect)<B>(self) -> B

where B: [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Transforms an iterator into a collection. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect)

§

#### fn [try_collect](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_collect)<B>( &mut self, ) -> <<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")>::[Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual "type core::ops::try_trait::Try::Residual") as [Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html "trait core::ops::try_trait::Residual")<B>>::[TryType](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html#associatedtype.TryType "type core::ops::try_trait::Residual::TryType")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try"), <Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")>::[Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual "type core::ops::try_trait::Try::Residual"): [Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html "trait core::ops::try_trait::Residual")<B>, B: [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")>::[Output](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Output "type core::ops::try_trait::Try::Output")>,

🔬This is a nightly-only experimental API. (`iterator_try_collect` [#94047](https://github.com/tokio-rs/tracing/issues/94047))

Fallibly transforms an iterator into a collection, short circuiting if a failure is encountered. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_collect)

§

#### fn [collect_into](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect_into)<E>(self, collection: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where E: [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

🔬This is a nightly-only experimental API. (`iter_collect_into` [#94780](https://github.com/tokio-rs/tracing/issues/94780))

Collects all the items from an iterator into a collection. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect_into)

1.0.0§

#### fn [partition](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition)<B, F>(self, f: F) -> [(B, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), B: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") \+ [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>, F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Consumes an iterator, creating two collections from it. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition)

§

#### fn [partition_in_place](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition_in_place)<'a, T, P>(self, predicate: P) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where T: 'a, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") \+ [DoubleEndedIterator](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator")<Item = [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)>, P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

🔬This is a nightly-only experimental API. (`iter_partition_in_place` [#62543](https://github.com/tokio-rs/tracing/issues/62543))

Reorders the elements of this iterator _in-place_ according to the given predicate, such that all those that return `true` precede all those that return `false`. Returns the number of `true` elements found. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition_in_place)

§

#### fn [is_partitioned](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_partitioned)<P>(self, predicate: P) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

🔬This is a nightly-only experimental API. (`iter_is_partitioned` [#62544](https://github.com/tokio-rs/tracing/issues/62544))

Checks if the elements of this iterator are partitioned according to the given predicate, such that all those that return `true` precede all those that return `false`. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_partitioned)

1.27.0§

#### fn [try_fold](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_fold)<B, F, R>(&mut self, init: B, f: F) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(B, Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> R, R: [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")<Output = B>,

An iterator method that applies a function as long as it returns successfully, producing a single, final value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_fold)

1.27.0§

#### fn [try_for_each](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_for_each)<F, R>(&mut self, f: F) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> R, R: [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")<Output = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)>,

An iterator method that applies a fallible function to each item in the iterator, stopping at the first error and returning that error. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_for_each)

1.0.0§

#### fn [fold](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold)<B, F>(self, init: B, f: F) -> B

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(B, Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> B,

Folds every element into an accumulator by applying an operation, returning the final result. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold)

1.51.0§

#### fn [reduce](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.reduce)<F>(self, f: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"),

Reduces the elements to a single one, by repeatedly applying a reducing operation. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.reduce)

§

#### fn [try_reduce](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_reduce)<R>( &mut self, f: impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> R, ) -> <<R as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")>::[Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual "type core::ops::try_trait::Try::Residual") as [Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html "trait core::ops::try_trait::Residual")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<R as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")>::[Output](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Output "type core::ops::try_trait::Try::Output")>>>::[TryType](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html#associatedtype.TryType "type core::ops::try_trait::Residual::TryType")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")<Output = Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>, <R as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")>::[Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual "type core::ops::try_trait::Try::Residual"): [Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html "trait core::ops::try_trait::Residual")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>>,

🔬This is a nightly-only experimental API. (`iterator_try_reduce` [#87053](https://github.com/tokio-rs/tracing/issues/87053))

Reduces the elements to a single one by repeatedly applying a reducing operation. If the closure returns a failure, the failure is propagated back to the caller immediately. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_reduce)

1.0.0§

#### fn [all](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.all)<F>(&mut self, f: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Tests if every element of the iterator matches a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.all)

1.0.0§

#### fn [any](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.any)<F>(&mut self, f: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Tests if any element of the iterator matches a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.any)

1.0.0§

#### fn [find](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find)<P>(&mut self, predicate: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Searches for an element of an iterator that satisfies a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find)

1.30.0§

#### fn [find_map](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find_map)<B, F>(&mut self, f: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

Applies function to the elements of iterator and returns the first non-none result. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find_map)

§

#### fn [try_find](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_find)<R>( &mut self, f: impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> R, ) -> <<R as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")>::[Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual "type core::ops::try_trait::Try::Residual") as [Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html "trait core::ops::try_trait::Residual")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>>>::[TryType](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html#associatedtype.TryType "type core::ops::try_trait::Residual::TryType")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)>, <R as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")>::[Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual "type core::ops::try_trait::Try::Residual"): [Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html "trait core::ops::try_trait::Residual")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>>,

🔬This is a nightly-only experimental API. (`try_find` [#63178](https://github.com/tokio-rs/tracing/issues/63178))

Applies function to the elements of iterator and returns the first true result or the first error. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_find)

1.0.0§

#### fn [position](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position)<P>(&mut self, predicate: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Searches for an element in an iterator, returning its index. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position)

1.0.0§

#### fn [rposition](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.rposition)<P>(&mut self, predicate: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)>

where P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") \+ [ExactSizeIterator](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html "trait core::iter::traits::exact_size::ExactSizeIterator") \+ [DoubleEndedIterator](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator"),

Searches for an element in an iterator from the right, returning its index. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.rposition)

1.0.0§

#### fn [max](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Returns the maximum element of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max)

1.0.0§

#### fn [min](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Returns the minimum element of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min)

1.6.0§

#### fn [max_by_key](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by_key)<B, F>(self, f: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>

where B: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> B,

Returns the element that gives the maximum value from the specified function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by_key)

1.15.0§

#### fn [max_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by)<F>(self, compare: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Returns the element that gives the maximum value with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by)

1.6.0§

#### fn [min_by_key](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by_key)<B, F>(self, f: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>

where B: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> B,

Returns the element that gives the minimum value from the specified function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by_key)

1.15.0§

#### fn [min_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by)<F>(self, compare: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Returns the element that gives the minimum value with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by)

1.0.0§

#### fn [rev](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.rev)(self) -> [Rev](https://doc.rust-lang.org/nightly/core/iter/adapters/rev/struct.Rev.html "struct core::iter::adapters::rev::Rev")<Self> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") \+ [DoubleEndedIterator](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator"),

Reverses an iterator’s direction. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.rev)

1.0.0§

#### fn [unzip](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.unzip)<A, B, FromA, FromB>(self) -> [(FromA, FromB)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where FromA: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") \+ [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<A>, FromB: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") \+ [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<B>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") \+ [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)>,

Converts an iterator of pairs into a pair of containers. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.unzip)

1.36.0§

#### fn [copied](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.copied)<'a, T>(self) -> [Copied](https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html "struct core::iter::adapters::copied::Copied")<Self> ⓘ

where T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") \+ 'a, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") \+ [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)>,

Creates an iterator which copies all of its elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.copied)

1.0.0§

#### fn [cloned](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cloned)<'a, T>(self) -> [Cloned](https://doc.rust-lang.org/nightly/core/iter/adapters/cloned/struct.Cloned.html "struct core::iter::adapters::cloned::Cloned")<Self> ⓘ

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") \+ 'a, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") \+ [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)>,

Creates an iterator which [`clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone "method core::clone::Clone::clone")s all of its elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cloned)

1.0.0§

#### fn [cycle](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cycle)(self) -> [Cycle](https://doc.rust-lang.org/nightly/core/iter/adapters/cycle/struct.Cycle.html "struct core::iter::adapters::cycle::Cycle")<Self> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") \+ [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Repeats an iterator endlessly. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cycle)

§

#### fn [array_chunks](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.array_chunks)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)>(self) -> [ArrayChunks](https://doc.rust-lang.org/nightly/core/iter/adapters/array_chunks/struct.ArrayChunks.html "struct core::iter::adapters::array_chunks::ArrayChunks")<Self, N> ⓘ

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

🔬This is a nightly-only experimental API. (`iter_array_chunks` [#100450](https://github.com/tokio-rs/tracing/issues/100450))

Returns an iterator over `N` elements of the iterator at a time. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.array_chunks)

1.11.0§

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.sum)<S>(self) -> S

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), S: [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>,

Sums the elements of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.sum)

1.11.0§

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.product)<P>(self) -> P

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>,

Iterates over the entire iterator, multiplying all the elements [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.product)

1.5.0§

#### fn [cmp](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp)<I>(self, other: I) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")>, Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp)

§

#### fn [cmp_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp_by)<I, F>(self, other: I, cmp: F) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

🔬This is a nightly-only experimental API. (`iter_order_by` [#64295](https://github.com/tokio-rs/tracing/issues/64295))

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp_by)

1.5.0§

#### fn [partial_cmp](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp)<I>(self, other: I) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")>

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another. The comparison works like short-circuit evaluation, returning a result without comparing the remaining elements. As soon as an order can be determined, the evaluation stops and a result is returned. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp)

§

#### fn [partial_cmp_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp_by)<I, F>(self, other: I, partial_cmp: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")>,

🔬This is a nightly-only experimental API. (`iter_order_by` [#64295](https://github.com/tokio-rs/tracing/issues/64295))

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp_by)

1.5.0§

#### fn [eq](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq)<I>(self, other: I) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq)

§

#### fn [eq_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq_by)<I, F>(self, other: I, eq: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

🔬This is a nightly-only experimental API. (`iter_order_by` [#64295](https://github.com/tokio-rs/tracing/issues/64295))

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are equal to those of another with respect to the specified equality function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq_by)

1.5.0§

#### fn [ne](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ne)<I>(self, other: I) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are not equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ne)

1.5.0§

#### fn [lt](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.lt)<I>(self, other: I) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") less than those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.lt)

1.5.0§

#### fn [le](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.le)<I>(self, other: I) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") less or equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.le)

1.5.0§

#### fn [gt](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.gt)<I>(self, other: I) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") greater than those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.gt)

1.5.0§

#### fn [ge](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ge)<I>(self, other: I) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") greater than or equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ge)

1.82.0§

#### fn [is_sorted](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

Checks if the elements of this iterator are sorted. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted)

1.82.0§

#### fn [is_sorted_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by)<F>(self, compare: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Checks if the elements of this iterator are sorted using the given comparator function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by)

1.82.0§

#### fn [is_sorted_by_key](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by_key)<F, K>(self, f: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K, K: [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

Checks if the elements of this iterator are sorted using the given key extraction function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by_key)

## Auto Trait Implementations§

§

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Iter](struct.Iter.md "struct tracing::field::Iter")

§

### impl ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Iter](struct.Iter.md "struct tracing::field::Iter")

§

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Iter](struct.Iter.md "struct tracing::field::Iter")

§

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Iter](struct.Iter.md "struct tracing::field::Iter")

§

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Iter](struct.Iter.md "struct tracing::field::Iter")

§

### impl ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Iter](struct.Iter.md "struct tracing::field::Iter")

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

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

§

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](../../src/tracing/instrument.rs.html#325)§

### impl<T> [Instrument](../trait.Instrument.md "trait tracing::Instrument") for T

[Source](../../src/tracing/instrument.rs.html#86-91)§

#### fn [instrument](../trait.Instrument.md#method.instrument)(self, span: [Span](../struct.Span.md "struct tracing::Span")) -> [Instrumented](../instrument/struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ

Instruments this type with the provided [`Span`](../struct.Span.md "struct tracing::Span"), returning an `Instrumented` wrapper. [Read more](../trait.Instrument.md#method.instrument)

[Source](../../src/tracing/instrument.rs.html#128-130)§

#### fn [in_current_span](../trait.Instrument.md#method.in_current_span)(self) -> [Instrumented](../instrument/struct.Instrumented.md "struct tracing::instrument::Instrumented")<Self> ⓘ

Instruments this type with the [current](../struct.Span.md#method.current "associated function tracing::Span::current") [`Span`](../struct.Span.md "struct tracing::Span"), returning an `Instrumented` wrapper. [Read more](../trait.Instrument.md#method.in_current_span)

§

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

§

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

§

### impl<I> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for I

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator"),

§

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = <I as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")

The type of the elements being iterated over.

§

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = I

Which kind of iterator are we turning this into?

§

#### fn [into_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> I

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

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

[Source](../../src/tracing/instrument.rs.html#393)§

### impl<T> [WithSubscriber](../instrument/trait.WithSubscriber.md "trait tracing::instrument::WithSubscriber") for T

[Source](../../src/tracing/instrument.rs.html#176-184)§

#### fn [with_subscriber](../instrument/trait.WithSubscriber.md#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../struct.Dispatch.md "struct tracing::Dispatch")>,

Available on **crate feature`std`** only.

Attaches the provided [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](../instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper. [Read more](../instrument/trait.WithSubscriber.md#method.with_subscriber)

[Source](../../src/tracing/instrument.rs.html#228-233)§

#### fn [with_current_subscriber](../instrument/trait.WithSubscriber.md#method.with_current_subscriber)(self) -> [WithDispatch](../instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch")<Self> ⓘ

Available on **crate feature`std`** only.

Attaches the current [default](../dispatcher/index.md#setting-the-default-subscriber "mod tracing::dispatcher") [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") to this type, returning a [`WithDispatch`](../instrument/struct.WithDispatch.md "struct tracing::instrument::WithDispatch") wrapper. [Read more](../instrument/trait.WithSubscriber.md#method.with_current_subscriber)
