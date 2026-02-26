<!-- Generated from rustdoc HTML: extract/connect_info/trait.Connected.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Connected

## [axum][1]0.8.8

## Connected

### Required Methods

  * connect_info



### Implementations on Foreign Types

  * SocketAddr
  * SocketAddr



### Dyn Compatibility

### Implementors

## [In axum::extract::connect_info][2]

[axum][3]::[extract][4]::[connect_info][2]

# Trait Connected Copy item path

[Source][5]
``` 
pub trait Connected<T>:
    [Clone][6]
    + [Send][7]
    + [Sync][8]
    + 'static {
    // Required method
    fn connect_info(stream: T) -> Self;
}
```

Available on **crate feature`tokio`** only.

Expand description

Trait that connected IO resources implement and use to produce information about the connection.

The goal for this trait is to allow users to implement custom IO types that can still provide the same connection metadata.

See [`Router::into_make_service_with_connect_info`][9] for more details.

## Required Methods§

[Source][10]

#### fn connect_info(stream: T) -> Self

Create type holding information about the connection.

## Dyn Compatibility§

This trait is **not** [dyn compatible][11].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementations on Foreign Types§

[Source][12]§

### impl [Connected][13]<[SocketAddr][14]> for [SocketAddr][14]

[Source][15]§

#### fn connect_info(remote_addr: Self) -> Self

[Source][16]§

### impl<L> [Connected][13]<[IncomingStream][17]<'_, L>> for [SocketAddr][14]

where L: [Listener][18]<Addr = Self>,

Available on **crate features`http1` or `http2`** only.

[Source][19]§

#### fn connect_info(stream: [IncomingStream][17]<'_, L>) -> Self

## Implementors§

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/connect_info.rs.html#74-77
   [6]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [7]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [8]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [9]: ../../struct.Router.html#method.into_make_service_with_connect_info (method axum::Router::into_make_service_with_connect_info)
   [10]: ../../../src/axum/extract/connect_info.rs.html#76
   [11]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
   [12]: ../../../src/axum/extract/connect_info.rs.html#93-97
   [13]: trait.Connected.html (trait axum::extract::connect_info::Connected)
   [14]: https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html (enum core::net::socket_addr::SocketAddr)
   [15]: ../../../src/axum/extract/connect_info.rs.html#94-96
   [16]: ../../../src/axum/extract/connect_info.rs.html#83-90
   [17]: ../../serve/struct.IncomingStream.html (struct axum::serve::IncomingStream)
   [18]: ../../serve/trait.Listener.html (trait axum::serve::Listener)
   [19]: ../../../src/axum/extract/connect_info.rs.html#87-89

