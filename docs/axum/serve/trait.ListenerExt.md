<!-- Generated from rustdoc HTML: serve/trait.ListenerExt.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## ListenerExt

## [axum][1]0.8.8

## ListenerExt

### Provided Methods

  * limit_connections
  * tap_io



### Dyn Compatibility

### Implementors

## [In axum::serve][2]

[axum][3]::[serve][2]

# Trait ListenerExt Copy item path

[Source][4]
``` 
pub trait ListenerExt: [Listener][5] + [Sized][6] {
    // Provided methods
    fn limit_connections(self, limit: [usize][7]) -> [ConnLimiter][8]<Self> { ... }
    fn tap_io<F>(self, tap_fn: F) -> [TapIo][9]<Self, F>
       where F: [FnMut][10](&mut Self::[Io][11]) + [Send][12] + 'static { ... }
}
```

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

Expand description

Extensions to [`Listener`][5].

## Provided Methods§

[Source][13]

#### fn limit_connections(self, limit: [usize][7]) -> [ConnLimiter][8]<Self>

Limit the number of concurrent connections. Once the limit has been reached, no additional connections will be accepted until an existing connection is closed. Listener implementations will typically continue to queue incoming connections, up to an OS and implementation-specific listener backlog limit.

Compare [`tower::limit::concurrency`][14], which provides ways to limit concurrent in-flight requests, but does not limit connections that are idle or in the process of sending request headers.

[Source][15]

#### fn tap_io<F>(self, tap_fn: F) -> [TapIo][9]<Self, F>

where F: [FnMut][10](&mut Self::[Io][11]) + [Send][12] \+ 'static,

Run a mutable closure on every accepted `Io`.

##### §Example
``` 
use axum::{Router, routing::get, serve::ListenerExt};
use tracing::trace;

let router = Router::new().route("/", get(|| async { "Hello, World!" }));

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap()
    .tap_io(|tcp_stream| {
        if let Err(err) = tcp_stream.set_nodelay(true) {
            trace!("failed to set TCP_NODELAY on incoming connection: {err:#}");
        }
    });
axum::serve(listener, router).await;
```

## Dyn Compatibility§

This trait is **not** [dyn compatible][16].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementors§

[Source][17]§

### impl<L: [Listener][5]> [ListenerExt][18] for L

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/serve/listener.rs.html#75-125
   [5]: trait.Listener.html (trait axum::serve::Listener)
   [6]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [7]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [8]: struct.ConnLimiter.html (struct axum::serve::ConnLimiter)
   [9]: struct.TapIo.html (struct axum::serve::TapIo)
   [10]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [11]: trait.Listener.html#associatedtype.Io (type axum::serve::Listener::Io)
   [12]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [13]: ../../src/axum/serve/listener.rs.html#87-92
   [14]: https://docs.rs/tower/latest/tower/limit/concurrency/
   [15]: ../../src/axum/serve/listener.rs.html#116-124
   [16]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
   [17]: ../../src/axum/serve/listener.rs.html#127
   [18]: trait.ListenerExt.html (trait axum::serve::ListenerExt)

