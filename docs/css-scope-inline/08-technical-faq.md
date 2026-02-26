## 🔎 Technical FAQ
* Why do you use `querySelectorAll()` and not just process the `MutationObserver` results directly?
  * This was indeed the original design; it will work well up until you begin recieving subtrees (ex: DOM swaps with [htmx](https://htmx.org), ajax, jquery, etc.) which requires walking all subtree elements to ensure we do not miss a `<style>`. This unfortunately involves re-scanning thousands of repeated elements. This is why `querySelectorAll()` ends up the performance (and simplicity) winner.
