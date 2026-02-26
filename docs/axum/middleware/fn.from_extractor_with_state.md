<!-- Generated from rustdoc HTML: middleware/fn.from_extractor_with_state.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## from_extractor_with_state

## [axum][1]0.8.8

## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Function from_extractor_with_state Copy item path

[Source][4]
``` 
pub fn from_extractor_with_state<E, S>(state: S) -> [FromExtractorLayer][5]<E, S>
```

Expand description

Create a middleware from an extractor with the given state.

See [`State`][6] for more details about accessing state.

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/from_extractor.rs.html#96-101
   [5]: struct.FromExtractorLayer.html (struct axum::middleware::FromExtractorLayer)
   [6]: ../extract/struct.State.html (struct axum::extract::State)

