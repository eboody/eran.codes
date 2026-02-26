# Security

- Source: `https://data-star.dev/reference/security`
- Retrieved: `2026-02-26 17:55 UTC`
- Section: Reference

[Datastar expressions](../guide/datastar_expressions.md) are strings that are evaluated in a sandboxed context. This means you can use JavaScript in Datastar expressions.

## Escape User Input 

The golden rule of security is to never trust user input. This is especially true when using Datastar expressions, which can execute arbitrary JavaScript. When using Datastar expressions, you should always escape user input. This helps prevent, among other issues, Cross-Site Scripting (XSS) attacks.

## Avoid Sensitive Data 

Keep in mind that signal values are visible in the source code in plain text, and can be modified by the user before being sent in requests. For this reason, you should avoid leaking sensitive data in signals and always implement backend validation.

## Ignore Unsafe Input 

If, for some reason, you cannot escape unsafe user input, you should ignore it using the [`data-ignore`](attributes.md#data-ignore) attribute. This tells Datastar to ignore an element and its descendants when processing DOM nodes.

## Content Security Policy 

When using a [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP) (CSP), `unsafe-eval` must be allowed for scripts, since Datastar evaluates expressions using a [`Function()` constructor](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/Function).
    
    
    <meta http-equiv="Content-Security-Policy" 
        content="script-src 'self' 'unsafe-eval';"
    >
