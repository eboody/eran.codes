<!-- Generated from rustdoc HTML: middleware/struct.FromFn.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## FromFn

## [axum][1]0.8.8

## FromFn

### Trait Implementations

  * Clone
  * Debug
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>
  * Service<Request<Body>>



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

# Struct FromFn Copy item path

[Source][4]
``` 
pub struct FromFn<F, S, I, T> { /* private fields */ }
```

Expand description

A middleware created from an async function.

Created with [`from_fn`][5] or [`from_fn_with_state`][6]. See those functions for more details.

## Trait Implementations§

[Source][7]§

### impl<F, S, I, T> [Clone][8] for [FromFn][9]<F, S, I, T>

where F: [Clone][8], I: [Clone][8], S: [Clone][8],

[Source][10]§

#### fn [clone][11](&self) -> Self

Returns a duplicate of the value. [Read more][11]

1.0.0 · [Source][12]§

#### fn [clone_from][13](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][13]

[Source][14]§

### impl<F, S, I, T> [Debug][15] for [FromFn][9]<F, S, I, T>

where S: [Debug][15], I: [Debug][15],

[Source][16]§

#### fn [fmt][17](&self, f: &mut [Formatter][18]<'_>) -> [Result][19]

Formats the value using the given formatter. [Read more][17]

[Source][20]§

### impl<F, Fut, Out, S, I, T1> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1,)][22]>

where F: [FnMut][23](T1, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2)][22]>

where F: [FnMut][23](T1, T2, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3)][22]>

where F: [FnMut][23](T1, T2, T3, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3, T4> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3, T4)][22]>

where F: [FnMut][23](T1, T2, T3, T4, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequestParts][37]<S> \+ [Send][25], T4: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3, T4, T5> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3, T4, T5)][22]>

where F: [FnMut][23](T1, T2, T3, T4, T5, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequestParts][37]<S> \+ [Send][25], T4: [FromRequestParts][37]<S> \+ [Send][25], T5: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3, T4, T5, T6> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3, T4, T5, T6)][22]>

where F: [FnMut][23](T1, T2, T3, T4, T5, T6, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequestParts][37]<S> \+ [Send][25], T4: [FromRequestParts][37]<S> \+ [Send][25], T5: [FromRequestParts][37]<S> \+ [Send][25], T6: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3, T4, T5, T6, T7> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7)][22]>

where F: [FnMut][23](T1, T2, T3, T4, T5, T6, T7, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequestParts][37]<S> \+ [Send][25], T4: [FromRequestParts][37]<S> \+ [Send][25], T5: [FromRequestParts][37]<S> \+ [Send][25], T6: [FromRequestParts][37]<S> \+ [Send][25], T7: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3, T4, T5, T6, T7, T8> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8)][22]>

where F: [FnMut][23](T1, T2, T3, T4, T5, T6, T7, T8, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequestParts][37]<S> \+ [Send][25], T4: [FromRequestParts][37]<S> \+ [Send][25], T5: [FromRequestParts][37]<S> \+ [Send][25], T6: [FromRequestParts][37]<S> \+ [Send][25], T7: [FromRequestParts][37]<S> \+ [Send][25], T8: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3, T4, T5, T6, T7, T8, T9> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9)][22]>

where F: [FnMut][23](T1, T2, T3, T4, T5, T6, T7, T8, T9, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequestParts][37]<S> \+ [Send][25], T4: [FromRequestParts][37]<S> \+ [Send][25], T5: [FromRequestParts][37]<S> \+ [Send][25], T6: [FromRequestParts][37]<S> \+ [Send][25], T7: [FromRequestParts][37]<S> \+ [Send][25], T8: [FromRequestParts][37]<S> \+ [Send][25], T9: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)][22]>

where F: [FnMut][23](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequestParts][37]<S> \+ [Send][25], T4: [FromRequestParts][37]<S> \+ [Send][25], T5: [FromRequestParts][37]<S> \+ [Send][25], T6: [FromRequestParts][37]<S> \+ [Send][25], T7: [FromRequestParts][37]<S> \+ [Send][25], T8: [FromRequestParts][37]<S> \+ [Send][25], T9: [FromRequestParts][37]<S> \+ [Send][25], T10: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)][22]>

where F: [FnMut][23](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequestParts][37]<S> \+ [Send][25], T4: [FromRequestParts][37]<S> \+ [Send][25], T5: [FromRequestParts][37]<S> \+ [Send][25], T6: [FromRequestParts][37]<S> \+ [Send][25], T7: [FromRequestParts][37]<S> \+ [Send][25], T8: [FromRequestParts][37]<S> \+ [Send][25], T9: [FromRequestParts][37]<S> \+ [Send][25], T10: [FromRequestParts][37]<S> \+ [Send][25], T11: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)][22]>

where F: [FnMut][23](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequestParts][37]<S> \+ [Send][25], T4: [FromRequestParts][37]<S> \+ [Send][25], T5: [FromRequestParts][37]<S> \+ [Send][25], T6: [FromRequestParts][37]<S> \+ [Send][25], T7: [FromRequestParts][37]<S> \+ [Send][25], T8: [FromRequestParts][37]<S> \+ [Send][25], T9: [FromRequestParts][37]<S> \+ [Send][25], T10: [FromRequestParts][37]<S> \+ [Send][25], T11: [FromRequestParts][37]<S> \+ [Send][25], T12: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)][22]>

where F: [FnMut][23](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequestParts][37]<S> \+ [Send][25], T4: [FromRequestParts][37]<S> \+ [Send][25], T5: [FromRequestParts][37]<S> \+ [Send][25], T6: [FromRequestParts][37]<S> \+ [Send][25], T7: [FromRequestParts][37]<S> \+ [Send][25], T8: [FromRequestParts][37]<S> \+ [Send][25], T9: [FromRequestParts][37]<S> \+ [Send][25], T10: [FromRequestParts][37]<S> \+ [Send][25], T11: [FromRequestParts][37]<S> \+ [Send][25], T12: [FromRequestParts][37]<S> \+ [Send][25], T13: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)][22]>

where F: [FnMut][23](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequestParts][37]<S> \+ [Send][25], T4: [FromRequestParts][37]<S> \+ [Send][25], T5: [FromRequestParts][37]<S> \+ [Send][25], T6: [FromRequestParts][37]<S> \+ [Send][25], T7: [FromRequestParts][37]<S> \+ [Send][25], T8: [FromRequestParts][37]<S> \+ [Send][25], T9: [FromRequestParts][37]<S> \+ [Send][25], T10: [FromRequestParts][37]<S> \+ [Send][25], T11: [FromRequestParts][37]<S> \+ [Send][25], T12: [FromRequestParts][37]<S> \+ [Send][25], T13: [FromRequestParts][37]<S> \+ [Send][25], T14: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)][22]>

where F: [FnMut][23](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequestParts][37]<S> \+ [Send][25], T4: [FromRequestParts][37]<S> \+ [Send][25], T5: [FromRequestParts][37]<S> \+ [Send][25], T6: [FromRequestParts][37]<S> \+ [Send][25], T7: [FromRequestParts][37]<S> \+ [Send][25], T8: [FromRequestParts][37]<S> \+ [Send][25], T9: [FromRequestParts][37]<S> \+ [Send][25], T10: [FromRequestParts][37]<S> \+ [Send][25], T11: [FromRequestParts][37]<S> \+ [Send][25], T12: [FromRequestParts][37]<S> \+ [Send][25], T13: [FromRequestParts][37]<S> \+ [Send][25], T14: [FromRequestParts][37]<S> \+ [Send][25], T15: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][20]§

### impl<F, Fut, Out, S, I, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> Service<Request<[Body][21]>> for [FromFn][9]<F, S, I, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)][22]>

where F: [FnMut][23](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, [Next][24]) -> Fut + [Clone][8] \+ [Send][25] \+ 'static, T1: [FromRequestParts][37]<S> \+ [Send][25], T2: [FromRequestParts][37]<S> \+ [Send][25], T3: [FromRequestParts][37]<S> \+ [Send][25], T4: [FromRequestParts][37]<S> \+ [Send][25], T5: [FromRequestParts][37]<S> \+ [Send][25], T6: [FromRequestParts][37]<S> \+ [Send][25], T7: [FromRequestParts][37]<S> \+ [Send][25], T8: [FromRequestParts][37]<S> \+ [Send][25], T9: [FromRequestParts][37]<S> \+ [Send][25], T10: [FromRequestParts][37]<S> \+ [Send][25], T11: [FromRequestParts][37]<S> \+ [Send][25], T12: [FromRequestParts][37]<S> \+ [Send][25], T13: [FromRequestParts][37]<S> \+ [Send][25], T14: [FromRequestParts][37]<S> \+ [Send][25], T15: [FromRequestParts][37]<S> \+ [Send][25], T16: [FromRequest][26]<S> \+ [Send][25], Fut: [Future][27]<Output = Out> \+ [Send][25] \+ 'static, Out: [IntoResponse][28] \+ 'static, I: Service<[Request][29], Error = [Infallible][30]> \+ [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static, I::Response: [IntoResponse][28], I::Future: [Send][25] \+ 'static, S: [Clone][8] \+ [Send][25] \+ [Sync][31] \+ 'static,

[Source][20]§

#### type Response = Response<[Body][21]>

Responses given by the service.

[Source][20]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][20]§

#### type Future = [ResponseFuture][32]

The future response value.

[Source][20]§

#### fn poll_ready(&mut self, cx: &mut [Context][33]<'_>) -> [Poll][34]<[Result][35]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][20]§

#### fn call(&mut self, req: [Request][29]) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl<F, S, I, T> [Freeze][38] for [FromFn][9]<F, S, I, T>

where F: [Freeze][38], I: [Freeze][38], S: [Freeze][38],

§

### impl<F, S, I, T> [RefUnwindSafe][39] for [FromFn][9]<F, S, I, T>

where F: [RefUnwindSafe][39], I: [RefUnwindSafe][39], S: [RefUnwindSafe][39],

§

### impl<F, S, I, T> [Send][25] for [FromFn][9]<F, S, I, T>

where F: [Send][25], I: [Send][25], S: [Send][25],

§

### impl<F, S, I, T> [Sync][31] for [FromFn][9]<F, S, I, T>

where F: [Sync][31], I: [Sync][31], S: [Sync][31],

§

### impl<F, S, I, T> [Unpin][40] for [FromFn][9]<F, S, I, T>

where F: [Unpin][40], I: [Unpin][40], S: [Unpin][40],

§

### impl<F, S, I, T> [UnwindSafe][41] for [FromFn][9]<F, S, I, T>

where F: [UnwindSafe][41], I: [UnwindSafe][41], S: [UnwindSafe][41],

## Blanket Implementations§

[Source][42]§

### impl<T> [Any][43] for T

where T: 'static + ?[Sized][44],

[Source][45]§

#### fn [type_id][46](&self) -> [TypeId][47]

Gets the `TypeId` of `self`. [Read more][46]

[Source][48]§

### impl<T> [Borrow][49]<T> for T

where T: ?[Sized][44],

[Source][50]§

#### fn [borrow][51](&self) -> [&T][52]

Immutably borrows from an owned value. [Read more][51]

[Source][53]§

### impl<T> [BorrowMut][54]<T> for T

where T: ?[Sized][44],

[Source][55]§

#### fn [borrow_mut][56](&mut self) -> [&mut T][52]

Mutably borrows from an owned value. [Read more][56]

[Source][57]§

### impl<T> [CloneToUninit][58] for T

where T: [Clone][8],

[Source][59]§

#### unsafe fn [clone_to_uninit][60](&self, dest: [*mut ][61][u8][62])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][60]

[Source][63]§

### impl<T> [From][64]<T> for T

[Source][65]§

#### fn [from][66](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][67]<T> for T

where T: [Clone][8],

§

#### fn [from_ref][68](input: [&T][52]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][69] [`Span`][70], returning an `Instrumented` wrapper. Read more

[Source][71]§

### impl<T, U> [Into][72]<U> for T

where U: [From][64]<T>,

[Source][73]§

#### fn [into][74](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][64]<T> for U` chooses to do.

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

#### fn poll_ready( &mut self, cx: &mut [Context][33]<'_>, ) -> [Poll][34]<[Result][35]<[()][36], <M as MakeService<Target, Request>>::MakeError>>

Returns [`Poll::Ready`][75] when the factory is able to create more services. Read more

§

#### fn make_service( &mut self, target: Target, ) -> <M as MakeService<Target, Request>>::Future

Create and return a new service value asynchronously.

§

#### fn into_service(self) -> IntoService<Self, Request>

where Self: [Sized][44],

Consume this [`MakeService`] and convert it into a [`Service`]. Read more

§

#### fn as_service(&mut self) -> AsService<'_, Self, Request>

where Self: [Sized][44],

Convert this [`MakeService`] into a [`Service`] without consuming the original [`MakeService`]. Read more

§

### impl<T> PolicyExt for T

where T: ?[Sized][44],

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

where T: Service<Request> \+ ?[Sized][44],

§

#### fn ready(&mut self) -> Ready<'_, Self, Request>

where Self: [Sized][44],

Yields a mutable reference to the service when it is ready to accept a request.

§

#### fn ready_oneshot(self) -> ReadyOneshot<Self, Request>

where Self: [Sized][44],

Yields the service when it is ready to accept a request.

§

#### fn oneshot(self, req: Request) -> Oneshot<Self, Request>

where Self: [Sized][44],

Consume this `Service`, calling it with the provided request once it is ready.

§

#### fn call_all<S>(self, reqs: S) -> CallAll<Self, S>

where Self: [Sized][44], S: Stream<Item = Request>,

Process all requests from the given [`Stream`][93], and produce a [`Stream`][93] of their responses. Read more

§

#### fn and_then<F>(self, f: F) -> AndThen<Self, F>

where Self: [Sized][44], F: [Clone][8],

Executes a new future after this service’s future resolves. This does not alter the behaviour of the [`poll_ready`][94] method. Read more

§

#### fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>

where Self: [Sized][44], F: [FnOnce][95](Self::Response) -> Response + [Clone][8],

Maps this service’s response value to a different value. This does not alter the behaviour of the [`poll_ready`][94] method. Read more

§

#### fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][44], F: [FnOnce][95](Self::Error) -> Error + [Clone][8],

Maps this service’s error value to a different value. This does not alter the behaviour of the [`poll_ready`][94] method. Read more

§

#### fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>

where Self: [Sized][44], Error: [From][64]<Self::Error>, F: [FnOnce][95]([Result][35]<Self::Response, Self::Error>) -> [Result][35]<Response, Error> \+ [Clone][8],

Maps this service’s result type (`Result<Self::Response, Self::Error>`) to a different value, regardless of whether the future succeeds or fails. Read more

§

#### fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>

where Self: [Sized][44], F: [FnMut][23](NewRequest) -> Request,

Composes a function _in front of_ the service. Read more

§

#### fn filter<F, NewRequest>(self, filter: F) -> Filter<Self, F>

where Self: [Sized][44], F: Predicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with a [`Filter`][96] that conditionally accepts or rejects requests based on a [predicate][97]. Read more

§

#### fn filter_async<F, NewRequest>(self, filter: F) -> AsyncFilter<Self, F>

where Self: [Sized][44], F: AsyncPredicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with an [`AsyncFilter`][98] that conditionally accepts or rejects requests based on an [async predicate]. Read more

§

#### fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>

where Self: [Sized][44], Error: [From][64]<Self::Error>, F: [FnOnce][95]([Result][35]<Self::Response, Self::Error>) -> Fut + [Clone][8], Fut: [Future][27]<Output = [Result][35]<Response, Error>>,

Composes an asynchronous function _after_ this service. Read more

§

#### fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>

where Self: [Sized][44], F: [FnMut][23](Self::Future) -> Fut, Error: [From][64]<Self::Error>, Fut: [Future][27]<Output = [Result][35]<Response, Error>>,

Composes a function that transforms futures produced by the service. Read more

§

#### fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>

where Self: [Sized][44] \+ [Send][25] \+ 'static, Self::Future: [Send][25] \+ 'static,

Convert the service into a [`Service`][99] \+ [`Send`][25] trait object. Read more

§

#### fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>

where Self: [Sized][44] \+ [Clone][8] \+ [Send][25] \+ 'static, Self::Future: [Send][25] \+ 'static,

Convert the service into a [`Service`][99] \+ [`Clone`][8] \+ [`Send`][25] trait object. Read more

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][44],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][44],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][100]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][44],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][44],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][44],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][44],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][44],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][44],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][44],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][101] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][102]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][103] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][102]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][103] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][102]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][103] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][44], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][44], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][44],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][44],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][44],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][104]) -> RequestBodyLimit<Self>

where Self: [Sized][44],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][44],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][44],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][105]§

### impl<T> [ToOwned][106] for T

where T: [Clone][8],

[Source][107]§

#### type [Owned][108] = T

The resulting type after obtaining ownership.

[Source][109]§

#### fn [to_owned][110](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][110]

[Source][111]§

#### fn [clone_into][112](&self, target: [&mut T][52])

Uses borrowed data to replace owned data, usually by cloning. [Read more][112]

[Source][113]§

### impl<T, U> [TryFrom][114]<U> for T

where U: [Into][72]<T>,

[Source][115]§

#### type [Error][116] = [Infallible][30]

The type returned in the event of a conversion error.

[Source][117]§

#### fn [try_from][118](value: U) -> [Result][35]<T, <T as [TryFrom][114]<U>>::[Error][119]>

Performs the conversion.

[Source][120]§

### impl<T, U> [TryInto][121]<U> for T

where U: [TryFrom][114]<T>,

[Source][122]§

#### type [Error][123] = <U as [TryFrom][114]<T>>::[Error][119]

The type returned in the event of a conversion error.

[Source][124]§

#### fn [try_into][125](self) -> [Result][35]<U, <U as [TryFrom][114]<T>>::[Error][119]>

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

where S: [Into][72]<Dispatch>,

Attaches the provided [`Subscriber`][126] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][127] [`Subscriber`][126] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/from_fn.rs.html#231-236
   [5]: fn.from_fn.html (fn axum::middleware::from_fn)
   [6]: fn.from_fn_with_state.html (fn axum::middleware::from_fn_with_state)
   [7]: ../../src/axum/middleware/from_fn.rs.html#238-252
   [8]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [9]: struct.FromFn.html (struct axum::middleware::FromFn)
   [10]: ../../src/axum/middleware/from_fn.rs.html#244-251
   [11]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [12]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [13]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [14]: ../../src/axum/middleware/from_fn.rs.html#322-334
   [15]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [16]: ../../src/axum/middleware/from_fn.rs.html#327-333
   [17]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [18]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [19]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [20]: ../../src/axum/middleware/from_fn.rs.html#320
   [21]: ../body/struct.Body.html (struct axum::body::Body)
   [22]: https://doc.rust-lang.org/nightly/std/primitive.tuple.html
   [23]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [24]: struct.Next.html (struct axum::middleware::Next)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [26]: ../extract/trait.FromRequest.html (trait axum::extract::FromRequest)
   [27]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [28]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [29]: ../extract/type.Request.html (type axum::extract::Request)
   [30]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [31]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [32]: future/struct.FromFnResponseFuture.html (struct axum::middleware::future::FromFnResponseFuture)
   [33]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [34]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [35]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [36]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [37]: ../extract/trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [39]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [40]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [41]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [42]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [43]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [44]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [45]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [46]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [47]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [48]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [49]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [50]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [51]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [52]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [53]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [54]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [55]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [56]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [57]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [58]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [59]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [60]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [61]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [62]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [64]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [65]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [66]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [67]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [68]: ../extract/trait.FromRef.html#tymethod.from_ref
   [69]: super::Span::current()
   [70]: crate::Span
   [71]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [72]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
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

