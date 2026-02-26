<!-- Generated from rustdoc HTML: all.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## All

## [axum][1]0.8.8

### Crate Items

  * Structs
  * Enums
  * Constants
  * Traits
  * Functions
  * Type Aliases
  * Attribute Macros
  * Derive Macros



# List of all items

### Structs

  * [Error][2]
  * [Extension][3]
  * [Form][4]
  * [Json][5]
  * [Router][6]
  * [body::Body][7]
  * [body::BodyDataStream][8]
  * [error_handling::HandleError][9]
  * [error_handling::HandleErrorLayer][10]
  * [error_handling::future::HandleErrorFuture][11]
  * [extract::ConnectInfo][12]
  * [extract::DefaultBodyLimit][13]
  * [extract::MatchedPath][14]
  * [extract::Multipart][15]
  * [extract::NestedPath][16]
  * [extract::OriginalUri][17]
  * [extract::Path][18]
  * [extract::Query][19]
  * [extract::RawForm][20]
  * [extract::RawPathParams][21]
  * [extract::RawQuery][22]
  * [extract::State][23]
  * [extract::WebSocketUpgrade][24]
  * [extract::connect_info::ConnectInfo][25]
  * [extract::connect_info::IntoMakeServiceWithConnectInfo][26]
  * [extract::connect_info::MockConnectInfo][27]
  * [extract::connect_info::ResponseFuture][28]
  * [extract::multipart::Field][29]
  * [extract::multipart::InvalidBoundary][30]
  * [extract::multipart::Multipart][31]
  * [extract::multipart::MultipartError][32]
  * [extract::path::FailedToDeserializePathParams][33]
  * [extract::path::InvalidUtf8InPathParam][34]
  * [extract::path::Path][35]
  * [extract::path::RawPathParams][36]
  * [extract::path::RawPathParamsIter][37]
  * [extract::rejection::FailedToDeserializeForm][38]
  * [extract::rejection::FailedToDeserializeFormBody][39]
  * [extract::rejection::FailedToDeserializeQueryString][40]
  * [extract::rejection::InvalidFormContentType][41]
  * [extract::rejection::InvalidUtf8][42]
  * [extract::rejection::JsonDataError][43]
  * [extract::rejection::JsonSyntaxError][44]
  * [extract::rejection::LengthLimitError][45]
  * [extract::rejection::MatchedPathMissing][46]
  * [extract::rejection::MissingExtension][47]
  * [extract::rejection::MissingJsonContentType][48]
  * [extract::rejection::MissingPathParams][49]
  * [extract::rejection::NestedPathRejection][50]
  * [extract::rejection::UnknownBodyError][51]
  * [extract::ws::CloseFrame][52]
  * [extract::ws::DefaultOnFailedUpgrade][53]
  * [extract::ws::Utf8Bytes][54]
  * [extract::ws::WebSocket][55]
  * [extract::ws::WebSocketUpgrade][56]
  * [extract::ws::rejection::ConnectionNotUpgradable][57]
  * [extract::ws::rejection::InvalidConnectionHeader][58]
  * [extract::ws::rejection::InvalidProtocolPseudoheader][59]
  * [extract::ws::rejection::InvalidUpgradeHeader][60]
  * [extract::ws::rejection::InvalidWebSocketVersionHeader][61]
  * [extract::ws::rejection::MethodNotConnect][62]
  * [extract::ws::rejection::MethodNotGet][63]
  * [extract::ws::rejection::WebSocketKeyHeaderMissing][64]
  * [handler::HandlerService][65]
  * [handler::Layered][66]
  * [handler::future::IntoServiceFuture][67]
  * [handler::future::LayeredFuture][68]
  * [middleware::AddExtension][69]
  * [middleware::FromExtractor][70]
  * [middleware::FromExtractorLayer][71]
  * [middleware::FromFn][72]
  * [middleware::FromFnLayer][73]
  * [middleware::MapRequest][74]
  * [middleware::MapRequestLayer][75]
  * [middleware::MapResponse][76]
  * [middleware::MapResponseLayer][77]
  * [middleware::Next][78]
  * [middleware::ResponseAxumBody][79]
  * [middleware::ResponseAxumBodyFuture][80]
  * [middleware::ResponseAxumBodyLayer][81]
  * [middleware::future::FromExtractorResponseFuture][82]
  * [middleware::future::FromFnResponseFuture][83]
  * [middleware::future::MapRequestResponseFuture][84]
  * [middleware::future::MapResponseResponseFuture][85]
  * [response::AppendHeaders][86]
  * [response::ErrorResponse][87]
  * [response::Html][88]
  * [response::IntoResponseFailed][89]
  * [response::NoContent][90]
  * [response::Redirect][91]
  * [response::ResponseParts][92]
  * [response::Sse][93]
  * [response::sse::Event][94]
  * [response::sse::EventDataWriter][95]
  * [response::sse::KeepAlive][96]
  * [response::sse::KeepAliveStream][97]
  * [response::sse::Sse][98]
  * [routing::IntoMakeService][99]
  * [routing::MethodFilter][100]
  * [routing::Route][101]
  * [routing::Router][102]
  * [routing::RouterAsService][103]
  * [routing::RouterIntoService][104]
  * [routing::future::InfallibleRouteFuture][105]
  * [routing::future::IntoMakeServiceFuture][106]
  * [routing::future::RouteFuture][107]
  * [routing::method_routing::MethodRouter][108]
  * [serve::ConnLimiter][109]
  * [serve::ConnLimiterIo][110]
  * [serve::IncomingStream][111]
  * [serve::Serve][112]
  * [serve::TapIo][113]
  * [serve::WithGracefulShutdown][114]



### Enums

  * [extract::multipart::MultipartRejection][115]
  * [extract::path::ErrorKind][116]
  * [extract::rejection::BytesRejection][117]
  * [extract::rejection::ExtensionRejection][118]
  * [extract::rejection::FailedToBufferBody][119]
  * [extract::rejection::FormRejection][120]
  * [extract::rejection::JsonRejection][121]
  * [extract::rejection::MatchedPathRejection][122]
  * [extract::rejection::PathRejection][123]
  * [extract::rejection::QueryRejection][124]
  * [extract::rejection::RawFormRejection][125]
  * [extract::rejection::RawPathParamsRejection][126]
  * [extract::rejection::StringRejection][127]
  * [extract::ws::Message][128]
  * [extract::ws::rejection::WebSocketUpgradeRejection][129]



### Traits

  * [RequestExt][130]
  * [RequestPartsExt][131]
  * [ServiceExt][132]
  * [extract::FromRef][133]
  * [extract::FromRequest][134]
  * [extract::FromRequestParts][135]
  * [extract::OptionalFromRequest][136]
  * [extract::OptionalFromRequestParts][137]
  * [extract::connect_info::Connected][138]
  * [extract::ws::OnFailedUpgrade][139]
  * [handler::Handler][140]
  * [handler::HandlerWithoutStateExt][141]
  * [middleware::IntoMapRequestResult][142]
  * [response::IntoResponse][143]
  * [response::IntoResponseParts][144]
  * [serve::Listener][145]
  * [serve::ListenerExt][146]



### Attribute Macros

  * [debug_handler][147]
  * [debug_middleware][148]



### Derive Macros

  * [extract::FromRef][149]
  * [extract::FromRequest][150]
  * [extract::FromRequestParts][151]



### Functions

  * [body::to_bytes][152]
  * [middleware::from_extractor][153]
  * [middleware::from_extractor_with_state][154]
  * [middleware::from_fn][155]
  * [middleware::from_fn_with_state][156]
  * [middleware::map_request][157]
  * [middleware::map_request_with_state][158]
  * [middleware::map_response][159]
  * [middleware::map_response_with_state][160]
  * [routing::method_routing::any][161]
  * [routing::method_routing::any_service][162]
  * [routing::method_routing::connect][163]
  * [routing::method_routing::connect_service][164]
  * [routing::method_routing::delete][165]
  * [routing::method_routing::delete_service][166]
  * [routing::method_routing::get][167]
  * [routing::method_routing::get_service][168]
  * [routing::method_routing::head][169]
  * [routing::method_routing::head_service][170]
  * [routing::method_routing::on][171]
  * [routing::method_routing::on_service][172]
  * [routing::method_routing::options][173]
  * [routing::method_routing::options_service][174]
  * [routing::method_routing::patch][175]
  * [routing::method_routing::patch_service][176]
  * [routing::method_routing::post][177]
  * [routing::method_routing::post_service][178]
  * [routing::method_routing::put][179]
  * [routing::method_routing::put_service][180]
  * [routing::method_routing::trace][181]
  * [routing::method_routing::trace_service][182]
  * [serve][183]
  * [serve::serve][184]



### Type Aliases

  * [BoxError][185]
  * [extract::Request][186]
  * [extract::ws::CloseCode][187]
  * [response::Response][188]
  * [response::Result][189]



### Constants

  * [extract::ws::close_code::ABNORMAL][190]
  * [extract::ws::close_code::AGAIN][191]
  * [extract::ws::close_code::AWAY][192]
  * [extract::ws::close_code::ERROR][193]
  * [extract::ws::close_code::EXTENSION][194]
  * [extract::ws::close_code::INVALID][195]
  * [extract::ws::close_code::NORMAL][196]
  * [extract::ws::close_code::POLICY][197]
  * [extract::ws::close_code::PROTOCOL][198]
  * [extract::ws::close_code::RESTART][199]
  * [extract::ws::close_code::SIZE][200]
  * [extract::ws::close_code::STATUS][201]
  * [extract::ws::close_code::UNSUPPORTED][202]



   [1]: ../axum/index.html
   [2]: struct.Error.html
   [3]: struct.Extension.html
   [4]: struct.Form.html
   [5]: struct.Json.html
   [6]: struct.Router.html
   [7]: body/struct.Body.html
   [8]: body/struct.BodyDataStream.html
   [9]: error_handling/struct.HandleError.html
   [10]: error_handling/struct.HandleErrorLayer.html
   [11]: error_handling/future/struct.HandleErrorFuture.html
   [12]: extract/struct.ConnectInfo.html
   [13]: extract/struct.DefaultBodyLimit.html
   [14]: extract/struct.MatchedPath.html
   [15]: extract/struct.Multipart.html
   [16]: extract/struct.NestedPath.html
   [17]: extract/struct.OriginalUri.html
   [18]: extract/struct.Path.html
   [19]: extract/struct.Query.html
   [20]: extract/struct.RawForm.html
   [21]: extract/struct.RawPathParams.html
   [22]: extract/struct.RawQuery.html
   [23]: extract/struct.State.html
   [24]: extract/struct.WebSocketUpgrade.html
   [25]: extract/connect_info/struct.ConnectInfo.html
   [26]: extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html
   [27]: extract/connect_info/struct.MockConnectInfo.html
   [28]: extract/connect_info/struct.ResponseFuture.html
   [29]: extract/multipart/struct.Field.html
   [30]: extract/multipart/struct.InvalidBoundary.html
   [31]: extract/multipart/struct.Multipart.html
   [32]: extract/multipart/struct.MultipartError.html
   [33]: extract/path/struct.FailedToDeserializePathParams.html
   [34]: extract/path/struct.InvalidUtf8InPathParam.html
   [35]: extract/path/struct.Path.html
   [36]: extract/path/struct.RawPathParams.html
   [37]: extract/path/struct.RawPathParamsIter.html
   [38]: extract/rejection/struct.FailedToDeserializeForm.html
   [39]: extract/rejection/struct.FailedToDeserializeFormBody.html
   [40]: extract/rejection/struct.FailedToDeserializeQueryString.html
   [41]: extract/rejection/struct.InvalidFormContentType.html
   [42]: extract/rejection/struct.InvalidUtf8.html
   [43]: extract/rejection/struct.JsonDataError.html
   [44]: extract/rejection/struct.JsonSyntaxError.html
   [45]: extract/rejection/struct.LengthLimitError.html
   [46]: extract/rejection/struct.MatchedPathMissing.html
   [47]: extract/rejection/struct.MissingExtension.html
   [48]: extract/rejection/struct.MissingJsonContentType.html
   [49]: extract/rejection/struct.MissingPathParams.html
   [50]: extract/rejection/struct.NestedPathRejection.html
   [51]: extract/rejection/struct.UnknownBodyError.html
   [52]: extract/ws/struct.CloseFrame.html
   [53]: extract/ws/struct.DefaultOnFailedUpgrade.html
   [54]: extract/ws/struct.Utf8Bytes.html
   [55]: extract/ws/struct.WebSocket.html
   [56]: extract/ws/struct.WebSocketUpgrade.html
   [57]: extract/ws/rejection/struct.ConnectionNotUpgradable.html
   [58]: extract/ws/rejection/struct.InvalidConnectionHeader.html
   [59]: extract/ws/rejection/struct.InvalidProtocolPseudoheader.html
   [60]: extract/ws/rejection/struct.InvalidUpgradeHeader.html
   [61]: extract/ws/rejection/struct.InvalidWebSocketVersionHeader.html
   [62]: extract/ws/rejection/struct.MethodNotConnect.html
   [63]: extract/ws/rejection/struct.MethodNotGet.html
   [64]: extract/ws/rejection/struct.WebSocketKeyHeaderMissing.html
   [65]: handler/struct.HandlerService.html
   [66]: handler/struct.Layered.html
   [67]: handler/future/struct.IntoServiceFuture.html
   [68]: handler/future/struct.LayeredFuture.html
   [69]: middleware/struct.AddExtension.html
   [70]: middleware/struct.FromExtractor.html
   [71]: middleware/struct.FromExtractorLayer.html
   [72]: middleware/struct.FromFn.html
   [73]: middleware/struct.FromFnLayer.html
   [74]: middleware/struct.MapRequest.html
   [75]: middleware/struct.MapRequestLayer.html
   [76]: middleware/struct.MapResponse.html
   [77]: middleware/struct.MapResponseLayer.html
   [78]: middleware/struct.Next.html
   [79]: middleware/struct.ResponseAxumBody.html
   [80]: middleware/struct.ResponseAxumBodyFuture.html
   [81]: middleware/struct.ResponseAxumBodyLayer.html
   [82]: middleware/future/struct.FromExtractorResponseFuture.html
   [83]: middleware/future/struct.FromFnResponseFuture.html
   [84]: middleware/future/struct.MapRequestResponseFuture.html
   [85]: middleware/future/struct.MapResponseResponseFuture.html
   [86]: response/struct.AppendHeaders.html
   [87]: response/struct.ErrorResponse.html
   [88]: response/struct.Html.html
   [89]: response/struct.IntoResponseFailed.html
   [90]: response/struct.NoContent.html
   [91]: response/struct.Redirect.html
   [92]: response/struct.ResponseParts.html
   [93]: response/struct.Sse.html
   [94]: response/sse/struct.Event.html
   [95]: response/sse/struct.EventDataWriter.html
   [96]: response/sse/struct.KeepAlive.html
   [97]: response/sse/struct.KeepAliveStream.html
   [98]: response/sse/struct.Sse.html
   [99]: routing/struct.IntoMakeService.html
   [100]: routing/struct.MethodFilter.html
   [101]: routing/struct.Route.html
   [102]: routing/struct.Router.html
   [103]: routing/struct.RouterAsService.html
   [104]: routing/struct.RouterIntoService.html
   [105]: routing/future/struct.InfallibleRouteFuture.html
   [106]: routing/future/struct.IntoMakeServiceFuture.html
   [107]: routing/future/struct.RouteFuture.html
   [108]: routing/method_routing/struct.MethodRouter.html
   [109]: serve/struct.ConnLimiter.html
   [110]: serve/struct.ConnLimiterIo.html
   [111]: serve/struct.IncomingStream.html
   [112]: serve/struct.Serve.html
   [113]: serve/struct.TapIo.html
   [114]: serve/struct.WithGracefulShutdown.html
   [115]: extract/multipart/enum.MultipartRejection.html
   [116]: extract/path/enum.ErrorKind.html
   [117]: extract/rejection/enum.BytesRejection.html
   [118]: extract/rejection/enum.ExtensionRejection.html
   [119]: extract/rejection/enum.FailedToBufferBody.html
   [120]: extract/rejection/enum.FormRejection.html
   [121]: extract/rejection/enum.JsonRejection.html
   [122]: extract/rejection/enum.MatchedPathRejection.html
   [123]: extract/rejection/enum.PathRejection.html
   [124]: extract/rejection/enum.QueryRejection.html
   [125]: extract/rejection/enum.RawFormRejection.html
   [126]: extract/rejection/enum.RawPathParamsRejection.html
   [127]: extract/rejection/enum.StringRejection.html
   [128]: extract/ws/enum.Message.html
   [129]: extract/ws/rejection/enum.WebSocketUpgradeRejection.html
   [130]: trait.RequestExt.html
   [131]: trait.RequestPartsExt.html
   [132]: trait.ServiceExt.html
   [133]: extract/trait.FromRef.html
   [134]: extract/trait.FromRequest.html
   [135]: extract/trait.FromRequestParts.html
   [136]: extract/trait.OptionalFromRequest.html
   [137]: extract/trait.OptionalFromRequestParts.html
   [138]: extract/connect_info/trait.Connected.html
   [139]: extract/ws/trait.OnFailedUpgrade.html
   [140]: handler/trait.Handler.html
   [141]: handler/trait.HandlerWithoutStateExt.html
   [142]: middleware/trait.IntoMapRequestResult.html
   [143]: response/trait.IntoResponse.html
   [144]: response/trait.IntoResponseParts.html
   [145]: serve/trait.Listener.html
   [146]: serve/trait.ListenerExt.html
   [147]: attr.debug_handler.html
   [148]: attr.debug_middleware.html
   [149]: extract/derive.FromRef.html
   [150]: extract/derive.FromRequest.html
   [151]: extract/derive.FromRequestParts.html
   [152]: body/fn.to_bytes.html
   [153]: middleware/fn.from_extractor.html
   [154]: middleware/fn.from_extractor_with_state.html
   [155]: middleware/fn.from_fn.html
   [156]: middleware/fn.from_fn_with_state.html
   [157]: middleware/fn.map_request.html
   [158]: middleware/fn.map_request_with_state.html
   [159]: middleware/fn.map_response.html
   [160]: middleware/fn.map_response_with_state.html
   [161]: routing/method_routing/fn.any.html
   [162]: routing/method_routing/fn.any_service.html
   [163]: routing/method_routing/fn.connect.html
   [164]: routing/method_routing/fn.connect_service.html
   [165]: routing/method_routing/fn.delete.html
   [166]: routing/method_routing/fn.delete_service.html
   [167]: routing/method_routing/fn.get.html
   [168]: routing/method_routing/fn.get_service.html
   [169]: routing/method_routing/fn.head.html
   [170]: routing/method_routing/fn.head_service.html
   [171]: routing/method_routing/fn.on.html
   [172]: routing/method_routing/fn.on_service.html
   [173]: routing/method_routing/fn.options.html
   [174]: routing/method_routing/fn.options_service.html
   [175]: routing/method_routing/fn.patch.html
   [176]: routing/method_routing/fn.patch_service.html
   [177]: routing/method_routing/fn.post.html
   [178]: routing/method_routing/fn.post_service.html
   [179]: routing/method_routing/fn.put.html
   [180]: routing/method_routing/fn.put_service.html
   [181]: routing/method_routing/fn.trace.html
   [182]: routing/method_routing/fn.trace_service.html
   [183]: fn.serve.html
   [184]: serve/fn.serve.html
   [185]: type.BoxError.html
   [186]: extract/type.Request.html
   [187]: extract/ws/type.CloseCode.html
   [188]: response/type.Response.html
   [189]: response/type.Result.html
   [190]: extract/ws/close_code/constant.ABNORMAL.html
   [191]: extract/ws/close_code/constant.AGAIN.html
   [192]: extract/ws/close_code/constant.AWAY.html
   [193]: extract/ws/close_code/constant.ERROR.html
   [194]: extract/ws/close_code/constant.EXTENSION.html
   [195]: extract/ws/close_code/constant.INVALID.html
   [196]: extract/ws/close_code/constant.NORMAL.html
   [197]: extract/ws/close_code/constant.POLICY.html
   [198]: extract/ws/close_code/constant.PROTOCOL.html
   [199]: extract/ws/close_code/constant.RESTART.html
   [200]: extract/ws/close_code/constant.SIZE.html
   [201]: extract/ws/close_code/constant.STATUS.html
   [202]: extract/ws/close_code/constant.UNSUPPORTED.html

