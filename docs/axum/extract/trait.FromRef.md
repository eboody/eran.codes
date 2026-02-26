<!-- Generated from rustdoc HTML: extract/trait.FromRef.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## FromRef

## [axum][1]0.8.8

## FromRef

### Required Methods

  * from_ref



### Dyn Compatibility

### Implementors

## [In axum::extract][2]

[axum][3]::[extract][2]

# Trait FromRef Copy item path
```
pub trait FromRef<T> {
    // Required method
    fn from_ref(input: [&T][4]) -> Self;
}
```

Expand description

Used to do reference-to-value conversions thus not consuming the input value.

This is mainly used with [`State`][5] to extract “substates” from a reference to main application state.

See [`State`][5] for more details on how library authors should use this trait.

This trait can be derived using `#[derive(FromRef)]`.

## Required Methods§

#### fn from_ref(input: [&T][4]) -> Self

Converts to this type from a reference to the input type.

## Dyn Compatibility§

This trait is **not** [dyn compatible][6].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementors§

§

### impl<T> [FromRef][7]<T> for T

where T: [Clone][8],

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [5]: https://docs.rs/axum/0.8/axum/extract/struct.State.html
   [6]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
   [7]: trait.FromRef.html (trait axum::extract::FromRef)
   [8]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)

