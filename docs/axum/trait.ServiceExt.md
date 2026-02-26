<!-- Generated from rustdoc HTML: trait.ServiceExt.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## ServiceExt

## [axum][1]0.8.8

## ServiceExt

### Required Methods

  * into_make_service
  * into_make_service_with_connect_info



### Provided Methods

  * handle_error



### Dyn Compatibility

### Implementors

## [In crate axum][2]

[axum][2]

# Trait ServiceExt Copy item path

[Source][3]
``` 
pub trait ServiceExt<R>: Service<R> + [Sized][4] {
    // Required methods
    fn into_make_service(self) -> [IntoMakeService][5]<Self>;
    fn into_make_service_with_connect_info<C>(
        self,
    ) -> [IntoMakeServiceWithConnectInfo][6]<Self, C>;

    // Provided method
    fn handle_error<F, T>(self, f: F) -> [HandleError][7]<Self, F, T> { ... }
}
```

Expand description

Extension trait that adds additional methods to any [`Service`].

## Required Methods§

[Source][8]

#### fn into_make_service(self) -> [IntoMakeService][5]<Self>

Convert this service into a [`MakeService`][9], that is a [`Service`] whose response is another service.

This is commonly used when applying middleware around an entire [`Router`][10]. See [“Rewriting request URI in middleware”][11] for more details.

[Source][12]

#### fn into_make_service_with_connect_info<C>( self, ) -> [IntoMakeServiceWithConnectInfo][6]<Self, C>

Available on **crate feature`tokio`** only.

Convert this service into a [`MakeService`][9], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][13] can extract it.

This enables extracting things like the client’s remote address. This is commonly used when applying middleware around an entire [`Router`][10]. See [“Rewriting request URI in middleware”][11] for more details.

## Provided Methods§

[Source][14]

#### fn handle_error<F, T>(self, f: F) -> [HandleError][7]<Self, F, T>

Convert this service into a [`HandleError`][7], that will handle errors by converting them into responses.

See [“error handling model”][15] for more details.

## Dyn Compatibility§

This trait is **not** [dyn compatible][16].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementors§

[Source][17]§

### impl<S, R> [ServiceExt][18]<R> for S

where S: Service<R> \+ [Sized][4],

   [1]: ../axum/index.html
   [2]: index.html
   [3]: ../src/axum/service_ext.rs.html#8-45
   [4]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [5]: routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [6]: extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [7]: error_handling/struct.HandleError.html (struct axum::error_handling::HandleError)
   [8]: ../src/axum/service_ext.rs.html#18
   [9]: tower::make::MakeService
   [10]: struct.Router.html (struct axum::Router)
   [11]: middleware/index.html#rewriting-request-uri-in-middleware (mod axum::middleware)
   [12]: ../src/axum/service_ext.rs.html#33
   [13]: extract/struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [14]: ../src/axum/service_ext.rs.html#42-44
   [15]: error_handling/index.html#axums-error-handling-model (mod axum::error_handling)
   [16]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
   [17]: ../src/axum/service_ext.rs.html#47-59
   [18]: trait.ServiceExt.html (trait axum::ServiceExt)

