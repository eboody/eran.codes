<!-- Generated from rustdoc HTML: routing/future/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module future

## [axum][1]0.8.8

## Module future

### Module Items

  * Structs



## [In axum::routing][2]

[axum][3]::[routing][2]

# Module future Copy item path

[Source][4]

Expand description

Future types.

## Structs§

[InfallibleRouteFuture][5]
    A [`RouteFuture`][6] that always yields a [`Response`][7].
[IntoMakeServiceFuture][8]
    Response future for [`IntoMakeService`][9].
[RouteFuture][6]
    Response future for [`Route`][10].

   [1]: ../../../axum/index.html
   [2]: ../index.html
   [3]: ../../index.html
   [4]: ../../../src/axum/routing/future.rs.html#1-6
   [5]: struct.InfallibleRouteFuture.html (struct axum::routing::future::InfallibleRouteFuture)
   [6]: struct.RouteFuture.html (struct axum::routing::future::RouteFuture)
   [7]: ../../response/type.Response.html (type axum::response::Response)
   [8]: struct.IntoMakeServiceFuture.html (struct axum::routing::future::IntoMakeServiceFuture)
   [9]: ../struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [10]: ../struct.Route.html (struct axum::routing::Route)

