<!-- Generated from rustdoc HTML: extract/ws/close_code/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module close_code

## [axum][1]0.8.8

## Module close_code

### Module Items

  * Constants



## [In axum::extract::ws][2]

[axum][3]::[extract][4]::[ws][2]

# Module close_code Copy item path

[Source][5]

Available on **crate feature`ws`** only.

Expand description

Constants for [`CloseCode`][6]s.

## Constants§

[ABNORMAL][7]
    Indicates an abnormal closure.
[AGAIN][8]
    Indicates that the server is overloaded and the client should either connect to a different IP (when multiple targets exist), or reconnect to the same IP when a user has performed an action.
[AWAY][9]
    Indicates that an endpoint is “going away”, such as a server going down or a browser having navigated away from a page.
[ERROR][10]
    Indicates that a server is terminating the connection because it encountered an unexpected condition that prevented it from fulfilling the request.
[EXTENSION][11]
    Indicates that an endpoint (client) is terminating the connection because the server did not respond to extension negotiation correctly.
[INVALID][12]
    Indicates that an endpoint is terminating the connection because it has received data within a message that was not consistent with the type of the message.
[NORMAL][13]
    Indicates a normal closure, meaning that the purpose for which the connection was established has been fulfilled.
[POLICY][14]
    Indicates that an endpoint is terminating the connection because it has received a message that violates its policy.
[PROTOCOL][15]
    Indicates that an endpoint is terminating the connection due to a protocol error.
[RESTART][16]
    Indicates that the server is restarting.
[SIZE][17]
    Indicates that an endpoint is terminating the connection because it has received a message that is too big for it to process.
[STATUS][18]
    Indicates that no status code was included in a closing frame.
[UNSUPPORTED][19]
    Indicates that an endpoint is terminating the connection because it has received a type of data that it cannot accept.

   [1]: ../../../../axum/index.html
   [2]: ../index.html
   [3]: ../../../index.html
   [4]: ../../index.html
   [5]: ../../../../src/axum/extract/ws.rs.html#1028
   [6]: ../type.CloseCode.html (type axum::extract::ws::CloseCode)
   [7]: constant.ABNORMAL.html (constant axum::extract::ws::close_code::ABNORMAL)
   [8]: constant.AGAIN.html (constant axum::extract::ws::close_code::AGAIN)
   [9]: constant.AWAY.html (constant axum::extract::ws::close_code::AWAY)
   [10]: constant.ERROR.html (constant axum::extract::ws::close_code::ERROR)
   [11]: constant.EXTENSION.html (constant axum::extract::ws::close_code::EXTENSION)
   [12]: constant.INVALID.html (constant axum::extract::ws::close_code::INVALID)
   [13]: constant.NORMAL.html (constant axum::extract::ws::close_code::NORMAL)
   [14]: constant.POLICY.html (constant axum::extract::ws::close_code::POLICY)
   [15]: constant.PROTOCOL.html (constant axum::extract::ws::close_code::PROTOCOL)
   [16]: constant.RESTART.html (constant axum::extract::ws::close_code::RESTART)
   [17]: constant.SIZE.html (constant axum::extract::ws::close_code::SIZE)
   [18]: constant.STATUS.html (constant axum::extract::ws::close_code::STATUS)
   [19]: constant.UNSUPPORTED.html (constant axum::extract::ws::close_code::UNSUPPORTED)

