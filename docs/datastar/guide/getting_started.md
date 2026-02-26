# Getting Started

- Source: `https://data-star.dev/guide/getting_started`
- Retrieved: `2026-02-26 17:55 UTC`
- Section: Guide

Datastar simplifies frontend development, allowing you to build backend-driven, interactive UIs using a [hypermedia-first](https://hypermedia.systems/hypermedia-a-reintroduction/) approach that extends and enhances HTML.

Datastar provides backend reactivity like [htmx](https://htmx.org/) and frontend reactivity like [Alpine.js](https://alpinejs.dev/) in a lightweight frontend framework that doesn’t require any npm packages or other dependencies. It provides two primary functions:

  1. Modify the DOM and state by sending events from your backend.
  2. Build reactivity into your frontend using standard `data-*` HTML attributes.

> Other useful resources include an AI-generated [deep wiki](https://deepwiki.com/starfederation/datastar), LLM-ingestible [code samples](https://context7.com/websites/data-star_dev), and [single-page docs](/docs).

## Installation 

The quickest way to use Datastar is to include it using a `script` tag that fetches it from a CDN.
    
    
    <script type="module" src="https://cdn.jsdelivr.net/gh/starfederation/datastar@1.0.0-RC.7/bundles/datastar.js"></script>

If you prefer to host the file yourself, download the [script](https://cdn.jsdelivr.net/gh/starfederation/datastar@1.0.0-RC.7/bundles/datastar.js) or create your own bundle using the [bundler](/bundler), then include it from the appropriate path.
    
    
    <script type="module" src="/path/to/datastar.js"></script>

To import Datastar using a package manager such as npm, Deno, or Bun, you can use an import statement.
    
    
    // @ts-expect-error (only required for TypeScript projects)
    import 'https://cdn.jsdelivr.net/gh/starfederation/datastar@1.0.0-RC.7/bundles/datastar.js'

## `data-*`

At the core of Datastar are `[data-*](https://developer.mozilla.org/en-US/docs/Web/HTML/How_to/Use_data_attributes)` HTML attributes (hence the name). They allow you to add reactivity to your frontend and interact with your backend in a declarative way.

> The Datastar [VSCode extension](https://marketplace.visualstudio.com/items?itemName=starfederation.datastar-vscode) and [IntelliJ plugin](https://plugins.jetbrains.com/plugin/26072-datastar-support) provide autocompletion for all available `data-*` attributes.

The [`data-on`](../reference/attributes.md#data-on) attribute can be used to attach an event listener to an element and execute an expression whenever the event is triggered. The value of the attribute is a [Datastar expression](datastar_expressions.md) in which JavaScript can be used.
    
    
    <button data-on:click="alert('I’m sorry, Dave. I’m afraid I can’t do that.')">
        Open the pod bay doors, HAL.
    </button>

Demo

Open the pod bay doors, HAL. 

We’ll explore more data attributes in the [next section of the guide](reactive_signals.md).

## Patching Elements 

With Datastar, the backend _drives_ the frontend by **patching** (adding, updating and removing) HTML elements in the DOM.

Datastar receives elements from the backend and manipulates the DOM using a morphing strategy (by default). Morphing ensures that only modified parts of the DOM are updated, and that only data attributes that have changed are [reapplied](../reference/attributes.md#attribute-evaluation-order), preserving state and improving performance.

Datastar provides [actions](../reference/actions.md#backend-actions) for sending requests to the backend. The [`@get()`](../reference/actions.md#get) action sends a `GET` request to the provided URL using a [fetch](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) request.
    
    
    <button data-on:click="@get('/endpoint')">
        Open the pod bay doors, HAL.
    </button>
    <div id="hal"></div>

> Actions in Datastar are helper functions that have the syntax `@actionName()`. Read more about actions in the [reference](../reference/actions.md).

If the response has a `content-type` of `text/html`, the top-level HTML elements will be morphed into the existing DOM based on the element IDs. 
    
    
    <div id="hal">
        I’m sorry, Dave. I’m afraid I can’t do that.
    </div>

We call this a “Patch Elements” event because multiple elements can be patched into the DOM at once.

Demo

Open the pod bay doors, HAL. `Waiting for an order...`

In the example above, the DOM must contain an element with a `hal` ID in order for morphing to work. Other [patching strategies](../reference/sse_events.md#datastar-patch-elements) are available, but morph is the best and simplest choice in most scenarios.

If the response has a `content-type` of `text/event-stream`, it can contain zero or more [SSE events](../reference/sse_events.md). The example above can be replicated using a `datastar-patch-elements` SSE event.
    
    
    event: datastar-patch-elements
    data: elements <div id="hal">
    data: elements     I’m sorry, Dave. I’m afraid I can’t do that.
    data: elements </div>

Because we can send as many events as we want in a stream, and because it can be a long-lived connection, we can extend the example above to first send HAL’s response and then, after a few seconds, reset the text.
    
    
    event: datastar-patch-elements
    data: elements <div id="hal">
    data: elements     I’m sorry, Dave. I’m afraid I can’t do that.
    data: elements </div>
    
    event: datastar-patch-elements
    data: elements <div id="hal">
    data: elements     Waiting for an order...
    data: elements </div>

Demo

Open the pod bay doors, HAL. `Waiting for an order...`

Here’s the code to generate the SSE events above using the SDKs.
    
    
    ;; Import the SDK's api and your adapter
    (require
     '[starfederation.datastar.clojure.api :as d*]
     '[starfederation.datastar.clojure.adapter.http-kit :refer [->sse-response on-open]])
    
    ;; in a ring handler
    (defn handler [request]
      ;; Create an SSE response
      (->sse-response request
                      {on-open
                       (fn [sse]
                         ;; Patches elements into the DOM
                         (d*/patch-elements! sse
                                             "<div id=\"hal\">I’m sorry, Dave. I’m afraid I can’t do that.</div>")
                         (Thread/sleep 1000)
                         (d*/patch-elements! sse
                                             "<div id=\"hal\">Waiting for an order...</div>"))}))
    
    
    using StarFederation.Datastar.DependencyInjection;
    
    // Adds Datastar as a service
    builder.Services.AddDatastar();
    
    app.MapGet("/", async (IDatastarService datastarService) =>
    {
        // Patches elements into the DOM.
        await datastarService.PatchElementsAsync(@"<div id=""hal"">I’m sorry, Dave. I’m afraid I can’t do that.</div>");
    
        await Task.Delay(TimeSpan.FromSeconds(1));
    
        await datastarService.PatchElementsAsync(@"<div id=""hal"">Waiting for an order...</div>");
    });
    
    
    import (
        "github.com/starfederation/datastar-go/datastar"
        time
    )
    
    // Creates a new `ServerSentEventGenerator` instance.
    sse := datastar.NewSSE(w,r)
    
    // Patches elements into the DOM.
    sse.PatchElements(
        `<div id="hal">I’m sorry, Dave. I’m afraid I can’t do that.</div>`
    )
    
    time.Sleep(1 * time.Second)
    
    sse.PatchElements(
        `<div id="hal">Waiting for an order...</div>`
    )
    
    
    import starfederation.datastar.utils.ServerSentEventGenerator;
    
    // Creates a new `ServerSentEventGenerator` instance.
    AbstractResponseAdapter responseAdapter = new HttpServletResponseAdapter(response);
    ServerSentEventGenerator generator = new ServerSentEventGenerator(responseAdapter);
    
    // Patches elements into the DOM.
    generator.send(PatchElements.builder()
        .data("<div id=\"hal\">I’m sorry, Dave. I’m afraid I can’t do that.</div>")
        .build()
    );
    
    Thread.sleep(1000);
    
    generator.send(PatchElements.builder()
        .data("<div id=\"hal\">Waiting for an order...</div>")
        .build()
    );
    
    
    val generator = ServerSentEventGenerator(response)
    
    generator.patchElements(
        elements = """<div id="hal">I’m sorry, Dave. I’m afraid I can’t do that.</div>""",
    )
    
    Thread.sleep(ONE_SECOND)
    
    generator.patchElements(
        elements = """<div id="hal">Waiting for an order...</div>""",
    )
    
    
    use starfederation\datastar\ServerSentEventGenerator;
    
    // Creates a new `ServerSentEventGenerator` instance.
    $sse = new ServerSentEventGenerator();
    
    // Patches elements into the DOM.
    $sse->patchElements(
        '<div id="hal">I’m sorry, Dave. I’m afraid I can’t do that.</div>'
    );
    
    sleep(1);
    
    $sse->patchElements(
        '<div id="hal">Waiting for an order...</div>'
    );
    
    
    from datastar_py import ServerSentEventGenerator as SSE
    from datastar_py.sanic import datastar_response
    
    @app.get('/open-the-bay-doors')
    @datastar_response
    async def open_doors(request):
        yield SSE.patch_elements('<div id="hal">I’m sorry, Dave. I’m afraid I can’t do that.</div>')
        await asyncio.sleep(1)
        yield SSE.patch_elements('<div id="hal">Waiting for an order...</div>')
    
    
    require 'datastar'
    
    # Create a Datastar::Dispatcher instance
    
    datastar = Datastar.new(request:, response:)
    
    # In a Rack handler, you can instantiate from the Rack env
    # datastar = Datastar.from_rack_env(env)
    
    # Start a streaming response
    datastar.stream do |sse|
      # Patches elements into the DOM.
      sse.patch_elements %(<div id="hal">I’m sorry, Dave. I’m afraid I can’t do that.</div>)
    
      sleep 1
      
      sse.patch_elements %(<div id="hal">Waiting for an order...</div>)
    end
    
    
    use async_stream::stream;
    use datastar::prelude::*;
    use std::thread;
    use std::time::Duration;
    
    Sse(stream! {
        // Patches elements into the DOM.
        yield PatchElements::new("<div id='hal'>I’m sorry, Dave. I’m afraid I can’t do that.</div>").into();
    
        thread::sleep(Duration::from_secs(1));
        
        yield PatchElements::new("<div id='hal'>Waiting for an order...</div>").into();
    })
    
    
    // Creates a new `ServerSentEventGenerator` instance (this also sends required headers)
    ServerSentEventGenerator.stream(req, res, (stream) => {
        // Patches elements into the DOM.
        stream.patchElements(`<div id="hal">I’m sorry, Dave. I’m afraid I can’t do that.</div>`);
    
        setTimeout(() => {
            stream.patchElements(`<div id="hal">Waiting for an order...</div>`);
        }, 1000);
    });

> In addition to your browser’s dev tools, the [Datastar Inspector](/datastar_pro#datastar-inspector) can be used to monitor and inspect SSE events received by Datastar.

We’ll cover event streams and [SSE events](../reference/sse_events.md) in more detail [later in the guide](backend_requests.md), but as you can see, they are just plain text events with a special syntax, made simpler by the [SDKs](../reference/sdks.md).
