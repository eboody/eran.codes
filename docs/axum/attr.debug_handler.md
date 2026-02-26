<!-- Generated from rustdoc HTML: attr.debug_handler.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## debug_handler

## [axum][1]0.8.8

## debug_handler

### Sections

  * Changing state type
  * Limitations
  * Performance



## [In crate axum][2]

[axum][2]

# Attribute Macro debug_handler Copy item path
```
#[debug_handler]
```

Available on **crate feature`macros`** only.

Expand description

Generates better error messages when applied to handler functions.

While using [`axum`][3], you can get long error messages for simple mistakes. For example:

ⓘ
```
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await;
}

fn handler() -> &'static str {
    "Hello, world"
}
```

You will get a long error message about function not implementing [`Handler`][4] trait. But why does this function not implement it? To figure it out, the [`debug_handler`][5] macro can be used.

ⓘ
```
#[debug_handler]
fn handler() -> &'static str {
    "Hello, world"
}
```
```
error: handlers must be async functions
  --> main.rs:xx:1
   |
xx | fn handler() -> &'static str {
   | ^^
```

As the error message says, handler function needs to be async.
``` 
use axum::{routing::get, Router, debug_handler};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await;
}

#[debug_handler]
async fn handler() -> &'static str {
    "Hello, world"
}
```

## §Changing state type

By default `#[debug_handler]` assumes your state type is `()` unless your handler has a [`axum::extract::State`][6] argument:
``` 
use axum::{debug_handler, extract::State};

#[debug_handler]
async fn handler(
    // this makes `#[debug_handler]` use `AppState`
    State(state): State<AppState>,
) {}

#[derive(Clone)]
struct AppState {}
```

If your handler takes multiple [`axum::extract::State`][6] arguments or you need to otherwise customize the state type you can set it with `#[debug_handler(state = ...)]`:
``` 
use axum::{debug_handler, extract::{State, FromRef}};

#[debug_handler(state = AppState)]
async fn handler(
    State(app_state): State<AppState>,
    State(inner_state): State<InnerState>,
) {}

#[derive(Clone)]
struct AppState {
    inner: InnerState,
}

#[derive(Clone)]
struct InnerState {}

impl FromRef<AppState> for InnerState {
    fn from_ref(state: &AppState) -> Self {
        state.inner.clone()
    }
}
```

## §Limitations

This macro does not work for functions in an `impl` block that don’t have a `self` parameter:

ⓘ
```
use axum::{debug_handler, extract::Path};

struct App {}

impl App {
    #[debug_handler]
    async fn handler(Path(_): Path<String>) {}
}
```

This will yield an error similar to this:
``` 
error[E0425]: cannot find function `__axum_macros_check_handler_0_from_request_check` in this scope
```

## §Performance

This macro has no effect when compiled with the release profile. (eg. `cargo build --release`)

   [1]: ../axum/index.html
   [2]: index.html
   [3]: https://docs.rs/axum/0.8
   [4]: https://docs.rs/axum/0.8/axum/handler/trait.Handler.html
   [5]: attr.debug_handler.html (attr axum::debug_handler)
   [6]: https://docs.rs/axum/0.8/axum/extract/struct.State.html

