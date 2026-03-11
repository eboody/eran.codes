use bon::Builder;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::{SseMode, UserNav};
use crate::views::partials::{
    DemoResultPlaceholder, EngineeringQuality, HomeHero, OperationalRequestFilter,
    RequestBurstDemo, SectionHeader, TabSetShowcase, button, chat,
};

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-section);
  margin-top: clamp(1.2rem, 0.9rem + 1.2vw, 2rem);
  padding-bottom: calc(var(--space-section) + var(--size-7));
}

me > :where(header, section) {
  margin-top: 0;
  scroll-margin-top: var(--nav-scroll-offset);
  transition:
    opacity var(--motion-standard),
    transform var(--motion-standard),
    border-color var(--motion-standard),
    box-shadow var(--motion-standard);
}

@starting-style {
  me > :where(header, section) {
    opacity: 0;
    transform: translateY(0.8rem);
  }
}

me [data-operations-surface] {
  --log-panel-gap: var(--size-3);
  --log-panel-padding: 0;
  --log-panel-border: 0;
  --log-panel-background: transparent;
  --log-panel-heading-size: 0.72rem;
  --log-panel-heading-tracking: 0.14em;
  --log-panel-heading-transform: uppercase;
  --log-panel-heading-color: var(--text-subtle);

  --log-scroll-max-height: 22rem;
  --log-scroll-max-height-mobile: 18rem;
  --log-scroll-max-height-compact: 18rem;
  --log-scroll-padding: var(--space-card);
  --log-scroll-padding-mobile: var(--space-card);
  --log-scroll-border: 1px solid var(--border-default);
  --log-scroll-radius: calc(var(--radius-card) - 2px);
  --log-scroll-background: var(--surface-fill-field);
  --log-scroll-shadow: inset 0 1px 0 var(--surface-edge-default);
  --log-scroll-shadow-mobile: inset 0 1px 0 var(--surface-edge-default);

  --log-flow-shell-gap: var(--size-4);
  --log-flow-item-padding: 0.8rem 0.9rem;
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
  --log-flow-details-radius: calc(var(--radius-card) - 2px);
  --log-flow-details-background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-warm-soft) 28%, transparent),
      transparent 42%
    ),
    var(--surface-raised);
  --log-flow-details-shadow: inset 0 1px 0 var(--surface-edge-default);
  --log-flow-detail-header-margin-block-end: var(--size-2);
  --log-flow-detail-header-padding-block-end: var(--size-2);
  --log-flow-detail-header-border: 1px solid var(--border-subtle);
  --log-flow-detail-title-size: 1rem;
  --log-flow-event-padding-block: 0.35rem;
  --log-flow-event-border:
    1px solid color-mix(in srgb, var(--border-subtle) 72%, transparent);
}

me [data-chat-surface][data-lab-chat-surface] {
  margin-top: 0;
  border-color: var(--border-default);
  background: var(--surface-fill-panel);
  box-shadow: var(--shadow-panel);
}

me [data-lab-chat-surface]:not([data-chat-surface]) {
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-signal-soft) 26%, transparent),
      transparent 42%
    ),
    var(--surface-panel);
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

  me [data-chat-surface][data-lab-chat-surface],
  me [data-lab-chat-surface]:not([data-chat-surface]) {
    background:
      linear-gradient(
        180deg,
        var(--surface-wash-top-soft),
        transparent 34%
      ),
      color-mix(in srgb, var(--surface-panel) 92%, black 8%);
  }

  me [data-chat-surface][data-lab-chat-surface],
  me [data-lab-chat-surface]:not([data-chat-surface]) {
    box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  }
}
"#
);

#[derive(Builder)]
pub struct Lab {
    pub user: Option<UserNav>,
    pub chat_demo: Option<chat::DemoSection>,
}

impl maud::Render for Lab {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="u-container" data-lab-page {
                (css())
                (HomeHero::builder().maybe_user(self.user.clone()).build())

                (TabSetShowcase::builder().build())

                (RequestBurstDemo::builder()
                    .endpoint(Text::from(Route::PartialRequestBurstProbe.as_str()))
                    .build())

                @if let Some(chat_demo) = &self.chat_demo { (chat_demo.render()) } @else {
                    section
                        id=(chat::DemoSection::ANCHOR_ID)
                        class="u-surface-card"
                        data-lab-chat-surface
                    {
                        ({
                            SectionHeader::builder()
                                .title(Text::from("Live chat room"))
                                .subtitle(
                                    Text::from(
                                        "Sign in to send messages and see the chat room.",
                                    ),
                                )
                                .action(button::Button::builder()
                                    .label(Text::from("Sign in"))
                                    .variant(button::Variant::Secondary)
                                    .role(button::Role::link(Route::Login.as_str()))
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
                        SectionHeader::builder()
                            .title(Text::from("Operational View"))
                            .subtitle(
                                Text::from(
                                    "Run a demo interaction, then follow request, backend, and SSE behavior in one timeline.",
                                ),
                            )
                            .build()
                    })
                    ({
                        OperationalRequestFilter::builder()
                            .target_id("network-log-target")
                            .build()
                    })
                    ({
                        DemoResultPlaceholder::builder()
                            .target_id(Text::from("network-log-target"))
                            .message(
                                Text::from(
                                    "No timeline events yet. Trigger a demo action to populate this view.",
                                ),
                            )
                            .build()
                    })
                }

                (EngineeringQuality::builder().build())
            }
        };

        crate::views::page::Layout::builder()
            .title("Live Lab")
            .content(content)
            .sse_mode(SseMode::Enabled)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
