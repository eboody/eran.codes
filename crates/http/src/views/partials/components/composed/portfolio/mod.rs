use maud::{Markup, Render};

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: clamp(1.7rem, 1.2rem + 1.55vw, 2.85rem);
  margin-top: clamp(1.45rem, 1rem + 1.35vw, 2.35rem);
  padding-bottom: var(--space-8);
}

me .ui-portfolio-surface {
  --surface-card-gap: clamp(1rem, 0.85rem + 0.8vw, 1.45rem);
}

me .ui-portfolio-hero {
  --surface-card-padding: var(--space-7);
  border-radius: var(--radius-5);
  background:
    linear-gradient(
      140deg,
      color-mix(in srgb, var(--portfolio-accent-a) 30%, transparent),
      transparent 60%
    ),
    var(--portfolio-surface);
  view-transition-name: portfolio-hero;
}

me .ui-portfolio-hero-grid {
  display: grid;
  gap: clamp(1.2rem, 1rem + 0.85vw, 1.9rem);
  align-items: start;
}

me .ui-portfolio-hero-main {
  display: grid;
  gap: inherit;
  align-content: start;
}

me .ui-portfolio-lead-surface {
  align-content: start;
  gap: clamp(1rem, 0.8rem + 0.65vw, 1.45rem);
}

me .ui-portfolio-lead-surface h1 {
  max-width: 18.4ch;
  font-size: var(--text-size-hero-lg);
  line-height: var(--text-line-title);
  text-wrap: balance;
}

me .ui-portfolio-eyebrow {
  margin: 0;
  font-size: var(--text-size-meta-sm);
  letter-spacing: var(--text-track-caps-sm);
  text-transform: uppercase;
  color: var(--ui-text-muted);
}

me .ui-portfolio-summary {
  margin: 0;
  max-width: 60ch;
  line-height: 1.62;
  color: color-mix(in srgb, var(--ui-text) 90%, var(--ui-text-muted) 10%);
}

me .ui-portfolio-badges {
  margin: 0;
  padding: 0;
  list-style: none;
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}

me .ui-portfolio-badges li {
  border: var(--border-size-1) solid var(--ui-border-soft);
  border-radius: var(--radius-pill);
  padding: var(--space-1) var(--space-2);
  background: var(--ui-surface-soft);
  font-size: var(--text-size-meta-xs);
  color: var(--ui-text-muted);
}

me .ui-portfolio-proof-strip {
  display: grid;
  gap: var(--space-3);
}

me .ui-portfolio-proof-item {
  --inset-card-padding: var(--space-3);
  --inset-card-border: var(--ui-border-soft);
  --inset-card-bg: var(--ui-surface-soft);
  transition:
    border-color var(--motion-fast),
    background-color var(--motion-fast),
    box-shadow var(--motion-fast),
    transform var(--motion-fast);
}

me .ui-portfolio-proof-item h3 {
  margin: 0 0 var(--space-2) 0;
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
  gap: var(--space-3);
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 18rem), 1fr));
}

me .ui-portfolio-showcase-stack {
  display: grid;
  gap: clamp(1.1rem, 0.95rem + 0.8vw, 1.9rem);
}

me .ui-portfolio-card {
  --inset-card-padding: var(--space-4);
  --inset-card-border: var(--ui-border-soft);
  --inset-card-bg: var(--ui-surface-soft);

  display: grid;
  gap: var(--space-2);
  align-content: start;
  overflow: visible;
  transition:
    border-color var(--motion-fast),
    background-color var(--motion-fast),
    box-shadow var(--motion-fast),
    transform var(--motion-fast);
}

me .ui-portfolio-card-kicker {
  margin: 0;
  font-size: var(--text-size-meta-xs);
  letter-spacing: var(--text-track-caps-sm);
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

me .ui-portfolio-work-card {
  gap: var(--space-3);
}

me .ui-portfolio-work-card .ui-portfolio-card-kicker {
  color: color-mix(in srgb, var(--accent-signal) 52%, var(--ui-text-muted));
}

me .ui-portfolio-experience-grid {
  align-items: start;
}

me .ui-portfolio-work-card h3 {
  font-size: var(--text-size-title-sm);
  line-height: var(--text-line-heading);
  text-wrap: balance;
}

me .ui-portfolio-experience-card,
me .ui-portfolio-skill-card {
  gap: var(--space-3);
}

me .ui-portfolio-experience-card--feature {
  inline-size: min(100%, 62rem);
  justify-self: start;
}

me .ui-portfolio-experience-card--feature .ui-portfolio-card-summary {
  max-width: 62ch;
}

me .ui-portfolio-experience-card--feature .ui-portfolio-list {
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-2) clamp(1rem, 0.7rem + 1vw, 1.85rem);
}

me .ui-portfolio-work-card .ui-portfolio-card-summary {
  max-width: 40ch;
}

me .ui-portfolio-card-outcome {
  margin: 0;
  display: grid;
  gap: var(--space-1);
  padding: var(--space-3);
  border-radius: var(--ui-radius-md-inset);
  border: var(--border-size-1) solid
    color-mix(in srgb, var(--portfolio-accent-a) 48%, transparent);
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--portfolio-accent-a) 15%, transparent),
      transparent 100%
    ),
    color-mix(in srgb, var(--portfolio-accent-a) 14%, transparent);
  box-shadow: inset 0 1px 0 color-mix(in srgb, white 28%, transparent);
  font-size: var(--text-size-body-md);
  line-height: var(--text-line-body);
}

me .ui-portfolio-card-outcome-label {
  font-size: var(--text-size-label-2xs);
  font-weight: 600;
  letter-spacing: var(--text-track-caps-md);
  text-transform: uppercase;
  color: var(--ui-text-muted);
}

me .ui-portfolio-card-outcome-text {
  color: color-mix(in srgb, var(--ui-text) 94%, var(--portfolio-accent-a) 6%);
  font-weight: 600;
}

me .ui-portfolio-archive-entry {
  scroll-margin-top: calc(var(--space-8) + 4rem);
}

me .ui-portfolio-archive-note {
  color: color-mix(in srgb, var(--portfolio-accent-b) 48%, var(--ui-text-muted));
}

me .ui-portfolio-list {
  margin: 0;
  padding-left: var(--space-4);
  display: grid;
  gap: var(--space-1);
}

me .ui-portfolio-work-card .ui-portfolio-list {
  gap: var(--space-2);
}

me .ui-portfolio-card > .button {
  width: fit-content;
}

me .ui-portfolio-card-footer {
  display: grid;
  gap: var(--space-2);
  margin-top: auto;
  padding-top: var(--space-2);
  border-top: 1px solid color-mix(in srgb, var(--ui-border-soft) 82%, transparent);
  align-content: start;
  justify-items: start;
}

me .ui-portfolio-card-footer .ui-portfolio-badges {
  gap: var(--space-1);
}

@media (hover: hover) {
  me .ui-portfolio-card:hover,
  me .ui-portfolio-proof-item:hover {
    transform: var(--motion-lift-subtle);
    box-shadow: var(--shadow-panel-hover);
  }

  me .ui-portfolio-crate-showcase:hover {
    border-color: color-mix(in srgb, var(--portfolio-accent-a) 30%, var(--ui-border-soft));
  }

  me .ui-portfolio-card:hover {
    border-color: color-mix(in srgb, var(--portfolio-accent-a) 22%, var(--ui-border-soft));
  }

  me .ui-portfolio-proof-item:hover {
    border-color: color-mix(in srgb, var(--portfolio-accent-a) 20%, var(--ui-border-soft));
  }

  me .ui-portfolio-proof-item[data-proof-kind="reliability"]:hover {
    border-color: color-mix(in srgb, var(--portfolio-accent-b) 26%, var(--ui-border-soft));
  }
}

me .ui-portfolio-card:focus-within {
  transform: var(--motion-lift-subtle);
  border-color: color-mix(in srgb, var(--portfolio-accent-a) 24%, var(--ui-border-soft));
  box-shadow: var(--shadow-panel-hover);
}

me .ui-portfolio-crate-showcase:focus-within,
me .ui-portfolio-crate-gallery:focus-within {
  border-color: color-mix(in srgb, var(--portfolio-accent-a) 34%, var(--ui-border-soft));
}

me .ui-portfolio-crate-showcase {
  display: grid;
  gap: clamp(1.25rem, 1.02rem + 0.78vw, 1.85rem);
  padding: clamp(1.35rem, 1.1rem + 0.9vw, 2rem);
  border-radius: var(--ui-radius-md-inset);
  border: var(--border-size-1) solid
    color-mix(in srgb, var(--portfolio-accent-a) 24%, var(--ui-border-soft));
  background:
    linear-gradient(
      140deg,
      color-mix(in srgb, var(--portfolio-accent-a) 10%, transparent),
      transparent 38%
    ),
    linear-gradient(
      180deg,
      color-mix(in srgb, white 18%, transparent),
      transparent 20%
    ),
    var(--ui-surface-soft);
  box-shadow:
    inset 0 1px 0 color-mix(in srgb, white 38%, transparent),
    0 18px 40px -34px color-mix(in srgb, var(--portfolio-accent-a) 34%, transparent);
  transition:
    border-color var(--motion-fast),
    background-color var(--motion-fast),
    box-shadow var(--motion-fast);
}

me .ui-portfolio-crate-showcase-header,
me .ui-portfolio-crate-showcase-copy {
  display: grid;
  gap: var(--space-3);
}

me .ui-portfolio-crate-showcase-copy h3 {
  margin: 0;
  font-size: var(--text-size-title-sm);
  line-height: var(--text-line-heading);
  letter-spacing: var(--text-track-tight-sm);
}

me .ui-portfolio-crate-showcase-copy .ui-portfolio-card-summary {
  max-width: 54ch;
}

me .ui-portfolio-inline-links {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  align-items: center;
}

me .ui-portfolio-inline-link {
  color: color-mix(in srgb, var(--ui-text) 84%, var(--accent-signal) 16%);
  font-size: var(--text-size-meta-sm);
  font-weight: 600;
  letter-spacing: var(--text-track-ui);
  text-decoration: none;
}

me .ui-portfolio-inline-link:hover,
me .ui-portfolio-inline-link:focus-visible {
  color: var(--accent-signal);
  text-decoration: underline;
  text-underline-offset: 0.16em;
}

me .ui-portfolio-crate-gallery {
  margin-top: 0;
  padding: clamp(0.9rem, 0.8rem + 0.55vw, 1.3rem);
  border: var(--border-size-1) solid
    color-mix(in srgb, var(--portfolio-accent-a) 28%, var(--ui-border-soft));
  border-radius: var(--ui-radius-md);
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--portfolio-accent-a) 14%, transparent),
      transparent 40%
    ),
    linear-gradient(
      160deg,
      color-mix(in srgb, white 14%, transparent),
      transparent 28%
    ),
    color-mix(in srgb, var(--ui-surface-card) 90%, transparent);
  box-shadow:
    inset 0 1px 0 color-mix(in srgb, white 30%, transparent),
    inset 0 0 0 1px color-mix(in srgb, black 2%, transparent);
  transition:
    border-color var(--motion-fast),
    background-color var(--motion-fast),
    box-shadow var(--motion-fast);
}

@media (prefers-color-scheme: dark) {
  me .ui-portfolio-crate-showcase {
    border-color: color-mix(in srgb, var(--ui-border-soft) 88%, transparent);
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 28%),
      color-mix(in srgb, var(--surface-panel) 94%, black 6%);
    box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  }

  me .ui-portfolio-crate-gallery {
    border-color: color-mix(in srgb, var(--ui-border-soft) 84%, transparent);
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 24%),
      color-mix(in srgb, var(--surface-panel) 92%, black 8%);
    box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  }
}

me .ui-portfolio-section-actions {
  margin-top: var(--space-1);
}

me .ui-portfolio-section-actions .ui-button-row,
me .ui-portfolio-lead-surface .ui-button-row,
me .ui-portfolio-closing .ui-button-row {
  margin-top: calc(var(--space-2) - var(--interactive-bleed));
}

me .ui-portfolio-lead-surface .ui-button-row {
  --button-row-gap: var(--space-2);

  padding-top: var(--space-3);
  border-top: 1px solid color-mix(in srgb, var(--ui-border-soft) 82%, transparent);
}

me .ui-portfolio-hero-aside {
  --inset-card-padding: clamp(1rem, 0.88rem + 0.55vw, 1.35rem);
  --inset-card-border: color-mix(in srgb, var(--portfolio-accent-a) 32%, var(--ui-border-soft));
  --inset-card-bg:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--portfolio-accent-a) 10%, transparent),
      transparent 30%
    ),
    color-mix(in srgb, var(--ui-surface-soft) 92%, transparent);

  display: grid;
  gap: var(--space-3);
  align-content: start;
  inline-size: min(100%, 22rem);
  justify-self: end;
}

me .ui-portfolio-hero-aside-kicker {
  margin: 0;
  font-size: var(--text-size-label-sm);
  font-weight: 700;
  letter-spacing: var(--text-track-caps-sm);
  text-transform: uppercase;
  color: color-mix(in srgb, var(--accent-signal) 58%, var(--ui-text-muted));
}

me .ui-portfolio-hero-aside h2 {
  margin: 0;
  font-size: var(--text-size-title-sm);
  line-height: var(--text-line-heading);
  text-wrap: balance;
}

me .ui-portfolio-hero-aside .ui-portfolio-card-summary {
  margin: 0;
  color: var(--ui-text-muted);
}

me .ui-portfolio-hero-aside-outcome {
  margin: 0;
  display: grid;
  gap: var(--space-1);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--ui-radius-md-inset);
  border: var(--border-size-1) solid color-mix(in srgb, var(--portfolio-accent-a) 42%, transparent);
  background: color-mix(in srgb, var(--portfolio-accent-a) 12%, transparent);
}

me .ui-portfolio-hero-aside-outcome-label {
  font-size: var(--text-size-label-2xs);
  font-weight: 600;
  letter-spacing: var(--text-track-caps-md);
  text-transform: uppercase;
  color: var(--ui-text-muted);
}

me .ui-portfolio-hero-aside-outcome-text {
  font-weight: 600;
  color: color-mix(in srgb, var(--ui-text) 94%, var(--portfolio-accent-a) 6%);
}

me .ui-portfolio-hero-aside .button {
  width: fit-content;
}

me .ui-portfolio-closing {
  justify-items: start;
  gap: var(--space-3);
}

me .ui-portfolio-supporting-teaser {
  --surface-card-border: color-mix(in srgb, var(--ui-border-soft) 84%, transparent);
  --surface-card-bg-image:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--portfolio-accent-b) 4%, transparent),
      transparent 18%
    ),
    color-mix(in srgb, var(--surface-fill-panel) 90%, transparent);

  justify-items: start;
  gap: var(--space-3);
}

me .ui-portfolio-supporting-teaser h2 {
  margin: 0;
}

me .ui-portfolio-supporting-teaser .ui-button-row {
  margin-top: calc(var(--space-1) - var(--interactive-bleed));
}

me .ui-portfolio-case-grid {
  display: grid;
  gap: var(--space-3);
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 17rem), 1fr));
}

me .ui-portfolio-case-section {
  --surface-card-border: color-mix(
    in srgb,
    var(--portfolio-accent-a) 26%,
    var(--portfolio-surface-border)
  );
  --surface-card-bg-image:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--portfolio-accent-a) 8%, transparent),
      transparent 22%
    ),
    var(--surface-fill-panel);
}

me .ui-portfolio-case-section h2 {
  margin: 0;
}

me .ui-portfolio-case-section .ui-portfolio-section-copy {
  --section-copy-gap: var(--space-1);
  --section-copy-max-inline: 56ch;
}

me .ui-portfolio-crate-section {
  --surface-card-border: color-mix(in srgb, var(--ui-border-soft) 88%, transparent);
  --surface-card-bg-image:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--portfolio-accent-b) 5%, transparent),
      transparent 20%
    ),
    color-mix(in srgb, var(--surface-fill-panel) 92%, transparent);
}

me .ui-portfolio-crate-section--standalone {
  --surface-card-gap: var(--space-3);
  --surface-card-margin-top: 0;
  --surface-card-padding: clamp(0.9rem, 0.8rem + 0.5vw, 1.2rem);
  --surface-card-border: color-mix(in srgb, var(--ui-border-soft) 76%, transparent);
  --surface-card-bg-image:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--portfolio-accent-a) 5%, transparent),
      transparent 24%
    ),
    color-mix(in srgb, var(--surface-fill-panel) 76%, transparent);
  --surface-card-shadow: inset 0 1px 0 var(--surface-edge-soft);

  max-inline-size: 72rem;
  margin-inline: auto;
}

me .ui-portfolio-crate-section--standalone .ui-portfolio-crate-gallery {
  padding: var(--space-2);
  border: var(--border-size-1) solid
    color-mix(in srgb, var(--portfolio-accent-a) 20%, var(--ui-border-soft));
  border-radius: var(--ui-radius-md-inset);
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--portfolio-accent-a) 7%, transparent),
      transparent 30%
    ),
    color-mix(in srgb, var(--surface-fill-panel) 88%, transparent);
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
}

me .ui-portfolio-crate-section--standalone .tab-set-showcase {
  background: transparent;
}

me .ui-portfolio-crate-section--standalone .ui-portfolio-inline-links {
  gap: var(--space-2);
}

@media (max-width: 48rem) {
  me {
    gap: var(--space-4);
    margin-top: var(--space-4);
    padding-bottom: var(--space-6);
  }

  me .ui-portfolio-surface {
    --surface-card-gap: var(--space-3);
  }

  me .ui-portfolio-hero {
    padding: var(--space-5) var(--space-4);
  }

  me .ui-portfolio-hero-grid {
    gap: var(--space-4);
  }

  me .ui-portfolio-lead-surface {
    gap: var(--space-4);
  }

  me .ui-portfolio-lead-surface h1 {
    font-size: var(--text-size-hero-md);
    max-width: 14.4ch;
  }

  me .ui-portfolio-summary {
    font-size: var(--text-size-body-lg);
    line-height: 1.6;
  }

  me .ui-portfolio-hero-aside {
    display: none;
  }

  me .ui-portfolio-card-grid {
    gap: var(--space-2);
  }

  me .ui-portfolio-showcase-stack {
    gap: var(--space-3);
  }

  me .ui-portfolio-card {
    gap: var(--space-2);
    --inset-card-padding: var(--space-3);
  }

  me .ui-portfolio-experience-card--feature {
    inline-size: 100%;
  }

  me .ui-portfolio-experience-card--feature .ui-portfolio-list {
    grid-template-columns: minmax(0, 1fr);
    gap: calc(var(--space-1) * 0.75);
    padding-left: var(--space-2);
  }

  me .ui-portfolio-work-card {
    gap: var(--space-2);
  }

  me .ui-portfolio-card-outcome {
    font-size: var(--text-size-body-sm);
    gap: calc(var(--space-1) * 0.75);
    padding: var(--space-2);
  }

  me .ui-portfolio-list {
    padding-left: var(--space-3);
  }

  me .ui-portfolio-work-card .ui-portfolio-list {
    gap: var(--space-1);
  }

  me .ui-portfolio-archive-entry {
    scroll-margin-top: calc(var(--space-7) + 3.5rem);
    gap: var(--space-2);
  }

  me .ui-portfolio-case-grid {
    gap: var(--space-2);
  }

  me .ui-portfolio-card-footer {
    gap: var(--space-1);
    padding-top: var(--space-1);
  }

  me .ui-portfolio-work-card .ui-portfolio-card-footer {
    gap: calc(var(--space-1) * 0.75);
    padding-top: calc(var(--space-1) * 0.75);
  }

  me .ui-portfolio-card-footer .ui-portfolio-badges {
    display: none;
  }

  me .ui-portfolio-crate-showcase {
    gap: var(--space-3);
    padding: var(--space-3);
  }

  me .ui-portfolio-crate-showcase-copy,
  me .ui-portfolio-crate-showcase-header {
    gap: var(--space-1);
  }

  me .ui-portfolio-crate-gallery {
    padding: var(--space-2);
  }

  me .ui-portfolio-supporting-teaser {
    gap: var(--space-2);
  }

  me .ui-portfolio-crate-section--standalone {
    gap: var(--space-2);
  }

  me .ui-portfolio-closing,
  me .ui-portfolio-supporting-teaser {
    gap: var(--space-2);
  }

  me .ui-portfolio-section-actions .ui-button-row,
  me .ui-portfolio-lead-surface .ui-button-row,
  me .ui-portfolio-closing .ui-button-row {
    --button-row-gap: var(--space-2);
    --button-row-item-min-inline-size: 9rem;
  }

  me .ui-portfolio-section-actions .ui-button-row,
  me .ui-portfolio-closing .ui-button-row {
    margin-top: calc(var(--space-1) - var(--interactive-bleed));
  }
}

@media (min-width: 56.25rem) {
  me .ui-portfolio-hero-grid {
    grid-template-columns: minmax(0, 1.38fr) minmax(18rem, 0.78fr);
  }

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
  me .ui-portfolio-card > .button,
  me .ui-portfolio-section-actions .button,
  me .ui-portfolio-lead-surface .button,
  me .ui-portfolio-closing .button {
    width: 100%;
  }

  me .ui-portfolio-card-footer > .button {
    width: fit-content;
  }

}

@media (prefers-reduced-motion: reduce) {
  me .ui-portfolio-card,
  me .ui-portfolio-proof-item {
    transition: none;
  }
}
"#
);

pub mod content;
mod sections;

pub use sections::{
    ArchiveCaseDetailsSection, ClosingSection, CrateSection, ExperienceSection, PortfolioHero,
    SkillGroupsSection, SupportingTeaserSection, WorkCaseDetail, WorkIndexSection, WorkSection,
};

pub struct Page {
    pub content: Markup,
}

impl Render for Page {
    fn render(&self) -> Markup {
        maud::html! {
            div data-portfolio-page data-page-section {
                (css())
                (&self.content)
            }
        }
    }
}
