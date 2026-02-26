# Backend Requests

- Source: `https://data-star.dev/guide/backend_requests`
- Retrieved: `2026-02-26 17:55 UTC`
- Section: Guide

Between [attributes](../reference/attributes.md) and [actions](../reference/actions.md), Datastar provides you with everything you need to build hypermedia-driven applications. Using this approach, the backend drives state to the frontend and acts as the single source of truth, determining what actions the user can take next.

## Sending Signals 

By default, all signals (except for local signals whose keys begin with an underscore) are sent in an object with every backend request. When using a `GET` request, the signals are sent as a `datastar` query parameter, otherwise they are sent as a JSON body.

By sending **all** signals in every request, the backend has full access to the frontend state. This is by design. It is **not** recommended to send partial signals, but if you must, you can use the [`filterSignals`](../reference/actions.md#filterSignals) option to filter the signals sent to the backend.

### Nesting Signals 

Signals can be nested, making it easier to target signals in a more granular way on the backend.

Using dot-notation:
    
    
    <div data-signals:foo.bar="1"></div>

Using object syntax:
    
    
    <div data-signals="{foo: {bar: 1}}"></div>

Using two-way binding:
    
    
    <input data-bind:foo.bar />

A practical use-case of nested signals is when you have repetition of state on a page. The following example tracks the open/closed state of a menu on both desktop and mobile devices, and the [toggleAll()](../reference/actions.md#toggleAll) action to toggle the state of all menus at once.
    
    
    <div data-signals="{menu: {isOpen: {desktop: false, mobile: false}}}">
        <button data-on:click="@toggleAll({include: /^menu\.isOpen\./})">
            Open/close menu
        </button>
    </div>

## Reading Signals 

To read signals from the backend, JSON decode the `datastar` query param for `GET` requests, and the request body for all other methods.

All [SDKs](../reference/sdks.md) provide a helper function to read signals. Here’s how you would read the nested signal `foo.bar` from an incoming request.

No example found for Clojure
    
    
    using StarFederation.Datastar.DependencyInjection;
    
    // Adds Datastar as a service
    builder.Services.AddDatastar();
    
    public record Signals
    {
        [JsonPropertyName("foo")] [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
        public FooSignals? Foo { get; set; } = null;
    
        public record FooSignals
        {
            [JsonPropertyName("bar")] [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
            public string? Bar { get; set; }
        }
    }
    
    app.MapGet("/read-signals", async (IDatastarService datastarService) =>
    {
        Signals? mySignals = await datastarService.ReadSignalsAsync<Signals>();
        var bar = mySignals?.Foo?.Bar;
    });
    
    
    import ("github.com/starfederation/datastar-go/datastar")
    
    type Signals struct {
        Foo struct {
            Bar string `json:"bar"`
        } `json:"foo"`
    }
    
    signals := &Signals{}
    if err := datastar.ReadSignals(request, signals); err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }

No example found for Java
    
    
    @Serializable
    data class Signals(
        val foo: String,
    )
    
    val jsonUnmarshaller: JsonUnmarshaller<Signals> = { json -> Json.decodeFromString(json) }
    
    val request: Request =
        postRequest(
            body =
                """
                {
                    "foo": "bar"
                }
                """.trimIndent(),
        )
    
    val signals = readSignals(request, jsonUnmarshaller)
    
    
    use starfederation\datastar\ServerSentEventGenerator;
    
    // Reads all signals from the request.
    $signals = ServerSentEventGenerator::readSignals();
    
    
    from datastar_py.fastapi import datastar_response, read_signals
    
    @app.get("/updates")
    @datastar_response
    async def updates(request: Request):
        # Retrieve a dictionary with the current state of the signals from the frontend
        signals = await read_signals(request)
    
    
    # Setup with request
    datastar = Datastar.new(request:, response:)
    
    # Read signals
    some_signal = datastar.signals[:some_signal]

No example found for Rust

No example found for TypeScript

## SSE Events 

Datastar can stream zero or more [Server-Sent Events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events) (SSE) from the web server to the browser. There’s no special backend plumbing required to use SSE, just some special syntax. Fortunately, SSE is straightforward and [provides us with some advantages](/essays/event_streams_all_the_way_down), in addition to allowing us to send multiple events in a single response (in contrast to sending `text/html` or `application/json` responses).

First, set up your backend in the language of your choice. Familiarize yourself with [sending SSE events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#sending_events_from_the_server), or use one of the backend [SDKs](../reference/sdks.md) to get up and running even faster. We’re going to use the SDKs in the examples below, which set the appropriate headers and format the events for us.

The following code would exist in a controller action endpoint in your backend.
    
    
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
                                             "<div id=\"question\">What do you put in a toaster?</div>")
    
                         ;; Patches signals
                         (d*/patch-signals! sse "{response: '', answer: 'bread'}"))}))
    
    
    using StarFederation.Datastar.DependencyInjection;
    
    // Adds Datastar as a service
    builder.Services.AddDatastar();
    
    app.MapGet("/", async (IDatastarService datastarService) =>
    {
        // Patches elements into the DOM.
        await datastarService.PatchElementsAsync(@"<div id=""question"">What do you put in a toaster?</div>");
    
        // Patches signals.
        await datastarService.PatchSignalsAsync(new { response = "", answer = "bread" });
    });
    
    
    import ("github.com/starfederation/datastar-go/datastar")
    
    // Creates a new `ServerSentEventGenerator` instance.
    sse := datastar.NewSSE(w,r)
    
    // Patches elements into the DOM.
    sse.PatchElements(
        `<div id="question">What do you put in a toaster?</div>`
    )
    
    // Patches signals.
    sse.PatchSignals([]byte(`{response: '', answer: 'bread'}`))
    
    
    import starfederation.datastar.utils.ServerSentEventGenerator;
    
    // Creates a new `ServerSentEventGenerator` instance.
    AbstractResponseAdapter responseAdapter = new HttpServletResponseAdapter(response);
    ServerSentEventGenerator generator = new ServerSentEventGenerator(responseAdapter);
    
    // Patches elements into the DOM.
    generator.send(PatchElements.builder()
        .data("<div id=\"question\">What do you put in a toaster?</div>")
        .build()
    );
    
    // Patches signals.
    generator.send(PatchSignals.builder()
        .data("{\"response\": \"\", \"answer\": \"\"}")
        .build()
    );
    
    
    val generator = ServerSentEventGenerator(response)
    
    generator.patchElements(
        elements = """<div id="question">What do you put in a toaster?</div>""",
    )
    
    generator.patchSignals(
        signals = """{"response": "", "answer": "bread"}""",
    )
    
    
    use starfederation\datastar\ServerSentEventGenerator;
    
    // Creates a new `ServerSentEventGenerator` instance.
    $sse = new ServerSentEventGenerator();
    
    // Patches elements into the DOM.
    $sse->patchElements(
        '<div id="question">What do you put in a toaster?</div>'
    );
    
    // Patches signals.
    $sse->patchSignals(['response' => '', 'answer' => 'bread']);
    
    
    from datastar_py import ServerSentEventGenerator as SSE
    from datastar_py.litestar import DatastarResponse
    
    async def endpoint():
        return DatastarResponse([
            SSE.patch_elements('<div id="question">What do you put in a toaster?</div>'),
            SSE.patch_signals({"response": "", "answer": "bread"})
        ])
    
    
    require 'datastar'
    
    # Create a Datastar::Dispatcher instance
    
    datastar = Datastar.new(request:, response:)
    
    # In a Rack handler, you can instantiate from the Rack env
    # datastar = Datastar.from_rack_env(env)
    
    # Start a streaming response
    datastar.stream do |sse|
      # Patches elements into the DOM
      sse.patch_elements %(<div id="question">What do you put in a toaster?</div>)
    
      # Patches signals
      sse.patch_signals(response: '', answer: 'bread')
    end
    
    
    use datastar::prelude::*;
    use async_stream::stream;
    
    Sse(stream! {
        // Patches elements into the DOM.
        yield PatchElements::new("<div id='question'>What do you put in a toaster?</div>").into();
    
        // Patches signals.
        yield PatchSignals::new("{response: '', answer: 'bread'}").into();
    })
    
    
    // Creates a new `ServerSentEventGenerator` instance (this also sends required headers)
    ServerSentEventGenerator.stream(req, res, (stream) => {
          // Patches elements into the DOM.
         stream.patchElements(`<div id="question">What do you put in a toaster?</div>`);
    
         // Patches signals.
         stream.patchSignals({'response':  '', 'answer': 'bread'});
    });

The `PatchElements()` function updates the provided HTML element into the DOM, replacing the element with `id="question"`. An element with the ID `question` must _already_ exist in the DOM.

The `PatchSignals()` function updates the `response` and `answer` signals into the frontend signals.

With our backend in place, we can now use the `data-on:click` attribute to trigger the [`@get()`](../reference/actions.md#get) action, which sends a `GET` request to the `/actions/quiz` endpoint on the server when a button is clicked.
    
    
    <div
        data-signals="{response: '', answer: ''}"
        data-computed:correct="$response.toLowerCase() == $answer"
    >
        <div id="question"></div>
        <button data-on:click="@get('/actions/quiz')">Fetch a question</button>
        <button
            data-show="$answer != ''"
            data-on:click="$response = prompt('Answer:') ?? ''"
        >
            BUZZ
        </button>
        <div data-show="$response != ''">
            You answered “<span data-text="$response"></span>”.
            <span data-show="$correct">That is correct ✅</span>
            <span data-show="!$correct">
            The correct answer is “<span data-text="$answer"></span>” 🤷
            </span>
        </div>
    </div>

Now when the `Fetch a question` button is clicked, the server will respond with an event to modify the `question` element in the DOM and an event to modify the `response` and `answer` signals. We’re driving state from the backend!

Demo

...

Fetch a question BUZZ

You answered “”. That is correct ✅ The correct answer is “” 🤷

### `data-indicator`

The [`data-indicator`](../reference/attributes.md#data-indicator) attribute sets the value of a signal to `true` while the request is in flight, otherwise `false`. We can use this signal to show a loading indicator, which may be desirable for slower responses.
    
    
    <div id="question"></div>
    <button
        data-on:click="@get('/actions/quiz')"
        data-indicator:fetching
    >
        Fetch a question
    </button>
    <div data-class:loading="$fetching" class="indicator"></div>

Demo

...

Fetch a question

## Backend Actions 

We’re not limited to sending just `GET` requests. Datastar provides [backend actions](../reference/actions.md#backend-actions) for each of the methods available: `@get()`, `@post()`, `@put()`, `@patch()` and `@delete()`.

Here’s how we can send an answer to the server for processing, using a `POST` request.
    
    
    <button data-on:click="@post('/actions/quiz')">
        Submit answer
    </button>

One of the benefits of using SSE is that we can send multiple events (patch elements and patch signals) in a single response.
    
    
    (d*/patch-elements! sse "<div id=\"question\">...</div>")
    (d*/patch-elements! sse "<div id=\"instructions\">...</div>")
    (d*/patch-signals! sse "{answer: '...', prize: '...'}")
    
    
    datastarService.PatchElementsAsync(@"<div id=""question"">...</div>");
    datastarService.PatchElementsAsync(@"<div id=""instructions"">...</div>");
    datastarService.PatchSignalsAsync(new { answer = "...", prize = "..." } );
    
    
    sse.PatchElements(`<div id="question">...</div>`)
    sse.PatchElements(`<div id="instructions">...</div>`)
    sse.PatchSignals([]byte(`{answer: '...', prize: '...'}`))
    
    
    generator.send(PatchElements.builder()
        .data("<div id=\"question\">...</div>")
        .build()
    );
    generator.send(PatchElements.builder()
        .data("<div id=\"instructions\">...</div>")
        .build()
    );
    generator.send(PatchSignals.builder()
        .data("{\"answer\": \"...\", \"prize\": \"...\"}")
        .build()
    );
    
    
    generator.patchElements(
        elements = """<div id="question">...</div>""",
    )
    generator.patchElements(
        elements = """<div id="instructions">...</div>""",
    )
    generator.patchSignals(
        signals = """{"answer": "...", "prize": "..."}""",
    )
    
    
    $sse->patchElements('<div id="question">...</div>');
    $sse->patchElements('<div id="instructions">...</div>');
    $sse->patchSignals(['answer' => '...', 'prize' => '...']);
    
    
    return DatastarResponse([
        SSE.patch_elements('<div id="question">...</div>'),
        SSE.patch_elements('<div id="instructions">...</div>'),
        SSE.patch_signals({"answer": "...", "prize": "..."})
    ])
    
    
    datastar.stream do |sse|
      sse.patch_elements('<div id="question">...</div>')
      sse.patch_elements('<div id="instructions">...</div>')
      sse.patch_signals(answer: '...', prize: '...')
    end
    
    
    yield PatchElements::new("<div id='question'>...</div>").into()
    yield PatchElements::new("<div id='instructions'>...</div>").into()
    yield PatchSignals::new("{answer: '...', prize: '...'}").into()
    
    
    stream.patchElements('<div id="question">...</div>');
    stream.patchElements('<div id="instructions">...</div>');
    stream.patchSignals({'answer': '...', 'prize': '...'});

> In addition to your browser’s dev tools, the [Datastar Inspector](/datastar_pro#datastar-inspector) can be used to monitor and inspect SSE events received by Datastar.

Read more about SSE events in the [reference](../reference/sse_events.md).

## Congratulations 

You’ve actually read the entire guide! You should now know how to use Datastar to build reactive applications that communicate with the backend using backend requests and SSE events.

Feel free to dive into the [reference](../reference.md) and explore the [examples](/examples) next, to learn more about what you can do with Datastar.
