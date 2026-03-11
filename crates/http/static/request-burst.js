(() => {
  if (window.__requestBurstBound === true) return;
  window.__requestBurstBound = true;

  const formatInt = (value) => new Intl.NumberFormat().format(value);

  const bindRoot = (root) => {
    if (!(root instanceof HTMLElement)) return;
    if (root.dataset.requestBurstBound === '1') return;

    const range = root.querySelector('[data-burst-count]');
    const countLabel = root.querySelector('[data-burst-count-label]');
    const runButton = root.querySelector('[data-burst-run]');
    const result = root.querySelector('[data-burst-result]');
    const endpoint = root.getAttribute('data-endpoint');

    if (
      !(range instanceof HTMLInputElement) ||
      !(runButton instanceof HTMLButtonElement) ||
      !countLabel ||
      !result ||
      !endpoint
    ) {
      return;
    }

    root.dataset.requestBurstBound = '1';

    const concurrencyRaw = Number(root.getAttribute('data-concurrency'));
    const concurrency = Number.isFinite(concurrencyRaw) && concurrencyRaw > 0
      ? Math.floor(concurrencyRaw)
      : 24;

    const updateCountLabel = () => {
      countLabel.textContent = formatInt(Number(range.value) || 0);
    };

    updateCountLabel();
    range.addEventListener('input', updateCountLabel);

    let running = false;

    runButton.addEventListener('click', async () => {
      if (running) return;

      const total = Number(range.value);
      if (!Number.isFinite(total) || total <= 0) return;

      running = true;
      runButton.disabled = true;
      range.disabled = true;

      let sent = 0;
      let succeeded = 0;
      let failed = 0;
      let failureReason = '';
      let nextIndex = 0;
      const startedAt = performance.now();

      const report = (prefix) => {
        const elapsedMs = performance.now() - startedAt;
        const elapsedSec = elapsedMs / 1000;
        const perSec = elapsedSec > 0 ? sent / elapsedSec : 0;
        const reasonSuffix = failureReason ? ` | last error ${failureReason}` : '';
        result.textContent = `${prefix}: ${formatInt(sent)}/${formatInt(total)} requests | ok ${formatInt(succeeded)} | failed ${formatInt(failed)} | ${perSec.toFixed(0)} req/s${reasonSuffix}`;
      };

      report('Running');

      const worker = async () => {
        while (true) {
          const current = nextIndex;
          if (current >= total) return;
          nextIndex += 1;

          const url = `${endpoint}?i=${current}&t=${Date.now()}`;

          try {
            const response = await fetch(url, {
              method: 'GET',
              cache: 'no-store',
              headers: {
                'x-request-burst': '1',
              },
            });

            if (response.ok) {
              succeeded += 1;
            } else {
              failed += 1;
              failureReason = `HTTP ${response.status}`;
            }
          } catch (error) {
            failed += 1;
            failureReason = error instanceof Error
              ? error.message || error.name
              : 'network error';
          }

          sent += 1;
          if (sent % 20 === 0 || sent === total) {
            report('Running');
          }
        }
      };

      const workers = Array.from(
        { length: Math.min(concurrency, total) },
        () => worker(),
      );
      await Promise.all(workers);

      const elapsedMs = performance.now() - startedAt;
      const elapsedSec = elapsedMs / 1000;
      const perSec = elapsedSec > 0 ? sent / elapsedSec : 0;
      const reasonSuffix = failureReason ? ` | last error ${failureReason}` : '';
      result.textContent = `Complete: ${formatInt(sent)} requests in ${elapsedSec.toFixed(2)}s | ${perSec.toFixed(0)} req/s | ok ${formatInt(succeeded)} | failed ${formatInt(failed)}${reasonSuffix}`;

      runButton.disabled = false;
      range.disabled = false;
      running = false;
    });
  };

  const bindAll = () => {
    document
      .querySelectorAll('[data-request-burst-root]')
      .forEach((root) => bindRoot(root));
  };

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', bindAll, { once: true });
  } else {
    bindAll();
  }

  const observer = new MutationObserver(bindAll);
  observer.observe(document.documentElement, { childList: true, subtree: true });
})();
