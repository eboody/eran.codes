use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::logs;

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--size-4);
  --log-panel-gap: var(--size-2);
  --log-panel-padding: var(--size-3) var(--size-4);
  --log-panel-border: var(--border-size-1) solid var(--ui-border-soft);
  --log-panel-background: var(--ui-surface-soft);
  --log-panel-heading-size: 0.95rem;
  --log-panel-heading-tracking: normal;
  --log-panel-heading-transform: none;
  --log-panel-heading-color: inherit;

  --log-scroll-max-height: 20rem;
  --log-scroll-max-height-mobile: 16.25rem;
  --log-scroll-max-height-compact: 13.75rem;
  --log-scroll-padding: 0 var(--size-1) 0 0;
  --log-scroll-padding-mobile: 0;
  --log-scroll-border: 0;
  --log-scroll-radius: 0;
  --log-scroll-background: transparent;
  --log-scroll-shadow:
    inset 12px 0 12px -12px rgba(0, 0, 0, 0.35),
    inset -12px 0 12px -12px rgba(0, 0, 0, 0.35);
  --log-scroll-shadow-mobile: none;

  --log-flow-shell-gap: var(--size-3);
  --log-flow-item-padding: var(--size-2) var(--size-3);
  --log-flow-item-radius: var(--ui-radius-sm);
  --log-flow-item-border:
    var(--border-size-1) solid
    color-mix(in srgb, var(--ui-text-muted) 24%, transparent);
  --log-flow-item-background: color-mix(
    in srgb,
    var(--ui-surface-card) 88%,
    transparent
  );
  --log-flow-item-transition: none;
  --log-flow-item-hover-border-color: color-mix(
    in srgb,
    var(--portfolio-accent-a) 52%,
    transparent
  );
  --log-flow-item-hover-background: color-mix(
    in srgb,
    var(--portfolio-accent-a) 14%,
    transparent
  );
  --log-flow-item-hover-transform: none;
  --log-flow-item-selected-border-color: color-mix(
    in srgb,
    var(--portfolio-accent-a) 42%,
    transparent
  );
  --log-flow-item-selected-background: var(--log-flow-item-background);
  --log-flow-item-selected-shadow: none;

  --log-flow-details-padding: 0;
  --log-flow-details-border: 0;
  --log-flow-details-radius: 0;
  --log-flow-details-background: transparent;
  --log-flow-details-shadow: none;
  --log-flow-detail-header-margin-block-end: 0;
  --log-flow-detail-header-padding-block-end: 0;
  --log-flow-detail-header-border: 0;
  --log-flow-detail-title-size: 0.9rem;
  --log-flow-event-padding-block: 0;
  --log-flow-event-border: 0;
}

me[data-log-panels] {
  grid-template-columns: repeat(auto-fit, minmax(17.5rem, 1fr));
}

me .ui-log-panel {
  display: flex;
  flex-direction: column;
  gap: var(--log-panel-gap);
  margin: 0;
  padding: var(--log-panel-padding);
  border-radius: var(--ui-radius-sm);
  border: var(--log-panel-border);
  background: var(--log-panel-background);
}

me [data-log-heading] h3 {
  margin: 0;
  font-size: var(--log-panel-heading-size);
  letter-spacing: var(--log-panel-heading-tracking);
  text-transform: var(--log-panel-heading-transform);
  color: var(--log-panel-heading-color);
}

me .ui-log-scroll {
  max-height: var(--log-scroll-max-height);
  overflow: auto;
  display: flex;
  flex-direction: column;
  gap: var(--size-4);
  min-height: 0;
  padding: var(--log-scroll-padding);
  border: var(--log-scroll-border);
  border-radius: var(--log-scroll-radius);
  background: var(--log-scroll-background);
  scrollbar-gutter: stable both-edges;
  box-shadow: var(--log-scroll-shadow);
}

me .ui-log-empty {
  margin: 0;
  font-size: 0.86rem;
  line-height: 1.45;
  color: color-mix(in srgb, var(--ui-text-muted) 94%, var(--ui-text) 6%);
}

me .ui-log-entries {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: var(--size-3);
}

me .ui-log-entry {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--size-2);
  width: 100%;
}

me [data-log-timestamp] {
  font-variant-numeric: tabular-nums;
  font-size: 0.72rem;
  line-height: 1;
}

me [data-log-message] {
  font-weight: 600;
  white-space: normal;
}

me .ui-pill--log-fields {
  font-size: 0.7rem;
  letter-spacing: 0.01rem;
}

me .ui-pill-cluster {
  display: inline-flex;
  flex-wrap: wrap;
  gap: 0.4rem;
}

me .ui-log-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.8rem;
}

me .ui-log-table th,
me .ui-log-table td {
  padding: 0.45rem 0.4rem;
  border-bottom: 1px solid color-mix(in srgb, var(--ui-text-muted) 28%, transparent);
  vertical-align: top;
}

me .ui-log-table th {
  text-align: left;
  color: var(--ui-text-muted);
  font-weight: 600;
  white-space: nowrap;
}

me .ui-log-table[data-chat-flow] td:last-child {
  min-width: 8.75rem;
}

me .ui-log-groups {
  display: flex;
  flex-direction: column;
  gap: var(--size-2);
}

me .ui-log-group {
  border: var(--border-size-1) solid color-mix(in srgb, var(--ui-text-muted) 24%, transparent);
  border-radius: var(--ui-radius-sm);
  padding: var(--size-2) var(--size-3);
  background: color-mix(
    in srgb,
    var(--ui-surface-card) 88%,
    var(--ui-text-muted) 12%
  );
}

me .ui-log-group-header {
  display: flex;
  align-items: center;
  gap: var(--size-2);
  margin-bottom: var(--size-2);
}

me .ui-log-flow-shell {
  display: grid;
  gap: var(--log-flow-shell-gap);
}

me .ui-log-flow-list {
  display: flex;
  flex-direction: column;
  gap: var(--size-2);
}

me .ui-log-flow-item {
  appearance: none;
  display: flex;
  flex-direction: column;
  gap: var(--size-1);
  margin: 0;
  padding: var(--log-flow-item-padding);
  border-radius: var(--log-flow-item-radius);
  border: var(--log-flow-item-border);
  background: var(--log-flow-item-background);
  color: inherit;
  cursor: pointer;
  font: inherit;
  text-decoration: none;
  text-align: left;
  transition: var(--log-flow-item-transition);
}

me .ui-log-flow-item:hover,
me .ui-log-flow-item:focus-visible {
  outline: none;
  border-color: var(--log-flow-item-hover-border-color);
  background: var(--log-flow-item-hover-background);
  transform: var(--log-flow-item-hover-transform);
  position: relative;
  z-index: 1;
}

me .ui-log-flow-item.is-selected {
  border-color: var(--log-flow-item-selected-border-color);
  background: var(--log-flow-item-selected-background);
  box-shadow: var(--log-flow-item-selected-shadow);
}

me .ui-log-flow-item-id {
  font-size: 0.7rem;
  letter-spacing: 0.01rem;
  color: var(--ui-text-muted);
}

me .ui-log-flow-item-title {
  font-size: 0.84rem;
  font-weight: 600;
  line-height: 1.3;
}

me .ui-log-flow-item-meta {
  display: inline-flex;
  align-items: center;
  gap: var(--size-2);
}

me .ui-log-flow-item-time {
  font-size: 0.72rem;
  font-variant-numeric: tabular-nums;
  color: var(--ui-text-muted);
}

me .ui-log-flow-details {
  min-height: 0;
  padding: var(--log-flow-details-padding);
  border: var(--log-flow-details-border);
  border-radius: var(--log-flow-details-radius);
  background: var(--log-flow-details-background);
  box-shadow: var(--log-flow-details-shadow);
}

me .ui-log-flow-detail {
  display: none;
  flex-direction: column;
  gap: var(--size-2);
}

me .ui-log-flow-detail.is-active {
  display: flex;
}

me .ui-log-flow-detail-header {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: var(--size-2);
  margin-bottom: var(--log-flow-detail-header-margin-block-end);
  padding-bottom: var(--log-flow-detail-header-padding-block-end);
  border-bottom: var(--log-flow-detail-header-border);
}

me .ui-log-flow-detail-title {
  margin: 0;
  font-size: var(--log-flow-detail-title-size);
}

me .ui-log-flow-event {
  display: flex;
  flex-direction: column;
  gap: var(--size-1);
  padding-block: var(--log-flow-event-padding-block);
  border-bottom: var(--log-flow-event-border);
}

me .ui-log-flow-event:last-child {
  padding-bottom: 0;
  border-bottom: 0;
}

me .ui-log-flow-event-head {
  display: flex;
  align-items: center;
  gap: var(--size-2);
}

me .ui-log-flow-event-summary {
  margin: 0;
  font-size: 0.82rem;
  line-height: 1.35;
}

me .ui-log-flow-event-summary-inline {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--size-1);
}

@media (min-width: 56.25rem) {
  me .ui-log-flow-shell {
    grid-template-columns: minmax(13rem, 16rem) minmax(0, 1fr);
    align-items: start;
  }

  me .ui-log-flow-list {
    max-height: 18rem;
    overflow: auto;
    padding-right: var(--size-1);
  }
}

@media (max-width: 48rem) {
  me[data-log-panels] {
    grid-template-columns: 1fr;
    gap: var(--size-3);
  }

  me .ui-log-scroll {
    max-height: var(--log-scroll-max-height-mobile);
    padding: var(--log-scroll-padding-mobile);
    box-shadow: var(--log-scroll-shadow-mobile);
  }

  me .ui-log-table {
    min-width: 35rem;
  }

  me .ui-log-group-header {
    flex-wrap: wrap;
  }

  me .ui-log-flow-list {
    max-height: none;
    overflow: visible;
    padding-right: 0;
  }
}

@media (max-width: 30rem) {
  me .ui-log-scroll {
    max-height: var(--log-scroll-max-height-compact);
  }

  me .ui-log-table {
    min-width: 32.5rem;
    font-size: 0.74rem;
  }

  me .ui-log-entry {
    align-items: flex-start;
  }
}
"#
);

#[derive(Clone, Copy, Debug, Default)]
pub enum SurfaceLayout {
    #[default]
    Stack,
    Panels,
}

impl SurfaceLayout {
    fn class_name(self) -> &'static str {
        match self {
            SurfaceLayout::Stack => "ui-log-surface",
            SurfaceLayout::Panels => "ui-log-surface ui-log-panels",
        }
    }

    fn is_panels(self) -> bool {
        matches!(self, SurfaceLayout::Panels)
    }
}

// ci: render-composition-component
#[derive(Clone, Debug, Builder)]
pub struct Surface {
    pub target_id: Option<Text>,
    #[builder(default)]
    pub layout: SurfaceLayout,
    #[builder(default)]
    pub classes: Vec<Text>,
    pub children: Vec<logs::primitives::Panel>,
    pub auto_scroll: Option<logs::primitives::AutoScroll>,
}

impl Render for Surface {
    fn render(&self) -> maud::Markup {
        let mut class_names = vec![self.layout.class_name().to_string()];
        class_names.extend(self.classes.iter().map(ToString::to_string));
        let class_attr = class_names.join(" ");

        if let Some(target_id) = &self.target_id {
            maud::html! {
                section
                    id=(target_id)
                    class=(class_attr)
                    data-log-panels[self.layout.is_panels()] {
                    (css())
                    @for child in &self.children {
                        (child)
                    }
                    @if let Some(auto_scroll) = &self.auto_scroll {
                        (auto_scroll)
                    }
                }
            }
        } else {
            maud::html! {
                section class=(class_attr) data-log-panels[self.layout.is_panels()] {
                    (css())
                    @for child in &self.children {
                        (child)
                    }
                    @if let Some(auto_scroll) = &self.auto_scroll {
                        (auto_scroll)
                    }
                }
            }
        }
    }
}
