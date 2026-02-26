<!-- Generated from rustdoc HTML: extract/trait.FromRequestParts.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## FromRequestParts

## [axum][1]0.8.8

## FromRequestParts

### Required Associated Types

  * Rejection



### Required Methods

  * from_request_parts



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
  * Method
  * Option<T>
  * Parts
  * Result<T, <T as FromRequestParts<S>>::Rejection>
  * Uri
  * Version



### Dyn Compatibility

### Implementors

## [In axum::extract][2]

[axum][3]::[extract][2]

# Trait FromRequestParts Copy item path
```
pub trait FromRequestParts<S>: [Sized][4] {
    type Rejection: [IntoResponse][5];

    // Required method
    fn from_request_parts(
        parts: &mut Parts,
        state: [&S][6],
    ) -> impl [Future][7]<Output = [Result][8]<Self, Self::[Rejection][9]>> + [Send][10];
}
```

Expand description

Types that can be created from request parts.

Extractors that implement `FromRequestParts` cannot consume the request body and can thus be run in any order for handlers.

If your extractor needs to consume the request body then you should implement [`FromRequest`][11] and not [`FromRequestParts`][12].

See [`axum::extract`][13] for more general docs about extractors.

## Required Associated Types§

#### type Rejection: [IntoResponse][5]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

## Required Methods§

#### fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> impl [Future][7]<Output = [Result][8]<Self, Self::[Rejection][9]>> \+ [Send][10]

Perform the extraction.

## Dyn Compatibility§

This trait is **not** [dyn compatible][14].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementations on Foreign Types§

§

### impl<S> [FromRequestParts][12]<S> for [()][15]

where S: [Send][10] \+ [Sync][16],

§

#### type Rejection = [Infallible][17]

§

#### async fn from_request_parts( _: &mut Parts, _: [&S][6], ) -> [Result][8]<[()][15], <[()][15] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S> [FromRequestParts][12]<S> for Extensions

where S: [Send][10] \+ [Sync][16],

§

#### type Rejection = [Infallible][17]

§

#### async fn from_request_parts( parts: &mut Parts, _state: [&S][6], ) -> [Result][8]<Extensions, <Extensions as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S> [FromRequestParts][12]<S> for HeaderMap

where S: [Send][10] \+ [Sync][16],

Clone the headers from the request.

Prefer using [`TypedHeader`][18] to extract only the headers you need.

§

#### type Rejection = [Infallible][17]

§

#### async fn from_request_parts( parts: &mut Parts, _: [&S][6], ) -> [Result][8]<HeaderMap, <HeaderMap as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S> [FromRequestParts][12]<S> for Method

where S: [Send][10] \+ [Sync][16],

§

#### type Rejection = [Infallible][17]

§

#### async fn from_request_parts( parts: &mut Parts, _: [&S][6], ) -> [Result][8]<Method, <Method as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S> [FromRequestParts][12]<S> for Parts

where S: [Send][10] \+ [Sync][16],

§

#### type Rejection = [Infallible][17]

§

#### async fn from_request_parts( parts: &mut Parts, _state: [&S][6], ) -> [Result][8]<Parts, <Parts as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S> [FromRequestParts][12]<S> for Uri

where S: [Send][10] \+ [Sync][16],

§

#### type Rejection = [Infallible][17]

§

#### async fn from_request_parts( parts: &mut Parts, _: [&S][6], ) -> [Result][8]<Uri, <Uri as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S> [FromRequestParts][12]<S> for Version

where S: [Send][10] \+ [Sync][16],

§

#### type Rejection = [Infallible][17]

§

#### async fn from_request_parts( parts: &mut Parts, _: [&S][6], ) -> [Result][8]<Version, <Version as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1> [FromRequestParts][12]<S> for [(T1,)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1,)][19], <[(T1,)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2> [FromRequestParts][12]<S> for [(T1, T2)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2)][19], <[(T1, T2)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3> [FromRequestParts][12]<S> for [(T1, T2, T3)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3)][19], <[(T1, T2, T3)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3, T4> [FromRequestParts][12]<S> for [(T1, T2, T3, T4)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], T4: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3, T4)][19], <[(T1, T2, T3, T4)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3, T4, T5> [FromRequestParts][12]<S> for [(T1, T2, T3, T4, T5)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], T4: [FromRequestParts][12]<S> \+ [Send][10], T5: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3, T4, T5)][19], <[(T1, T2, T3, T4, T5)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3, T4, T5, T6> [FromRequestParts][12]<S> for [(T1, T2, T3, T4, T5, T6)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], T4: [FromRequestParts][12]<S> \+ [Send][10], T5: [FromRequestParts][12]<S> \+ [Send][10], T6: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3, T4, T5, T6)][19], <[(T1, T2, T3, T4, T5, T6)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7> [FromRequestParts][12]<S> for [(T1, T2, T3, T4, T5, T6, T7)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], T4: [FromRequestParts][12]<S> \+ [Send][10], T5: [FromRequestParts][12]<S> \+ [Send][10], T6: [FromRequestParts][12]<S> \+ [Send][10], T7: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3, T4, T5, T6, T7)][19], <[(T1, T2, T3, T4, T5, T6, T7)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8> [FromRequestParts][12]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], T4: [FromRequestParts][12]<S> \+ [Send][10], T5: [FromRequestParts][12]<S> \+ [Send][10], T6: [FromRequestParts][12]<S> \+ [Send][10], T7: [FromRequestParts][12]<S> \+ [Send][10], T8: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3, T4, T5, T6, T7, T8)][19], <[(T1, T2, T3, T4, T5, T6, T7, T8)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9> [FromRequestParts][12]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], T4: [FromRequestParts][12]<S> \+ [Send][10], T5: [FromRequestParts][12]<S> \+ [Send][10], T6: [FromRequestParts][12]<S> \+ [Send][10], T7: [FromRequestParts][12]<S> \+ [Send][10], T8: [FromRequestParts][12]<S> \+ [Send][10], T9: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9)][19], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> [FromRequestParts][12]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], T4: [FromRequestParts][12]<S> \+ [Send][10], T5: [FromRequestParts][12]<S> \+ [Send][10], T6: [FromRequestParts][12]<S> \+ [Send][10], T7: [FromRequestParts][12]<S> \+ [Send][10], T8: [FromRequestParts][12]<S> \+ [Send][10], T9: [FromRequestParts][12]<S> \+ [Send][10], T10: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)][19], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> [FromRequestParts][12]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], T4: [FromRequestParts][12]<S> \+ [Send][10], T5: [FromRequestParts][12]<S> \+ [Send][10], T6: [FromRequestParts][12]<S> \+ [Send][10], T7: [FromRequestParts][12]<S> \+ [Send][10], T8: [FromRequestParts][12]<S> \+ [Send][10], T9: [FromRequestParts][12]<S> \+ [Send][10], T10: [FromRequestParts][12]<S> \+ [Send][10], T11: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)][19], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> [FromRequestParts][12]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], T4: [FromRequestParts][12]<S> \+ [Send][10], T5: [FromRequestParts][12]<S> \+ [Send][10], T6: [FromRequestParts][12]<S> \+ [Send][10], T7: [FromRequestParts][12]<S> \+ [Send][10], T8: [FromRequestParts][12]<S> \+ [Send][10], T9: [FromRequestParts][12]<S> \+ [Send][10], T10: [FromRequestParts][12]<S> \+ [Send][10], T11: [FromRequestParts][12]<S> \+ [Send][10], T12: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)][19], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> [FromRequestParts][12]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], T4: [FromRequestParts][12]<S> \+ [Send][10], T5: [FromRequestParts][12]<S> \+ [Send][10], T6: [FromRequestParts][12]<S> \+ [Send][10], T7: [FromRequestParts][12]<S> \+ [Send][10], T8: [FromRequestParts][12]<S> \+ [Send][10], T9: [FromRequestParts][12]<S> \+ [Send][10], T10: [FromRequestParts][12]<S> \+ [Send][10], T11: [FromRequestParts][12]<S> \+ [Send][10], T12: [FromRequestParts][12]<S> \+ [Send][10], T13: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)][19], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> [FromRequestParts][12]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], T4: [FromRequestParts][12]<S> \+ [Send][10], T5: [FromRequestParts][12]<S> \+ [Send][10], T6: [FromRequestParts][12]<S> \+ [Send][10], T7: [FromRequestParts][12]<S> \+ [Send][10], T8: [FromRequestParts][12]<S> \+ [Send][10], T9: [FromRequestParts][12]<S> \+ [Send][10], T10: [FromRequestParts][12]<S> \+ [Send][10], T11: [FromRequestParts][12]<S> \+ [Send][10], T12: [FromRequestParts][12]<S> \+ [Send][10], T13: [FromRequestParts][12]<S> \+ [Send][10], T14: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)][19], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> [FromRequestParts][12]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], T4: [FromRequestParts][12]<S> \+ [Send][10], T5: [FromRequestParts][12]<S> \+ [Send][10], T6: [FromRequestParts][12]<S> \+ [Send][10], T7: [FromRequestParts][12]<S> \+ [Send][10], T8: [FromRequestParts][12]<S> \+ [Send][10], T9: [FromRequestParts][12]<S> \+ [Send][10], T10: [FromRequestParts][12]<S> \+ [Send][10], T11: [FromRequestParts][12]<S> \+ [Send][10], T12: [FromRequestParts][12]<S> \+ [Send][10], T13: [FromRequestParts][12]<S> \+ [Send][10], T14: [FromRequestParts][12]<S> \+ [Send][10], T15: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)][19], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> [FromRequestParts][12]<S> for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)][19]

where T1: [FromRequestParts][12]<S> \+ [Send][10], T2: [FromRequestParts][12]<S> \+ [Send][10], T3: [FromRequestParts][12]<S> \+ [Send][10], T4: [FromRequestParts][12]<S> \+ [Send][10], T5: [FromRequestParts][12]<S> \+ [Send][10], T6: [FromRequestParts][12]<S> \+ [Send][10], T7: [FromRequestParts][12]<S> \+ [Send][10], T8: [FromRequestParts][12]<S> \+ [Send][10], T9: [FromRequestParts][12]<S> \+ [Send][10], T10: [FromRequestParts][12]<S> \+ [Send][10], T11: [FromRequestParts][12]<S> \+ [Send][10], T12: [FromRequestParts][12]<S> \+ [Send][10], T13: [FromRequestParts][12]<S> \+ [Send][10], T14: [FromRequestParts][12]<S> \+ [Send][10], T15: [FromRequestParts][12]<S> \+ [Send][10], T16: [FromRequestParts][12]<S> \+ [Send][10], S: [Send][10] \+ [Sync][16],

§

#### type Rejection = Response<[Body][20]>

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)][19], <[(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)][19] as [FromRequestParts][12]<S>>::[Rejection][9]>

§

### impl<S, T> [FromRequestParts][12]<S> for [Option][21]<T>

where T: [OptionalFromRequestParts][22]<S>, S: [Send][10] \+ [Sync][16],

§

#### type Rejection = <T as [OptionalFromRequestParts][22]<S>>::[Rejection][23]

§

#### fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> impl [Future][7]<Output = [Result][8]<[Option][21]<T>, <[Option][21]<T> as [FromRequestParts][12]<S>>::[Rejection][9]>>

§

### impl<S, T> [FromRequestParts][12]<S> for [Result][8]<T, <T as [FromRequestParts][12]<S>>::[Rejection][9]>

where T: [FromRequestParts][12]<S>, S: [Send][10] \+ [Sync][16],

§

#### type Rejection = [Infallible][17]

§

#### async fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> [Result][8]<[Result][8]<T, <T as [FromRequestParts][12]<S>>::[Rejection][9]>, <[Result][8]<T, <T as [FromRequestParts][12]<S>>::[Rejection][9]> as [FromRequestParts][12]<S>>::[Rejection][9]>

## Implementors§

[Source][24]§

### impl<OuterState, InnerState> [FromRequestParts][12]<OuterState> for [State][25]<InnerState>

where InnerState: [FromRef][26]<OuterState>, OuterState: [Send][10] \+ [Sync][16],

[Source][27]§

#### type Rejection = [Infallible][17]

[Source][28]§

### impl<S> [FromRequestParts][12]<S> for [MatchedPath][29]

where S: [Send][10] \+ [Sync][16],

Available on **crate feature`matched-path`** only.

[Source][30]§

#### type Rejection = [MatchedPathRejection][31]

[Source][32]§

### impl<S> [FromRequestParts][12]<S> for [NestedPath][33]

where S: [Send][10] \+ [Sync][16],

[Source][34]§

#### type Rejection = [NestedPathRejection][35]

[Source][36]§

### impl<S> [FromRequestParts][12]<S> for [OriginalUri][37]

where S: [Send][10] \+ [Sync][16],

Available on **crate feature`original-uri`** only.

[Source][38]§

#### type Rejection = [Infallible][17]

[Source][39]§

### impl<S> [FromRequestParts][12]<S> for [RawPathParams][40]

where S: [Send][10] \+ [Sync][16],

[Source][41]§

#### type Rejection = [RawPathParamsRejection][42]

[Source][43]§

### impl<S> [FromRequestParts][12]<S> for [RawQuery][44]

where S: [Send][10] \+ [Sync][16],

[Source][45]§

#### type Rejection = [Infallible][17]

[Source][46]§

### impl<S> [FromRequestParts][12]<S> for [WebSocketUpgrade][47]<[DefaultOnFailedUpgrade][48]>

where S: [Send][10] \+ [Sync][16],

Available on **crate feature`ws`** only.

[Source][49]§

#### type Rejection = [WebSocketUpgradeRejection][50]

[Source][51]§

### impl<S, T> [FromRequestParts][12]<S> for [ConnectInfo][52]<T>

where S: [Send][10] \+ [Sync][16], T: [Clone][53] \+ [Send][10] \+ [Sync][16] \+ 'static,

Available on **crate feature`tokio`** only.

[Source][54]§

#### type Rejection = <[Extension][55]<[ConnectInfo][52]<T>> as [FromRequestParts][12]<S>>::[Rejection][9]

[Source][56]§

### impl<T, S> [FromRequestParts][12]<S> for [Extension][55]<T>

where T: [Clone][53] \+ [Send][10] \+ [Sync][16] \+ 'static, S: [Send][10] \+ [Sync][16],

[Source][57]§

#### type Rejection = [ExtensionRejection][58]

[Source][59]§

### impl<T, S> [FromRequestParts][12]<S> for [Path][60]<T>

where T: [DeserializeOwned][61] \+ [Send][10], S: [Send][10] \+ [Sync][16],

[Source][62]§

#### type Rejection = [PathRejection][63]

[Source][64]§

### impl<T, S> [FromRequestParts][12]<S> for [Query][65]<T>

where T: [DeserializeOwned][61], S: [Send][10] \+ [Sync][16],

Available on **crate feature`query`** only.

[Source][66]§

#### type Rejection = [QueryRejection][67]

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [5]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [6]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [7]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [8]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [9]: trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [10]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [11]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [12]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [13]: https://docs.rs/axum/0.8/axum/extract/index.html
   [14]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
   [15]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [16]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [17]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [18]: https://docs.rs/axum-extra/0.10/axum_extra/struct.TypedHeader.html
   [19]: https://doc.rust-lang.org/nightly/std/primitive.tuple.html
   [20]: ../body/struct.Body.html (struct axum::body::Body)
   [21]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [22]: trait.OptionalFromRequestParts.html (trait axum::extract::OptionalFromRequestParts)
   [23]: trait.OptionalFromRequestParts.html#associatedtype.Rejection (type axum::extract::OptionalFromRequestParts::Rejection)
   [24]: ../../src/axum/extract/state.rs.html#298-312
   [25]: struct.State.html (struct axum::extract::State)
   [26]: trait.FromRef.html (trait axum::extract::FromRef)
   [27]: ../../src/axum/extract/state.rs.html#303
   [28]: ../../src/axum/extract/matched_path.rs.html#67-82
   [29]: struct.MatchedPath.html (struct axum::extract::MatchedPath)
   [30]: ../../src/axum/extract/matched_path.rs.html#71
   [31]: rejection/enum.MatchedPathRejection.html (enum axum::extract::rejection::MatchedPathRejection)
   [32]: ../../src/axum/extract/nested_path.rs.html#51-63
   [33]: struct.NestedPath.html (struct axum::extract::NestedPath)
   [34]: ../../src/axum/extract/nested_path.rs.html#55
   [35]: rejection/struct.NestedPathRejection.html (struct axum::extract::rejection::NestedPathRejection)
   [36]: ../../src/axum/extract/original_uri.rs.html#70-83
   [37]: struct.OriginalUri.html (struct axum::extract::OriginalUri)
   [38]: ../../src/axum/extract/original_uri.rs.html#74
   [39]: ../../src/axum/extract/path/mod.rs.html#503-525
   [40]: struct.RawPathParams.html (struct axum::extract::RawPathParams)
   [41]: ../../src/axum/extract/path/mod.rs.html#507
   [42]: rejection/enum.RawPathParamsRejection.html (enum axum::extract::rejection::RawPathParamsRejection)
   [43]: ../../src/axum/extract/raw_query.rs.html#27-37
   [44]: struct.RawQuery.html (struct axum::extract::RawQuery)
   [45]: ../../src/axum/extract/raw_query.rs.html#31
   [46]: ../../src/axum/extract/ws.rs.html#442-517
   [47]: struct.WebSocketUpgrade.html (struct axum::extract::WebSocketUpgrade)
   [48]: ws/struct.DefaultOnFailedUpgrade.html (struct axum::extract::ws::DefaultOnFailedUpgrade)
   [49]: ../../src/axum/extract/ws.rs.html#446
   [50]: ws/rejection/enum.WebSocketUpgradeRejection.html (enum axum::extract::ws::rejection::WebSocketUpgradeRejection)
   [51]: ../../src/axum/extract/connect_info.rs.html#138-154
   [52]: struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [53]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [54]: ../../src/axum/extract/connect_info.rs.html#143
   [55]: ../struct.Extension.html (struct axum::Extension)
   [56]: ../../src/axum/extension.rs.html#83-98
   [57]: ../../src/axum/extension.rs.html#88
   [58]: rejection/enum.ExtensionRejection.html (enum axum::extract::rejection::ExtensionRejection)
   [59]: ../../src/axum/extract/path/mod.rs.html#157-190
   [60]: struct.Path.html (struct axum::extract::Path)
   [61]: https://docs.rs/serde_core/1.0.228/serde_core/de/trait.DeserializeOwned.html (trait serde_core::de::DeserializeOwned)
   [62]: ../../src/axum/extract/path/mod.rs.html#162
   [63]: rejection/enum.PathRejection.html (enum axum::extract::rejection::PathRejection)
   [64]: ../../src/axum/extract/query.rs.html#43-53
   [65]: struct.Query.html (struct axum::extract::Query)
   [66]: ../../src/axum/extract/query.rs.html#48
   [67]: rejection/enum.QueryRejection.html (enum axum::extract::rejection::QueryRejection)

