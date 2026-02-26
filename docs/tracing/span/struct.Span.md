<!-- Generated from rustdoc HTML: span/struct.Span.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/span/struct.Span.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/span/struct.Span.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/span/struct.Span.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/span/struct.Span.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/span/struct.Span.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/span/struct.Span.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/span/struct.Span.html)
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

## Span

[](../index.md)

## [tracing](../index.md)0.1.44

## Span

### Methods

  * child_of
  * current
  * enter
  * entered
  * field
  * follows_from
  * has_field
  * id
  * in_scope
  * is_disabled
  * is_none
  * metadata
  * new
  * new_disabled
  * new_root
  * none
  * or_current
  * record
  * with_subscriber

### Trait Implementations

  * Clone
  * Debug
  * Drop
  * From<&'a Span>
  * From<&'a Span>
  * From<Span>
  * Hash
  * PartialEq

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
  * CloneToUninit
  * From<T>
  * Instrument
  * Into<U>
  * ToOwned
  * TryFrom<U>
  * TryInto<U>
  * WithSubscriber

## [In tracing::span](index.md)

[tracing](../index.md)::[span](index.md)

# Struct Span Copy item path

[Source](../../src/tracing/span.rs.html#349-359)
    
    
    pub struct Span { /* private fields */ }

Expand description

A handle representing a span, with the capability to enter the span if it exists.

If the span was rejected by the current `Subscriber`’s filter, entering the span will silently do nothing. Thus, the handle can be used in the same manner regardless of whether or not the trace is currently being collected.

## Implementations§

[Source](../../src/tracing/span.rs.html#423-1373)§

### impl [Span](../struct.Span.md "struct tracing::Span")

[Source](../../src/tracing/span.rs.html#437-439)

#### pub fn new(meta: &'static [Metadata](../struct.Metadata.md "struct tracing::Metadata")<'static>, values: &[ValueSet](../field/struct.ValueSet.md "struct tracing::field::ValueSet")<'_>) -> [Span](../struct.Span.md "struct tracing::Span")

Constructs a new `Span` with the given [metadata](../struct.Metadata.md "struct tracing::Metadata") and set of [field values](../field/struct.ValueSet.md "struct tracing::field::ValueSet").

The new span will be constructed by the currently-active [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber"), with the current span as its parent (if one exists).

After the span is constructed, [field values](../field/struct.ValueSet.md "struct tracing::field::ValueSet") and/or [`follows_from`](../struct.Span.md#method.follows_from "method tracing::Span::follows_from") annotations may be added to it.

[Source](../../src/tracing/span.rs.html#461-463)

#### pub fn new_root(meta: &'static [Metadata](../struct.Metadata.md "struct tracing::Metadata")<'static>, values: &[ValueSet](../field/struct.ValueSet.md "struct tracing::field::ValueSet")<'_>) -> [Span](../struct.Span.md "struct tracing::Span")

Constructs a new `Span` as the root of its own trace tree, with the given [metadata](../struct.Metadata.md "struct tracing::Metadata") and set of [field values](../field/struct.ValueSet.md "struct tracing::field::ValueSet").

After the span is constructed, [field values](../field/struct.ValueSet.md "struct tracing::field::ValueSet") and/or [`follows_from`](../struct.Span.md#method.follows_from "method tracing::Span::follows_from") annotations may be added to it.

[Source](../../src/tracing/span.rs.html#485-494)

#### pub fn child_of( parent: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Id](struct.Id.md "struct tracing::span::Id")>>, meta: &'static [Metadata](../struct.Metadata.md "struct tracing::Metadata")<'static>, values: &[ValueSet](../field/struct.ValueSet.md "struct tracing::field::ValueSet")<'_>, ) -> [Span](../struct.Span.md "struct tracing::Span")

Constructs a new `Span` as child of the given parent span, with the given [metadata](../struct.Metadata.md "struct tracing::Metadata") and set of [field values](../field/struct.ValueSet.md "struct tracing::field::ValueSet").

After the span is constructed, [field values](../field/struct.ValueSet.md "struct tracing::field::ValueSet") and/or [`follows_from`](../struct.Span.md#method.follows_from "method tracing::Span::follows_from") annotations may be added to it.

[Source](../../src/tracing/span.rs.html#520-525)

#### pub fn new_disabled(meta: &'static [Metadata](../struct.Metadata.md "struct tracing::Metadata")<'static>) -> [Span](../struct.Span.md "struct tracing::Span")

Constructs a new disabled span with the given `Metadata`.

This should be used when a span is constructed from a known callsite, but the subscriber indicates that it is disabled.

Entering, exiting, and recording values on this span will not notify the `Subscriber` but _may_ record log messages if the `log` feature flag is enabled.

[Source](../../src/tracing/span.rs.html#534-539)

#### pub const fn none() -> [Span](../struct.Span.md "struct tracing::Span")

Constructs a new span that is _completely disabled_.

This can be used rather than `Option<Span>` to represent cases where a span is not present.

Entering, exiting, and recording values on this span will do nothing.

[Source](../../src/tracing/span.rs.html#550-562)

#### pub fn current() -> [Span](../struct.Span.md "struct tracing::Span")

Returns a handle to the span [considered by the `Subscriber`](../trait.Subscriber.md#method.current_span "method tracing::Subscriber::current_span") to be the current span.

If the subscriber indicates that it does not track the current span, or that the thread from which this function is called is not currently inside a span, the returned span will be disabled.

[Source](../../src/tracing/span.rs.html#786-789)

#### pub fn enter(&self) -> [Entered](struct.Entered.md "struct tracing::span::Entered")<'_>

Enters this span, returning a guard that will exit the span when dropped.

If this span is enabled by the current subscriber, then this function will call [`Subscriber::enter`](../trait.Subscriber.md#tymethod.enter "method tracing::Subscriber::enter") with the span’s [`Id`](struct.Id.md "struct tracing::span::Id"), and dropping the guard will call [`Subscriber::exit`](../trait.Subscriber.md#tymethod.exit "method tracing::Subscriber::exit"). If the span is disabled, this does nothing.

##### §In Asynchronous Code

**Warning** : in asynchronous code that uses [async/await syntax](https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html), `Span::enter` should be used very carefully or avoided entirely. Holding the drop guard returned by `Span::enter` across `.await` points will result in incorrect traces. For example,
    
    
    async fn my_async_function() {
        let span = info_span!("my_async_function");
    
        // WARNING: This span will remain entered until this
        // guard is dropped...
        let _enter = span.enter();
        // ...but the `await` keyword may yield, causing the
        // runtime to switch to another task, while remaining in
        // this span!
        some_other_async_function().await
    
        // ...
    }

The drop guard returned by `Span::enter` exits the span when it is dropped. When an async function or async block yields at an `.await` point, the current scope is _exited_ , but values in that scope are **not** dropped (because the async block will eventually resume execution from that await point). This means that _another_ task will begin executing while _remaining_ in the entered span. This results in an incorrect trace.

Instead of using `Span::enter` in asynchronous code, prefer the following:

  * To enter a span for a synchronous section of code within an async block or function, prefer [`Span::in_scope`](../struct.Span.md#method.in_scope "method tracing::Span::in_scope"). Since `in_scope` takes a synchronous closure and exits the span when the closure returns, the span will always be exited before the next await point. For example:
        
        async fn my_async_function() {
            let span = info_span!("my_async_function");
        
            let some_value = span.in_scope(|| {
                // run some synchronous code inside the span...
            });
        
            // This is okay! The span has already been exited before we reach
            // the await point.
            some_other_async_function(some_value).await;
        
            // ...
        }

  * For instrumenting asynchronous code, `tracing` provides the [`Future::instrument` combinator](../trait.Instrument.md "trait tracing::Instrument") for attaching a span to a future (async function or block). This will enter the span _every_ time the future is polled, and exit it whenever the future yields.

`Instrument` can be used with an async block inside an async function:

ⓘ
        
        use tracing::Instrument;
        
        async fn my_async_function() {
            let span = info_span!("my_async_function");
            async move {
               // This is correct! If we yield here, the span will be exited,
               // and re-entered when we resume.
               some_other_async_function().await;
        
               //more asynchronous code inside the span...
        
            }
              // instrument the async block with the span...
              .instrument(span)
              // ...and await it.
              .await
        }

It can also be used to instrument calls to async functions at the callsite:

ⓘ
        
        use tracing::Instrument;
        
        async fn my_async_function() {
            let some_value = some_other_async_function()
               .instrument(debug_span!("some_other_async_function"))
               .await;
        
            // ...
        }

  * The [`#[instrument]` attribute macro](../attr.instrument.md "attr tracing::instrument") can automatically generate correct code when used on an async function:

ⓘ
        
        #[tracing::instrument(level = "info")]
        async fn my_async_function() {
        
            // This is correct! If we yield here, the span will be exited,
            // and re-entered when we resume.
            some_other_async_function().await;
        
            // ...
        
        }

##### §Examples
    
    
    let span = span!(Level::INFO, "my_span");
    let guard = span.enter();
    
    // code here is within the span
    
    drop(guard);
    
    // code here is no longer within the span
    

Guards need not be explicitly dropped:
    
    
    fn my_function() -> String {
        // enter a span for the duration of this function.
        let span = trace_span!("my_function");
        let _enter = span.enter();
    
        // anything happening in functions we call is still inside the span...
        my_other_function();
    
        // returning from the function drops the guard, exiting the span.
        return "Hello world".to_owned();
    }
    
    fn my_other_function() {
        // ...
    }

Sub-scopes may be created to limit the duration for which the span is entered:
    
    
    let span = info_span!("my_great_span");
    
    {
        let _enter = span.enter();
    
        // this event occurs inside the span.
        info!("i'm in the span!");
    
        // exiting the scope drops the guard, exiting the span.
    }
    
    // this event is not inside the span.
    info!("i'm outside the span!")

[Source](../../src/tracing/span.rs.html#896-902)

#### pub fn entered(self) -> [EnteredSpan](struct.EnteredSpan.md "struct tracing::span::EnteredSpan")

Enters this span, consuming it and returning a [guard](struct.EnteredSpan.md "struct tracing::span::EnteredSpan") that will exit the span when dropped.
    
    
        **Warning** : In asynchronous code that uses async/await syntax,
        Span::entered may produce incorrect traces if the returned drop
        guard is held across an await point. See the
        Span::enter documentation for details.
    

If this span is enabled by the current subscriber, then this function will call [`Subscriber::enter`](../trait.Subscriber.md#tymethod.enter "method tracing::Subscriber::enter") with the span’s [`Id`](struct.Id.md "struct tracing::span::Id"), and dropping the guard will call [`Subscriber::exit`](../trait.Subscriber.md#tymethod.exit "method tracing::Subscriber::exit"). If the span is disabled, this does nothing.

This is similar to the [`Span::enter`](../struct.Span.md#method.enter "method tracing::Span::enter") method, except that it moves the span by value into the returned guard, rather than borrowing it. Therefore, this method can be used to create and enter a span in a single expression, without requiring a `let`-binding. For example:
    
    
    let _span = info_span!("something_interesting").entered();

rather than:
    
    
    let span = info_span!("something_interesting");
    let _e = span.enter();

Furthermore, `entered` may be used when the span must be stored in some other struct or be passed to a function while remaining entered.
    
    
        **Note** : The returned [
        EnteredSpan](../struct.EnteredSpan.html) guard does not implement Send.
        Dropping the guard will exit _this_ span, and if the guard is sent
        to another thread and dropped there, that thread may never have entered
        this span. Thus, EnteredSpans should not be sent between threads.
    

##### §Examples

The returned guard can be [explicitly exited](struct.EnteredSpan.md#method.exit "method tracing::span::EnteredSpan::exit"), returning the un-entered span:
    
    
    let span = span!(Level::INFO, "doing_something").entered();
    
    // code here is within the span
    
    // explicitly exit the span, returning it
    let span = span.exit();
    
    // code here is no longer within the span
    
    // enter the span again
    let span = span.entered();
    
    // now we are inside the span once again

Guards need not be explicitly dropped:
    
    
    fn my_function() -> String {
        // enter a span for the duration of this function.
        let span = trace_span!("my_function").entered();
    
        // anything happening in functions we call is still inside the span...
        my_other_function();
    
        // returning from the function drops the guard, exiting the span.
        return "Hello world".to_owned();
    }
    
    fn my_other_function() {
        // ...
    }

Since the [`EnteredSpan`](struct.EnteredSpan.md "struct tracing::span::EnteredSpan") guard can dereference to the [`Span`](../struct.Span.md "struct tracing::Span") itself, the span may still be accessed while entered. For example:
    
    
    use tracing::field;
    
    // create the span with an empty field, and enter it.
    let span = info_span!("my_span", some_field = field::Empty).entered();
    
    // we can still record a value for the field while the span is entered.
    span.record("some_field", &"hello world!");

[Source](../../src/tracing/span.rs.html#1027-1032)

#### pub fn or_current(self) -> Self

Returns this span, if it was [enabled](../trait.Subscriber.md#tymethod.enabled "method tracing::Subscriber::enabled") by the current [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber"), or the [current span](../struct.Span.md#method.current "associated function tracing::Span::current") (whose lexical distance may be further than expected), if this span [is disabled](../struct.Span.md#method.is_disabled "method tracing::Span::is_disabled").

This method can be useful when propagating spans to spawned threads or [async tasks](https://doc.rust-lang.org/nightly/std/task/index.html "mod std::task"). Consider the following:
    
    
    let _parent_span = tracing::info_span!("parent").entered();
    
    // ...
    
    let child_span = tracing::debug_span!("child");
    
    std::thread::spawn(move || {
        let _entered = child_span.entered();
    
        tracing::info!("spawned a thread!");
    
        // ...
    });

If the current [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") enables the [`DEBUG`](../struct.Level.md#associatedconstant.DEBUG "associated constant tracing::Level::DEBUG") level, then both the “parent” and “child” spans will be enabled. Thus, when the “spawned a thread!” event occurs, it will be inside of the “child” span. Because “parent” is the parent of “child”, the event will _also_ be inside of “parent”.

However, if the [`Subscriber`](../trait.Subscriber.md "trait tracing::Subscriber") only enables the [`INFO`](../struct.Level.md#associatedconstant.INFO "associated constant tracing::Level::INFO") level, the “child” span will be disabled. When the thread is spawned, the `child_span.entered()` call will do nothing, since “child” is not enabled. In this case, the “spawned a thread!” event occurs outside of _any_ span, since the “child” span was responsible for propagating its parent to the spawned thread.

If this is not the desired behavior, `Span::or_current` can be used to ensure that the “parent” span is propagated in both cases, either as a parent of “child” _or_ directly. For example:
    
    
    let _parent_span = tracing::info_span!("parent").entered();
    
    // ...
    
    // If DEBUG is enabled, then "child" will be enabled, and `or_current`
    // returns "child". Otherwise, if DEBUG is not enabled, "child" will be
    // disabled, and `or_current` returns "parent".
    let child_span = tracing::debug_span!("child").or_current();
    
    std::thread::spawn(move || {
        let _entered = child_span.entered();
    
        tracing::info!("spawned a thread!");
    
        // ...
    });

When spawning [asynchronous tasks](https://doc.rust-lang.org/nightly/std/task/index.html "mod std::task"), `Span::or_current` can be used similarly, in combination with [`instrument`](../trait.Instrument.md#method.instrument "method tracing::Instrument::instrument"):
    
    
    use tracing::Instrument;
    
    let _parent_span = tracing::info_span!("parent").entered();
    
    // ...
    
    let child_span = tracing::debug_span!("child");
    
    tokio::spawn(
        async {
            tracing::info!("spawned a task!");
    
            // ...
    
        }.instrument(child_span.or_current())
    );

In general, `or_current` should be preferred over nesting an [`instrument`](../trait.Instrument.md#method.instrument "method tracing::Instrument::instrument") call inside of an [`in_current_span`](../trait.Instrument.md#method.in_current_span "method tracing::Instrument::in_current_span") call, as using `or_current` will be more efficient.
    
    
    use tracing::Instrument;
    async fn my_async_fn() {
        // ...
    }
    
    let _parent_span = tracing::info_span!("parent").entered();
    
    // Do this:
    tokio::spawn(
        my_async_fn().instrument(tracing::debug_span!("child").or_current())
    );
    
    // ...rather than this:
    tokio::spawn(
        my_async_fn()
            .instrument(tracing::debug_span!("child"))
            .in_current_span()
    );

[Source](../../src/tracing/span.rs.html#1100-1103)

#### pub fn in_scope<F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")() -> T, T>(&self, f: F) -> T

Executes the given function in the context of this span.

If this span is enabled, then this function enters the span, invokes `f` and then exits the span. If the span is disabled, `f` will still be invoked, but in the context of the currently-executing span (if there is one).

Returns the result of evaluating `f`.

##### §Examples
    
    
    let my_span = span!(Level::TRACE, "my_span");
    
    my_span.in_scope(|| {
        // this event occurs within the span.
        trace!("i'm in the span!");
    });
    
    // this event occurs outside the span.
    trace!("i'm not in the span!");

Calling a function and returning the result:
    
    
    fn hello_world() -> String {
        "Hello world!".to_owned()
    }
    
    let span = info_span!("hello_world");
    // the span will be entered for the duration of the call to
    // `hello_world`.
    let a_string = span.in_scope(hello_world);

[Source](../../src/tracing/span.rs.html#1107-1109)

#### pub fn field<Q: [AsField](../field/trait.AsField.md "trait tracing::field::AsField") \+ ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized")>(&self, field: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Field](../field/struct.Field.md "struct tracing::field::Field")>

Returns a [`Field`](../field/struct.Field.md "struct tracing::field::Field") for the field with the given `name`, if one exists,

[Source](../../src/tracing/span.rs.html#1114-1116)

#### pub fn has_field<Q: [AsField](../field/trait.AsField.md "trait tracing::field::AsField") \+ ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized")>(&self, field: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if this `Span` has a field for the given [`Field`](../field/struct.Field.md "struct tracing::field::Field") or field name.

[Source](../../src/tracing/span.rs.html#1193-1209)

#### pub fn record<Q: [AsField](../field/trait.AsField.md "trait tracing::field::AsField") \+ ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), V: [Value](../trait.Value.md "trait tracing::Value")>( &self, field: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html), value: V, ) -> &Self

Records that the field described by `field` has the value `value`.

This may be used with [`field::Empty`](../field/struct.Empty.md "struct tracing::field::Empty") to declare fields whose values are not known when the span is created, and record them later:
    
    
    use tracing::{trace_span, field};
    
    // Create a span with two fields: `greeting`, with the value "hello world", and
    // `parting`, without a value.
    let span = trace_span!("my_span", greeting = "hello world", parting = field::Empty);
    
    // ...
    
    // Now, record a value for parting as well.
    // (note that the field name is passed as a string slice)
    span.record("parting", "goodbye world!");

However, it may also be used to record a _new_ value for a field whose value was already recorded:
    
    
    use tracing::info_span;
    
    // Initially, let's assume that our attempt to do something is going okay...
    let span = info_span!("doing_something", is_okay = true);
    let _e = span.enter();
    
    match do_something() {
        Ok(something) => {
            // ...
        }
        Err(_) => {
            // Things are no longer okay!
            span.record("is_okay", false);
        }
    }
    
    
        **Note** : The fields associated with a span are part
        of its [Metadata](../struct.Metadata.md).
        The [Metadata](../struct.Metadata.md)
        describing a particular span is constructed statically when the span
        is created and cannot be extended later to add new fields. Therefore,
        you cannot record a value for a field that was not specified when the
        span was created:
    
    
    
    use tracing::{trace_span, field};
    
    // Create a span with two fields: `greeting`, with the value "hello world", and
    // `parting`, without a value.
    let span = trace_span!("my_span", greeting = "hello world", parting = field::Empty);
    
    // ...
    
    // Now, you try to record a value for a new field, `new_field`, which was not
    // declared as `Empty` or populated when you created `span`.
    // You won't get any error, but the assignment will have no effect!
    span.record("new_field", "interesting_value_you_really_need");
    
    // Instead, all fields that may be recorded after span creation should be declared up front,
    // using field::Empty when a value is not known, as we did for `parting`.
    // This `record` call will indeed replace field::Empty with "you will be remembered".
    span.record("parting", "you will be remembered");
    
    
    **Note**: To record several values in just one call, see the [`record_all!`](crate::record_all!) macro.
    

[Source](../../src/tracing/span.rs.html#1244-1246)

#### pub fn is_disabled(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this span was disabled by the subscriber and does not exist.

See also [`is_none`](../struct.Span.md#method.is_none "method tracing::Span::is_none").

[Source](../../src/tracing/span.rs.html#1259-1261)

#### pub fn is_none(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this span was constructed by [`Span::none`](../struct.Span.md#method.none "associated function tracing::Span::none") and is empty.

If `is_none` returns `true` for a given span, then [`is_disabled`](../struct.Span.md#method.is_disabled "method tracing::Span::is_disabled") will also return `true`. However, when a span is disabled by the subscriber rather than constructed by `Span::none`, this method will return `false`, while `is_disabled` will return `true`.

[Source](../../src/tracing/span.rs.html#1310-1317)

#### pub fn follows_from(&self, from: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Id](struct.Id.md "struct tracing::span::Id")>>) -> &Self

Indicates that the span with the given ID has an indirect causal relationship with this span.

This relationship differs somewhat from the parent-child relationship: a span may have any number of prior spans, rather than a single one; and spans are not considered to be executing _inside_ of the spans they follow from. This means that a span may close even if subsequent spans that follow from it are still open, and time spent inside of a subsequent span should not be included in the time its precedents were executing. This is used to model causal relationships such as when a single future spawns several related background tasks, et cetera.

If this span is disabled, or the resulting follows-from relationship would be invalid, this function will do nothing.

##### §Examples

Setting a `follows_from` relationship with a `Span`:
    
    
    let span1 = span!(Level::INFO, "span_1");
    let span2 = span!(Level::DEBUG, "span_2");
    span2.follows_from(span1);

Setting a `follows_from` relationship with the current span:
    
    
    let span = span!(Level::INFO, "hello!");
    span.follows_from(Span::current());

Setting a `follows_from` relationship with a `Span` reference:
    
    
    let span = span!(Level::INFO, "hello!");
    let curr = Span::current();
    span.follows_from(&curr);

Setting a `follows_from` relationship with an `Id`:
    
    
    let span = span!(Level::INFO, "hello!");
    let id = span.id();
    span.follows_from(id);

[Source](../../src/tracing/span.rs.html#1320-1322)

#### pub fn id(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Id](struct.Id.md "struct tracing::span::Id")>

Returns this span’s `Id`, if it is enabled.

[Source](../../src/tracing/span.rs.html#1325-1327)

#### pub fn metadata(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Metadata](../struct.Metadata.md "struct tracing::Metadata")<'static>>

Returns this span’s `Metadata`, if it is enabled.

[Source](../../src/tracing/span.rs.html#1368-1372)

#### pub fn with_subscriber<T>( &self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")((&[Id](struct.Id.md "struct tracing::span::Id"), &[Dispatch](../struct.Dispatch.md "struct tracing::Dispatch"))) -> T, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Invokes a function with a reference to this span’s ID and subscriber.

if this span is enabled, the provided function is called, and the result is returned. If the span is disabled, the function is not called, and this method returns `None` instead.

## Trait Implementations§

[Source](../../src/tracing/span.rs.html#348)§

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Span](../struct.Span.md "struct tracing::Span")

[Source](../../src/tracing/span.rs.html#348)§

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Span](../struct.Span.md "struct tracing::Span")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0§

#### fn [clone_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](../../src/tracing/span.rs.html#1392-1423)§

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Span](../struct.Span.md "struct tracing::Span")

[Source](../../src/tracing/span.rs.html#1393-1422)§

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](../../src/tracing/span.rs.html#1455-1476)§

### impl [Drop](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html "trait core::ops::drop::Drop") for [Span](../struct.Span.md "struct tracing::Span")

[Source](../../src/tracing/span.rs.html#1457-1475)§

#### fn [drop](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

[Source](../../src/tracing/span.rs.html#1425-1429)§

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a [Span](../struct.Span.md "struct tracing::Span")> for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [Id](struct.Id.md "struct tracing::span::Id")>

[Source](../../src/tracing/span.rs.html#1426-1428)§

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(span: &'a [Span](../struct.Span.md "struct tracing::Span")) -> Self

Converts to this type from the input type.

[Source](../../src/tracing/span.rs.html#1431-1435)§

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a [Span](../struct.Span.md "struct tracing::Span")> for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Id](struct.Id.md "struct tracing::span::Id")>

[Source](../../src/tracing/span.rs.html#1432-1434)§

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(span: &'a [Span](../struct.Span.md "struct tracing::Span")) -> Self

Converts to this type from the input type.

[Source](../../src/tracing/span.rs.html#1437-1441)§

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Span](../struct.Span.md "struct tracing::Span")> for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Id](struct.Id.md "struct tracing::span::Id")>

[Source](../../src/tracing/span.rs.html#1438-1440)§

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(span: [Span](../struct.Span.md "struct tracing::Span")) -> Self

Converts to this type from the input type.

[Source](../../src/tracing/span.rs.html#1386-1390)§

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [Span](../struct.Span.md "struct tracing::Span")

[Source](../../src/tracing/span.rs.html#1387-1389)§

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher")>(&self, hasher: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0§

#### fn [hash_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &[Self], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](../../src/tracing/span.rs.html#1375-1384)§

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Span](../struct.Span.md "struct tracing::Span")

[Source](../../src/tracing/span.rs.html#1376-1383)§

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0§

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations§

§

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Span](../struct.Span.md "struct tracing::Span")

§

### impl ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Span](../struct.Span.md "struct tracing::Span")

§

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Span](../struct.Span.md "struct tracing::Span")

§

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Span](../struct.Span.md "struct tracing::Span")

§

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Span](../struct.Span.md "struct tracing::Span")

§

### impl ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Span](../struct.Span.md "struct tracing::Span")

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
