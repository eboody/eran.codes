(() => {
  if (window.__requestBurstBound === true) return;
  window.__requestBurstBound = true;

  const integerFormatter = new Intl.NumberFormat();
  const oneDecimalFormatter = new Intl.NumberFormat(undefined, {
    maximumFractionDigits: 1,
    minimumFractionDigits: 1,
  });
  const twoDecimalFormatter = new Intl.NumberFormat(undefined, {
    maximumFractionDigits: 2,
    minimumFractionDigits: 2,
  });

  const formatInt = (value) => integerFormatter.format(Math.round(value));
  const formatDuration = (value) => `${twoDecimalFormatter.format(value)} s`;
  const formatRate = (value) =>
    `${value >= 100 ? formatInt(value) : oneDecimalFormatter.format(value)} req/s`;
  const formatLatency = (value) =>
    `${value >= 100 ? formatInt(value) : oneDecimalFormatter.format(value)} ms`;
  const formatSignedPercent = (value) =>
    `${value > 0 ? '+' : ''}${oneDecimalFormatter.format(value)}%`;
  const formatSignedLatency = (value) =>
    `${value > 0 ? '+' : ''}${value <= -100 || value >= 100 ? formatInt(value) : oneDecimalFormatter.format(value)} ms`;

  const percentile = (sortedValues, value) => {
    if (sortedValues.length === 0) return null;

    const index = Math.max(
      0,
      Math.min(
        sortedValues.length - 1,
        Math.ceil((value / 100) * sortedValues.length) - 1,
      ),
    );
    return sortedValues[index];
  };

  const setText = (node, value) => {
    if (!node) return;
    node.textContent = value;
  };

  const summarizeRun = ({ completed, failed, latencies, startedAt }) => {
    const elapsedMs = Math.max(performance.now() - startedAt, 0);
    const elapsedSec = elapsedMs / 1000;
    const rate = elapsedSec > 0 ? completed / elapsedSec : 0;
    const sortedLatencies = [...latencies].sort((left, right) => left - right);

    return {
      completed,
      failed,
      succeeded: completed - failed,
      durationSec: elapsedSec,
      rate,
      p50: percentile(sortedLatencies, 50),
      p95: percentile(sortedLatencies, 95),
      p99: percentile(sortedLatencies, 99),
    };
  };

  const formatPreviousRun = (summary) => {
    if (!summary) {
      return 'Run once to set a baseline.';
    }

    return `${formatRate(summary.rate)} | p95 ${summary.p95 == null ? '—' : formatLatency(summary.p95)}`;
  };

  const formatDelta = (summary, previousSummary) => {
    if (!summary || !previousSummary) {
      return '—';
    }

    const parts = [];

    if (previousSummary.rate > 0) {
      const rateDelta = ((summary.rate - previousSummary.rate) / previousSummary.rate) * 100;
      parts.push(`throughput ${formatSignedPercent(rateDelta)}`);
    }

    if (summary.p95 != null && previousSummary.p95 != null) {
      parts.push(`p95 ${formatSignedLatency(summary.p95 - previousSummary.p95)}`);
    }

    return parts.join(' | ') || '—';
  };

  const bindRoot = (root) => {
    if (!(root instanceof HTMLElement)) return;
    if (root.dataset.requestBurstBound === '1') return;

    const range = root.querySelector('[data-burst-count]');
    const countLabel = root.querySelector('[data-burst-count-label]');
    const runButton = root.querySelector('[data-burst-run]');
    const status = root.querySelector('[data-burst-status]');
    const endpoint = root.getAttribute('data-endpoint');
    const endpointValue = root.querySelector('[data-burst-endpoint]');
    const workersValue = root.querySelector('[data-burst-workers]');
    const rateValue = root.querySelector('[data-burst-rate]');
    const durationValue = root.querySelector('[data-burst-duration]');
    const okValue = root.querySelector('[data-burst-ok]');
    const failedValue = root.querySelector('[data-burst-failed]');
    const p50Value = root.querySelector('[data-burst-p50]');
    const p95Value = root.querySelector('[data-burst-p95]');
    const p99Value = root.querySelector('[data-burst-p99]');
    const previousValue = root.querySelector('[data-burst-previous]');
    const deltaValue = root.querySelector('[data-burst-delta]');

    if (
      !(range instanceof HTMLInputElement) ||
      !(runButton instanceof HTMLButtonElement) ||
      !countLabel ||
      !status ||
      !endpoint
    ) {
      return;
    }

    root.dataset.requestBurstBound = '1';

    const concurrencyRaw = Number(root.getAttribute('data-concurrency'));
    const concurrency = Number.isFinite(concurrencyRaw) && concurrencyRaw > 0
      ? Math.floor(concurrencyRaw)
      : 24;
    let previousSummary = null;

    const updateCountLabel = () => {
      countLabel.textContent = formatInt(Number(range.value) || 0);
    };

    const applySummary = (summary, workerCount) => {
      setText(endpointValue, endpoint);
      setText(workersValue, formatInt(workerCount));
      setText(rateValue, formatRate(summary.rate));
      setText(durationValue, formatDuration(summary.durationSec));
      setText(okValue, formatInt(summary.succeeded));
      setText(failedValue, formatInt(summary.failed));
      setText(p50Value, summary.p50 == null ? '—' : formatLatency(summary.p50));
      setText(p95Value, summary.p95 == null ? '—' : formatLatency(summary.p95));
      setText(p99Value, summary.p99 == null ? '—' : formatLatency(summary.p99));
      setText(previousValue, formatPreviousRun(previousSummary));
      setText(deltaValue, formatDelta(summary, previousSummary));
    };

    updateCountLabel();
    setText(endpointValue, endpoint);
    setText(workersValue, formatInt(concurrency));
    setText(previousValue, formatPreviousRun(previousSummary));
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
      const latencies = [];
      const activeWorkers = Math.min(concurrency, total);

      const report = (prefix) => {
        const summary = summarizeRun({
          completed: sent,
          failed,
          latencies,
          startedAt,
        });
        const reasonSuffix = failureReason ? ` Last error: ${failureReason}.` : '';
        status.textContent = `${prefix}: ${formatInt(sent)} / ${formatInt(total)} requests completed with ${formatInt(failed)} failures.${reasonSuffix}`;
        applySummary(summary, activeWorkers);
      };

      report('Running');

      const worker = async () => {
        while (true) {
          const current = nextIndex;
          if (current >= total) return;
          nextIndex += 1;

          const url = `${endpoint}?i=${current}&t=${Date.now()}`;
          const requestStartedAt = performance.now();

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
            latencies.push(Math.max(performance.now() - requestStartedAt, 0));
          } catch (error) {
            failed += 1;
            failureReason = error instanceof Error
              ? error.message || error.name
              : 'network error';
            latencies.push(Math.max(performance.now() - requestStartedAt, 0));
          }

          sent += 1;
          if (sent % 20 === 0 || sent === total) {
            report('Running');
          }
        }
      };

      const workers = Array.from(
        { length: activeWorkers },
        () => worker(),
      );
      await Promise.all(workers);

      const summary = summarizeRun({
        completed: sent,
        failed,
        latencies,
        startedAt,
      });
      const reasonSuffix = failureReason ? ` Last error: ${failureReason}.` : '';
      status.textContent = `Complete: ${formatInt(sent)} requests in ${formatDuration(summary.durationSec)} with ${formatInt(failed)} failures.${reasonSuffix}`;
      applySummary(summary, activeWorkers);
      previousSummary = summary;
      setText(previousValue, formatPreviousRun(previousSummary));

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
