use maud::Render;
use maud_extensions::inline_css;

#[derive(Clone, Copy, Debug, Default)]
pub struct Styles;

inline_css! {
    me {
      --chat-space-1: 0.5rem;
      --chat-space-2: 0.7rem;
      --chat-space-3: 1rem;
      --chat-space-4: 1.35rem;
      --chat-space-5: 1.45rem;
      --chat-gap-columns: 1rem;
      --chat-surface-margin-top: 2.8rem;
      --chat-window-height: clamp(15.5rem, 38vh, 21.25rem);

      --chat-radius-surface: 18px;
      --chat-radius-window: 14px;
      --chat-radius-compose: 12px;
      --chat-send-min-width: 7.25rem;
      --chat-readonly-pad-inline: 0.8rem;

      --chat-font-label: 0.74rem;
      --chat-font-micro: 0.66rem;
      --chat-font-status: 0.64rem;
      --chat-font-avatar: 0.68rem;
      --chat-font-body: 0.92rem;
      --chat-font-body-sm: 0.88rem;
      --chat-line-body: 1.38;

      --chat-space-dot-gap: 0.22rem;
      --chat-live-dot-size: 0.45rem;
      --chat-live-dot-ring-size: 0.1rem;

      --chat-message-gap: 0.58rem;
      --chat-avatar-gap: 0.42rem;
      --chat-meta-gap: 0.55rem;
      --chat-message-body-top: 0.18rem;
      --chat-avatar-size: 1.65rem;
      --chat-bubble-max: min(88%, 34ch);
      --chat-bubble-left-radius: 14px 14px 14px 5px;
      --chat-bubble-right-radius: 14px 14px 5px 14px;

      --chat-shell-border: #1e293b;
      --chat-shell-bg:
        radial-gradient(circle at 0% 0%, color-mix(in srgb, #1d4ed8 30%, transparent), transparent 40%),
        linear-gradient(180deg, #0f172a, #111827);
      --chat-shell-shadow: 0 10px 22px color-mix(in srgb, black 35%, transparent);
      --chat-shell-header-bg: color-mix(in srgb, rgb(11 18 32) 84%, #111827 16%);
      --chat-shell-header-separator: #1e293b;
      --chat-shell-title: #dbeafe;
      --chat-shell-live: #93c5fd;
      --chat-shell-live-dot: color-mix(in srgb, #22c55e 78%, #bbf7d0 22%);
      --chat-shell-live-ring: color-mix(in srgb, #22c55e 24%, transparent);

      --chat-compose-border: color-mix(in srgb, var(--ui-text-muted) 18%, transparent);
      --chat-compose-surface: color-mix(in srgb, var(--ui-surface-card) 90%, transparent);

      --chat-avatar-border: #334155;
      --chat-avatar-bg-left: #334155;
      --chat-avatar-fg-left: #e2e8f0;
      --chat-avatar-bg-right: #1d4ed8;
      --chat-avatar-border-right: #3b82f6;
      --chat-avatar-fg-right: #eff6ff;
      --chat-bubble-left-border: #334155;
      --chat-bubble-left-bg: #1f2937;
      --chat-bubble-left-fg: #e2e8f0;
      --chat-bubble-right-border: #3b82f6;
      --chat-bubble-right-bg: #1d4ed8;
      --chat-bubble-right-fg: #eff6ff;
      --chat-meta-fg-strong: #f8fafc;
      --chat-meta-fg-muted: #94a3b8;
      --chat-body-fg: #f8fafc;

      margin-top: var(--chat-surface-margin-top);
      border: 1px solid var(--portfolio-surface-border);
      border-radius: var(--chat-radius-surface);
      padding: var(--chat-space-4) var(--chat-space-4) var(--chat-space-5);
      background: var(--portfolio-surface);
      box-shadow: 0 6px 16px color-mix(in srgb, black 8%, transparent);
    }
    me > [data-chat-columns] {
      display: grid;
      gap: var(--chat-gap-columns);
      grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    }
    @media (max-width: 960px) {
      me > [data-chat-columns] {
        grid-template-columns: 1fr;
      }
    }
    @media (max-width: 768px) {
      me {
        --chat-space-4: 1rem;
        --chat-space-5: 1.1rem;
        --chat-surface-margin-top: 1.8rem;
        --chat-window-height: clamp(14rem, 35vh, 17.875rem);
        --chat-radius-surface: 16px;
      }
    }
    @media (max-width: 520px) {
      me {
        --chat-window-height: clamp(13.5rem, 32vh, 15.5rem);
      }
    }
}

impl Render for Styles {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (css())
        }
    }
}
