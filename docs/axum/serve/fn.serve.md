<!-- Generated from rustdoc HTML: serve/fn.serve.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## serve

## [axum][1]0.8.8

## serve

### Sections

  * Examples
  * Return Value



## [In axum::serve][2]

[axum][3]::[serve][2]

# Function serve Copy item path

[Source][4]
``` 
pub fn serve<L, M, S, B>(listener: L, make_service: M) -> [Serve][5]<L, M, S, B>

where
    L: [Listener][6],
    M: for<'a> Service<[IncomingStream][7]<'a, L>, Error = [Infallible][8], Response = S>,
    S: Service<[Request][9], Response = [Response][10]<B>, Error = [Infallible][8]> + [Clone][11] + [Send][12] + 'static,
    S::Future: [Send][12],
    B: HttpBody + [Send][12] + 'static,
    B::Data: [Send][12],
    B::Error: [Into][13]<[Box][14]<dyn [StdError][15] + [Send][12] + [Sync][16]>>,
```

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

Expand description

Serve the service with the supplied listener.

This method of running a service is intentionally simple and doesn’t support any configuration. hyper’s default configuration applies (including [timeouts][17]); use hyper or hyper-util if you need configuration.

It supports both HTTP/1 as well as HTTP/2.

## §Examples

Serving a [`Router`][18]:
``` 
use axum::{Router, routing::get};

let router = Router::new().route("/", get(|| async { "Hello, World!" }));

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, router).await;
```

See also [`Router::into_make_service_with_connect_info`][19].

Serving a [`MethodRouter`][20]:
``` 
use axum::routing::get;

let router = get(|| async { "Hello, World!" });

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, router).await;
```

See also [`MethodRouter::into_make_service_with_connect_info`][21].

Serving a [`Handler`][22]:
``` 
use axum::handler::HandlerWithoutStateExt;

async fn handler() -> &'static str {
    "Hello, World!"
}

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, handler.into_make_service()).await;
```

See also [`HandlerWithoutStateExt::into_make_service_with_connect_info`][23] and [`HandlerService::into_make_service_with_connect_info`][24].

## §Return Value

Although this future resolves to `io::Result<()>`, it will never actually complete or return an error. Errors on the TCP socket will be handled by sleeping for a short while (currently, one second).

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/serve/mod.rs.html#101-116
   [5]: struct.Serve.html (struct axum::serve::Serve)
   [6]: trait.Listener.html (trait axum::serve::Listener)
   [7]: struct.IncomingStream.html (struct axum::serve::IncomingStream)
   [8]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [9]: ../extract/type.Request.html (type axum::extract::Request)
   [10]: ../response/type.Response.html (type axum::response::Response)
   [11]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [12]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [13]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [14]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [15]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [16]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [17]: hyper::server::conn::http1::Builder::header_read_timeout
   [18]: ../struct.Router.html (struct axum::Router)
   [19]: ../struct.Router.html#method.into_make_service_with_connect_info (method axum::Router::into_make_service_with_connect_info)
   [20]: ../routing/method_routing/struct.MethodRouter.html (struct axum::routing::method_routing::MethodRouter)
   [21]: ../routing/method_routing/struct.MethodRouter.html#method.into_make_service_with_connect_info (method axum::routing::method_routing::MethodRouter::into_make_service_with_connect_info)
   [22]: ../handler/trait.Handler.html (trait axum::handler::Handler)
   [23]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service_with_connect_info (method axum::handler::HandlerWithoutStateExt::into_make_service_with_connect_info)
   [24]: ../handler/struct.HandlerService.html#method.into_make_service_with_connect_info (method axum::handler::HandlerService::into_make_service_with_connect_info)

