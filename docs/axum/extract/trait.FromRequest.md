<!-- Generated from rustdoc HTML: extract/trait.FromRequest.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## FromRequest

## [axum][1]0.8.8

## FromRequest

### Required Associated Types

  * Rejection



### Required Methods

  * from_request



### Implementations on Foreign Types

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
  * Bytes
  * BytesMut
  * Option<T>
  * Request<Body>
  * Result<T, <T as FromRequest<S>>::Rejection>
  * String



### Dyn Compatibility

### Implementors

## [In axum::extract][2]

[axum][3]::[extract][2]

# Trait FromRequest Copy item path
```
pub trait FromRequest<S, M = ViaRequest>: [Sized][4] {
    type Rejection: [IntoResponse][5];

    // Required method
    fn from_request(
        req: Request<[Body][6]>,
        state: [&S][7],
    ) -> impl [Future][8]<Output = [Result][9]<Self, Self::[Rejection][10]>> + [Send][11];
}
```

Expand description

Types that can be created from requests.

Extractors that implement `FromRequest` can consume the request body and can thus only be run once for handlers.

If your extractor doesn’t need to consume the request body then you should implement [`FromRequestParts`][12] and not [`FromRequest`][13].

See [`axum::extract`][14] for more general docs about extractors.

## Required Associated Types§

#### type Rejection: [IntoResponse][5]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

## Required Methods§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<Self, Self::[Rejection][10]>> \+ [Send][11]

Perform the extraction.

## Dyn Compatibility§

This trait is **not** [dyn compatible][15].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementations on Foreign Types§

§

### impl<S> [FromRequest][13]<S> for [String][16]

where S: [Send][11] \+ [Sync][17],

§

#### type Rejection = [StringRejection][18]

§

#### async fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> [Result][9]<[String][16], <[String][16] as [FromRequest][13]<S>>::[Rejection][10]>

§

### impl<S> [FromRequest][13]<S> for Bytes

where S: [Send][11] \+ [Sync][17],

§

#### type Rejection = [BytesRejection][19]

§

#### async fn from_request( req: Request<[Body][6]>, _: [&S][7], ) -> [Result][9]<Bytes, <Bytes as [FromRequest][13]<S>>::[Rejection][10]>

§

### impl<S> [FromRequest][13]<S> for BytesMut

where S: [Send][11] \+ [Sync][17],

§

#### type Rejection = [BytesRejection][19]

§

#### async fn from_request( req: Request<[Body][6]>, _: [&S][7], ) -> [Result][9]<BytesMut, <BytesMut as [FromRequest][13]<S>>::[Rejection][10]>

§

### impl<S> [FromRequest][13]<S> for Request<[Body][6]>

where S: [Send][11] \+ [Sync][17],

§

#### type Rejection = [Infallible][20]

§

#### async fn from_request( req: Request<[Body][6]>, _: [&S][7], ) -> [Result][9]<Request<[Body][6]>, <Request<[Body][6]> as [FromRequest][13]<S>>::[Rejection][10]>

§

### impl<S, T1> [FromRequest][13]<S> for [(T1,)][21]

where T1: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1,)][21], <[(T1,)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2> [FromRequest][13]<S> for [(T1, T2)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2)][21], <[(T1, T2)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3> [FromRequest][13]<S> for [(T1, T2, T3)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3)][21], <[(T1, T2, T3)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3, T4> [FromRequest][13]<S> for [(T1, T2, T3, T4)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequestParts][12]<S> \+ [Send][11], T4: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3, T4)][21], <[(T1, T2, T3, T4)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3, T4, T5> [FromRequest][13]<S> for [(T1, T2, T3, T4, T5)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequestParts][12]<S> \+ [Send][11], T4: [FromRequestParts][12]<S> \+ [Send][11], T5: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3, T4, T5)][21], <[(T1, T2, T3, T4, T5)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3, T4, T5, T6> [FromRequest][13]<S> for [(T1, T2, T3, T4, T5, T6)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequestParts][12]<S> \+ [Send][11], T4: [FromRequestParts][12]<S> \+ [Send][11], T5: [FromRequestParts][12]<S> \+ [Send][11], T6: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3, T4, T5, T6)][21], <[(T1, T2, T3, T4, T5, T6)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7> [FromRequest][13]<S> for [(T1, T2, T3, T4, T5, T6, T7)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequestParts][12]<S> \+ [Send][11], T4: [FromRequestParts][12]<S> \+ [Send][11], T5: [FromRequestParts][12]<S> \+ [Send][11], T6: [FromRequestParts][12]<S> \+ [Send][11], T7: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3, T4, T5, T6, T7)][21], <[(T1, T2, T3, T4, T5, T6, T7)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8> [FromRequest][13]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequestParts][12]<S> \+ [Send][11], T4: [FromRequestParts][12]<S> \+ [Send][11], T5: [FromRequestParts][12]<S> \+ [Send][11], T6: [FromRequestParts][12]<S> \+ [Send][11], T7: [FromRequestParts][12]<S> \+ [Send][11], T8: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3, T4, T5, T6, T7, T8)][21], <[(T1, T2, T3, T4, T5, T6, T7, T8)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9> [FromRequest][13]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequestParts][12]<S> \+ [Send][11], T4: [FromRequestParts][12]<S> \+ [Send][11], T5: [FromRequestParts][12]<S> \+ [Send][11], T6: [FromRequestParts][12]<S> \+ [Send][11], T7: [FromRequestParts][12]<S> \+ [Send][11], T8: [FromRequestParts][12]<S> \+ [Send][11], T9: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9)][21], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> [FromRequest][13]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequestParts][12]<S> \+ [Send][11], T4: [FromRequestParts][12]<S> \+ [Send][11], T5: [FromRequestParts][12]<S> \+ [Send][11], T6: [FromRequestParts][12]<S> \+ [Send][11], T7: [FromRequestParts][12]<S> \+ [Send][11], T8: [FromRequestParts][12]<S> \+ [Send][11], T9: [FromRequestParts][12]<S> \+ [Send][11], T10: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)][21], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> [FromRequest][13]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequestParts][12]<S> \+ [Send][11], T4: [FromRequestParts][12]<S> \+ [Send][11], T5: [FromRequestParts][12]<S> \+ [Send][11], T6: [FromRequestParts][12]<S> \+ [Send][11], T7: [FromRequestParts][12]<S> \+ [Send][11], T8: [FromRequestParts][12]<S> \+ [Send][11], T9: [FromRequestParts][12]<S> \+ [Send][11], T10: [FromRequestParts][12]<S> \+ [Send][11], T11: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)][21], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> [FromRequest][13]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequestParts][12]<S> \+ [Send][11], T4: [FromRequestParts][12]<S> \+ [Send][11], T5: [FromRequestParts][12]<S> \+ [Send][11], T6: [FromRequestParts][12]<S> \+ [Send][11], T7: [FromRequestParts][12]<S> \+ [Send][11], T8: [FromRequestParts][12]<S> \+ [Send][11], T9: [FromRequestParts][12]<S> \+ [Send][11], T10: [FromRequestParts][12]<S> \+ [Send][11], T11: [FromRequestParts][12]<S> \+ [Send][11], T12: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)][21], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> [FromRequest][13]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequestParts][12]<S> \+ [Send][11], T4: [FromRequestParts][12]<S> \+ [Send][11], T5: [FromRequestParts][12]<S> \+ [Send][11], T6: [FromRequestParts][12]<S> \+ [Send][11], T7: [FromRequestParts][12]<S> \+ [Send][11], T8: [FromRequestParts][12]<S> \+ [Send][11], T9: [FromRequestParts][12]<S> \+ [Send][11], T10: [FromRequestParts][12]<S> \+ [Send][11], T11: [FromRequestParts][12]<S> \+ [Send][11], T12: [FromRequestParts][12]<S> \+ [Send][11], T13: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)][21], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> [FromRequest][13]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequestParts][12]<S> \+ [Send][11], T4: [FromRequestParts][12]<S> \+ [Send][11], T5: [FromRequestParts][12]<S> \+ [Send][11], T6: [FromRequestParts][12]<S> \+ [Send][11], T7: [FromRequestParts][12]<S> \+ [Send][11], T8: [FromRequestParts][12]<S> \+ [Send][11], T9: [FromRequestParts][12]<S> \+ [Send][11], T10: [FromRequestParts][12]<S> \+ [Send][11], T11: [FromRequestParts][12]<S> \+ [Send][11], T12: [FromRequestParts][12]<S> \+ [Send][11], T13: [FromRequestParts][12]<S> \+ [Send][11], T14: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)][21], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> [FromRequest][13]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequestParts][12]<S> \+ [Send][11], T4: [FromRequestParts][12]<S> \+ [Send][11], T5: [FromRequestParts][12]<S> \+ [Send][11], T6: [FromRequestParts][12]<S> \+ [Send][11], T7: [FromRequestParts][12]<S> \+ [Send][11], T8: [FromRequestParts][12]<S> \+ [Send][11], T9: [FromRequestParts][12]<S> \+ [Send][11], T10: [FromRequestParts][12]<S> \+ [Send][11], T11: [FromRequestParts][12]<S> \+ [Send][11], T12: [FromRequestParts][12]<S> \+ [Send][11], T13: [FromRequestParts][12]<S> \+ [Send][11], T14: [FromRequestParts][12]<S> \+ [Send][11], T15: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)][21], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> [FromRequest][13]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)][21]

where T1: [FromRequestParts][12]<S> \+ [Send][11], T2: [FromRequestParts][12]<S> \+ [Send][11], T3: [FromRequestParts][12]<S> \+ [Send][11], T4: [FromRequestParts][12]<S> \+ [Send][11], T5: [FromRequestParts][12]<S> \+ [Send][11], T6: [FromRequestParts][12]<S> \+ [Send][11], T7: [FromRequestParts][12]<S> \+ [Send][11], T8: [FromRequestParts][12]<S> \+ [Send][11], T9: [FromRequestParts][12]<S> \+ [Send][11], T10: [FromRequestParts][12]<S> \+ [Send][11], T11: [FromRequestParts][12]<S> \+ [Send][11], T12: [FromRequestParts][12]<S> \+ [Send][11], T13: [FromRequestParts][12]<S> \+ [Send][11], T14: [FromRequestParts][12]<S> \+ [Send][11], T15: [FromRequestParts][12]<S> \+ [Send][11], T16: [FromRequest][13]<S> \+ [Send][11], S: [Send][11] \+ [Sync][17],

§

#### type Rejection = Response<[Body][6]>

§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)][21], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)][21] as [FromRequest][13]<S>>::[Rejection][10]>>

§

### impl<S, T> [FromRequest][13]<S> for [Option][22]<T>

where T: [OptionalFromRequest][23]<S>, S: [Send][11] \+ [Sync][17],

§

#### type Rejection = <T as [OptionalFromRequest][23]<S>>::[Rejection][24]

§

#### async fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> [Result][9]<[Option][22]<T>, <[Option][22]<T> as [FromRequest][13]<S>>::[Rejection][10]>

§

### impl<S, T> [FromRequest][13]<S> for [Result][9]<T, <T as [FromRequest][13]<S>>::[Rejection][10]>

where T: [FromRequest][13]<S>, S: [Send][11] \+ [Sync][17],

§

#### type Rejection = [Infallible][20]

§

#### async fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> [Result][9]<[Result][9]<T, <T as [FromRequest][13]<S>>::[Rejection][10]>, <[Result][9]<T, <T as [FromRequest][13]<S>>::[Rejection][10]> as [FromRequest][13]<S>>::[Rejection][10]>

## Implementors§

§

### impl<S> [FromRequest][13]<S> for [Body][6]

where S: [Send][11] \+ [Sync][17],

§

#### type Rejection = [Infallible][20]

[Source][25]§

### impl<S> [FromRequest][13]<S> for [Multipart][26]

where S: [Send][11] \+ [Sync][17],

Available on **crate feature`multipart`** only.

[Source][27]§

#### type Rejection = [MultipartRejection][28]

[Source][29]§

### impl<S> [FromRequest][13]<S> for [RawForm][30]

where S: [Send][11] \+ [Sync][17],

[Source][31]§

#### type Rejection = [RawFormRejection][32]

§

### impl<S, T> [FromRequest][13]<S, ViaParts> for T

where S: [Send][11] \+ [Sync][17], T: [FromRequestParts][12]<S>,

§

#### type Rejection = <T as [FromRequestParts][12]<S>>::[Rejection][33]

[Source][34]§

### impl<T, S> [FromRequest][13]<S> for [Form][35]<T>

where T: [DeserializeOwned][36], S: [Send][11] \+ [Sync][17],

Available on **crate feature`form`** only.

[Source][37]§

#### type Rejection = [FormRejection][38]

[Source][39]§

### impl<T, S> [FromRequest][13]<S> for [Json][40]<T>

where T: [DeserializeOwned][36], S: [Send][11] \+ [Sync][17],

Available on **crate feature`json`** only.

[Source][41]§

#### type Rejection = [JsonRejection][42]

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [5]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [6]: ../body/struct.Body.html (struct axum::body::Body)
   [7]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [8]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [9]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [10]: trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [11]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [12]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [13]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [14]: https://docs.rs/axum/0.8/axum/extract/index.html
   [15]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
   [16]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [17]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [18]: rejection/enum.StringRejection.html (enum axum::extract::rejection::StringRejection)
   [19]: rejection/enum.BytesRejection.html (enum axum::extract::rejection::BytesRejection)
   [20]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [21]: https://doc.rust-lang.org/nightly/std/primitive.tuple.html
   [22]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [23]: trait.OptionalFromRequest.html (trait axum::extract::OptionalFromRequest)
   [24]: trait.OptionalFromRequest.html#associatedtype.Rejection (type axum::extract::OptionalFromRequest::Rejection)
   [25]: ../../src/axum/extract/multipart.rs.html#68-82
   [26]: struct.Multipart.html (struct axum::extract::Multipart)
   [27]: ../../src/axum/extract/multipart.rs.html#72
   [28]: multipart/enum.MultipartRejection.html (enum axum::extract::multipart::MultipartRejection)
   [29]: ../../src/axum/extract/raw_form.rs.html#32-53
   [30]: struct.RawForm.html (struct axum::extract::RawForm)
   [31]: ../../src/axum/extract/raw_form.rs.html#36
   [32]: rejection/enum.RawFormRejection.html (enum axum::extract::rejection::RawFormRejection)
   [33]: trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [34]: ../../src/axum/form.rs.html#73-105
   [35]: ../struct.Form.html (struct axum::Form)
   [36]: https://docs.rs/serde_core/1.0.228/serde_core/de/trait.DeserializeOwned.html (trait serde_core::de::DeserializeOwned)
   [37]: ../../src/axum/form.rs.html#78
   [38]: rejection/enum.FormRejection.html (enum axum::extract::rejection::FormRejection)
   [39]: ../../src/axum/json.rs.html#99-114
   [40]: ../struct.Json.html (struct axum::Json)
   [41]: ../../src/axum/json.rs.html#104
   [42]: rejection/enum.JsonRejection.html (enum axum::extract::rejection::JsonRejection)

