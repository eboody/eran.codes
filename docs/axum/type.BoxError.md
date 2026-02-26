<!-- Generated from rustdoc HTML: type.BoxError.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## BoxError

## [axum][1]0.8.8

## BoxError

### Aliased Type

## [In crate axum][2]

[axum][2]

# Type Alias BoxError Copy item path
```
pub type BoxError = [Box][3]<dyn [Error][4] + [Send][5] + [Sync][6]>;
```

Expand description

Alias for a type-erased error type.

## Aliased Type§
```
pub struct BoxError(/* private fields */);
```

   [1]: ../axum/index.html
   [2]: index.html
   [3]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [4]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [5]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [6]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)

