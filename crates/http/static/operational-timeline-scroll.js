(() => {
  if (typeof window.scrollOperationalTimelineTop === 'function') return;

  window.scrollOperationalTimelineTop = () => {
    const target = document.getElementById('network-log-target');
    if (!(target instanceof HTMLElement)) return;

    const flowList = target.querySelector('[data-log-flow-list]');
    if (flowList instanceof HTMLElement) {
      flowList.scrollTop = 0;
      flowList.scrollIntoView({ behavior: 'smooth', block: 'start' });
      return;
    }

    target.scrollIntoView({ behavior: 'smooth', block: 'start' });
  };
})();
