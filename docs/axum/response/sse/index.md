<!-- Generated from rustdoc HTML: response/sse/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module sse

## [axum][1]0.8.8

## Module sse

### Sections

  * Example



### Module Items

  * Structs



## [In axum::response][2]

[axum][3]::[response][2]

# Module sse Copy item path

[Source][4]

Expand description

Server-Sent Events (SSE) responses.

## §Example
``` 
use axum::{
    Router,
    routing::get,
    response::sse::{Event, KeepAlive, Sse},
};
use std::{time::Duration, convert::Infallible};
use tokio_stream::StreamExt as _ ;
use futures_util::stream::{self, Stream};

let app = Router::new().route("/sse", get(sse_handler));

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // A `Stream` that repeats an event every second
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}
```

## Structs§

[Event][5]
    Server-sent event
[EventDataWriter][6]
    Expose [`Event`][5] as a [`std::fmt::Write`][7] such that any form of data can be written as data safely.
[KeepAlive][8]
    Configure the interval between keep-alive messages, the content of each message, and the associated stream.
[KeepAliveStream][9]
    A wrapper around a stream that produces keep-alive events
[Sse][10]
    An SSE response

   [1]: ../../../axum/index.html
   [2]: ../index.html
   [3]: ../../index.html
   [4]: ../../../src/axum/response/sse.rs.html#1-835
   [5]: struct.Event.html (struct axum::response::sse::Event)
   [6]: struct.EventDataWriter.html (struct axum::response::sse::EventDataWriter)
   [7]: https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html (trait core::fmt::Write)
   [8]: struct.KeepAlive.html (struct axum::response::sse::KeepAlive)
   [9]: struct.KeepAliveStream.html (struct axum::response::sse::KeepAliveStream)
   [10]: struct.Sse.html (struct axum::response::sse::Sse)

