use bon::Builder;
use maud::{PreEscaped, Render};

use crate::types::Text;

use super::SectionHeader;

#[derive(Clone, Debug, Builder)]
pub struct RequestBurstDemo {
    pub endpoint: Text,
    #[builder(default = 100)]
    pub min_requests: usize,
    #[builder(default = 5000)]
    pub max_requests: usize,
    #[builder(default = 100)]
    pub request_step: usize,
    #[builder(default = 1000)]
    pub default_requests: usize,
    #[builder(default = 24)]
    pub concurrency: usize,
}

impl Render for RequestBurstDemo {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section
                id="request-burst-demo"
                class="request-burst-demo"
                data-endpoint=(&self.endpoint)
                data-concurrency=(self.concurrency)
            {
                (SectionHeader::builder()
                    .title(Text::from("High-Volume Request Burst"))
                    .subtitle(Text::from(
                        "Use the slider to send a large burst of requests from this browser and watch live request logs and SSE updates in real time.",
                    ))
                    .build()
                    .render())
                div class="request-burst-controls" {
                    label class="request-burst-slider-row" {
                        span { "Request count" }
                        input
                            type="range"
                            data-burst-count
                            min=(self.min_requests)
                            max=(self.max_requests)
                            step=(self.request_step)
                            value=(self.default_requests);
                    }
                    p class="request-burst-selected" {
                        "Burst size: "
                        strong data-burst-count-label { (self.default_requests) }
                        " requests"
                    }
                    div class="request-burst-actions" {
                        button type="button" data-burst-run { "Send burst" }
                        p class="muted" {
                            "Concurrency: "
                            strong { (self.concurrency) }
                            " workers"
                        }
                    }
                    p class="request-burst-result muted" data-burst-result {
                        "Ready. Choose a burst size and run the load."
                    }
                }
                script { (PreEscaped(request_burst_script())) }
            }
        }
    }
}

fn request_burst_script() -> &'static str {
    r#"
(() => {
  const root = document.getElementById('request-burst-demo');
  if (!root) return;

  const range = root.querySelector('[data-burst-count]');
  const countLabel = root.querySelector('[data-burst-count-label]');
  const runButton = root.querySelector('[data-burst-run]');
  const result = root.querySelector('[data-burst-result]');
  const endpoint = root.getAttribute('data-endpoint');
  if (
    !(range instanceof HTMLInputElement) ||
    !countLabel ||
    !(runButton instanceof HTMLButtonElement) ||
    !result ||
    !endpoint
  ) {
    return;
  }

  const formatInt = (value) => new Intl.NumberFormat().format(value);
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
    let nextIndex = 0;
    const startedAt = performance.now();

    const report = (prefix) => {
      const elapsedMs = performance.now() - startedAt;
      const elapsedSec = elapsedMs / 1000;
      const perSec = elapsedSec > 0 ? sent / elapsedSec : 0;
      result.textContent = `${prefix}: ${formatInt(sent)}/${formatInt(total)} requests | ok ${formatInt(succeeded)} | failed ${formatInt(failed)} | ${perSec.toFixed(0)} req/s`;
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
          }
        } catch (_error) {
          failed += 1;
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
    result.textContent = `Complete: ${formatInt(sent)} requests in ${elapsedSec.toFixed(2)}s | ${perSec.toFixed(0)} req/s | ok ${formatInt(succeeded)} | failed ${formatInt(failed)}`;

    runButton.disabled = false;
    range.disabled = false;
    running = false;
  });
})();
    "#
}
