<!-- Generated from rustdoc HTML: extract/connect_info/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module connect_info

## [axum][1]0.8.8

## Module connect_info

### Module Items

  * Structs
  * Traits



## [In axum::extract][2]

[axum][3]::[extract][2]

# Module connect_info Copy item path

[Source][4]

Available on **crate feature`tokio`** only.

Expand description

Extractor for getting connection information from a client.

See [`Router::into_make_service_with_connect_info`][5] for more details.

## Structs§

[ConnectInfo][6]
    Extractor for getting connection information produced by a [`Connected`][7].
[IntoMakeServiceWithConnectInfo][8]
    A [`MakeService`][9] created from a router.
[MockConnectInfo][10]
    Middleware used to mock [`ConnectInfo`][11] during tests.
[ResponseFuture][12]
    Response future for [`IntoMakeServiceWithConnectInfo`][8].

## Traits§

[Connected][7]
    Trait that connected IO resources implement and use to produce information about the connection.

   [1]: ../../../axum/index.html
   [2]: ../index.html
   [3]: ../../index.html
   [4]: ../../../src/axum/extract/connect_info.rs.html#1-420
   [5]: ../../struct.Router.html#method.into_make_service_with_connect_info (method axum::Router::into_make_service_with_connect_info)
   [6]: struct.ConnectInfo.html (struct axum::extract::connect_info::ConnectInfo)
   [7]: trait.Connected.html (trait axum::extract::connect_info::Connected)
   [8]: struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [9]: tower::make::MakeService
   [10]: struct.MockConnectInfo.html (struct axum::extract::connect_info::MockConnectInfo)
   [11]: ../struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [12]: struct.ResponseFuture.html (struct axum::extract::connect_info::ResponseFuture)

