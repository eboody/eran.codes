(() => {
  const script = document.currentScript;
  if (!(script instanceof HTMLScriptElement)) return;

  const rootId = script.dataset.autoScrollRootId;
  const selector = script.dataset.autoScrollSelector;
  const scope = script.dataset.autoScrollScope ?? 'all';

  if (!rootId || !selector) return;

  const root = document.getElementById(rootId);
  if (!(root instanceof HTMLElement)) return;

  const bindPanel = (panel) => {
    if (!(panel instanceof HTMLElement)) return;
    if (panel.__logAutoScrollBound === true) return;

    panel.__logAutoScrollBound = true;

    const scroll = () => {
      panel.scrollTop = panel.scrollHeight;
    };

    requestAnimationFrame(scroll);

    const observer = new MutationObserver(scroll);
    observer.observe(panel, { childList: true, subtree: true });
  };

  if (scope === 'single') {
    bindPanel(root.querySelector(selector));
    return;
  }

  root.querySelectorAll(selector).forEach(bindPanel);
})();
