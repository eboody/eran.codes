use bon::Builder;
use maud::{PreEscaped, Render};
use maud_extensions::css;

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
                data-endpoint=(&self.endpoint)
                data-concurrency=(self.concurrency)
            {
                (SectionHeader::builder()
                    .title(Text::from("High-Volume Request Burst"))
                    .subtitle(Text::from(
                        "Use the slider to send a large burst of requests from this browser and watch live request logs and SSE updates in real time.",
                    ))
                    .build())
                div data-burst-controls {
                    label data-burst-slider {
                        span { "Request count" }
                        input
                            type="range"
                            data-burst-count
                            min=(self.min_requests)
                            max=(self.max_requests)
                            step=(self.request_step)
                            value=(self.default_requests);
                    }
                    p data-burst-selected {
                        "Burst size: "
                        strong data-burst-count-label { (self.default_requests) }
                        " requests"
                    }
                    div data-burst-actions {
                        button type="button" data-burst-run { "Send burst" }
                        p data-muted {
                            "Concurrency: "
                            strong { (self.concurrency) }
                            " workers"
                        }
                    }
                    p data-burst-result {
                        "Ready. Choose a burst size and run the load."
                    }
                }
                script { (PreEscaped(request_burst_script())) }
                ({
                    css! {
                        me {
                          margin-top: 2.8rem;
                          border: 1px solid var(--portfolio-surface-border);
                          border-radius: 18px;
                          padding: 1.35rem 1.35rem 1.45rem;
                          background: var(--portfolio-surface);
                          box-shadow: 0 6px 16px color-mix(in srgb, black 8%, transparent);
                        }
                        me [data-burst-controls] {
                          display: grid;
                          gap: 0.8rem;
                        }
                        me [data-burst-slider] {
                          display: grid;
                          gap: 0.45rem;
                          font-size: 0.9rem;
                          font-weight: 600;
                        }
                        me [data-burst-slider] input[type="range"] {
                          width: 100%;
                          margin: 0;
                          accent-color: var(--ui-accent-primary);
                        }
                        me [data-burst-selected] {
                          margin: 0;
                          font-size: 0.94rem;
                          color: var(--ui-text-muted);
                        }
                        me [data-burst-actions] {
                          display: flex;
                          flex-wrap: wrap;
                          align-items: center;
                          gap: 0.6rem 0.9rem;
                        }
                        me [data-burst-actions] p {
                          margin: 0;
                          font-size: 0.84rem;
                        }
                        me [data-burst-result] {
                          margin: 0;
                          border: 1px solid var(--ui-border-soft);
                          border-radius: var(--ui-radius-sm);
                          padding: 0.7rem 0.8rem;
                          background: var(--ui-surface-soft-alt);
                          font-family: var(--ui-font-mono);
                          font-size: 0.8rem;
                          line-height: 1.42;
                          color: var(--ui-text-muted);
                        }
                        @media (max-width: 768px) {
                          me {
                            margin-top: 1.8rem;
                            padding: 1rem 0.95rem 1.1rem;
                            border-radius: 16px;
                          }
                          me [data-burst-actions] {
                            align-items: stretch;
                          }
                          me [data-burst-actions] button {
                            width: 100%;
                          }
                        }
                    }
                })
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
