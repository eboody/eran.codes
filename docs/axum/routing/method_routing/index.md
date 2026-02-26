<!-- Generated from rustdoc HTML: routing/method_routing/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module method_routing

## [axum][1]0.8.8

## Module method_routing

### Module Items

  * Structs
  * Functions



## [In axum::routing][2]

[axum][3]::[routing][2]

# Module method_routing Copy item path

[Source][4]

Expand description

Route to services and handlers based on HTTP methods.

## Structs§

[MethodRouter][5]
    A [`Service`] that accepts requests based on a [`MethodFilter`][6] and allows chaining additional handlers and services.

## Functions§

[any][7]
    Route requests with the given handler regardless of the method.
[any_service][8]
    Route requests to the given service regardless of its method.
[connect][9]
    Route `CONNECT` requests to the given handler.
[connect_service][10]
    Route `CONNECT` requests to the given service.
[delete][11]
    Route `DELETE` requests to the given handler.
[delete_service][12]
    Route `DELETE` requests to the given service.
[get][13]
    Route `GET` requests to the given handler.
[get_service][14]
    Route `GET` requests to the given service.
[head][15]
    Route `HEAD` requests to the given handler.
[head_service][16]
    Route `HEAD` requests to the given service.
[on][17]
    Route requests with the given method to the handler.
[on_service][18]
    Route requests with the given method to the service.
[options][19]
    Route `OPTIONS` requests to the given handler.
[options_service][20]
    Route `OPTIONS` requests to the given service.
[patch][21]
    Route `PATCH` requests to the given handler.
[patch_service][22]
    Route `PATCH` requests to the given service.
[post][23]
    Route `POST` requests to the given handler.
[post_service][24]
    Route `POST` requests to the given service.
[put][25]
    Route `PUT` requests to the given handler.
[put_service][26]
    Route `PUT` requests to the given service.
[trace][27]
    Route `TRACE` requests to the given handler.
[trace_service][28]
    Route `TRACE` requests to the given service.

   [1]: ../../../axum/index.html
   [2]: ../index.html
   [3]: ../../index.html
   [4]: ../../../src/axum/routing/method_routing.rs.html#1-1723
   [5]: struct.MethodRouter.html (struct axum::routing::method_routing::MethodRouter)
   [6]: ../struct.MethodFilter.html (struct axum::routing::MethodFilter)
   [7]: fn.any.html (fn axum::routing::method_routing::any)
   [8]: fn.any_service.html (fn axum::routing::method_routing::any_service)
   [9]: fn.connect.html (fn axum::routing::method_routing::connect)
   [10]: fn.connect_service.html (fn axum::routing::method_routing::connect_service)
   [11]: fn.delete.html (fn axum::routing::method_routing::delete)
   [12]: fn.delete_service.html (fn axum::routing::method_routing::delete_service)
   [13]: fn.get.html (fn axum::routing::method_routing::get)
   [14]: fn.get_service.html (fn axum::routing::method_routing::get_service)
   [15]: fn.head.html (fn axum::routing::method_routing::head)
   [16]: fn.head_service.html (fn axum::routing::method_routing::head_service)
   [17]: fn.on.html (fn axum::routing::method_routing::on)
   [18]: fn.on_service.html (fn axum::routing::method_routing::on_service)
   [19]: fn.options.html (fn axum::routing::method_routing::options)
   [20]: fn.options_service.html (fn axum::routing::method_routing::options_service)
   [21]: fn.patch.html (fn axum::routing::method_routing::patch)
   [22]: fn.patch_service.html (fn axum::routing::method_routing::patch_service)
   [23]: fn.post.html (fn axum::routing::method_routing::post)
   [24]: fn.post_service.html (fn axum::routing::method_routing::post_service)
   [25]: fn.put.html (fn axum::routing::method_routing::put)
   [26]: fn.put_service.html (fn axum::routing::method_routing::put_service)
   [27]: fn.trace.html (fn axum::routing::method_routing::trace)
   [28]: fn.trace_service.html (fn axum::routing::method_routing::trace_service)

