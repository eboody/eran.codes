<!-- Generated from rustdoc HTML: extract/ws/close_code/constant.INVALID.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## INVALID

## [axum][1]0.8.8

## [In axum::extract::ws::close_code][2]

[axum][3]::[extract][4]::[ws][5]::[close_code][2]

# Constant INVALID Copy item path

[Source][6]
``` 
pub const INVALID: [u16][7] = 1007;
```

Available on **crate feature`ws`** only.

Expand description

Indicates that an endpoint is terminating the connection because it has received data within a message that was not consistent with the type of the message.

For example, an endpoint received non-UTF-8 RFC3629 data within a text message.

   [1]: ../../../../axum/index.html
   [2]: index.html
   [3]: ../../../index.html
   [4]: ../../index.html
   [5]: ../index.html
   [6]: ../../../../src/axum/extract/ws.rs.html#1060
   [7]: https://doc.rust-lang.org/nightly/std/primitive.u16.html

