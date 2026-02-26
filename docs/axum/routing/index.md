<!-- Generated from rustdoc HTML: routing/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module routing

## [axum][1]0.8.8

## Module routing

### Module Items

  * Re-exports
  * Modules
  * Structs



## [In crate axum][2]

[axum][2]

# Module routing Copy item path

[Source][3]

Expand description

Routing between [`Service`]s and handlers.

## Re-exports§

`pub use self::method_routing::[any][4];`
`pub use self::method_routing::[any_service][5];`
`pub use self::method_routing::[connect][6];`
`pub use self::method_routing::[connect_service][7];`
`pub use self::method_routing::[delete][8];`
`pub use self::method_routing::[delete_service][9];`
`pub use self::method_routing::[get][10];`
`pub use self::method_routing::[get_service][11];`
`pub use self::method_routing::[head][12];`
`pub use self::method_routing::[head_service][13];`
`pub use self::method_routing::[on][14];`
`pub use self::method_routing::[on_service][15];`
`pub use self::method_routing::[options][16];`
`pub use self::method_routing::[options_service][17];`
`pub use self::method_routing::[patch][18];`
`pub use self::method_routing::[patch_service][19];`
`pub use self::method_routing::[post][20];`
`pub use self::method_routing::[post_service][21];`
`pub use self::method_routing::[put][22];`
`pub use self::method_routing::[put_service][23];`
`pub use self::method_routing::[trace][24];`
`pub use self::method_routing::[trace_service][25];`
`pub use self::method_routing::[MethodRouter][26];`

## Modules§

[future][27]
    Future types.
[method_routing][28]
    Route to services and handlers based on HTTP methods.

## Structs§

[IntoMakeService][29]
    A [`MakeService`][30] that produces axum router services.
[MethodFilter][31]
    A filter that matches one or more HTTP methods.
[Route][32]
    How routes are stored inside a [`Router`][33].
[Router][34]
    The router type for composing handlers and services.
[RouterAsService][35]
    A [`Router`][33] converted into a borrowed [`Service`] with a fixed body type.
[RouterIntoService][36]
    A [`Router`][33] converted into an owned [`Service`] with a fixed body type.

   [1]: ../../axum/index.html
   [2]: ../index.html
   [3]: ../../src/axum/routing/mod.rs.html#1-840
   [4]: method_routing/fn.any.html (fn axum::routing::method_routing::any)
   [5]: method_routing/fn.any_service.html (fn axum::routing::method_routing::any_service)
   [6]: method_routing/fn.connect.html (fn axum::routing::method_routing::connect)
   [7]: method_routing/fn.connect_service.html (fn axum::routing::method_routing::connect_service)
   [8]: method_routing/fn.delete.html (fn axum::routing::method_routing::delete)
   [9]: method_routing/fn.delete_service.html (fn axum::routing::method_routing::delete_service)
   [10]: method_routing/fn.get.html (fn axum::routing::method_routing::get)
   [11]: method_routing/fn.get_service.html (fn axum::routing::method_routing::get_service)
   [12]: method_routing/fn.head.html (fn axum::routing::method_routing::head)
   [13]: method_routing/fn.head_service.html (fn axum::routing::method_routing::head_service)
   [14]: method_routing/fn.on.html (fn axum::routing::method_routing::on)
   [15]: method_routing/fn.on_service.html (fn axum::routing::method_routing::on_service)
   [16]: method_routing/fn.options.html (fn axum::routing::method_routing::options)
   [17]: method_routing/fn.options_service.html (fn axum::routing::method_routing::options_service)
   [18]: method_routing/fn.patch.html (fn axum::routing::method_routing::patch)
   [19]: method_routing/fn.patch_service.html (fn axum::routing::method_routing::patch_service)
   [20]: method_routing/fn.post.html (fn axum::routing::method_routing::post)
   [21]: method_routing/fn.post_service.html (fn axum::routing::method_routing::post_service)
   [22]: method_routing/fn.put.html (fn axum::routing::method_routing::put)
   [23]: method_routing/fn.put_service.html (fn axum::routing::method_routing::put_service)
   [24]: method_routing/fn.trace.html (fn axum::routing::method_routing::trace)
   [25]: method_routing/fn.trace_service.html (fn axum::routing::method_routing::trace_service)
   [26]: method_routing/struct.MethodRouter.html (struct axum::routing::method_routing::MethodRouter)
   [27]: future/index.html (mod axum::routing::future)
   [28]: method_routing/index.html (mod axum::routing::method_routing)
   [29]: struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [30]: tower::make::MakeService
   [31]: struct.MethodFilter.html (struct axum::routing::MethodFilter)
   [32]: struct.Route.html (struct axum::routing::Route)
   [33]: ../struct.Router.html (struct axum::Router)
   [34]: struct.Router.html (struct axum::routing::Router)
   [35]: struct.RouterAsService.html (struct axum::routing::RouterAsService)
   [36]: struct.RouterIntoService.html (struct axum::routing::RouterIntoService)

