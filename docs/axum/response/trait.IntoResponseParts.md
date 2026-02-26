<!-- Generated from rustdoc HTML: response/trait.IntoResponseParts.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## IntoResponseParts

## [axum][1]0.8.8

## IntoResponseParts

### Sections

  * Example



### Required Associated Types

  * Error



### Required Methods

  * into_response_parts



### Implementations on Foreign Types

  * ()
  * (T1, T2)
  * (T1, T2, T3)
  * (T1, T2, T3, T4)
  * (T1, T2, T3, T4, T5)
  * (T1, T2, T3, T4, T5, T6)
  * (T1, T2, T3, T4, T5, T6, T7)
  * (T1, T2, T3, T4, T5, T6, T7, T8)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
  * (T1,)
  * Extensions
  * HeaderMap
  * Option<T>
  * [(K, V); N]



### Implementors

## [In axum::response][2]

[axum][3]::[response][2]

# Trait IntoResponseParts Copy item path
```
pub trait IntoResponseParts {
    type Error: [IntoResponse][4];

    // Required method
    fn into_response_parts(
        self,
        res: [ResponseParts][5],
    ) -> [Result][6]<[ResponseParts][5], Self::[Error][7]>;
}
```

Expand description

Trait for adding headers and extensions to a response.

## §Example
``` 
use axum::{
    response::{ResponseParts, IntoResponse, IntoResponseParts, Response},
    http::{StatusCode, header::{HeaderName, HeaderValue}},
};

// Hypothetical helper type for setting a single header
struct SetHeader<'a>(&'a str, &'a str);

impl<'a> IntoResponseParts for SetHeader<'a> {
    type Error = (StatusCode, String);

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        match (self.0.parse::<HeaderName>(), self.1.parse::<HeaderValue>()) {
            (Ok(name), Ok(value)) => {
                res.headers_mut().insert(name, value);
            },
            (Err(_), _) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header name {}", self.0),
                ));
            },
            (_, Err(_)) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header value {}", self.1),
                ));
            },
        }

        Ok(res)
    }
}

// It's also recommended to implement `IntoResponse` so `SetHeader` can be used on its own as
// the response
impl<'a> IntoResponse for SetHeader<'a> {
    fn into_response(self) -> Response {
        // This gives an empty response with the header
        (self, ()).into_response()
    }
}

// We can now return `SetHeader` in responses
//
// Note that returning `impl IntoResponse` might be easier if the response has many parts to
// it. The return type is written out here for clarity.
async fn handler() -> (SetHeader<'static>, SetHeader<'static>, &'static str) {
    (
        SetHeader("server", "axum"),
        SetHeader("x-foo", "custom"),
        "body",
    )
}

// Or on its own as the whole response
async fn other_handler() -> SetHeader<'static> {
    SetHeader("x-foo", "custom")
}
```

## Required Associated Types§

#### type Error: [IntoResponse][4]

The type returned in the event of an error.

This can be used to fallibly convert types into headers or extensions.

## Required Methods§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], Self::[Error][7]>

Set parts of the response

## Implementations on Foreign Types§

§

### impl [IntoResponseParts][8] for [()][9]

§

#### type Error = [Infallible][10]

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[()][9] as [IntoResponseParts][8]>::[Error][7]>

§

### impl [IntoResponseParts][8] for Extensions

§

#### type Error = [Infallible][10]

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <Extensions as [IntoResponseParts][8]>::[Error][7]>

§

### impl [IntoResponseParts][8] for HeaderMap

§

#### type Error = [Infallible][10]

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <HeaderMap as [IntoResponseParts][8]>::[Error][7]>

§

### impl<K, V, const N: [usize][11]> [IntoResponseParts][8] for [[(K, V)][12]; [N][13]]

where K: [TryInto][14]<HeaderName>, <K as [TryInto][14]<HeaderName>>::[Error][15]: [Display][16], V: [TryInto][14]<HeaderValue>, <V as [TryInto][14]<HeaderValue>>::[Error][15]: [Display][16],

§

#### type Error = TryIntoHeaderError<<K as [TryInto][14]<HeaderName>>::[Error][15], <V as [TryInto][14]<HeaderValue>>::[Error][15]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[[(K, V)][12]; [N][13]] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1> [IntoResponseParts][8] for [(T1,)][12]

where T1: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1,)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2> [IntoResponseParts][8] for [(T1, T2)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3> [IntoResponseParts][8] for [(T1, T2, T3)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3, T4> [IntoResponseParts][8] for [(T1, T2, T3, T4)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8], T4: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3, T4)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3, T4, T5> [IntoResponseParts][8] for [(T1, T2, T3, T4, T5)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8], T4: [IntoResponseParts][8], T5: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3, T4, T5)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3, T4, T5, T6> [IntoResponseParts][8] for [(T1, T2, T3, T4, T5, T6)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8], T4: [IntoResponseParts][8], T5: [IntoResponseParts][8], T6: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3, T4, T5, T6)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3, T4, T5, T6, T7> [IntoResponseParts][8] for [(T1, T2, T3, T4, T5, T6, T7)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8], T4: [IntoResponseParts][8], T5: [IntoResponseParts][8], T6: [IntoResponseParts][8], T7: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3, T4, T5, T6, T7)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3, T4, T5, T6, T7, T8> [IntoResponseParts][8] for [(T1, T2, T3, T4, T5, T6, T7, T8)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8], T4: [IntoResponseParts][8], T5: [IntoResponseParts][8], T6: [IntoResponseParts][8], T7: [IntoResponseParts][8], T8: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3, T4, T5, T6, T7, T8)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> [IntoResponseParts][8] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8], T4: [IntoResponseParts][8], T5: [IntoResponseParts][8], T6: [IntoResponseParts][8], T7: [IntoResponseParts][8], T8: [IntoResponseParts][8], T9: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> [IntoResponseParts][8] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8], T4: [IntoResponseParts][8], T5: [IntoResponseParts][8], T6: [IntoResponseParts][8], T7: [IntoResponseParts][8], T8: [IntoResponseParts][8], T9: [IntoResponseParts][8], T10: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> [IntoResponseParts][8] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8], T4: [IntoResponseParts][8], T5: [IntoResponseParts][8], T6: [IntoResponseParts][8], T7: [IntoResponseParts][8], T8: [IntoResponseParts][8], T9: [IntoResponseParts][8], T10: [IntoResponseParts][8], T11: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> [IntoResponseParts][8] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8], T4: [IntoResponseParts][8], T5: [IntoResponseParts][8], T6: [IntoResponseParts][8], T7: [IntoResponseParts][8], T8: [IntoResponseParts][8], T9: [IntoResponseParts][8], T10: [IntoResponseParts][8], T11: [IntoResponseParts][8], T12: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> [IntoResponseParts][8] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8], T4: [IntoResponseParts][8], T5: [IntoResponseParts][8], T6: [IntoResponseParts][8], T7: [IntoResponseParts][8], T8: [IntoResponseParts][8], T9: [IntoResponseParts][8], T10: [IntoResponseParts][8], T11: [IntoResponseParts][8], T12: [IntoResponseParts][8], T13: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> [IntoResponseParts][8] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8], T4: [IntoResponseParts][8], T5: [IntoResponseParts][8], T6: [IntoResponseParts][8], T7: [IntoResponseParts][8], T8: [IntoResponseParts][8], T9: [IntoResponseParts][8], T10: [IntoResponseParts][8], T11: [IntoResponseParts][8], T12: [IntoResponseParts][8], T13: [IntoResponseParts][8], T14: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> [IntoResponseParts][8] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8], T4: [IntoResponseParts][8], T5: [IntoResponseParts][8], T6: [IntoResponseParts][8], T7: [IntoResponseParts][8], T8: [IntoResponseParts][8], T9: [IntoResponseParts][8], T10: [IntoResponseParts][8], T11: [IntoResponseParts][8], T12: [IntoResponseParts][8], T13: [IntoResponseParts][8], T14: [IntoResponseParts][8], T15: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> [IntoResponseParts][8] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)][12]

where T1: [IntoResponseParts][8], T2: [IntoResponseParts][8], T3: [IntoResponseParts][8], T4: [IntoResponseParts][8], T5: [IntoResponseParts][8], T6: [IntoResponseParts][8], T7: [IntoResponseParts][8], T8: [IntoResponseParts][8], T9: [IntoResponseParts][8], T10: [IntoResponseParts][8], T11: [IntoResponseParts][8], T12: [IntoResponseParts][8], T13: [IntoResponseParts][8], T14: [IntoResponseParts][8], T15: [IntoResponseParts][8], T16: [IntoResponseParts][8],

§

#### type Error = Response<[Body][17]>

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)][12] as [IntoResponseParts][8]>::[Error][7]>

§

### impl<T> [IntoResponseParts][8] for [Option][18]<T>

where T: [IntoResponseParts][8],

§

#### type Error = <T as [IntoResponseParts][8]>::[Error][7]

§

#### fn into_response_parts( self, res: [ResponseParts][5], ) -> [Result][6]<[ResponseParts][5], <[Option][18]<T> as [IntoResponseParts][8]>::[Error][7]>

## Implementors§

§

### impl [IntoResponseParts][8] for [IntoResponseFailed][19]

§

#### type Error = [Infallible][10]

§

### impl<I, K, V> [IntoResponseParts][8] for [AppendHeaders][20]<I>

where I: [IntoIterator][21]<Item = [(K, V)][12]>, K: [TryInto][14]<HeaderName>, <K as [TryInto][14]<HeaderName>>::[Error][15]: [Display][16], V: [TryInto][14]<HeaderValue>, <V as [TryInto][14]<HeaderValue>>::[Error][15]: [Display][16],

§

#### type Error = TryIntoHeaderError<<K as [TryInto][14]<HeaderName>>::[Error][15], <V as [TryInto][14]<HeaderValue>>::[Error][15]>

[Source][22]§

### impl<T> [IntoResponseParts][8] for [Extension][23]<T>

where T: [Clone][24] \+ [Send][25] \+ [Sync][26] \+ 'static,

[Source][27]§

#### type Error = [Infallible][10]

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: trait.IntoResponse.html (trait axum::response::IntoResponse)
   [5]: struct.ResponseParts.html (struct axum::response::ResponseParts)
   [6]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [7]: trait.IntoResponseParts.html#associatedtype.Error (type axum::response::IntoResponseParts::Error)
   [8]: trait.IntoResponseParts.html (trait axum::response::IntoResponseParts)
   [9]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [10]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [11]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [12]: https://doc.rust-lang.org/nightly/std/primitive.tuple.html
   [13]: https://doc.rust-lang.org/nightly/std/primitive.array.html
   [14]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [15]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error (type core::convert::TryInto::Error)
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html (trait core::fmt::Display)
   [17]: ../body/struct.Body.html (struct axum::body::Body)
   [18]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [19]: struct.IntoResponseFailed.html (struct axum::response::IntoResponseFailed)
   [20]: struct.AppendHeaders.html (struct axum::response::AppendHeaders)
   [21]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [22]: ../../src/axum/extension.rs.html#117-127
   [23]: ../struct.Extension.html (struct axum::Extension)
   [24]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [26]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [27]: ../../src/axum/extension.rs.html#121

