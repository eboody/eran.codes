# http::src

This directory is the transport runtime for the portfolio site. If you want to follow one request from router entry to UI patch, start here.

## Read order
1. [router/README.md](./router/README.md)
2. [handlers/README.md](./handlers/README.md)
3. [request.rs](./request.rs) and [request_context_flow.rs](./request_context_flow.rs)
4. [sse/README.md](./sse/README.md)
5. [trace_log.rs](./trace_log.rs)
6. [views/README.md](./views/README.md)

## Core files
- [router/README.md](./router/README.md)
- [handlers/README.md](./handlers/README.md)
- [sse/README.md](./sse/README.md)
- [views/README.md](./views/README.md)
- [trace_log.rs](./trace_log.rs)

## Rule of thumb
HTTP owns transport, rendering, and runtime delivery details. It should translate into `app` calls early and keep storage and policy logic out.
