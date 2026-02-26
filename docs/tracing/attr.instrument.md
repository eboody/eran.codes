<!-- Generated from rustdoc HTML: attr.instrument.html -->
<!-- Source: https://docs.rs/tracing/latest/tracing/attr.instrument.html -->

[ Docs.rs ](/)

  * tracing-0.1.44

    * tracing 0.1.44 
    * [ Permalink ](/tracing/0.1.44/tracing/attr.instrument.html "Get a link to this specific version")
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
    * [aarch64-apple-darwin](/crate/tracing/latest/target-redirect/aarch64-apple-darwin/tracing/attr.instrument.html)
    * [aarch64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/aarch64-unknown-linux-gnu/tracing/attr.instrument.html)
    * [i686-pc-windows-msvc](/crate/tracing/latest/target-redirect/i686-pc-windows-msvc/tracing/attr.instrument.html)
    * [x86_64-pc-windows-msvc](/crate/tracing/latest/target-redirect/x86_64-pc-windows-msvc/tracing/attr.instrument.html)
    * [x86_64-unknown-linux-gnu](/crate/tracing/latest/target-redirect/tracing/attr.instrument.html)
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

## instrument

[](index.md)

## [tracing](index.md)0.1.44

## instrument

### Sections

  * Overriding Span Attributes
  * Skipping Fields
    * Examples
  * Adding Fields
    * Examples
  * Examples

## [In crate tracing](index.md)

[tracing](index.md)

# Attribute Macro instrument Copy item path

[Source](https://docs.rs/tracing-attributes/0.1.31/x86_64-unknown-linux-gnu/src/tracing_attributes/lib.rs.html#576-579)
    
    
    #[instrument]

Available on **crate feature`attributes`** only.

Expand description

Instruments a function to create and enter a `tracing` [span](span/index.md) every time the function is called.

Unless overridden, a span with the [`INFO`](struct.Level.md#associatedconstant.INFO) [level](struct.Level.md) will be generated. The generated span’s name will be the name of the function. By default, all arguments to the function are included as fields on the span. Arguments that are `tracing` [primitive types](field/trait.Value.md#foreign-impls) implementing the [`Value` trait](field/trait.Value.md) will be recorded as fields of that type. Types which do not implement `Value` will be recorded using [`fmt::Debug`](std::fmt::Debug).

## §Overriding Span Attributes

To change the [name](struct.Metadata.md#method.name) of the generated span, add a `name` argument to the `#[instrument]` macro, followed by an equals sign and a string literal. For example:
    
    
    // The generated span's name will be "my_span" rather than "my_function".
    #[instrument(name = "my_span")]
    pub fn my_function() {
        // ... do something incredibly interesting and important ...
    }

To override the [target](struct.Metadata.md#method.target) of the generated span, add a `target` argument to the `#[instrument]` macro, followed by an equals sign and a string literal for the new target. The [module path](struct.Metadata.md#method.module_path) is still recorded separately. For example:
    
    
    pub mod my_module {
        // The generated span's target will be "my_crate::some_special_target",
        // rather than "my_crate::my_module".
        #[instrument(target = "my_crate::some_special_target")]
        pub fn my_function() {
            // ... all kinds of neat code in here ...
        }
    }

Finally, to override the [level](struct.Level.md) of the generated span, add a `level` argument, followed by an equals sign and a string literal with the name of the desired level. Level names are not case sensitive. For example:
    
    
    // The span's level will be TRACE rather than INFO.
    #[instrument(level = "trace")]
    pub fn my_function() {
        // ... I have written a truly marvelous implementation of this function,
        // which this example is too narrow to contain ...
    }

## §Skipping Fields

To skip recording one or more arguments to a function or method, pass the argument’s name inside the `skip()` argument on the `#[instrument]` macro. This can be used when an argument to an instrumented function does not implement [`fmt::Debug`](std::fmt::Debug), or to exclude an argument with a verbose or costly `Debug` implementation. Note that:

  * multiple argument names can be passed to `skip`.
  * arguments passed to `skip` do _not_ need to implement `fmt::Debug`.

You can also use `skip_all` to skip all arguments.

### §Examples
    
    
    // This type doesn't implement `fmt::Debug`!
    struct NonDebug;
    
    // `arg` will be recorded, while `non_debug` will not.
    #[instrument(skip(non_debug))]
    fn my_function(arg: usize, non_debug: NonDebug) {
        // ...
    }
    
    // These arguments are huge
    #[instrument(skip_all)]
    fn my_big_data_function(large: Vec<u8>, also_large: HashMap<String, String>) {
        // ...
    }

Skipping the `self` parameter:
    
    
    #[derive(Debug)]
    struct MyType {
       data: Vec<u8>, // Suppose this buffer is often quite long...
    }
    
    impl MyType {
        // Suppose we don't want to print an entire kilobyte of `data`
        // every time this is called...
        #[instrument(skip(self))]
        pub fn my_method(&mut self, an_interesting_argument: usize) {
             // ... do something (hopefully, using all that `data`!)
        }
    }

## §Adding Fields

Additional fields (key-value pairs with arbitrary data) can be passed to the generated span through the `fields` argument on the `#[instrument]` macro. Arbitrary expressions are accepted as value for each field. The name of the field must be a single valid Rust identifier, or a constant expression that evaluates to one, enclosed in curly braces. Note that nested (dotted) field names are also supported. Any Rust expression can be used as a field value in this manner. These expressions will be evaluated at the beginning of the function’s body, so arguments to the function may be used in these expressions. Field names may also be specified _without_ values. Doing so will result in an [empty field](field/struct.Empty.md) whose value may be recorded later within the function body.

Note that defining a field with the same name as a (non-skipped) argument will implicitly skip the argument, unless the field is provided via a constant expression (e.g. {EXPR} or {const_fn()}) as deduplicating would incur a runtime cost. In this case, the field must be explicitly skipped.

### §Examples

Adding a new field based on the value of an argument:
    
    
    // This will record a field named "i" with the value of `i` *and* a field
    // named "next" with the value of `i` + 1.
    #[instrument(fields(next = i + 1))]
    pub fn my_function(i: usize) {
        // ...
    }

Recording specific properties of a struct as their own fields:
    
    
    // This will record the request's URI and HTTP method as their own separate
    // fields.
    #[instrument(fields(http.uri = req.uri(), http.method = req.method()))]
    pub fn handle_request<B>(req: http::Request<B>) -> http::Response<B> {
        // ... handle the request ...
    }

This can be used in conjunction with `skip` or `skip_all` to record only some fields of a struct:
    
    
    // Remember the struct with the very large `data` field from the earlier
    // example? Now it also has a `name`, which we might want to include in
    // our span.
    #[derive(Debug)]
    struct MyType {
       name: &'static str,
       data: Vec<u8>,
    }
    
    impl MyType {
        // This will skip the `data` field, but will include `self.name`,
        // formatted using `fmt::Display`.
        #[instrument(skip(self), fields(self.name = %self.name))]
        pub fn my_method(&mut self, an_interesting_argument: usize) {
             // ... do something (hopefully, using all that `data`!)
        }
    }

Adding an empty field to be recorded later:
    
    
    // This function does a very interesting and important mathematical calculation.
    // Suppose we want to record both the inputs to the calculation *and* its result...
    #[instrument(fields(result))]
    pub fn do_calculation(input_1: usize, input_2: usize) -> usize {
        // Rerform the calculation.
        let result = input_1 + input_2;
    
        // Record the result as part of the current span.
        tracing::Span::current().record("result", &result);
    
        // Now, the result will also be included on this event!
        tracing::info!("calculation complete!");
    
        // ... etc ...
    }

## §Examples

Instrumenting a function:
    
    
    #[instrument]
    pub fn my_function(my_arg: usize) {
        // This event will be recorded inside a span named `my_function` with the
        // field `my_arg`.
        tracing::info!("inside my_function!");
        // ...
    }

Setting the level for the generated span:
    
    
    #[instrument(level = Level::DEBUG)]
    pub fn my_function() {
        // ...
    }

Levels can be specified either with [`Level`](struct.Level.md) constants, literal strings (e.g., `"debug"`, `"info"`) or numerically (1—5, corresponding to [`Level::TRACE`](struct.Level.md#associatedconstant.TRACE)—[`Level::ERROR`](struct.Level.md#associatedconstant.ERROR)).

Overriding the generated span’s name:
    
    
    #[instrument(name = "my_name")]
    pub fn my_function() {
        // ...
    }

Overriding the generated span’s target:
    
    
    #[instrument(target = "my_target")]
    pub fn my_function() {
        // ...
    }

Overriding the generated span’s parent:
    
    
    #[instrument(parent = None)]
    pub fn my_function() {
        // ...
    }
    
    
    // A struct which owns a span handle.
    struct MyStruct
    {
        span: tracing::Span
    }
    
    impl MyStruct
    {
        // Use the struct's `span` field as the parent span
        #[instrument(parent = &self.span, skip(self))]
        fn my_method(&self) {}
    }

Specifying [`follows_from`](struct.Span.md#method.follows_from) relationships:
    
    
    #[instrument(follows_from = causes)]
    pub fn my_function(causes: &[tracing::Id]) {
        // ...
    }

Any expression of type `impl IntoIterator<Item = impl Into<Option<Id>>>` may be provided to `follows_from`; e.g.:
    
    
    #[instrument(follows_from = [cause])]
    pub fn my_function(cause: &tracing::span::EnteredSpan) {
        // ...
    }

To skip recording an argument, pass the argument’s name to the `skip`:
    
    
    struct NonDebug;
    
    #[instrument(skip(non_debug))]
    fn my_function(arg: usize, non_debug: NonDebug) {
        // ...
    }

To add additional context to the span, pass key-value pairs to `fields`:
    
    
    #[derive(Debug)]
    struct Argument;
    impl Argument {
        fn bar(&self) -> &'static str {
            "bar"
        }
    }
    const FOOBAR: &'static str = "foo.bar";
    #[instrument(fields(foo="bar", id=1, show=true, {FOOBAR}=%arg.bar()))]
    fn my_function(arg: Argument) {
        // ...
    }

Adding the `ret` argument to `#[instrument]` will emit an event with the function’s return value when the function returns:
    
    
    #[instrument(ret)]
    fn my_function() -> i32 {
        42
    }

The return value event will have the same level as the span generated by `#[instrument]`. By default, this will be [`INFO`](struct.Level.md#associatedconstant.INFO), but if the level is overridden, the event will be at the same level.

It’s also possible to override the level for the `ret` event independently:
    
    
    #[instrument(ret(level = Level::WARN))]
    fn my_function() -> i32 {
        42
    }

**Note** : if the function returns a `Result<T, E>`, `ret` will record returned values if and only if the function returns [`Result::Ok`].

By default, returned values will be recorded using their [`std::fmt::Debug`] implementations. If a returned value implements [`std::fmt::Display`], it can be recorded using its `Display` implementation instead, by writing `ret(Display)`:
    
    
    #[instrument(ret(Display))]
    fn my_function() -> i32 {
        42
    }

If the function returns a `Result<T, E>` and `E` implements `std::fmt::Display`, adding `err` or `err(Display)` will emit error events when the function returns `Err`:
    
    
    #[instrument(err)]
    fn my_function(arg: usize) -> Result<(), std::io::Error> {
        Ok(())
    }

The level of the error value event defaults to `ERROR`.

Similarly, overriding the level of the `err` event :
    
    
    #[instrument(err(level = Level::INFO))]
    fn my_function(arg: usize) -> Result<(), std::io::Error> {
        Ok(())
    }

By default, error values will be recorded using their `std::fmt::Display` implementations. If an error implements `std::fmt::Debug`, it can be recorded using its `Debug` implementation instead by writing `err(Debug)`:
    
    
    #[instrument(err(Debug))]
    fn my_function(arg: usize) -> Result<(), std::io::Error> {
        Ok(())
    }

If a `target` is specified, both the `ret` and `err` arguments will emit outputs to the declared target (or the default channel if `target` is not specified).

The `ret` and `err` arguments can be combined in order to record an event if a function returns [`Result::Ok`] or [`Result::Err`]:
    
    
    #[instrument(err, ret)]
    fn my_function(arg: usize) -> Result<(), std::io::Error> {
        Ok(())
    }

`async fn`s may also be instrumented:
    
    
    #[instrument]
    pub async fn my_function() -> Result<(), ()> {
        // ...
    }

It also works with [async-trait](https://crates.io/crates/async-trait) (a crate that allows defining async functions in traits, something not currently possible in Rust), and hopefully most libraries that exhibit similar behaviors:
    
    
    use async_trait::async_trait;
    
    #[async_trait]
    pub trait Foo {
        async fn foo(&self, arg: usize);
    }
    
    #[derive(Debug)]
    struct FooImpl(usize);
    
    #[async_trait]
    impl Foo for FooImpl {
        #[instrument(fields(value = self.0, tmp = std::any::type_name::<Self>()))]
        async fn foo(&self, arg: usize) {}
    }

`const fn` cannot be instrumented, and will result in a compilation failure:

ⓘ
    
    
    #[instrument]
    const fn my_const_function() {}
