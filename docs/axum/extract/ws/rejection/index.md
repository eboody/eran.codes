<!-- Generated from rustdoc HTML: extract/ws/rejection/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module rejection

## [axum][1]0.8.8

## Module rejection

### Module Items

  * Structs
  * Enums



## [In axum::extract::ws][2]

[axum][3]::[extract][4]::[ws][2]

# Module rejection Copy item path

[Source][5]

Available on **crate feature`ws`** only.

Expand description

WebSocket specific rejections.

## Structs§

[ConnectionNotUpgradable][6]
    Rejection type for [`WebSocketUpgrade`][7].
[InvalidConnectionHeader][8]
    Rejection type for [`WebSocketUpgrade`][7].
[InvalidProtocolPseudoheader][9]
    Rejection type for [`WebSocketUpgrade`][7].
[InvalidUpgradeHeader][10]
    Rejection type for [`WebSocketUpgrade`][7].
[InvalidWebSocketVersionHeader][11]
    Rejection type for [`WebSocketUpgrade`][7].
[MethodNotConnect][12]
    Rejection type for [`WebSocketUpgrade`][7].
[MethodNotGet][13]
    Rejection type for [`WebSocketUpgrade`][7].
[WebSocketKeyHeaderMissing][14]
    Rejection type for [`WebSocketUpgrade`][7].

## Enums§

[WebSocketUpgradeRejection][15]
    Rejection used for [`WebSocketUpgrade`][7].

   [1]: ../../../../axum/index.html
   [2]: ../index.html
   [3]: ../../../index.html
   [4]: ../../index.html
   [5]: ../../../../src/axum/extract/ws.rs.html#941
   [6]: struct.ConnectionNotUpgradable.html (struct axum::extract::ws::rejection::ConnectionNotUpgradable)
   [7]: ../../struct.WebSocketUpgrade.html (struct axum::extract::WebSocketUpgrade)
   [8]: struct.InvalidConnectionHeader.html (struct axum::extract::ws::rejection::InvalidConnectionHeader)
   [9]: struct.InvalidProtocolPseudoheader.html (struct axum::extract::ws::rejection::InvalidProtocolPseudoheader)
   [10]: struct.InvalidUpgradeHeader.html (struct axum::extract::ws::rejection::InvalidUpgradeHeader)
   [11]: struct.InvalidWebSocketVersionHeader.html (struct axum::extract::ws::rejection::InvalidWebSocketVersionHeader)
   [12]: struct.MethodNotConnect.html (struct axum::extract::ws::rejection::MethodNotConnect)
   [13]: struct.MethodNotGet.html (struct axum::extract::ws::rejection::MethodNotGet)
   [14]: struct.WebSocketKeyHeaderMissing.html (struct axum::extract::ws::rejection::WebSocketKeyHeaderMissing)
   [15]: enum.WebSocketUpgradeRejection.html (enum axum::extract::ws::rejection::WebSocketUpgradeRejection)

