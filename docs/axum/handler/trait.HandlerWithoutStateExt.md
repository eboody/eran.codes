<!-- Generated from rustdoc HTML: handler/trait.HandlerWithoutStateExt.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## HandlerWithoutStateExt

## [axum][1]0.8.8

## HandlerWithoutStateExt

### Required Methods

  * into_make_service
  * into_make_service_with_connect_info
  * into_service



### Dyn Compatibility

### Implementors

## [In axum::handler][2]

[axum][3]::[handler][2]

# Trait HandlerWithoutStateExt Copy item path

[Source][4]
``` 
pub trait HandlerWithoutStateExt<T>: [Handler][5]<T, [()][6]> {
    // Required methods
    fn into_service(self) -> [HandlerService][7]<Self, T, [()][6]>;
    fn into_make_service(self) -> [IntoMakeService][8]<[HandlerService][7]<Self, T, [()][6]>>;
    fn into_make_service_with_connect_info<C>(
        self,
    ) -> [IntoMakeServiceWithConnectInfo][9]<[HandlerService][7]<Self, T, [()][6]>, C>;
}
```

Expand description

Extension trait for [`Handler`][5]s that don’t have state.

This provides convenience methods to convert the [`Handler`][5] into a [`Service`] or [`MakeService`][10].

## Required Methods§

[Source][11]

#### fn into_service(self) -> [HandlerService][7]<Self, T, [()][6]>

Convert the handler into a [`Service`] and no state.

[Source][12]

#### fn into_make_service(self) -> [IntoMakeService][8]<[HandlerService][7]<Self, T, [()][6]>>

Convert the handler into a [`MakeService`][10] and no state.

See [`HandlerService::into_make_service`][13] for more details.

[Source][14]

#### fn into_make_service_with_connect_info<C>( self, ) -> [IntoMakeServiceWithConnectInfo][9]<[HandlerService][7]<Self, T, [()][6]>, C>

Available on **crate feature`tokio`** only.

Convert the handler into a [`MakeService`][10] which stores information about the incoming connection and has no state.

See [`HandlerService::into_make_service_with_connect_info`][15] for more details.

## Dyn Compatibility§

This trait is **not** [dyn compatible][16].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementors§

[Source][17]§

### impl<H, T> [HandlerWithoutStateExt][18]<T> for H

where H: [Handler][5]<T, [()][6]>,

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/handler/mod.rs.html#357-378
   [5]: trait.Handler.html (trait axum::handler::Handler)
   [6]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [7]: struct.HandlerService.html (struct axum::handler::HandlerService)
   [8]: ../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [9]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [10]: tower::make::MakeService
   [11]: ../../src/axum/handler/mod.rs.html#359
   [12]: ../../src/axum/handler/mod.rs.html#366
   [13]: struct.HandlerService.html#method.into_make_service (method axum::handler::HandlerService::into_make_service)
   [14]: ../../src/axum/handler/mod.rs.html#375-377
   [15]: struct.HandlerService.html#method.into_make_service_with_connect_info (method axum::handler::HandlerService::into_make_service_with_connect_info)
   [16]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
   [17]: ../../src/axum/handler/mod.rs.html#380-398
   [18]: trait.HandlerWithoutStateExt.html (trait axum::handler::HandlerWithoutStateExt)

