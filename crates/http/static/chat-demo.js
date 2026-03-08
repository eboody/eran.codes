(() => {
  if (window.__chatDemoBound === true) return;
  window.__chatDemoBound = true;

  const bindWindow = (windowElement) => {
    if (!(windowElement instanceof HTMLElement)) return;
    if (windowElement.dataset.chatWindowBound === '1') return;

    const list = windowElement.querySelector('[data-chat-messages]');
    if (!(list instanceof HTMLElement)) return;

    windowElement.dataset.chatWindowBound = '1';

    const scrollToLatest = () => {
      list.scrollTop = 0;
    };

    requestAnimationFrame(scrollToLatest);

    const observer = new MutationObserver(scrollToLatest);
    observer.observe(list, { childList: true, subtree: true });
  };

  const bindRoot = (root) => {
    if (!(root instanceof HTMLElement)) return;
    if (root.dataset.chatDemoBound === '1') return;

    root.dataset.chatDemoBound = '1';

    const bindWindows = () => {
      root.querySelectorAll('[data-chat-window]').forEach(bindWindow);
    };

    bindWindows();

    const observer = new MutationObserver(bindWindows);
    observer.observe(root, { childList: true, subtree: true });
  };

  const bindAll = () => {
    document.querySelectorAll('[data-chat-surface]').forEach(bindRoot);
  };

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', bindAll, { once: true });
  } else {
    bindAll();
  }

  const observer = new MutationObserver(bindAll);
  observer.observe(document.documentElement, { childList: true, subtree: true });
})();
