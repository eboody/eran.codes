use maud::{Markup, Render};

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: clamp(1.35rem, 1rem + 1.2vw, 2.3rem);
  margin-top: var(--size-5);
  padding-bottom: var(--size-8);
}

me .ui-portfolio-surface {
  display: grid;
  gap: clamp(1rem, 0.85rem + 0.8vw, 1.45rem);
}

me .ui-portfolio-hero {
  padding: var(--size-7);
  border-radius: var(--radius-5);
  background:
    linear-gradient(
      140deg,
      color-mix(in srgb, var(--portfolio-accent-a) 30%, transparent),
      transparent 60%
    ),
    var(--portfolio-surface);
}

me .ui-portfolio-lead-surface {
  align-content: start;
}

me .ui-portfolio-lead-surface h1 {
  max-width: 14ch;
  font-size: clamp(2.35rem, 1.75rem + 2.35vw, 3.6rem);
  line-height: 0.95;
  text-wrap: balance;
}

me .ui-portfolio-eyebrow {
  margin: 0;
  font-size: 0.8rem;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--ui-text-muted);
}

me .ui-portfolio-summary {
  margin: 0;
  max-width: 64ch;
  color: color-mix(in srgb, var(--ui-text) 90%, var(--ui-text-muted) 10%);
}

me .ui-portfolio-section-copy {
  display: grid;
  gap: var(--size-2);
}

me .ui-portfolio-section-copy h2,
me .ui-portfolio-section-copy p {
  margin: 0;
}

me .ui-portfolio-section-copy p {
  max-width: 60ch;
  color: var(--ui-text-muted);
}

me .ui-portfolio-badges {
  margin: 0;
  padding: 0;
  list-style: none;
  display: flex;
  flex-wrap: wrap;
  gap: var(--size-2);
}

me .ui-portfolio-badges li {
  border: var(--border-size-1) solid var(--ui-border-soft);
  border-radius: var(--radius-4);
  padding: var(--size-1) var(--size-2);
  background: var(--ui-surface-soft);
  font-size: 0.78rem;
  color: var(--ui-text-muted);
}

me .ui-portfolio-proof-strip {
  display: grid;
  gap: var(--size-3);
}

me .ui-portfolio-proof-item {
  padding: var(--size-3);
  border-radius: var(--ui-radius-md);
  border: var(--border-size-1) solid var(--ui-border-soft);
  background: var(--ui-surface-soft);
}

me .ui-portfolio-proof-item h3 {
  margin: 0 0 var(--size-2) 0;
}

me .ui-portfolio-proof-item p {
  margin: 0;
  color: var(--ui-text-muted);
}

me .ui-portfolio-proof-item[data-proof-kind="architecture"] {
  border-color: color-mix(in srgb, var(--portfolio-accent-a) 42%, transparent);
}

me .ui-portfolio-proof-item[data-proof-kind="reliability"] {
  border-color: color-mix(in srgb, var(--portfolio-accent-b) 42%, transparent);
}

me .ui-portfolio-card-grid {
  display: grid;
  gap: var(--size-3);
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 18rem), 1fr));
}

me .ui-portfolio-card {
  display: grid;
  gap: var(--size-2);
  align-content: start;
  padding: var(--size-4);
  border-radius: var(--ui-radius-md);
  border: var(--border-size-1) solid var(--ui-border-soft);
  background: var(--ui-surface-soft);
  overflow: visible;
}

me .ui-portfolio-card-kicker {
  margin: 0;
  font-size: 0.78rem;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--ui-text-muted);
}

me .ui-portfolio-card h3,
me .ui-portfolio-card-summary {
  margin: 0;
}

me .ui-portfolio-card-summary {
  color: var(--ui-text-muted);
}

me .ui-portfolio-card-outcome {
  margin: 0;
  padding: var(--size-2);
  border-radius: var(--ui-radius-sm);
  border: var(--border-size-1) solid
    color-mix(in srgb, var(--portfolio-accent-a) 42%, transparent);
  background: color-mix(in srgb, var(--portfolio-accent-a) 12%, transparent);
  font-size: 0.85rem;
}

me .ui-portfolio-card-preview {
  display: grid;
  gap: var(--size-1);
  margin: 0;
  padding: var(--size-2);
  border-radius: var(--ui-radius-sm);
  border: var(--border-size-1) dashed var(--ui-border-soft);
  background: var(--ui-surface-soft-alt);
}

me .ui-portfolio-preview-key {
  font-family: var(--ui-font-mono);
  font-size: 0.76rem;
}

me .ui-portfolio-preview-alt {
  font-size: 0.8rem;
  color: var(--ui-text-muted);
}

me .ui-portfolio-list {
  margin: 0;
  padding-left: var(--size-4);
  display: grid;
  gap: var(--size-1);
}

me .ui-portfolio-card-links {
  display: flex;
  flex-wrap: wrap;
  gap: var(--size-2);
  position: relative;
  isolation: isolate;
  align-items: center;
  padding: var(--interactive-bleed);
  margin: calc(var(--interactive-bleed) * -1);
}

me .ui-portfolio-card > .button,
me .ui-portfolio-card-links > .button {
  width: fit-content;
}

me .ui-portfolio-section-actions {
  margin-top: var(--size-1);
}

me .ui-portfolio-section-actions .ui-button-row,
me .ui-portfolio-lead-surface .ui-button-row,
me .ui-portfolio-closing .ui-button-row {
  margin-top: calc(var(--size-2) - var(--interactive-bleed));
}

me .ui-portfolio-closing {
  justify-items: start;
  gap: var(--size-3);
}

me .ui-portfolio-case-grid {
  display: grid;
  gap: var(--size-3);
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 17rem), 1fr));
}

me .ui-portfolio-case-section h2 {
  margin: 0;
}

@media (max-width: 48rem) {
  me .ui-portfolio-hero {
    padding: var(--size-4);
  }

  me .ui-portfolio-lead-surface h1 {
    max-width: 12ch;
  }
}

@media (min-width: 56.25rem) {
  me .ui-portfolio-proof-strip {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }

  me .ui-portfolio-card-grid {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }

  me .ui-portfolio-case-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 768px) {
  me .ui-portfolio-card-links > .button,
  me .ui-portfolio-card > .button,
  me .ui-portfolio-section-actions .button,
  me .ui-portfolio-lead-surface .button,
  me .ui-portfolio-closing .button {
    width: 100%;
  }
}
"#
);

pub mod content;
mod sections;

pub use sections::{
    ClosingSection, CrateSection, PortfolioHero, ProofStrip, WorkCaseDetail,
    WorkIndexSection, WorkSection,
};

pub struct Page {
    pub content: Markup,
}

impl Render for Page {
    fn render(&self) -> Markup {
        maud::html! {
            main class="u-container" data-portfolio-page {
                (css())
                (&self.content)
            }
        }
    }
}
