<!-- Generated from rustdoc HTML: extract/ws/close_code/constant.POLICY.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## POLICY

## [axum][1]0.8.8

## [In axum::extract::ws::close_code][2]

[axum][3]::[extract][4]::[ws][5]::[close_code][2]

# Constant POLICY Copy item path

[Source][6]
``` 
pub const POLICY: [u16][7] = 1008;
```

Available on **crate feature`ws`** only.

Expand description

Indicates that an endpoint is terminating the connection because it has received a message that violates its policy.

This is a generic status code that can be returned when there is no other more suitable status code (e.g., `UNSUPPORTED` or `SIZE`) or if there is a need to hide specific details about the policy.

   [1]: ../../../../axum/index.html
   [2]: index.html
   [3]: ../../../index.html
   [4]: ../../index.html
   [5]: ../index.html
   [6]: ../../../../src/axum/extract/ws.rs.html#1068
   [7]: https://doc.rust-lang.org/nightly/std/primitive.u16.html

