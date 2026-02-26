<!-- Generated from rustdoc HTML: serve/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module serve

## [axum][1]0.8.8

## Module serve

### Module Items

  * Structs
  * Traits
  * Functions



## [In crate axum][2]

[axum][2]

# Module serve Copy item path

[Source][3]

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

Expand description

Serve services.

## Structs§

[ConnLimiter][4]
    Return type of [`ListenerExt::limit_connections`][5].
[ConnLimiterIo][6]
    A connection counted by [`ConnLimiter`][4].
[IncomingStream][7]
    An incoming stream.
[Serve][8]
    Future returned by [`serve`][9].
[TapIo][10]
    Return type of [`ListenerExt::tap_io`][11].
[WithGracefulShutdown][12]
    Serve future with graceful shutdown enabled.

## Traits§

[Listener][13]
    Types that can listen for connections.
[ListenerExt][14]
    Extensions to [`Listener`][13].

## Functions§

[serve][15]
    Serve the service with the supplied listener.

   [1]: ../../axum/index.html
   [2]: ../index.html
   [3]: ../../src/axum/serve/mod.rs.html#1-836
   [4]: struct.ConnLimiter.html (struct axum::serve::ConnLimiter)
   [5]: trait.ListenerExt.html#method.limit_connections (method axum::serve::ListenerExt::limit_connections)
   [6]: struct.ConnLimiterIo.html (struct axum::serve::ConnLimiterIo)
   [7]: struct.IncomingStream.html (struct axum::serve::IncomingStream)
   [8]: struct.Serve.html (struct axum::serve::Serve)
   [9]: ../fn.serve.html (fn axum::serve)
   [10]: struct.TapIo.html (struct axum::serve::TapIo)
   [11]: trait.ListenerExt.html#method.tap_io (method axum::serve::ListenerExt::tap_io)
   [12]: struct.WithGracefulShutdown.html (struct axum::serve::WithGracefulShutdown)
   [13]: trait.Listener.html (trait axum::serve::Listener)
   [14]: trait.ListenerExt.html (trait axum::serve::ListenerExt)
   [15]: fn.serve.html (fn axum::serve::serve)

