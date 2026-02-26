<!-- Generated from rustdoc HTML: extract/derive.FromRef.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## FromRef

## [axum][1]0.8.8

## FromRef

### Sections

  * Example



## [In axum::extract][2]

[axum][3]::[extract][2]

# Derive Macro FromRef Copy item path
```
#[derive(FromRef)]
{
    // Attributes available to this derive:
    #[from_ref]
}

```

Available on **crate feature`macros`** only.

Expand description

Derive an implementation of [`FromRef`][4] for each field in a struct.

## §Example
``` 
use axum::{
    Router,
    routing::get,
    extract::{State, FromRef},
};

// This will implement `FromRef` for each field in the struct.
#[derive(FromRef, Clone)]
struct AppState {
    auth_token: AuthToken,
    database_pool: DatabasePool,
    // fields can also be skipped
    #[from_ref(skip)]
    api_token: String,
}

// So those types can be extracted via `State`
async fn handler(State(auth_token): State<AuthToken>) {}

async fn other_handler(State(database_pool): State<DatabasePool>) {}

let state = AppState {
    auth_token,
    database_pool,
    api_token: "secret".to_owned(),
};

let app = Router::new()
    .route("/", get(handler).post(other_handler))
    .with_state(state);
```

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: https://docs.rs/axum/0.8/axum/extract/trait.FromRef.html

