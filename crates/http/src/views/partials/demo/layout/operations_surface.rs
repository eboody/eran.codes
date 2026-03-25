use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

use super::{Attr, SurfaceSection};

crate::views::scoped::inline_css!(
    r#"
me[data-operations-surface] {
  --log-panel-gap: var(--space-3);
  --log-panel-padding: 0;
  --log-panel-border: 0;
  --log-panel-background: transparent;
  --log-panel-heading-size: var(--text-size-label-xs);
  --log-panel-heading-tracking: var(--text-track-caps-wider);
  --log-panel-heading-transform: uppercase;
  --log-panel-heading-color: var(--text-subtle);

  --log-scroll-max-height: 22rem;
  --log-scroll-max-height-mobile: 16rem;
  --log-scroll-max-height-compact: 18rem;
  --log-scroll-padding: var(--space-card);
  --log-scroll-padding-mobile: var(--space-card);
  --log-scroll-border: 1px solid var(--border-default);
  --log-scroll-radius: var(--ui-radius-md-inset);
  --log-scroll-background: var(--surface-fill-field);
  --log-scroll-shadow: inset 0 1px 0 var(--surface-edge-default);
  --log-scroll-shadow-mobile: inset 0 1px 0 var(--surface-edge-default);

  --log-flow-shell-gap: var(--space-4);
  --log-flow-item-padding: var(--space-3) var(--space-4);
  --log-flow-item-radius: var(--radius-control);
  --log-flow-item-border:
    1px solid color-mix(in srgb, var(--border-default) 90%, transparent);
  --log-flow-item-background: color-mix(
    in srgb,
    var(--surface-field) 82%,
    transparent
  );
  --log-flow-item-transition:
    border-color var(--motion-fast),
    background-color var(--motion-fast),
    box-shadow var(--motion-fast),
    transform var(--motion-fast);
  --log-flow-item-hover-transform: translateY(-1px);
  --log-flow-item-selected-border-color: color-mix(
    in srgb,
    var(--accent-signal) 30%,
    var(--border-default)
  );
  --log-flow-item-selected-background: color-mix(
    in srgb,
    var(--accent-signal) 9%,
    var(--surface-panel)
  );
  --log-flow-item-selected-shadow: inset 0 1px 0 var(--surface-edge-strong);

  --log-flow-details-padding: var(--space-card);
  --log-flow-details-border: 1px solid var(--border-default);
  --log-flow-details-radius: var(--ui-radius-md-inset);
  --log-flow-details-background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-warm-soft) 28%, transparent),
      transparent 42%
    ),
    var(--surface-raised);
  --log-flow-details-shadow: inset 0 1px 0 var(--surface-edge-default);
  --log-flow-detail-header-margin-block-end: var(--space-2);
  --log-flow-detail-header-padding-block-end: var(--space-2);
  --log-flow-detail-header-border: 1px solid var(--border-subtle);
  --log-flow-detail-title-size: var(--text-size-body-lg);
  --log-flow-event-padding-block: var(--space-1);
  --log-flow-event-border:
    1px solid color-mix(in srgb, var(--border-subtle) 72%, transparent);
}

@media (prefers-color-scheme: dark) {
  me[data-operations-surface] {
    --log-scroll-shadow: inset 0 1px 0 var(--surface-edge-soft);
    --log-scroll-shadow-mobile: inset 0 1px 0 var(--surface-edge-soft);
    --log-flow-details-background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 28%),
      color-mix(in srgb, var(--surface-field) 92%, black 8%);
    --log-flow-details-shadow: inset 0 1px 0 var(--surface-edge-soft);
    --log-flow-item-background: color-mix(
      in srgb,
      var(--surface-field) 90%,
      black 10%
    );
    --log-flow-item-selected-background:
      linear-gradient(
        180deg,
        color-mix(
          in srgb,
          var(--accent-signal) 10%,
          var(--surface-wash-top-soft)
        ),
        transparent 30%
      ),
      color-mix(in srgb, var(--accent-signal) 14%, var(--surface-field));
  }
}
"#
);

#[derive(Clone, Debug, Builder)]
pub struct OperationsSurface {}

impl Render for OperationsSurface {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::lab_page_content();

        SurfaceSection::builder()
            .id(Text::from("operations-surface"))
            .title(content.operations_surface.title.clone())
            .subtitle(content.operations_surface.subtitle.clone())
            .attrs(vec![Attr::flag("data-operations-surface")])
            .content(maud::html! {
                (css())
                (partials::OperationalRequestFilter::builder()
                    .target_id("network-log-target")
                    .build())
                (partials::DemoResultPlaceholder::builder()
                    .target_id(Text::from("network-log-target"))
                    .message(content.operations_surface.empty_message.clone())
                    .build())
            })
            .build()
            .render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_operations_surface_target() {
        let markup = OperationsSurface::builder().build().render().into_string();

        assert!(markup.contains("id=\"operations-surface\""));
        assert!(markup.contains("data-operations-surface"));
        assert!(markup.contains("network-log-target"));
    }
}
