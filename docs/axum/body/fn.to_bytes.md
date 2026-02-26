<!-- Generated from rustdoc HTML: body/fn.to_bytes.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## to_bytes

## [axum][1]0.8.8

## to_bytes

### Sections

  * Example



## [In axum::body][2]

[axum][3]::[body][2]

# Function to_bytes Copy item path

[Source][4]
``` 
pub async fn to_bytes(body: [Body][5], limit: [usize][6]) -> [Result][7]<Bytes, [Error][8]>
```

Expand description

Converts [`Body`][5] into [`Bytes`] and limits the maximum size of the body.

## §Example
``` 
use axum::body::{to_bytes, Body};

let body = Body::from(vec![1, 2, 3]);
// Use `usize::MAX` if you don't care about the maximum size.
let bytes = to_bytes(body, usize::MAX).await?;
assert_eq!(&bytes[..], &[1, 2, 3]);
```

You can detect if the limit was hit by checking the source of the error:
``` 
use axum::body::{to_bytes, Body};
use http_body_util::LengthLimitError;

let body = Body::from(vec![1, 2, 3]);
match to_bytes(body, 1).await {
    Ok(_bytes) => panic!("should have hit the limit"),
    Err(err) => {
        let source = std::error::Error::source(&err).unwrap();
        assert!(source.is::<LengthLimitError>());
    }
}
```

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/body/mod.rs.html#48-54
   [5]: struct.Body.html (struct axum::body::Body)
   [6]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [7]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [8]: ../struct.Error.html (struct axum::Error)

