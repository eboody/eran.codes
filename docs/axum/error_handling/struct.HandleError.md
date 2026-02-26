<!-- Generated from rustdoc HTML: error_handling/struct.HandleError.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## HandleError

## [axum][1]0.8.8

## HandleError

### Methods

  * new



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



## [In axum::error_handling][2]

[axum][3]::[error_handling][2]

# Struct HandleError Copy item path

[Source][4]
``` 
pub struct HandleError<S, F, T> { /* private fields */ }
```

Expand description

A [`Service`] adapter that handles errors by converting them into responses.

See [module docs][5] for more details on axum’s error handling model.

## Implementations§

[Source][6]§

### impl<S, F, T> [HandleError][7]<S, F, T>

[Source][8]

#### pub fn new(inner: S, f: F) -> Self

Create a new `HandleError`.

## Trait Implementations§

[Source][9]§

### impl<S, F, T> [Clone][10] for [HandleError][7]<S, F, T>

where S: [Clone][10], F: [Clone][10],

[Source][11]§

#### fn [clone][12](&self) -> Self

Returns a duplicate of the value. [Read more][12]

1.0.0 · [Source][13]§

#### fn [clone_from][14](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][14]

[Source][15]§

### impl<S, F, E> [Debug][16] for [HandleError][7]<S, F, E>

where S: [Debug][16],

[Source][17]§

#### fn [fmt][18](&self, f: &mut [Formatter][19]<'_>) -> [Result][20]

Formats the value using the given formatter. [Read more][18]

[Source][21]§

### impl<S, F, B, Fut, Res> Service<Request<B>> for [HandleError][7]<S, F, [()][22]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], B: [Send][23] \+ 'static,

[Source][27]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][29]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][31]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][33]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][37]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][38]§

### impl<S, F, B, Res, Fut, T1> Service<Request<B>> for [HandleError][7]<S, F, [(T1,)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][38]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][38]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][38]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][38]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][38]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][41]§

### impl<S, F, B, Res, Fut, T1, T2> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][41]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][41]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][41]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][41]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][41]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][42]§

### impl<S, F, B, Res, Fut, T1, T2, T3> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][42]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][42]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][42]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][42]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][42]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][43]§

### impl<S, F, B, Res, Fut, T1, T2, T3, T4> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3, T4)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, T4, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], T4: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][43]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][43]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][43]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][43]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][43]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][44]§

### impl<S, F, B, Res, Fut, T1, T2, T3, T4, T5> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3, T4, T5)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, T4, T5, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], T4: [FromRequestParts][40]<[()][22]> \+ [Send][23], T5: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][44]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][44]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][44]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][44]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][44]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][45]§

### impl<S, F, B, Res, Fut, T1, T2, T3, T4, T5, T6> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3, T4, T5, T6)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, T4, T5, T6, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], T4: [FromRequestParts][40]<[()][22]> \+ [Send][23], T5: [FromRequestParts][40]<[()][22]> \+ [Send][23], T6: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][45]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][45]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][45]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][45]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][45]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][46]§

### impl<S, F, B, Res, Fut, T1, T2, T3, T4, T5, T6, T7> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3, T4, T5, T6, T7)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, T4, T5, T6, T7, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], T4: [FromRequestParts][40]<[()][22]> \+ [Send][23], T5: [FromRequestParts][40]<[()][22]> \+ [Send][23], T6: [FromRequestParts][40]<[()][22]> \+ [Send][23], T7: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][46]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][46]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][46]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][46]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][46]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][47]§

### impl<S, F, B, Res, Fut, T1, T2, T3, T4, T5, T6, T7, T8> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3, T4, T5, T6, T7, T8)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, T4, T5, T6, T7, T8, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], T4: [FromRequestParts][40]<[()][22]> \+ [Send][23], T5: [FromRequestParts][40]<[()][22]> \+ [Send][23], T6: [FromRequestParts][40]<[()][22]> \+ [Send][23], T7: [FromRequestParts][40]<[()][22]> \+ [Send][23], T8: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][47]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][47]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][47]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][47]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][47]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][48]§

### impl<S, F, B, Res, Fut, T1, T2, T3, T4, T5, T6, T7, T8, T9> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3, T4, T5, T6, T7, T8, T9)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, T4, T5, T6, T7, T8, T9, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], T4: [FromRequestParts][40]<[()][22]> \+ [Send][23], T5: [FromRequestParts][40]<[()][22]> \+ [Send][23], T6: [FromRequestParts][40]<[()][22]> \+ [Send][23], T7: [FromRequestParts][40]<[()][22]> \+ [Send][23], T8: [FromRequestParts][40]<[()][22]> \+ [Send][23], T9: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][48]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][48]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][48]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][48]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][48]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][49]§

### impl<S, F, B, Res, Fut, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], T4: [FromRequestParts][40]<[()][22]> \+ [Send][23], T5: [FromRequestParts][40]<[()][22]> \+ [Send][23], T6: [FromRequestParts][40]<[()][22]> \+ [Send][23], T7: [FromRequestParts][40]<[()][22]> \+ [Send][23], T8: [FromRequestParts][40]<[()][22]> \+ [Send][23], T9: [FromRequestParts][40]<[()][22]> \+ [Send][23], T10: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][49]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][49]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][49]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][49]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][49]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][50]§

### impl<S, F, B, Res, Fut, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], T4: [FromRequestParts][40]<[()][22]> \+ [Send][23], T5: [FromRequestParts][40]<[()][22]> \+ [Send][23], T6: [FromRequestParts][40]<[()][22]> \+ [Send][23], T7: [FromRequestParts][40]<[()][22]> \+ [Send][23], T8: [FromRequestParts][40]<[()][22]> \+ [Send][23], T9: [FromRequestParts][40]<[()][22]> \+ [Send][23], T10: [FromRequestParts][40]<[()][22]> \+ [Send][23], T11: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][50]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][50]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][50]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][50]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][50]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][51]§

### impl<S, F, B, Res, Fut, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], T4: [FromRequestParts][40]<[()][22]> \+ [Send][23], T5: [FromRequestParts][40]<[()][22]> \+ [Send][23], T6: [FromRequestParts][40]<[()][22]> \+ [Send][23], T7: [FromRequestParts][40]<[()][22]> \+ [Send][23], T8: [FromRequestParts][40]<[()][22]> \+ [Send][23], T9: [FromRequestParts][40]<[()][22]> \+ [Send][23], T10: [FromRequestParts][40]<[()][22]> \+ [Send][23], T11: [FromRequestParts][40]<[()][22]> \+ [Send][23], T12: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][51]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][51]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][51]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][51]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][51]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][52]§

### impl<S, F, B, Res, Fut, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], T4: [FromRequestParts][40]<[()][22]> \+ [Send][23], T5: [FromRequestParts][40]<[()][22]> \+ [Send][23], T6: [FromRequestParts][40]<[()][22]> \+ [Send][23], T7: [FromRequestParts][40]<[()][22]> \+ [Send][23], T8: [FromRequestParts][40]<[()][22]> \+ [Send][23], T9: [FromRequestParts][40]<[()][22]> \+ [Send][23], T10: [FromRequestParts][40]<[()][22]> \+ [Send][23], T11: [FromRequestParts][40]<[()][22]> \+ [Send][23], T12: [FromRequestParts][40]<[()][22]> \+ [Send][23], T13: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][52]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][52]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][52]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][52]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][52]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][53]§

### impl<S, F, B, Res, Fut, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], T4: [FromRequestParts][40]<[()][22]> \+ [Send][23], T5: [FromRequestParts][40]<[()][22]> \+ [Send][23], T6: [FromRequestParts][40]<[()][22]> \+ [Send][23], T7: [FromRequestParts][40]<[()][22]> \+ [Send][23], T8: [FromRequestParts][40]<[()][22]> \+ [Send][23], T9: [FromRequestParts][40]<[()][22]> \+ [Send][23], T10: [FromRequestParts][40]<[()][22]> \+ [Send][23], T11: [FromRequestParts][40]<[()][22]> \+ [Send][23], T12: [FromRequestParts][40]<[()][22]> \+ [Send][23], T13: [FromRequestParts][40]<[()][22]> \+ [Send][23], T14: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][53]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][53]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][53]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][53]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][53]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][54]§

### impl<S, F, B, Res, Fut, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], T4: [FromRequestParts][40]<[()][22]> \+ [Send][23], T5: [FromRequestParts][40]<[()][22]> \+ [Send][23], T6: [FromRequestParts][40]<[()][22]> \+ [Send][23], T7: [FromRequestParts][40]<[()][22]> \+ [Send][23], T8: [FromRequestParts][40]<[()][22]> \+ [Send][23], T9: [FromRequestParts][40]<[()][22]> \+ [Send][23], T10: [FromRequestParts][40]<[()][22]> \+ [Send][23], T11: [FromRequestParts][40]<[()][22]> \+ [Send][23], T12: [FromRequestParts][40]<[()][22]> \+ [Send][23], T13: [FromRequestParts][40]<[()][22]> \+ [Send][23], T14: [FromRequestParts][40]<[()][22]> \+ [Send][23], T15: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][54]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][54]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][54]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][54]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][54]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][55]§

### impl<S, F, B, Res, Fut, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> Service<Request<B>> for [HandleError][7]<S, F, [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)][39]>

where S: Service<Request<B>> \+ [Clone][10] \+ [Send][23] \+ 'static, S::Response: [IntoResponse][24] \+ [Send][23], S::Error: [Send][23], S::Future: [Send][23], F: [FnOnce][25](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, S::Error) -> Fut + [Clone][10] \+ [Send][23] \+ 'static, Fut: [Future][26]<Output = Res> \+ [Send][23], Res: [IntoResponse][24], T1: [FromRequestParts][40]<[()][22]> \+ [Send][23], T2: [FromRequestParts][40]<[()][22]> \+ [Send][23], T3: [FromRequestParts][40]<[()][22]> \+ [Send][23], T4: [FromRequestParts][40]<[()][22]> \+ [Send][23], T5: [FromRequestParts][40]<[()][22]> \+ [Send][23], T6: [FromRequestParts][40]<[()][22]> \+ [Send][23], T7: [FromRequestParts][40]<[()][22]> \+ [Send][23], T8: [FromRequestParts][40]<[()][22]> \+ [Send][23], T9: [FromRequestParts][40]<[()][22]> \+ [Send][23], T10: [FromRequestParts][40]<[()][22]> \+ [Send][23], T11: [FromRequestParts][40]<[()][22]> \+ [Send][23], T12: [FromRequestParts][40]<[()][22]> \+ [Send][23], T13: [FromRequestParts][40]<[()][22]> \+ [Send][23], T14: [FromRequestParts][40]<[()][22]> \+ [Send][23], T15: [FromRequestParts][40]<[()][22]> \+ [Send][23], T16: [FromRequestParts][40]<[()][22]> \+ [Send][23], B: [Send][23] \+ 'static,

[Source][55]§

#### type Response = Response<[Body][28]>

Responses given by the service.

[Source][55]§

#### type Error = [Infallible][30]

Errors produced by the service.

[Source][55]§

#### type Future = [HandleErrorFuture][32]

The future response value.

[Source][55]§

#### fn poll_ready(&mut self, _: &mut [Context][34]<'_>) -> [Poll][35]<[Result][36]<[()][22], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][55]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl<S, F, T> [Freeze][56] for [HandleError][7]<S, F, T>

where S: [Freeze][56], F: [Freeze][56],

§

### impl<S, F, T> [RefUnwindSafe][57] for [HandleError][7]<S, F, T>

where S: [RefUnwindSafe][57], F: [RefUnwindSafe][57],

§

### impl<S, F, T> [Send][23] for [HandleError][7]<S, F, T>

where S: [Send][23], F: [Send][23],

§

### impl<S, F, T> [Sync][58] for [HandleError][7]<S, F, T>

where S: [Sync][58], F: [Sync][58],

§

### impl<S, F, T> [Unpin][59] for [HandleError][7]<S, F, T>

where S: [Unpin][59], F: [Unpin][59],

§

### impl<S, F, T> [UnwindSafe][60] for [HandleError][7]<S, F, T>

where S: [UnwindSafe][60], F: [UnwindSafe][60],

## Blanket Implementations§

[Source][61]§

### impl<T> [Any][62] for T

where T: 'static + ?[Sized][63],

[Source][64]§

#### fn [type_id][65](&self) -> [TypeId][66]

Gets the `TypeId` of `self`. [Read more][65]

[Source][67]§

### impl<T> [Borrow][68]<T> for T

where T: ?[Sized][63],

[Source][69]§

#### fn [borrow][70](&self) -> [&T][71]

Immutably borrows from an owned value. [Read more][70]

[Source][72]§

### impl<T> [BorrowMut][73]<T> for T

where T: ?[Sized][63],

[Source][74]§

#### fn [borrow_mut][75](&mut self) -> [&mut T][71]

Mutably borrows from an owned value. [Read more][75]

[Source][76]§

### impl<T> [CloneToUninit][77] for T

where T: [Clone][10],

[Source][78]§

#### unsafe fn [clone_to_uninit][79](&self, dest: [*mut ][80][u8][81])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][79]

[Source][82]§

### impl<T> [From][83]<T> for T

[Source][84]§

#### fn [from][85](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][86]<T> for T

where T: [Clone][10],

§

#### fn [from_ref][87](input: [&T][71]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][88] [`Span`][89], returning an `Instrumented` wrapper. Read more

[Source][90]§

### impl<T, U> [Into][91]<U> for T

where U: [From][83]<T>,

[Source][92]§

#### fn [into][93](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][83]<T> for U` chooses to do.

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

#### fn poll_ready( &mut self, cx: &mut [Context][34]<'_>, ) -> [Poll][35]<[Result][36]<[()][22], <M as MakeService<Target, Request>>::MakeError>>

Returns [`Poll::Ready`][94] when the factory is able to create more services. Read more

§

#### fn make_service( &mut self, target: Target, ) -> <M as MakeService<Target, Request>>::Future

Create and return a new service value asynchronously.

§

#### fn into_service(self) -> IntoService<Self, Request>

where Self: [Sized][63],

Consume this [`MakeService`] and convert it into a [`Service`]. Read more

§

#### fn as_service(&mut self) -> AsService<'_, Self, Request>

where Self: [Sized][63],

Convert this [`MakeService`] into a [`Service`] without consuming the original [`MakeService`]. Read more

§

### impl<T> PolicyExt for T

where T: ?[Sized][63],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][95]§

### impl<T> [Same][96] for T

[Source][97]§

#### type [Output][98] = T

Should always be `Self`

[Source][99]§

### impl<S, R> [ServiceExt][100]<R> for S

where S: Service<R>,

[Source][101]§

#### fn [into_make_service][102](self) -> [IntoMakeService][103]<S>

Convert this service into a [`MakeService`][104], that is a [`Service`] whose response is another service. [Read more][102]

[Source][105]§

#### fn [into_make_service_with_connect_info][106]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][107]<S, C>

Available on **crate feature`tokio`** only.

Convert this service into a [`MakeService`][104], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][108] can extract it. [Read more][106]

[Source][109]§

#### fn [handle_error][110]<F, T>(self, f: F) -> [HandleError][7]<Self, F, T>

Convert this service into a [`HandleError`][7], that will handle errors by converting them into responses. [Read more][110]

§

### impl<T, Request> ServiceExt<Request> for T

where T: Service<Request> \+ ?[Sized][63],

§

#### fn ready(&mut self) -> Ready<'_, Self, Request>

where Self: [Sized][63],

Yields a mutable reference to the service when it is ready to accept a request.

§

#### fn ready_oneshot(self) -> ReadyOneshot<Self, Request>

where Self: [Sized][63],

Yields the service when it is ready to accept a request.

§

#### fn oneshot(self, req: Request) -> Oneshot<Self, Request>

where Self: [Sized][63],

Consume this `Service`, calling it with the provided request once it is ready.

§

#### fn call_all<S>(self, reqs: S) -> CallAll<Self, S>

where Self: [Sized][63], S: Stream<Item = Request>,

Process all requests from the given [`Stream`][111], and produce a [`Stream`][111] of their responses. Read more

§

#### fn and_then<F>(self, f: F) -> AndThen<Self, F>

where Self: [Sized][63], F: [Clone][10],

Executes a new future after this service’s future resolves. This does not alter the behaviour of the [`poll_ready`][112] method. Read more

§

#### fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>

where Self: [Sized][63], F: [FnOnce][25](Self::Response) -> Response + [Clone][10],

Maps this service’s response value to a different value. This does not alter the behaviour of the [`poll_ready`][112] method. Read more

§

#### fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][63], F: [FnOnce][25](Self::Error) -> Error + [Clone][10],

Maps this service’s error value to a different value. This does not alter the behaviour of the [`poll_ready`][112] method. Read more

§

#### fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>

where Self: [Sized][63], Error: [From][83]<Self::Error>, F: [FnOnce][25]([Result][36]<Self::Response, Self::Error>) -> [Result][36]<Response, Error> \+ [Clone][10],

Maps this service’s result type (`Result<Self::Response, Self::Error>`) to a different value, regardless of whether the future succeeds or fails. Read more

§

#### fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>

where Self: [Sized][63], F: [FnMut][113](NewRequest) -> Request,

Composes a function _in front of_ the service. Read more

§

#### fn filter<F, NewRequest>(self, filter: F) -> Filter<Self, F>

where Self: [Sized][63], F: Predicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with a [`Filter`][114] that conditionally accepts or rejects requests based on a [predicate][115]. Read more

§

#### fn filter_async<F, NewRequest>(self, filter: F) -> AsyncFilter<Self, F>

where Self: [Sized][63], F: AsyncPredicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with an [`AsyncFilter`][116] that conditionally accepts or rejects requests based on an [async predicate]. Read more

§

#### fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>

where Self: [Sized][63], Error: [From][83]<Self::Error>, F: [FnOnce][25]([Result][36]<Self::Response, Self::Error>) -> Fut + [Clone][10], Fut: [Future][26]<Output = [Result][36]<Response, Error>>,

Composes an asynchronous function _after_ this service. Read more

§

#### fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>

where Self: [Sized][63], F: [FnMut][113](Self::Future) -> Fut, Error: [From][83]<Self::Error>, Fut: [Future][26]<Output = [Result][36]<Response, Error>>,

Composes a function that transforms futures produced by the service. Read more

§

#### fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>

where Self: [Sized][63] \+ [Send][23] \+ 'static, Self::Future: [Send][23] \+ 'static,

Convert the service into a [`Service`][117] \+ [`Send`][23] trait object. Read more

§

#### fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>

where Self: [Sized][63] \+ [Clone][10] \+ [Send][23] \+ 'static, Self::Future: [Send][23] \+ 'static,

Convert the service into a [`Service`][117] \+ [`Clone`][10] \+ [`Send`][23] trait object. Read more

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][63],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][63],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][118]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][63],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][63],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][63],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][63],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][63],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][63],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][63],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][119] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][120]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][63],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][121] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][120]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][63],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][121] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][120]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][63],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][121] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][63],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][63],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][63],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][63],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][63],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][63],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][63], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][63], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][63],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][63],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][63],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][122]) -> RequestBodyLimit<Self>

where Self: [Sized][63],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][63],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][63],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][123]§

### impl<T> [ToOwned][124] for T

where T: [Clone][10],

[Source][125]§

#### type [Owned][126] = T

The resulting type after obtaining ownership.

[Source][127]§

#### fn [to_owned][128](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][128]

[Source][129]§

#### fn [clone_into][130](&self, target: [&mut T][71])

Uses borrowed data to replace owned data, usually by cloning. [Read more][130]

[Source][131]§

### impl<T, U> [TryFrom][132]<U> for T

where U: [Into][91]<T>,

[Source][133]§

#### type [Error][134] = [Infallible][30]

The type returned in the event of a conversion error.

[Source][135]§

#### fn [try_from][136](value: U) -> [Result][36]<T, <T as [TryFrom][132]<U>>::[Error][137]>

Performs the conversion.

[Source][138]§

### impl<T, U> [TryInto][139]<U> for T

where U: [TryFrom][132]<T>,

[Source][140]§

#### type [Error][141] = <U as [TryFrom][132]<T>>::[Error][137]

The type returned in the event of a conversion error.

[Source][142]§

#### fn [try_into][143](self) -> [Result][36]<U, <U as [TryFrom][132]<T>>::[Error][137]>

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

where S: [Into][91]<Dispatch>,

Attaches the provided [`Subscriber`][144] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][145] [`Subscriber`][144] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/error_handling/mod.rs.html#72-76
   [5]: index.html (mod axum::error_handling)
   [6]: ../../src/axum/error_handling/mod.rs.html#78-87
   [7]: struct.HandleError.html (struct axum::error_handling::HandleError)
   [8]: ../../src/axum/error_handling/mod.rs.html#80-86
   [9]: ../../src/axum/error_handling/mod.rs.html#89-101
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [11]: ../../src/axum/error_handling/mod.rs.html#94-100
   [12]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [13]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [14]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [15]: ../../src/axum/error_handling/mod.rs.html#103-113
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [17]: ../../src/axum/error_handling/mod.rs.html#107-112
   [18]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [19]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [20]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [21]: ../../src/axum/error_handling/mod.rs.html#115-149
   [22]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [23]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [24]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [25]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [26]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [27]: ../../src/axum/error_handling/mod.rs.html#126
   [28]: ../body/struct.Body.html (struct axum::body::Body)
   [29]: ../../src/axum/error_handling/mod.rs.html#127
   [30]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [31]: ../../src/axum/error_handling/mod.rs.html#128
   [32]: future/struct.HandleErrorFuture.html (struct axum::error_handling::future::HandleErrorFuture)
   [33]: ../../src/axum/error_handling/mod.rs.html#130-132
   [34]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [35]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [36]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [37]: ../../src/axum/error_handling/mod.rs.html#134-148
   [38]: ../../src/axum/error_handling/mod.rs.html#207
   [39]: https://doc.rust-lang.org/nightly/std/primitive.tuple.html
   [40]: ../extract/trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [41]: ../../src/axum/error_handling/mod.rs.html#208
   [42]: ../../src/axum/error_handling/mod.rs.html#209
   [43]: ../../src/axum/error_handling/mod.rs.html#210
   [44]: ../../src/axum/error_handling/mod.rs.html#211
   [45]: ../../src/axum/error_handling/mod.rs.html#212
   [46]: ../../src/axum/error_handling/mod.rs.html#213
   [47]: ../../src/axum/error_handling/mod.rs.html#214
   [48]: ../../src/axum/error_handling/mod.rs.html#215
   [49]: ../../src/axum/error_handling/mod.rs.html#216
   [50]: ../../src/axum/error_handling/mod.rs.html#217
   [51]: ../../src/axum/error_handling/mod.rs.html#218
   [52]: ../../src/axum/error_handling/mod.rs.html#219
   [53]: ../../src/axum/error_handling/mod.rs.html#220
   [54]: ../../src/axum/error_handling/mod.rs.html#221
   [55]: ../../src/axum/error_handling/mod.rs.html#222
   [56]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [57]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [58]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [59]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [60]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [61]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [62]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [63]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [64]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [65]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [66]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [67]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [68]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [69]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [70]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [71]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [72]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [73]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [74]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [75]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [76]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [77]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [78]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [79]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [80]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [81]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [82]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [83]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [84]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [85]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [86]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [87]: ../extract/trait.FromRef.html#tymethod.from_ref
   [88]: super::Span::current()
   [89]: crate::Span
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [92]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [93]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [94]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Ready (variant core::task::poll::Poll::Ready)
   [95]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [96]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [97]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [98]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [99]: ../../src/axum/service_ext.rs.html#47-59
   [100]: ../trait.ServiceExt.html (trait axum::ServiceExt)
   [101]: ../../src/axum/service_ext.rs.html#51-53
   [102]: ../trait.ServiceExt.html#tymethod.into_make_service
   [103]: ../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [104]: tower::make::MakeService
   [105]: ../../src/axum/service_ext.rs.html#56-58
   [106]: ../trait.ServiceExt.html#tymethod.into_make_service_with_connect_info
   [107]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [108]: ../extract/struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [109]: ../../src/axum/service_ext.rs.html#42-44
   [110]: ../trait.ServiceExt.html#method.handle_error
   [111]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
   [112]: crate::Service::poll_ready
   [113]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [114]: crate::filter::Filter
   [115]: crate::filter::Predicate
   [116]: crate::filter::AsyncFilter
   [117]: crate::Service
   [118]: https://docs.rs/http/latest/http/struct.Extensions.html
   [119]: crate::follow_redirect::policy::Standard
   [120]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [121]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [122]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [123]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [124]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [125]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [126]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [127]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [128]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [129]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [130]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [131]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [132]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [133]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [134]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [135]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [136]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [137]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [138]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [139]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [140]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [141]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [142]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [143]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [144]: super::Subscriber
   [145]: dispatcher#setting-the-default-subscriber

