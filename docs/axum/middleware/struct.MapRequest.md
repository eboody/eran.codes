<!-- Generated from rustdoc HTML: middleware/struct.MapRequest.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## MapRequest

## [axum][1]0.8.8

## MapRequest

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

# Struct MapRequest Copy item path

[Source][4]
``` 
pub struct MapRequest<F, S, I, T> { /* private fields */ }
```

Expand description

A middleware created from an async function that transforms a request.

Created with [`map_request`][5]. See that function for more details.

## Trait Implementations§

[Source][6]§

### impl<F, S, I, T> [Clone][7] for [MapRequest][8]<F, S, I, T>

where F: [Clone][7], I: [Clone][7], S: [Clone][7],

[Source][9]§

#### fn [clone][10](&self) -> Self

Returns a duplicate of the value. [Read more][10]

1.0.0 · [Source][11]§

#### fn [clone_from][12](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][12]

[Source][13]§

### impl<F, S, I, T> [Debug][14] for [MapRequest][8]<F, S, I, T>

where S: [Debug][14], I: [Debug][14],

[Source][15]§

#### fn [fmt][16](&self, f: &mut [Formatter][17]<'_>) -> [Result][18]

Formats the value using the given formatter. [Read more][16]

[Source][19]§

### impl<F, Fut, S, I, B, T1> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1,)][20]>

where F: [FnMut][21](T1) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2)][20]>

where F: [FnMut][21](T1, T2) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3)][20]>

where F: [FnMut][21](T1, T2, T3) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3, T4> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3, T4)][20]>

where F: [FnMut][21](T1, T2, T3, T4) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequestParts][38]<S> \+ [Send][22], T4: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3, T4, T5> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3, T4, T5)][20]>

where F: [FnMut][21](T1, T2, T3, T4, T5) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequestParts][38]<S> \+ [Send][22], T4: [FromRequestParts][38]<S> \+ [Send][22], T5: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3, T4, T5, T6> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3, T4, T5, T6)][20]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequestParts][38]<S> \+ [Send][22], T4: [FromRequestParts][38]<S> \+ [Send][22], T5: [FromRequestParts][38]<S> \+ [Send][22], T6: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3, T4, T5, T6, T7> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7)][20]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequestParts][38]<S> \+ [Send][22], T4: [FromRequestParts][38]<S> \+ [Send][22], T5: [FromRequestParts][38]<S> \+ [Send][22], T6: [FromRequestParts][38]<S> \+ [Send][22], T7: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3, T4, T5, T6, T7, T8> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8)][20]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequestParts][38]<S> \+ [Send][22], T4: [FromRequestParts][38]<S> \+ [Send][22], T5: [FromRequestParts][38]<S> \+ [Send][22], T6: [FromRequestParts][38]<S> \+ [Send][22], T7: [FromRequestParts][38]<S> \+ [Send][22], T8: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3, T4, T5, T6, T7, T8, T9> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9)][20]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequestParts][38]<S> \+ [Send][22], T4: [FromRequestParts][38]<S> \+ [Send][22], T5: [FromRequestParts][38]<S> \+ [Send][22], T6: [FromRequestParts][38]<S> \+ [Send][22], T7: [FromRequestParts][38]<S> \+ [Send][22], T8: [FromRequestParts][38]<S> \+ [Send][22], T9: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)][20]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequestParts][38]<S> \+ [Send][22], T4: [FromRequestParts][38]<S> \+ [Send][22], T5: [FromRequestParts][38]<S> \+ [Send][22], T6: [FromRequestParts][38]<S> \+ [Send][22], T7: [FromRequestParts][38]<S> \+ [Send][22], T8: [FromRequestParts][38]<S> \+ [Send][22], T9: [FromRequestParts][38]<S> \+ [Send][22], T10: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)][20]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequestParts][38]<S> \+ [Send][22], T4: [FromRequestParts][38]<S> \+ [Send][22], T5: [FromRequestParts][38]<S> \+ [Send][22], T6: [FromRequestParts][38]<S> \+ [Send][22], T7: [FromRequestParts][38]<S> \+ [Send][22], T8: [FromRequestParts][38]<S> \+ [Send][22], T9: [FromRequestParts][38]<S> \+ [Send][22], T10: [FromRequestParts][38]<S> \+ [Send][22], T11: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)][20]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequestParts][38]<S> \+ [Send][22], T4: [FromRequestParts][38]<S> \+ [Send][22], T5: [FromRequestParts][38]<S> \+ [Send][22], T6: [FromRequestParts][38]<S> \+ [Send][22], T7: [FromRequestParts][38]<S> \+ [Send][22], T8: [FromRequestParts][38]<S> \+ [Send][22], T9: [FromRequestParts][38]<S> \+ [Send][22], T10: [FromRequestParts][38]<S> \+ [Send][22], T11: [FromRequestParts][38]<S> \+ [Send][22], T12: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)][20]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequestParts][38]<S> \+ [Send][22], T4: [FromRequestParts][38]<S> \+ [Send][22], T5: [FromRequestParts][38]<S> \+ [Send][22], T6: [FromRequestParts][38]<S> \+ [Send][22], T7: [FromRequestParts][38]<S> \+ [Send][22], T8: [FromRequestParts][38]<S> \+ [Send][22], T9: [FromRequestParts][38]<S> \+ [Send][22], T10: [FromRequestParts][38]<S> \+ [Send][22], T11: [FromRequestParts][38]<S> \+ [Send][22], T12: [FromRequestParts][38]<S> \+ [Send][22], T13: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)][20]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequestParts][38]<S> \+ [Send][22], T4: [FromRequestParts][38]<S> \+ [Send][22], T5: [FromRequestParts][38]<S> \+ [Send][22], T6: [FromRequestParts][38]<S> \+ [Send][22], T7: [FromRequestParts][38]<S> \+ [Send][22], T8: [FromRequestParts][38]<S> \+ [Send][22], T9: [FromRequestParts][38]<S> \+ [Send][22], T10: [FromRequestParts][38]<S> \+ [Send][22], T11: [FromRequestParts][38]<S> \+ [Send][22], T12: [FromRequestParts][38]<S> \+ [Send][22], T13: [FromRequestParts][38]<S> \+ [Send][22], T14: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)][20]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequestParts][38]<S> \+ [Send][22], T4: [FromRequestParts][38]<S> \+ [Send][22], T5: [FromRequestParts][38]<S> \+ [Send][22], T6: [FromRequestParts][38]<S> \+ [Send][22], T7: [FromRequestParts][38]<S> \+ [Send][22], T8: [FromRequestParts][38]<S> \+ [Send][22], T9: [FromRequestParts][38]<S> \+ [Send][22], T10: [FromRequestParts][38]<S> \+ [Send][22], T11: [FromRequestParts][38]<S> \+ [Send][22], T12: [FromRequestParts][38]<S> \+ [Send][22], T13: [FromRequestParts][38]<S> \+ [Send][22], T14: [FromRequestParts][38]<S> \+ [Send][22], T15: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][19]§

### impl<F, Fut, S, I, B, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> Service<Request<B>> for [MapRequest][8]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)][20]>

where F: [FnMut][21](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> Fut + [Clone][7] \+ [Send][22] \+ 'static, T1: [FromRequestParts][38]<S> \+ [Send][22], T2: [FromRequestParts][38]<S> \+ [Send][22], T3: [FromRequestParts][38]<S> \+ [Send][22], T4: [FromRequestParts][38]<S> \+ [Send][22], T5: [FromRequestParts][38]<S> \+ [Send][22], T6: [FromRequestParts][38]<S> \+ [Send][22], T7: [FromRequestParts][38]<S> \+ [Send][22], T8: [FromRequestParts][38]<S> \+ [Send][22], T9: [FromRequestParts][38]<S> \+ [Send][22], T10: [FromRequestParts][38]<S> \+ [Send][22], T11: [FromRequestParts][38]<S> \+ [Send][22], T12: [FromRequestParts][38]<S> \+ [Send][22], T13: [FromRequestParts][38]<S> \+ [Send][22], T14: [FromRequestParts][38]<S> \+ [Send][22], T15: [FromRequestParts][38]<S> \+ [Send][22], T16: [FromRequest][23]<S> \+ [Send][22], Fut: [Future][24] \+ [Send][22] \+ 'static, Fut::[Output][25]: [IntoMapRequestResult][26]<B> \+ [Send][22] \+ 'static, I: Service<Request<B>, Error = [Infallible][27]> \+ [Clone][7] \+ [Send][22] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][22] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][29]<[BoxError][30]>, S: [Clone][7] \+ [Send][22] \+ [Sync][31] \+ 'static,

[Source][19]§

#### type Response = Response<[Body][32]>

Responses given by the service.

[Source][19]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][19]§

#### type Future = [ResponseFuture][33]

The future response value.

[Source][19]§

#### fn poll_ready(&mut self, cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][37], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][19]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl<F, S, I, T> [Freeze][39] for [MapRequest][8]<F, S, I, T>

where F: [Freeze][39], I: [Freeze][39], S: [Freeze][39],

§

### impl<F, S, I, T> [RefUnwindSafe][40] for [MapRequest][8]<F, S, I, T>

where F: [RefUnwindSafe][40], I: [RefUnwindSafe][40], S: [RefUnwindSafe][40],

§

### impl<F, S, I, T> [Send][22] for [MapRequest][8]<F, S, I, T>

where F: [Send][22], I: [Send][22], S: [Send][22],

§

### impl<F, S, I, T> [Sync][31] for [MapRequest][8]<F, S, I, T>

where F: [Sync][31], I: [Sync][31], S: [Sync][31],

§

### impl<F, S, I, T> [Unpin][41] for [MapRequest][8]<F, S, I, T>

where F: [Unpin][41], I: [Unpin][41], S: [Unpin][41],

§

### impl<F, S, I, T> [UnwindSafe][42] for [MapRequest][8]<F, S, I, T>

where F: [UnwindSafe][42], I: [UnwindSafe][42], S: [UnwindSafe][42],

## Blanket Implementations§

[Source][43]§

### impl<T> [Any][44] for T

where T: 'static + ?[Sized][45],

[Source][46]§

#### fn [type_id][47](&self) -> [TypeId][48]

Gets the `TypeId` of `self`. [Read more][47]

[Source][49]§

### impl<T> [Borrow][50]<T> for T

where T: ?[Sized][45],

[Source][51]§

#### fn [borrow][52](&self) -> [&T][53]

Immutably borrows from an owned value. [Read more][52]

[Source][54]§

### impl<T> [BorrowMut][55]<T> for T

where T: ?[Sized][45],

[Source][56]§

#### fn [borrow_mut][57](&mut self) -> [&mut T][53]

Mutably borrows from an owned value. [Read more][57]

[Source][58]§

### impl<T> [CloneToUninit][59] for T

where T: [Clone][7],

[Source][60]§

#### unsafe fn [clone_to_uninit][61](&self, dest: [*mut ][62][u8][63])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][61]

[Source][64]§

### impl<T> [From][65]<T> for T

[Source][66]§

#### fn [from][67](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][68]<T> for T

where T: [Clone][7],

§

#### fn [from_ref][69](input: [&T][53]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][70] [`Span`][71], returning an `Instrumented` wrapper. Read more

[Source][72]§

### impl<T, U> [Into][29]<U> for T

where U: [From][65]<T>,

[Source][73]§

#### fn [into][74](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][65]<T> for U` chooses to do.

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

#### fn poll_ready( &mut self, cx: &mut [Context][34]<'_>, ) -> [Poll][35]<[Result][36]<[()][37], <M as MakeService<Target, Request>>::MakeError>>

Returns [`Poll::Ready`][75] when the factory is able to create more services. Read more

§

#### fn make_service( &mut self, target: Target, ) -> <M as MakeService<Target, Request>>::Future

Create and return a new service value asynchronously.

§

#### fn into_service(self) -> IntoService<Self, Request>

where Self: [Sized][45],

Consume this [`MakeService`] and convert it into a [`Service`]. Read more

§

#### fn as_service(&mut self) -> AsService<'_, Self, Request>

where Self: [Sized][45],

Convert this [`MakeService`] into a [`Service`] without consuming the original [`MakeService`]. Read more

§

### impl<T> PolicyExt for T

where T: ?[Sized][45],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][76]§

### impl<T> [Same][77] for T

[Source][78]§

#### type [Output][79] = T

Should always be `Self`

[Source][80]§

### impl<S, R> [ServiceExt][81]<R> for S

where S: Service<R>,

[Source][82]§

#### fn [into_make_service][83](self) -> [IntoMakeService][84]<S>

Convert this service into a [`MakeService`][85], that is a [`Service`] whose response is another service. [Read more][83]

[Source][86]§

#### fn [into_make_service_with_connect_info][87]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][88]<S, C>

Available on **crate feature`tokio`** only.

Convert this service into a [`MakeService`][85], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][89] can extract it. [Read more][87]

[Source][90]§

#### fn [handle_error][91]<F, T>(self, f: F) -> [HandleError][92]<Self, F, T>

Convert this service into a [`HandleError`][92], that will handle errors by converting them into responses. [Read more][91]

§

### impl<T, Request> ServiceExt<Request> for T

where T: Service<Request> \+ ?[Sized][45],

§

#### fn ready(&mut self) -> Ready<'_, Self, Request>

where Self: [Sized][45],

Yields a mutable reference to the service when it is ready to accept a request.

§

#### fn ready_oneshot(self) -> ReadyOneshot<Self, Request>

where Self: [Sized][45],

Yields the service when it is ready to accept a request.

§

#### fn oneshot(self, req: Request) -> Oneshot<Self, Request>

where Self: [Sized][45],

Consume this `Service`, calling it with the provided request once it is ready.

§

#### fn call_all<S>(self, reqs: S) -> CallAll<Self, S>

where Self: [Sized][45], S: Stream<Item = Request>,

Process all requests from the given [`Stream`][93], and produce a [`Stream`][93] of their responses. Read more

§

#### fn and_then<F>(self, f: F) -> AndThen<Self, F>

where Self: [Sized][45], F: [Clone][7],

Executes a new future after this service’s future resolves. This does not alter the behaviour of the [`poll_ready`][94] method. Read more

§

#### fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>

where Self: [Sized][45], F: [FnOnce][95](Self::Response) -> Response + [Clone][7],

Maps this service’s response value to a different value. This does not alter the behaviour of the [`poll_ready`][94] method. Read more

§

#### fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][45], F: [FnOnce][95](Self::Error) -> Error + [Clone][7],

Maps this service’s error value to a different value. This does not alter the behaviour of the [`poll_ready`][94] method. Read more

§

#### fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>

where Self: [Sized][45], Error: [From][65]<Self::Error>, F: [FnOnce][95]([Result][36]<Self::Response, Self::Error>) -> [Result][36]<Response, Error> \+ [Clone][7],

Maps this service’s result type (`Result<Self::Response, Self::Error>`) to a different value, regardless of whether the future succeeds or fails. Read more

§

#### fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>

where Self: [Sized][45], F: [FnMut][21](NewRequest) -> Request,

Composes a function _in front of_ the service. Read more

§

#### fn filter<F, NewRequest>(self, filter: F) -> Filter<Self, F>

where Self: [Sized][45], F: Predicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with a [`Filter`][96] that conditionally accepts or rejects requests based on a [predicate][97]. Read more

§

#### fn filter_async<F, NewRequest>(self, filter: F) -> AsyncFilter<Self, F>

where Self: [Sized][45], F: AsyncPredicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with an [`AsyncFilter`][98] that conditionally accepts or rejects requests based on an [async predicate]. Read more

§

#### fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>

where Self: [Sized][45], Error: [From][65]<Self::Error>, F: [FnOnce][95]([Result][36]<Self::Response, Self::Error>) -> Fut + [Clone][7], Fut: [Future][24]<Output = [Result][36]<Response, Error>>,

Composes an asynchronous function _after_ this service. Read more

§

#### fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>

where Self: [Sized][45], F: [FnMut][21](Self::Future) -> Fut, Error: [From][65]<Self::Error>, Fut: [Future][24]<Output = [Result][36]<Response, Error>>,

Composes a function that transforms futures produced by the service. Read more

§

#### fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>

where Self: [Sized][45] \+ [Send][22] \+ 'static, Self::Future: [Send][22] \+ 'static,

Convert the service into a [`Service`][99] \+ [`Send`][22] trait object. Read more

§

#### fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>

where Self: [Sized][45] \+ [Clone][7] \+ [Send][22] \+ 'static, Self::Future: [Send][22] \+ 'static,

Convert the service into a [`Service`][99] \+ [`Clone`][7] \+ [`Send`][22] trait object. Read more

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][45],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][45],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][100]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][45],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][45],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][45],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][45],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][45],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][45],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][45],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][101] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][102]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][45],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][103] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][102]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][45],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][103] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][102]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][45],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][103] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][45], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][45], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][45],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][45],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][45],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][104]) -> RequestBodyLimit<Self>

where Self: [Sized][45],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][45],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][45],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][105]§

### impl<T> [ToOwned][106] for T

where T: [Clone][7],

[Source][107]§

#### type [Owned][108] = T

The resulting type after obtaining ownership.

[Source][109]§

#### fn [to_owned][110](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][110]

[Source][111]§

#### fn [clone_into][112](&self, target: [&mut T][53])

Uses borrowed data to replace owned data, usually by cloning. [Read more][112]

[Source][113]§

### impl<T, U> [TryFrom][114]<U> for T

where U: [Into][29]<T>,

[Source][115]§

#### type [Error][116] = [Infallible][27]

The type returned in the event of a conversion error.

[Source][117]§

#### fn [try_from][118](value: U) -> [Result][36]<T, <T as [TryFrom][114]<U>>::[Error][119]>

Performs the conversion.

[Source][120]§

### impl<T, U> [TryInto][121]<U> for T

where U: [TryFrom][114]<T>,

[Source][122]§

#### type [Error][123] = <U as [TryFrom][114]<T>>::[Error][119]

The type returned in the event of a conversion error.

[Source][124]§

#### fn [try_into][125](self) -> [Result][36]<U, <U as [TryFrom][114]<T>>::[Error][119]>

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

where S: [Into][29]<Dispatch>,

Attaches the provided [`Subscriber`][126] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][127] [`Subscriber`][126] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/map_request.rs.html#224-229
   [5]: fn.map_request.html (fn axum::middleware::map_request)
   [6]: ../../src/axum/middleware/map_request.rs.html#231-245
   [7]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [8]: struct.MapRequest.html (struct axum::middleware::MapRequest)
   [9]: ../../src/axum/middleware/map_request.rs.html#237-244
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [11]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [12]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [13]: ../../src/axum/middleware/map_request.rs.html#322-334
   [14]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [15]: ../../src/axum/middleware/map_request.rs.html#327-333
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [17]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [18]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [19]: ../../src/axum/middleware/map_request.rs.html#320
   [20]: https://doc.rust-lang.org/nightly/std/primitive.tuple.html
   [21]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [22]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [23]: ../extract/trait.FromRequest.html (trait axum::extract::FromRequest)
   [24]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [25]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output (type core::future::future::Future::Output)
   [26]: trait.IntoMapRequestResult.html (trait axum::middleware::IntoMapRequestResult)
   [27]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [28]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [29]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [30]: ../type.BoxError.html (type axum::BoxError)
   [31]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [32]: ../body/struct.Body.html (struct axum::body::Body)
   [33]: future/struct.MapRequestResponseFuture.html (struct axum::middleware::future::MapRequestResponseFuture)
   [34]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [35]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [36]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [37]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [38]: ../extract/trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [40]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [41]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [42]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [43]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [44]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [45]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [46]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [47]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [48]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [49]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [50]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [51]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [52]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [53]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [54]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [55]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [56]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [57]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [58]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [59]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [60]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [61]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [62]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [63]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [64]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [65]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [66]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [67]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [68]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [69]: ../extract/trait.FromRef.html#tymethod.from_ref
   [70]: super::Span::current()
   [71]: crate::Span
   [72]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [73]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [74]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [75]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Ready (variant core::task::poll::Poll::Ready)
   [76]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [77]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [78]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [79]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [80]: ../../src/axum/service_ext.rs.html#47-59
   [81]: ../trait.ServiceExt.html (trait axum::ServiceExt)
   [82]: ../../src/axum/service_ext.rs.html#51-53
   [83]: ../trait.ServiceExt.html#tymethod.into_make_service
   [84]: ../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [85]: tower::make::MakeService
   [86]: ../../src/axum/service_ext.rs.html#56-58
   [87]: ../trait.ServiceExt.html#tymethod.into_make_service_with_connect_info
   [88]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [89]: ../extract/struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [90]: ../../src/axum/service_ext.rs.html#42-44
   [91]: ../trait.ServiceExt.html#method.handle_error
   [92]: ../error_handling/struct.HandleError.html (struct axum::error_handling::HandleError)
   [93]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
   [94]: crate::Service::poll_ready
   [95]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [96]: crate::filter::Filter
   [97]: crate::filter::Predicate
   [98]: crate::filter::AsyncFilter
   [99]: crate::Service
   [100]: https://docs.rs/http/latest/http/struct.Extensions.html
   [101]: crate::follow_redirect::policy::Standard
   [102]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [103]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [104]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [105]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [106]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [107]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [108]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [109]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [110]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [111]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [112]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [113]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [114]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [115]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [116]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [117]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [118]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [119]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [120]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [121]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [122]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [123]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [124]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [125]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [126]: super::Subscriber
   [127]: dispatcher#setting-the-default-subscriber

