<!-- Generated from rustdoc HTML: serve/trait.Listener.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Listener

## [axum][1]0.8.8

## Listener

### Required Associated Types

  * Addr
  * Io



### Required Methods

  * accept
  * local_addr



### Implementations on Foreign Types

  * TcpListener
  * UnixListener



### Dyn Compatibility

### Implementors

## [In axum::serve][2]

[axum][3]::[serve][2]

# Trait Listener Copy item path

[Source][4]
``` 
pub trait Listener: [Send][5] + 'static {
    type Io: AsyncRead + AsyncWrite + [Unpin][6] + [Send][5] + 'static;
    type Addr: [Send][5];

    // Required methods
    fn accept(&mut self) -> impl [Future][7]<Output = (Self::[Io][8], Self::[Addr][9])> + [Send][5];
    fn local_addr(&self) -> [Result][10]<Self::[Addr][9]>;
}
```

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

Expand description

Types that can listen for connections.

## Required Associated Types§

[Source][11]

#### type Io: AsyncRead + AsyncWrite + [Unpin][6] \+ [Send][5] \+ 'static

The listener’s IO type.

[Source][12]

#### type Addr: [Send][5]

The listener’s address type.

## Required Methods§

[Source][13]

#### fn accept(&mut self) -> impl [Future][7]<Output = (Self::[Io][8], Self::[Addr][9])> \+ [Send][5]

Accept a new incoming connection to this listener.

If the underlying accept call can return an error, this function must take care of logging and retrying.

[Source][14]

#### fn local_addr(&self) -> [Result][10]<Self::[Addr][9]>

Returns the local address that this listener is bound to.

## Dyn Compatibility§

This trait is **not** [dyn compatible][15].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementations on Foreign Types§

[Source][16]§

### impl [Listener][17] for TcpListener

[Source][18]§

#### type Io = TcpStream

[Source][19]§

#### type Addr = [SocketAddr][20]

[Source][21]§

#### async fn accept(&mut self) -> (Self::[Io][8], Self::[Addr][9])

[Source][22]§

#### fn local_addr(&self) -> [Result][10]<Self::[Addr][9]>

[Source][23]§

### impl [Listener][17] for UnixListener

Available on **Unix** only.

[Source][24]§

#### type Io = UnixStream

[Source][25]§

#### type Addr = SocketAddr

[Source][26]§

#### async fn accept(&mut self) -> (Self::[Io][8], Self::[Addr][9])

[Source][27]§

#### fn local_addr(&self) -> [Result][10]<Self::[Addr][9]>

## Implementors§

[Source][28]§

### impl<L, F> [Listener][17] for [TapIo][29]<L, F>

where L: [Listener][17], F: [FnMut][30](&mut L::[Io][8]) + [Send][5] \+ 'static,

[Source][31]§

#### type Io = <L as [Listener][17]>::[Io][8]

[Source][32]§

#### type Addr = <L as [Listener][17]>::[Addr][9]

[Source][33]§

### impl<T: [Listener][17]> [Listener][17] for [ConnLimiter][34]<T>

[Source][35]§

#### type Io = [ConnLimiterIo][36]<<T as [Listener][17]>::[Io][8]>

[Source][37]§

#### type Addr = <T as [Listener][17]>::[Addr][9]

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/serve/listener.rs.html#18-33
   [5]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [6]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [7]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [8]: trait.Listener.html#associatedtype.Io (type axum::serve::Listener::Io)
   [9]: trait.Listener.html#associatedtype.Addr (type axum::serve::Listener::Addr)
   [10]: https://doc.rust-lang.org/nightly/std/io/error/type.Result.html (type std::io::error::Result)
   [11]: ../../src/axum/serve/listener.rs.html#20
   [12]: ../../src/axum/serve/listener.rs.html#23
   [13]: ../../src/axum/serve/listener.rs.html#29
   [14]: ../../src/axum/serve/listener.rs.html#32
   [15]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
   [16]: ../../src/axum/serve/listener.rs.html#35-52
   [17]: trait.Listener.html (trait axum::serve::Listener)
   [18]: ../../src/axum/serve/listener.rs.html#36
   [19]: ../../src/axum/serve/listener.rs.html#37
   [20]: https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html (enum core::net::socket_addr::SocketAddr)
   [21]: ../../src/axum/serve/listener.rs.html#39-46
   [22]: ../../src/axum/serve/listener.rs.html#49-51
   [23]: ../../src/axum/serve/listener.rs.html#55-72
   [24]: ../../src/axum/serve/listener.rs.html#56
   [25]: ../../src/axum/serve/listener.rs.html#57
   [26]: ../../src/axum/serve/listener.rs.html#59-66
   [27]: ../../src/axum/serve/listener.rs.html#69-71
   [28]: ../../src/axum/serve/listener.rs.html#226-243
   [29]: struct.TapIo.html (struct axum::serve::TapIo)
   [30]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [31]: ../../src/axum/serve/listener.rs.html#231
   [32]: ../../src/axum/serve/listener.rs.html#232
   [33]: ../../src/axum/serve/listener.rs.html#138-151
   [34]: struct.ConnLimiter.html (struct axum::serve::ConnLimiter)
   [35]: ../../src/axum/serve/listener.rs.html#139
   [36]: struct.ConnLimiterIo.html (struct axum::serve::ConnLimiterIo)
   [37]: ../../src/axum/serve/listener.rs.html#140

