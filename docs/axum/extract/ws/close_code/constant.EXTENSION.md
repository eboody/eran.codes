<!-- Generated from rustdoc HTML: extract/ws/close_code/constant.EXTENSION.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## EXTENSION

## [axum][1]0.8.8

## [In axum::extract::ws::close_code][2]

[axum][3]::[extract][4]::[ws][5]::[close_code][2]

# Constant EXTENSION Copy item path

[Source][6]
``` 
pub const EXTENSION: [u16][7] = 1010;
```

Available on **crate feature`ws`** only.

Expand description

Indicates that an endpoint (client) is terminating the connection because the server did not respond to extension negotiation correctly.

Specifically, the client has expected the server to negotiate one or more extension(s), but the server didn’t return them in the response message of the WebSocket handshake. The list of extensions that are needed should be given as the reason for closing. Note that this status code is not used by the server, because it can fail the WebSocket handshake instead.

   [1]: ../../../../axum/index.html
   [2]: index.html
   [3]: ../../../index.html
   [4]: ../../index.html
   [5]: ../index.html
   [6]: ../../../../src/axum/extract/ws.rs.html#1082
   [7]: https://doc.rust-lang.org/nightly/std/primitive.u16.html

