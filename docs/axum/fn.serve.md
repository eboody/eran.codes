<!-- Generated from rustdoc HTML: fn.serve.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## serve

## [axum][1]0.8.8

## serve

### Sections

  * Examples
  * Return Value



## [In crate axum][2]

[axum][2]

# Function serve Copy item path

[Source][3]
``` 
pub fn serve<L, M, S, B>(listener: L, make_service: M) -> [Serve][4]<L, M, S, B>

where
    L: [Listener][5],
    M: for<'a> Service<[IncomingStream][6]<'a, L>, Error = [Infallible][7], Response = S>,
    S: Service<[Request][8], Response = [Response][9]<B>, Error = [Infallible][7]> + [Clone][10] + [Send][11] + 'static,
    S::Future: [Send][11],
    B: HttpBody + [Send][11] + 'static,
    B::Data: [Send][11],
    B::Error: [Into][12]<[Box][13]<dyn [StdError][14] + [Send][11] + [Sync][15]>>,
```

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

Expand description

Serve the service with the supplied listener.

This method of running a service is intentionally simple and doesn’t support any configuration. hyper’s default configuration applies (including [timeouts][16]); use hyper or hyper-util if you need configuration.

It supports both HTTP/1 as well as HTTP/2.

## §Examples

Serving a [`Router`][17]:
``` 
use axum::{Router, routing::get};

let router = Router::new().route("/", get(|| async { "Hello, World!" }));

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, router).await;
```

See also [`Router::into_make_service_with_connect_info`][18].

Serving a [`MethodRouter`][19]:
``` 
use axum::routing::get;

let router = get(|| async { "Hello, World!" });

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, router).await;
```

See also [`MethodRouter::into_make_service_with_connect_info`][20].

Serving a [`Handler`][21]:
``` 
use axum::handler::HandlerWithoutStateExt;

async fn handler() -> &'static str {
    "Hello, World!"
}

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, handler.into_make_service()).await;
```

See also [`HandlerWithoutStateExt::into_make_service_with_connect_info`][22] and [`HandlerService::into_make_service_with_connect_info`][23].

## §Return Value

Although this future resolves to `io::Result<()>`, it will never actually complete or return an error. Errors on the TCP socket will be handled by sleeping for a short while (currently, one second).

   [1]: ../axum/index.html
   [2]: index.html
   [3]: ../src/axum/serve/mod.rs.html#101-116
   [4]: serve/struct.Serve.html (struct axum::serve::Serve)
   [5]: serve/trait.Listener.html (trait axum::serve::Listener)
   [6]: serve/struct.IncomingStream.html (struct axum::serve::IncomingStream)
   [7]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [8]: extract/type.Request.html (type axum::extract::Request)
   [9]: response/type.Response.html (type axum::response::Response)
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [11]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [12]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [13]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [14]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [15]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [16]: hyper::server::conn::http1::Builder::header_read_timeout
   [17]: struct.Router.html (struct axum::Router)
   [18]: struct.Router.html#method.into_make_service_with_connect_info (method axum::Router::into_make_service_with_connect_info)
   [19]: routing/method_routing/struct.MethodRouter.html (struct axum::routing::method_routing::MethodRouter)
   [20]: routing/method_routing/struct.MethodRouter.html#method.into_make_service_with_connect_info (method axum::routing::method_routing::MethodRouter::into_make_service_with_connect_info)
   [21]: handler/trait.Handler.html (trait axum::handler::Handler)
   [22]: handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service_with_connect_info (method axum::handler::HandlerWithoutStateExt::into_make_service_with_connect_info)
   [23]: handler/struct.HandlerService.html#method.into_make_service_with_connect_info (method axum::handler::HandlerService::into_make_service_with_connect_info)

