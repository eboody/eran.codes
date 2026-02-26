<!-- Generated from rustdoc HTML: middleware/struct.MapResponse.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## MapResponse

## [axum][1]0.8.8

## MapResponse

### Trait Implementations

  * Clone
  * Debug
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>
  * Service<Request<B>>



### Auto Trait Implementations

  * Freeze
  * RefUnwindSafe
  * Send
  * Sync
  * Unpin
  * UnwindSafe



### Blanket Implementations

  * Any
  * Borrow<T>
  * BorrowMut<T>
  * CloneToUninit
  * From<T>
  * FromRef<T>
  * Instrument
  * Into<U>
  * MakeService<Target, Request>
  * PolicyExt
  * Same
  * ServiceExt
  * ServiceExt<R>
  * ServiceExt<Request>
  * ToOwned
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Struct MapResponse Copy item path

[Source][4]
``` 
pub struct MapResponse<F, S, I, T> { /* private fields */ }
```

Expand description

A middleware created from an async function that transforms a response.

Created with [`map_response`][5]. See that function for more details.

## Trait Implementations§

[Source][6]§

### impl<F, S, I, T> [Clone][7] for [MapResponse][8]<F, S, I, T>

where F: [Clone][7], I: [Clone][7], S: [Clone][7],

[Source][9]§

#### fn [clone][10](&self) -> Self

Returns a duplicate of the value. [Read more][10]

1.0.0 · [Source][11]§

#### fn [clone_from][12](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][12]

[Source][13]§

### impl<F, S, I, T> [Debug][14] for [MapResponse][8]<F, S, I, T>

where S: [Debug][14], I: [Debug][14],

[Source][15]§

#### fn [fmt][16](&self, f: &mut [Formatter][17]<'_>) -> [Result][18]

Formats the value using the given formatter. [Read more][16]

[Source][19]§

### impl<F, Fut, S, I, B, ResBody> Service<Request<B>> for [MapResponse][8]<F, S, I, [()][20]>

where F: [FnMut][21]([Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][34]§

### impl<F, Fut, S, I, B, ResBody, T1> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1,)][35]>

where F: [FnMut][21](T1, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][34]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][34]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][34]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][34]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][34]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][37]§

### impl<F, Fut, S, I, B, ResBody, T1, T2> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2)][35]>

where F: [FnMut][21](T1, T2, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][37]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][37]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][37]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][37]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][37]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][38]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3)][35]>

where F: [FnMut][21](T1, T2, T3, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][38]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][38]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][38]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][38]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][38]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][39]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3, T4> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3, T4)][35]>

where F: [FnMut][21](T1, T2, T3, T4, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], T4: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][39]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][39]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][39]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][39]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][39]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][40]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3, T4, T5> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3, T4, T5)][35]>

where F: [FnMut][21](T1, T2, T3, T4, T5, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], T4: [FromRequestParts][36]<S> \+ [Send][23], T5: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][40]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][40]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][40]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][40]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][40]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][41]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3, T4, T5, T6> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3, T4, T5, T6)][35]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], T4: [FromRequestParts][36]<S> \+ [Send][23], T5: [FromRequestParts][36]<S> \+ [Send][23], T6: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][41]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][41]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][41]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][41]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][41]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][42]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3, T4, T5, T6, T7> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7)][35]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], T4: [FromRequestParts][36]<S> \+ [Send][23], T5: [FromRequestParts][36]<S> \+ [Send][23], T6: [FromRequestParts][36]<S> \+ [Send][23], T7: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][42]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][42]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][42]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][42]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][42]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][43]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3, T4, T5, T6, T7, T8> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8)][35]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], T4: [FromRequestParts][36]<S> \+ [Send][23], T5: [FromRequestParts][36]<S> \+ [Send][23], T6: [FromRequestParts][36]<S> \+ [Send][23], T7: [FromRequestParts][36]<S> \+ [Send][23], T8: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][43]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][43]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][43]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][43]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][43]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][44]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3, T4, T5, T6, T7, T8, T9> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9)][35]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], T4: [FromRequestParts][36]<S> \+ [Send][23], T5: [FromRequestParts][36]<S> \+ [Send][23], T6: [FromRequestParts][36]<S> \+ [Send][23], T7: [FromRequestParts][36]<S> \+ [Send][23], T8: [FromRequestParts][36]<S> \+ [Send][23], T9: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][44]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][44]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][44]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][44]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][44]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][45]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)][35]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], T4: [FromRequestParts][36]<S> \+ [Send][23], T5: [FromRequestParts][36]<S> \+ [Send][23], T6: [FromRequestParts][36]<S> \+ [Send][23], T7: [FromRequestParts][36]<S> \+ [Send][23], T8: [FromRequestParts][36]<S> \+ [Send][23], T9: [FromRequestParts][36]<S> \+ [Send][23], T10: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][45]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][45]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][45]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][45]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][45]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][46]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)][35]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], T4: [FromRequestParts][36]<S> \+ [Send][23], T5: [FromRequestParts][36]<S> \+ [Send][23], T6: [FromRequestParts][36]<S> \+ [Send][23], T7: [FromRequestParts][36]<S> \+ [Send][23], T8: [FromRequestParts][36]<S> \+ [Send][23], T9: [FromRequestParts][36]<S> \+ [Send][23], T10: [FromRequestParts][36]<S> \+ [Send][23], T11: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][46]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][46]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][46]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][46]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][46]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][47]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)][35]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], T4: [FromRequestParts][36]<S> \+ [Send][23], T5: [FromRequestParts][36]<S> \+ [Send][23], T6: [FromRequestParts][36]<S> \+ [Send][23], T7: [FromRequestParts][36]<S> \+ [Send][23], T8: [FromRequestParts][36]<S> \+ [Send][23], T9: [FromRequestParts][36]<S> \+ [Send][23], T10: [FromRequestParts][36]<S> \+ [Send][23], T11: [FromRequestParts][36]<S> \+ [Send][23], T12: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][47]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][47]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][47]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][47]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][47]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][48]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)][35]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], T4: [FromRequestParts][36]<S> \+ [Send][23], T5: [FromRequestParts][36]<S> \+ [Send][23], T6: [FromRequestParts][36]<S> \+ [Send][23], T7: [FromRequestParts][36]<S> \+ [Send][23], T8: [FromRequestParts][36]<S> \+ [Send][23], T9: [FromRequestParts][36]<S> \+ [Send][23], T10: [FromRequestParts][36]<S> \+ [Send][23], T11: [FromRequestParts][36]<S> \+ [Send][23], T12: [FromRequestParts][36]<S> \+ [Send][23], T13: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][48]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][48]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][48]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][48]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][48]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][49]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)][35]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], T4: [FromRequestParts][36]<S> \+ [Send][23], T5: [FromRequestParts][36]<S> \+ [Send][23], T6: [FromRequestParts][36]<S> \+ [Send][23], T7: [FromRequestParts][36]<S> \+ [Send][23], T8: [FromRequestParts][36]<S> \+ [Send][23], T9: [FromRequestParts][36]<S> \+ [Send][23], T10: [FromRequestParts][36]<S> \+ [Send][23], T11: [FromRequestParts][36]<S> \+ [Send][23], T12: [FromRequestParts][36]<S> \+ [Send][23], T13: [FromRequestParts][36]<S> \+ [Send][23], T14: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][49]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][49]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][49]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][49]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][49]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][50]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)][35]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], T4: [FromRequestParts][36]<S> \+ [Send][23], T5: [FromRequestParts][36]<S> \+ [Send][23], T6: [FromRequestParts][36]<S> \+ [Send][23], T7: [FromRequestParts][36]<S> \+ [Send][23], T8: [FromRequestParts][36]<S> \+ [Send][23], T9: [FromRequestParts][36]<S> \+ [Send][23], T10: [FromRequestParts][36]<S> \+ [Send][23], T11: [FromRequestParts][36]<S> \+ [Send][23], T12: [FromRequestParts][36]<S> \+ [Send][23], T13: [FromRequestParts][36]<S> \+ [Send][23], T14: [FromRequestParts][36]<S> \+ [Send][23], T15: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][50]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][50]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][50]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][50]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][50]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][51]§

### impl<F, Fut, S, I, B, ResBody, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> Service<Request<B>> for [MapResponse][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)][35]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, [Response][22]<ResBody>) -> Fut + [Clone][7] \+ [Send][23] \+ 'static, T1: [FromRequestParts][36]<S> \+ [Send][23], T2: [FromRequestParts][36]<S> \+ [Send][23], T3: [FromRequestParts][36]<S> \+ [Send][23], T4: [FromRequestParts][36]<S> \+ [Send][23], T5: [FromRequestParts][36]<S> \+ [Send][23], T6: [FromRequestParts][36]<S> \+ [Send][23], T7: [FromRequestParts][36]<S> \+ [Send][23], T8: [FromRequestParts][36]<S> \+ [Send][23], T9: [FromRequestParts][36]<S> \+ [Send][23], T10: [FromRequestParts][36]<S> \+ [Send][23], T11: [FromRequestParts][36]<S> \+ [Send][23], T12: [FromRequestParts][36]<S> \+ [Send][23], T13: [FromRequestParts][36]<S> \+ [Send][23], T14: [FromRequestParts][36]<S> \+ [Send][23], T15: [FromRequestParts][36]<S> \+ [Send][23], T16: [FromRequestParts][36]<S> \+ [Send][23], Fut: [Future][24] \+ [Send][23] \+ 'static, Fut::[Output][25]: [IntoResponse][26] \+ [Send][23] \+ 'static, I: Service<Request<B>, Response = [Response][22]<ResBody>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][23] \+ 'static, I::Future: [Send][23] \+ 'static, B: [Send][23] \+ 'static, ResBody: [Send][23] \+ 'static, S: [Clone][7] \+ [Send][23] \+ [Sync][28] \+ 'static,

[Source][51]§

#### type Response = Response<[Body][29]>

Responses given by the service.

[Source][51]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][51]§

#### type Future = [ResponseFuture][30]

The future response value.

[Source][51]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][20], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][51]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl<F, S, I, T> [Freeze][52] for [MapResponse][8]<F, S, I, T>

where F: [Freeze][52], I: [Freeze][52], S: [Freeze][52],

§

### impl<F, S, I, T> [RefUnwindSafe][53] for [MapResponse][8]<F, S, I, T>

where F: [RefUnwindSafe][53], I: [RefUnwindSafe][53], S: [RefUnwindSafe][53],

§

### impl<F, S, I, T> [Send][23] for [MapResponse][8]<F, S, I, T>

where F: [Send][23], I: [Send][23], S: [Send][23],

§

### impl<F, S, I, T> [Sync][28] for [MapResponse][8]<F, S, I, T>

where F: [Sync][28], I: [Sync][28], S: [Sync][28],

§

### impl<F, S, I, T> [Unpin][54] for [MapResponse][8]<F, S, I, T>

where F: [Unpin][54], I: [Unpin][54], S: [Unpin][54],

§

### impl<F, S, I, T> [UnwindSafe][55] for [MapResponse][8]<F, S, I, T>

where F: [UnwindSafe][55], I: [UnwindSafe][55], S: [UnwindSafe][55],

## Blanket Implementations§

[Source][56]§

### impl<T> [Any][57] for T

where T: 'static + ?[Sized][58],

[Source][59]§

#### fn [type_id][60](&self) -> [TypeId][61]

Gets the `TypeId` of `self`. [Read more][60]

[Source][62]§

### impl<T> [Borrow][63]<T> for T

where T: ?[Sized][58],

[Source][64]§

#### fn [borrow][65](&self) -> [&T][66]

Immutably borrows from an owned value. [Read more][65]

[Source][67]§

### impl<T> [BorrowMut][68]<T> for T

where T: ?[Sized][58],

[Source][69]§

#### fn [borrow_mut][70](&mut self) -> [&mut T][66]

Mutably borrows from an owned value. [Read more][70]

[Source][71]§

### impl<T> [CloneToUninit][72] for T

where T: [Clone][7],

[Source][73]§

#### unsafe fn [clone_to_uninit][74](&self, dest: [*mut ][75][u8][76])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][74]

[Source][77]§

### impl<T> [From][78]<T> for T

[Source][79]§

#### fn [from][80](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][81]<T> for T

where T: [Clone][7],

§

#### fn [from_ref][82](input: [&T][66]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][83] [`Span`][84], returning an `Instrumented` wrapper. Read more

[Source][85]§

### impl<T, U> [Into][86]<U> for T

where U: [From][78]<T>,

[Source][87]§

#### fn [into][88](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][78]<T> for U` chooses to do.

§

### impl<M, S, Target, Request> MakeService<Target, Request> for M

where M: Service<Target, Response = S>, S: Service<Request>,

§

#### type Response = <S as Service<Request>>::Response

Responses given by the service

§

#### type Error = <S as Service<Request>>::Error

Errors produced by the service

§

#### type Service = S

The [`Service`] value created by this factory

§

#### type MakeError = <M as Service<Target>>::Error

Errors produced while building a service.

§

#### type Future = <M as Service<Target>>::Future

The future of the [`Service`] instance.

§

#### fn poll_ready( &mut self, cx: &mut [Context][31]<'_>, ) -> [Poll][32]<[Result][33]<[()][20], <M as MakeService<Target, Request>>::MakeError>>

Returns [`Poll::Ready`][89] when the factory is able to create more services. Read more

§

#### fn make_service( &mut self, target: Target, ) -> <M as MakeService<Target, Request>>::Future

Create and return a new service value asynchronously.

§

#### fn into_service(self) -> IntoService<Self, Request>

where Self: [Sized][58],

Consume this [`MakeService`] and convert it into a [`Service`]. Read more

§

#### fn as_service(&mut self) -> AsService<'_, Self, Request>

where Self: [Sized][58],

Convert this [`MakeService`] into a [`Service`] without consuming the original [`MakeService`]. Read more

§

### impl<T> PolicyExt for T

where T: ?[Sized][58],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][90]§

### impl<T> [Same][91] for T

[Source][92]§

#### type [Output][93] = T

Should always be `Self`

[Source][94]§

### impl<S, R> [ServiceExt][95]<R> for S

where S: Service<R>,

[Source][96]§

#### fn [into_make_service][97](self) -> [IntoMakeService][98]<S>

Convert this service into a [`MakeService`][99], that is a [`Service`] whose response is another service. [Read more][97]

[Source][100]§

#### fn [into_make_service_with_connect_info][101]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][102]<S, C>

Available on **crate feature`tokio`** only.

Convert this service into a [`MakeService`][99], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][103] can extract it. [Read more][101]

[Source][104]§

#### fn [handle_error][105]<F, T>(self, f: F) -> [HandleError][106]<Self, F, T>

Convert this service into a [`HandleError`][106], that will handle errors by converting them into responses. [Read more][105]

§

### impl<T, Request> ServiceExt<Request> for T

where T: Service<Request> \+ ?[Sized][58],

§

#### fn ready(&mut self) -> Ready<'_, Self, Request>

where Self: [Sized][58],

Yields a mutable reference to the service when it is ready to accept a request.

§

#### fn ready_oneshot(self) -> ReadyOneshot<Self, Request>

where Self: [Sized][58],

Yields the service when it is ready to accept a request.

§

#### fn oneshot(self, req: Request) -> Oneshot<Self, Request>

where Self: [Sized][58],

Consume this `Service`, calling it with the provided request once it is ready.

§

#### fn call_all<S>(self, reqs: S) -> CallAll<Self, S>

where Self: [Sized][58], S: Stream<Item = Request>,

Process all requests from the given [`Stream`][107], and produce a [`Stream`][107] of their responses. Read more

§

#### fn and_then<F>(self, f: F) -> AndThen<Self, F>

where Self: [Sized][58], F: [Clone][7],

Executes a new future after this service’s future resolves. This does not alter the behaviour of the [`poll_ready`][108] method. Read more

§

#### fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>

where Self: [Sized][58], F: [FnOnce][109](Self::Response) -> Response + [Clone][7],

Maps this service’s response value to a different value. This does not alter the behaviour of the [`poll_ready`][108] method. Read more

§

#### fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][58], F: [FnOnce][109](Self::Error) -> Error + [Clone][7],

Maps this service’s error value to a different value. This does not alter the behaviour of the [`poll_ready`][108] method. Read more

§

#### fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>

where Self: [Sized][58], Error: [From][78]<Self::Error>, F: [FnOnce][109]([Result][33]<Self::Response, Self::Error>) -> [Result][33]<Response, Error> \+ [Clone][7],

Maps this service’s result type (`Result<Self::Response, Self::Error>`) to a different value, regardless of whether the future succeeds or fails. Read more

§

#### fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>

where Self: [Sized][58], F: [FnMut][21](NewRequest) -> Request,

Composes a function _in front of_ the service. Read more

§

#### fn filter<F, NewRequest>(self, filter: F) -> Filter<Self, F>

where Self: [Sized][58], F: Predicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with a [`Filter`][110] that conditionally accepts or rejects requests based on a [predicate][111]. Read more

§

#### fn filter_async<F, NewRequest>(self, filter: F) -> AsyncFilter<Self, F>

where Self: [Sized][58], F: AsyncPredicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with an [`AsyncFilter`][112] that conditionally accepts or rejects requests based on an [async predicate]. Read more

§

#### fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>

where Self: [Sized][58], Error: [From][78]<Self::Error>, F: [FnOnce][109]([Result][33]<Self::Response, Self::Error>) -> Fut + [Clone][7], Fut: [Future][24]<Output = [Result][33]<Response, Error>>,

Composes an asynchronous function _after_ this service. Read more

§

#### fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>

where Self: [Sized][58], F: [FnMut][21](Self::Future) -> Fut, Error: [From][78]<Self::Error>, Fut: [Future][24]<Output = [Result][33]<Response, Error>>,

Composes a function that transforms futures produced by the service. Read more

§

#### fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>

where Self: [Sized][58] \+ [Send][23] \+ 'static, Self::Future: [Send][23] \+ 'static,

Convert the service into a [`Service`][113] \+ [`Send`][23] trait object. Read more

§

#### fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>

where Self: [Sized][58] \+ [Clone][7] \+ [Send][23] \+ 'static, Self::Future: [Send][23] \+ 'static,

Convert the service into a [`Service`][113] \+ [`Clone`][7] \+ [`Send`][23] trait object. Read more

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][58],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][58],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][114]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][58],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][58],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][58],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][58],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][58],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][58],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][58],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][115] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][116]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][58],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][117] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][116]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][58],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][117] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][116]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][58],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][117] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][58],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][58],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][58],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][58],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][58],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][58],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][58], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][58], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][58],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][58],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][58],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][118]) -> RequestBodyLimit<Self>

where Self: [Sized][58],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][58],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][58],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][119]§

### impl<T> [ToOwned][120] for T

where T: [Clone][7],

[Source][121]§

#### type [Owned][122] = T

The resulting type after obtaining ownership.

[Source][123]§

#### fn [to_owned][124](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][124]

[Source][125]§

#### fn [clone_into][126](&self, target: [&mut T][66])

Uses borrowed data to replace owned data, usually by cloning. [Read more][126]

[Source][127]§

### impl<T, U> [TryFrom][128]<U> for T

where U: [Into][86]<T>,

[Source][129]§

#### type [Error][130] = [Infallible][27]

The type returned in the event of a conversion error.

[Source][131]§

#### fn [try_from][132](value: U) -> [Result][33]<T, <T as [TryFrom][128]<U>>::[Error][133]>

Performs the conversion.

[Source][134]§

### impl<T, U> [TryInto][135]<U> for T

where U: [TryFrom][128]<T>,

[Source][136]§

#### type [Error][137] = <U as [TryFrom][128]<T>>::[Error][133]

The type returned in the event of a conversion error.

[Source][138]§

#### fn [try_into][139](self) -> [Result][33]<U, <U as [TryFrom][128]<T>>::[Error][133]>

Performs the conversion.

§

### impl<V, T> VZip<V> for T

where V: MultiLane<T>,

§

#### fn vzip(self) -> V

§

### impl<T> WithSubscriber for T

§

#### fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>

where S: [Into][86]<Dispatch>,

Attaches the provided [`Subscriber`][140] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][141] [`Subscriber`][140] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/map_response.rs.html#206-211
   [5]: fn.map_response.html (fn axum::middleware::map_response)
   [6]: ../../src/axum/middleware/map_response.rs.html#213-227
   [7]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [8]: struct.MapResponse.html (struct axum::middleware::MapResponse)
   [9]: ../../src/axum/middleware/map_response.rs.html#219-226
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [11]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [12]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [13]: ../../src/axum/middleware/map_response.rs.html#310-322
   [14]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [15]: ../../src/axum/middleware/map_response.rs.html#315-321
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [17]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [18]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [19]: ../../src/axum/middleware/map_response.rs.html#292
   [20]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [21]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [22]: ../response/type.Response.html (type axum::response::Response)
   [23]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [24]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [25]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output (type core::future::future::Future::Output)
   [26]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [27]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [28]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [29]: ../body/struct.Body.html (struct axum::body::Body)
   [30]: future/struct.MapResponseResponseFuture.html (struct axum::middleware::future::MapResponseResponseFuture)
   [31]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [32]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [33]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [34]: ../../src/axum/middleware/map_response.rs.html#293
   [35]: https://doc.rust-lang.org/nightly/std/primitive.tuple.html
   [36]: ../extract/trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [37]: ../../src/axum/middleware/map_response.rs.html#294
   [38]: ../../src/axum/middleware/map_response.rs.html#295
   [39]: ../../src/axum/middleware/map_response.rs.html#296
   [40]: ../../src/axum/middleware/map_response.rs.html#297
   [41]: ../../src/axum/middleware/map_response.rs.html#298
   [42]: ../../src/axum/middleware/map_response.rs.html#299
   [43]: ../../src/axum/middleware/map_response.rs.html#300
   [44]: ../../src/axum/middleware/map_response.rs.html#301
   [45]: ../../src/axum/middleware/map_response.rs.html#302
   [46]: ../../src/axum/middleware/map_response.rs.html#303
   [47]: ../../src/axum/middleware/map_response.rs.html#304
   [48]: ../../src/axum/middleware/map_response.rs.html#305
   [49]: ../../src/axum/middleware/map_response.rs.html#306
   [50]: ../../src/axum/middleware/map_response.rs.html#307
   [51]: ../../src/axum/middleware/map_response.rs.html#308
   [52]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [53]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [54]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [55]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [56]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [57]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [58]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [59]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [60]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [61]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [62]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [63]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [64]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [65]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [66]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [67]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [68]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [69]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [70]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [71]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [72]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [73]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [74]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [75]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [76]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [77]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [78]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [79]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [80]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [81]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [82]: ../extract/trait.FromRef.html#tymethod.from_ref
   [83]: super::Span::current()
   [84]: crate::Span
   [85]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [86]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [87]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [88]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [89]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Ready (variant core::task::poll::Poll::Ready)
   [90]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [91]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [92]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [93]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [94]: ../../src/axum/service_ext.rs.html#47-59
   [95]: ../trait.ServiceExt.html (trait axum::ServiceExt)
   [96]: ../../src/axum/service_ext.rs.html#51-53
   [97]: ../trait.ServiceExt.html#tymethod.into_make_service
   [98]: ../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [99]: tower::make::MakeService
   [100]: ../../src/axum/service_ext.rs.html#56-58
   [101]: ../trait.ServiceExt.html#tymethod.into_make_service_with_connect_info
   [102]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [103]: ../extract/struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [104]: ../../src/axum/service_ext.rs.html#42-44
   [105]: ../trait.ServiceExt.html#method.handle_error
   [106]: ../error_handling/struct.HandleError.html (struct axum::error_handling::HandleError)
   [107]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
   [108]: crate::Service::poll_ready
   [109]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [110]: crate::filter::Filter
   [111]: crate::filter::Predicate
   [112]: crate::filter::AsyncFilter
   [113]: crate::Service
   [114]: https://docs.rs/http/latest/http/struct.Extensions.html
   [115]: crate::follow_redirect::policy::Standard
   [116]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [117]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [118]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [119]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [120]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [121]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [122]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [123]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [124]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [125]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [126]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [127]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [128]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [129]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [130]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [131]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [132]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [133]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [134]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [135]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [136]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [137]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [138]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [139]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [140]: super::Subscriber
   [141]: dispatcher#setting-the-default-subscriber

