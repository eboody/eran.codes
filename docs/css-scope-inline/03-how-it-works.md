## 🌘 How does it work?

This uses `MutationObserver` to monitor the DOM, and the moment a `<style>` tag is seen, it scopes the styles to whatever the parent element is. No flashing or popping. 

This method also leaves your existing styles untouched, allowing you to mix and match at your leisure.

