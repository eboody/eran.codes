(() => {
  if (window.__localTabsBound === true) return;
  window.__localTabsBound = true;

  const isButton = (value) => value instanceof HTMLButtonElement;
  const isPanel = (value) => value instanceof HTMLElement;

  const panelForTab = (root, tab) => {
    if (!isButton(tab)) return null;

    const panelId = tab.getAttribute('aria-controls');
    if (!panelId) return null;

    const panel = document.getElementById(panelId);
    if (!isPanel(panel) || !root.contains(panel)) return null;

    return panel;
  };

  const bindRoot = (root) => {
    if (!(root instanceof HTMLElement)) return;
    if (root.dataset.localTabsBound === '1') return;

    const tabList = root.querySelector(':scope > [role="tablist"]');
    if (!(tabList instanceof HTMLElement)) return;

    const tabs = Array.from(
      tabList.querySelectorAll('[role="tab"][data-local-tab-value]'),
    ).filter(isButton);
    if (tabs.length === 0) return;

    root.dataset.localTabsBound = '1';

    const select = (value, focus = false) => {
      root.dataset.localTabsActive = value;

      tabs.forEach((tab) => {
        const selected = tab.dataset.localTabValue === value;
        tab.classList.toggle('is-selected', selected);
        tab.setAttribute('aria-selected', selected ? 'true' : 'false');
        tab.tabIndex = selected ? 0 : -1;

        const panel = panelForTab(root, tab);
        if (panel) {
          panel.hidden = !selected;
          panel.tabIndex = selected ? 0 : -1;
        }

        if (selected && focus) {
          tab.focus();
        }
      });
    };

    const selectedValue =
      root.dataset.localTabsActive ||
      tabs.find((tab) => tab.getAttribute('aria-selected') === 'true')?.dataset.localTabValue ||
      tabs[0]?.dataset.localTabValue;

    if (selectedValue) {
      select(selectedValue);
    }

    tabs.forEach((tab, index) => {
      tab.addEventListener('click', () => {
        const value = tab.dataset.localTabValue;
        if (value) {
          select(value);
        }
      });

      tab.addEventListener('keydown', (event) => {
        let nextIndex = index;

        switch (event.key) {
          case 'ArrowLeft':
          case 'ArrowUp':
            nextIndex = (index - 1 + tabs.length) % tabs.length;
            break;
          case 'ArrowRight':
          case 'ArrowDown':
            nextIndex = (index + 1) % tabs.length;
            break;
          case 'Home':
            nextIndex = 0;
            break;
          case 'End':
            nextIndex = tabs.length - 1;
            break;
          default:
            return;
        }

        event.preventDefault();

        const nextValue = tabs[nextIndex]?.dataset.localTabValue;
        if (nextValue) {
          select(nextValue, true);
        }
      });
    });
  };

  const bindAll = () => {
    document.querySelectorAll('[data-local-tabs-root]').forEach(bindRoot);
  };

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', bindAll, { once: true });
  } else {
    bindAll();
  }

  const observer = new MutationObserver(bindAll);
  observer.observe(document.documentElement, { childList: true, subtree: true });
})();
