use bon::Builder;

use crate::paths::Route;
use crate::types::Text;
use crate::views::{page, partials};

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-section);
  margin-top: clamp(1.2rem, 0.9rem + 1.2vw, 2rem);
  padding-bottom: calc(var(--space-section) + var(--space-7));
}

me > :where(header, section) {
  margin-top: 0;
  scroll-margin-top: var(--nav-scroll-offset);
}

me [data-operations-surface] {
  --log-panel-gap: var(--space-3);
  --log-panel-padding: 0;
  --log-panel-border: 0;
  --log-panel-background: transparent;
  --log-panel-heading-size: var(--text-size-label-xs);
  --log-panel-heading-tracking: var(--text-track-caps-wider);
  --log-panel-heading-transform: uppercase;
  --log-panel-heading-color: var(--text-subtle);

  --log-scroll-max-height: 22rem;
  --log-scroll-max-height-mobile: 18rem;
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
  me [data-operations-surface] {
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

#[derive(Builder)]
pub struct Lab {
    pub user: Option<page::UserNav>,
    pub chat_demo: Option<partials::chat::DemoSection>,
    pub sse_tab_id: Option<crate::types::SseTabId>,
}

impl maud::Render for Lab {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            div data-lab-page data-page-section {
                (css())
                (partials::HomeHero::builder().maybe_user(self.user.clone()).build())

                (partials::TabSetShowcase::builder().build())

                (partials::RequestBurstDemo::builder()
                    .endpoint(Text::from(Route::PartialRequestBurstProbe.as_str()))
                    .build())

                @if let Some(chat_demo) = &self.chat_demo { (chat_demo.render()) } @else {
                    section
                        id=(partials::chat::DemoSection::ANCHOR_ID)
                        class="u-surface-card"
                        data-chat-surface-variant="lab"
                    {
                        ({
                            partials::SectionHeader::builder()
                                .title(Text::from("Live chat room"))
                                .subtitle(
                                    Text::from(
                                        "Sign in to send messages and see the chat room.",
                                    ),
                                )
                                .action(partials::button::Button::builder()
                                    .label(Text::from("Sign in"))
                                    .variant(partials::button::Variant::Secondary)
                                    .role(partials::button::Role::link(Route::Login.as_str()))
                                    .build())
                                .build()
                        })
                    }
                }

                section
                    id="operations-surface"
                    class="u-surface-card"
                    data-operations-surface
                {
                    ({
                        partials::SectionHeader::builder()
                            .title(Text::from("Operational View"))
                            .subtitle(
                                Text::from(
                                    "Run a demo interaction, then follow request, backend, and SSE behavior in one timeline.",
                                ),
                            )
                            .build()
                    })
                    ({
                        partials::OperationalRequestFilter::builder()
                            .target_id("network-log-target")
                            .build()
                    })
                    ({
                        partials::DemoResultPlaceholder::builder()
                            .target_id(Text::from("network-log-target"))
                            .message(
                                Text::from(
                                    "No timeline events yet. Trigger a demo action to populate this view.",
                                ),
                            )
                            .build()
                    })
                }

                (partials::EngineeringQuality::builder().build())
            }
        };
        let content = page::Frame::builder().content(content).build().render();

        page::Layout::builder()
            .title("Live Lab")
            .content(content)
            .sse_mode(page::SseMode::Enabled)
            .maybe_with_user(self.user.clone())
            .maybe_sse_tab_id(self.sse_tab_id.clone())
            .build()
            .render()
    }
}
