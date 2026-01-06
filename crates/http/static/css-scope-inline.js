// 🌘 CSS Scope Inline (https://github.com/gnat/css-scope-inline)
window.cssScopeCount ??= 1;

function cssScopeProcess() {
    document?.body?.querySelectorAll('style:not([ready])').forEach(node => {
        var scope = 'me__' + (window.cssScopeCount++);
        node.parentNode.classList.add(scope);
        node.textContent = node.textContent
            .replace(/(?:^|\.|(\s|[^a-zA-Z0-9\-\_]))(me|this|self)(?![a-zA-Z])/g, '$1.' + scope)
            .replace(/((@keyframes|animation:|animation-name:)[^{};]*)\.me__/g, '$1me__')
            .replace(/(?:@media)\s(xs-|sm-|md-|lg-|xl-|sm|md|lg|xl|xx)/g,
                (match, part1) => '@media ' + ({
                    'sm': '(min-width: 640px)', 'md': '(min-width: 768px)', 'lg': '(min-width: 1024px)',
                    'xl': '(min-width: 1280px)', 'xx': '(min-width: 1536px)',
                    'xs-': '(max-width: 639px)', 'sm-': '(max-width: 767px)', 'md-': '(max-width: 1023px)',
                    'lg-': '(max-width: 1279px)', 'xl-': '(max-width: 1535px)'
                }[part1])
            );
        node.setAttribute('ready', '');
    });
}

window.cssScope ??= new MutationObserver(cssScopeProcess);

// Start observing as early as possible
window.cssScope.observe(document.documentElement, { childList: true, subtree: true });

// Critical: process existing <style> tags too
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', cssScopeProcess, { once: true });
} else {
    cssScopeProcess();
}
