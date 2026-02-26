<!-- Generated from rustdoc HTML: extract/ws/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module ws

## [axum][1]0.8.8

## Module ws

### Sections

  * Example
  * Passing data and/or state to an `on_upgrade` callback
  * Read and write concurrently



### Module Items

  * Modules
  * Structs
  * Enums
  * Traits
  * Type Aliases



## [In axum::extract][2]

[axum][3]::[extract][2]

# Module ws Copy item path

[Source][4]

Available on **crate feature`ws`** only.

Expand description

Handle WebSocket connections.

## §Example
``` 
use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    routing::any,
    response::{IntoResponse, Response},
    Router,
};

let app = Router::new().route("/ws", any(handler));

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}
```

## §Passing data and/or state to an `on_upgrade` callback
``` 
use axum::{
    extract::{ws::{WebSocketUpgrade, WebSocket}, State},
    response::Response,
    routing::any,
    Router,
};

#[derive(Clone)]
struct AppState {
    // ...
}

async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    // ...
}

let app = Router::new()
    .route("/ws", any(handler))
    .with_state(AppState { /* ... */ });
```

## §Read and write concurrently

If you need to read and write concurrently from a [`WebSocket`][5] you can use [`StreamExt::split`][6]:
``` 
use axum::{Error, extract::ws::{WebSocket, Message}};
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};

async fn handle_socket(mut socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();

    tokio::spawn(write(sender));
    tokio::spawn(read(receiver));
}

async fn read(receiver: SplitStream<WebSocket>) {
    // ...
}

async fn write(sender: SplitSink<WebSocket, Message>) {
    // ...
}
```

## Modules§

[close_code][7]
    Constants for [`CloseCode`][8]s.
[rejection][9]
    WebSocket specific rejections.

## Structs§

[CloseFrame][10]
    A struct representing the close command.
[DefaultOnFailedUpgrade][11]
    The default `OnFailedUpgrade` used by `WebSocketUpgrade`.
[Utf8Bytes][12]
    UTF-8 wrapper for [Bytes].
[WebSocket][5]
    A stream of WebSocket messages.
[WebSocketUpgrade][13]
    Extractor for establishing WebSocket connections.

## Enums§

[Message][14]
    A WebSocket message.

## Traits§

[OnFailedUpgrade][15]
    What to do when a connection upgrade fails.

## Type Aliases§

[CloseCode][8]
    Status code used to indicate why an endpoint is closing the WebSocket connection.

   [1]: ../../../axum/index.html
   [2]: ../index.html
   [3]: ../../index.html
   [4]: ../../../src/axum/extract/ws.rs.html#1-1256
   [5]: struct.WebSocket.html (struct axum::extract::ws::WebSocket)
   [6]: https://docs.rs/futures/0.3.17/futures/stream/trait.StreamExt.html#method.split
   [7]: close_code/index.html (mod axum::extract::ws::close_code)
   [8]: type.CloseCode.html (type axum::extract::ws::CloseCode)
   [9]: rejection/index.html (mod axum::extract::ws::rejection)
   [10]: struct.CloseFrame.html (struct axum::extract::ws::CloseFrame)
   [11]: struct.DefaultOnFailedUpgrade.html (struct axum::extract::ws::DefaultOnFailedUpgrade)
   [12]: struct.Utf8Bytes.html (struct axum::extract::ws::Utf8Bytes)
   [13]: struct.WebSocketUpgrade.html (struct axum::extract::ws::WebSocketUpgrade)
   [14]: enum.Message.html (enum axum::extract::ws::Message)
   [15]: trait.OnFailedUpgrade.html (trait axum::extract::ws::OnFailedUpgrade)

