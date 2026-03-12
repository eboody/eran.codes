// ci: descriptive-module-import crate::views::partials::components::chat
mod composer;
mod connection;
mod hero;
mod message;
mod notice;
mod panel;
mod set;
mod surface;
mod window;

pub use hero::Hero;
pub use message::{Message, Status};
pub use surface::{Mode, Surface, Variant};

pub(crate) use composer::Composer;
pub(crate) use connection::Connection;
pub(crate) use message::Side;
pub(crate) use notice::Notice;
pub(crate) use panel::Panel;
pub(crate) use set::Set;
pub(crate) use window::Window;

crate::views::scoped::inline_css!(
    r#"
me {
  --chat-space-1: var(--space-1);
  --chat-space-2: var(--space-2);
  --chat-space-4: var(--space-card);
  --chat-space-5: calc(var(--space-card) + var(--space-1));
  --chat-gap-columns: var(--space-3);
  --chat-window-height: clamp(15.5rem, 38vh, 21.25rem);

  --chat-radius-window: calc(var(--radius-card) - 4px);
  --chat-radius-compose: var(--radius-control);
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

  --chat-shell-border: var(--border-default);
  --chat-shell-bg: var(--surface-fill-field);
  --chat-shell-shadow: inset 0 1px 0 var(--surface-edge-soft);
  --chat-shell-header-bg: color-mix(
    in srgb,
    var(--surface-raised) 94%,
    transparent
  );
  --chat-shell-header-separator: var(--border-subtle);
  --chat-shell-title: var(--text-strong);
  --chat-shell-live: var(--status-success);
  --chat-shell-live-dot: var(--status-success);
  --chat-shell-live-ring: color-mix(in srgb, var(--status-success) 20%, transparent);

  --chat-compose-border: var(--border-default);
  --chat-compose-surface: var(--surface-fill-field);

  --chat-avatar-border: color-mix(in srgb, var(--border-default) 92%, transparent);
  --chat-avatar-bg-left:
    color-mix(in srgb, var(--accent-warm-soft) 24%, var(--surface-raised));
  --chat-avatar-fg-left: var(--text-strong);
  --chat-avatar-bg-right:
    color-mix(in srgb, var(--accent-signal-soft) 34%, var(--surface-raised));
  --chat-avatar-border-right:
    color-mix(in srgb, var(--accent-signal) 14%, var(--border-default));
  --chat-avatar-fg-right: var(--text-strong);
  --chat-bubble-left-border: var(--border-subtle);
  --chat-bubble-left-bg:
    color-mix(in srgb, var(--surface-raised) 90%, var(--surface-field));
  --chat-bubble-left-fg: var(--text-body);
  --chat-bubble-right-border:
    color-mix(in srgb, var(--accent-signal) 14%, var(--border-default));
  --chat-bubble-right-bg:
    color-mix(in srgb, var(--accent-signal-soft) 24%, var(--surface-panel));
  --chat-bubble-right-fg: var(--text-body);
  --chat-meta-fg-strong: var(--text-strong);
  --chat-meta-fg-muted: var(--text-subtle);
  --chat-body-fg: var(--text-body);

  margin-top: 0;
}

me .button {
  position: relative;
  z-index: 0;
}

me .button:hover,
me .button:focus-visible {
  z-index: 1;
}

me [data-chat-connection-row] {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.4rem;
  margin: 0;
}

me [data-chat-connection-state='connected'] {
  border-color: color-mix(in srgb, var(--status-success) 34%, transparent);
  color: var(--status-success);
}

me [data-chat-connection-state='disconnected'] {
  border-color: color-mix(in srgb, var(--status-warn) 34%, transparent);
  color: var(--status-warn);
}

me > [data-chat-columns] {
  display: grid;
  gap: var(--chat-gap-columns);
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
}

me > [data-chat-columns] > [data-chat-panel] {
  display: grid;
  grid-template-rows: minmax(0, 1fr) auto;
  gap: var(--chat-space-2);
  min-height: 0;
}

me > [data-chat-columns] > [data-chat-panel] > [data-chat-compose] {
  display: grid;
  gap: var(--chat-space-1);
  border: 1px solid var(--chat-compose-border);
  border-radius: var(--chat-radius-compose);
  background: var(--chat-compose-surface);
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  padding: var(--chat-space-2);
}

me > [data-chat-columns] > [data-chat-panel] > [data-chat-compose] > label {
  margin: 0;
}

me > [data-chat-columns] > [data-chat-panel] > [data-chat-compose] > label > [data-chat-compose-label] {
  display: block;
  margin: 0;
  font-size: var(--chat-font-label);
  font-weight: 700;
  letter-spacing: 0.05rem;
  text-transform: uppercase;
  color: color-mix(in srgb, var(--text-subtle) 92%, var(--text-body) 8%);
}

me > [data-chat-columns] > [data-chat-panel] > [data-chat-compose] > [data-chat-compose-row] {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: var(--chat-space-1);
}

me > [data-chat-columns] > [data-chat-panel] > [data-chat-compose] > [data-chat-compose-row] > input[type="text"] {
  margin: 0;
  min-width: 0;
}

me > [data-chat-columns] > [data-chat-panel] > [data-chat-compose] > [data-chat-compose-row] > button {
  margin: 0;
  min-width: var(--chat-send-min-width);
}

me > [data-chat-columns] > [data-chat-panel] > [data-chat-readonly] {
  border: 1px dashed var(--border-subtle);
  border-radius: var(--chat-radius-compose);
  background: var(--surface-fill-field);
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  padding: var(--chat-space-2) var(--chat-readonly-pad-inline);
  font-size: var(--chat-font-body-sm);
  color: var(--text-muted);
}

me [data-chat-window] {
  border: 1px solid var(--chat-shell-border);
  border-radius: var(--chat-radius-window);
  background: var(--chat-shell-bg);
  box-shadow: var(--chat-shell-shadow);
  overflow: hidden;
  height: var(--chat-window-height);
  display: grid;
  grid-template-rows: auto minmax(0, 1fr);
}

me [data-chat-window] > header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--chat-space-1);
  margin: 0;
  padding: var(--chat-space-1) var(--chat-space-2);
  border-bottom: 1px solid var(--chat-shell-header-separator);
  background: var(--chat-shell-header-bg);
}

me [data-chat-window] > header > [data-chat-role] {
  font-size: var(--chat-font-label);
  font-weight: 700;
  letter-spacing: 0.04rem;
  text-transform: uppercase;
  color: var(--chat-shell-title);
}

me [data-chat-window] > header > [data-chat-room-state="live"] {
  display: inline-flex;
  align-items: center;
  gap: var(--chat-space-dot-gap);
  font-size: var(--chat-font-micro);
  font-weight: 700;
  letter-spacing: 0.05rem;
  text-transform: uppercase;
  color: var(--chat-shell-live);
}

me [data-chat-window] > header > [data-chat-room-state="live"]::before {
  content: "";
  width: var(--chat-live-dot-size);
  height: var(--chat-live-dot-size);
  border-radius: 999px;
  background: var(--chat-shell-live-dot);
  box-shadow: 0 0 0 var(--chat-live-dot-ring-size) var(--chat-shell-live-ring);
}

me [data-chat-window] > header > [data-chat-room-state="offline"] {
  font-size: var(--chat-font-micro);
  font-weight: 700;
  letter-spacing: 0.05rem;
  text-transform: uppercase;
  color: color-mix(in srgb, var(--text-subtle) 90%, var(--chat-shell-title) 10%);
}

me [data-chat-feed] {
  min-height: 0;
  display: flex;
}

me [data-chat-feed] > [data-chat-messages] {
  list-style: none;
  margin: 0;
  padding: var(--chat-space-2);
  display: flex;
  flex-direction: column;
  gap: var(--chat-message-gap);
  overflow-y: auto;
  flex: 1;
  min-height: 0;
}

me [data-chat-feed] > [data-chat-messages] > [data-chat-message] {
  display: flex;
  align-items: flex-end;
  gap: var(--chat-avatar-gap);
}

me [data-chat-feed] > [data-chat-messages][data-chat-side="right"] > [data-chat-message] {
  justify-content: flex-end;
}

me [data-chat-feed] > [data-chat-messages] > [data-chat-message] > [data-chat-avatar] {
  width: var(--chat-avatar-size);
  height: var(--chat-avatar-size);
  border-radius: 999px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: var(--chat-font-avatar);
  font-weight: 700;
  border: 1px solid var(--chat-avatar-border);
  background: var(--chat-avatar-bg-left);
  color: var(--chat-avatar-fg-left);
}

me [data-chat-feed] > [data-chat-messages][data-chat-side="right"] > [data-chat-message] > [data-chat-avatar] {
  order: 2;
  background: var(--chat-avatar-bg-right);
  border-color: var(--chat-avatar-border-right);
  color: var(--chat-avatar-fg-right);
}

me [data-chat-feed] > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] {
  border: 1px solid var(--chat-bubble-left-border);
  border-radius: var(--chat-bubble-left-radius);
  background: var(--chat-bubble-left-bg);
  color: var(--chat-bubble-left-fg);
  padding: var(--chat-space-1) var(--chat-space-2) var(--chat-space-2);
  min-width: 0;
  max-width: var(--chat-bubble-max);
}

me [data-chat-feed] > [data-chat-messages][data-chat-side="right"] > [data-chat-message] > [data-chat-bubble] {
  order: 1;
  border-radius: var(--chat-bubble-right-radius);
  border-color: var(--chat-bubble-right-border);
  background: var(--chat-bubble-right-bg);
  color: var(--chat-bubble-right-fg);
}

me [data-chat-feed] > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--chat-meta-gap);
}

me [data-chat-feed] > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] > strong {
  font-size: var(--chat-font-label);
  font-weight: 700;
  color: var(--chat-meta-fg-strong);
}

me [data-chat-feed] > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] > [data-chat-timestamp] {
  font-size: var(--chat-font-micro);
  line-height: 1.1;
  color: var(--chat-meta-fg-muted);
}

me [data-chat-feed] > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] > [data-chat-status] {
  font-size: var(--chat-font-status);
  line-height: 1.1;
  padding: 0.1rem 0.32rem;
  border-radius: 999px;
  border: 1px solid color-mix(in srgb, var(--border-default) 92%, transparent);
  background: color-mix(in srgb, var(--surface-panel) 76%, transparent);
  color: color-mix(in srgb, var(--text-subtle) 92%, var(--text-body) 8%);
}

me [data-chat-feed] > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] > [data-chat-status-kind="visible"] {
  display: none;
}

me [data-chat-feed] > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] > [data-chat-status-kind="pending"] {
  border-color: color-mix(in srgb, var(--status-warn) 38%, transparent);
  color: color-mix(in srgb, var(--status-warn) 74%, var(--text-body) 26%);
}

me [data-chat-feed] > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] > [data-chat-status-kind="removed"] {
  border-color: color-mix(in srgb, var(--status-danger) 38%, transparent);
  color: color-mix(in srgb, var(--status-danger) 74%, var(--text-body) 26%);
}

me [data-chat-feed] > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-body] {
  margin: var(--chat-message-body-top) 0 0;
  font-size: var(--chat-font-body);
  line-height: var(--chat-line-body);
  overflow-wrap: anywhere;
  color: var(--chat-body-fg);
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
    --chat-window-height: clamp(14rem, 35vh, 17.875rem);
  }
}

@media (max-width: 520px) {
  me {
    --chat-window-height: clamp(13.5rem, 32vh, 15.5rem);
  }

  me > [data-chat-columns] > [data-chat-panel] > [data-chat-compose] > [data-chat-compose-row] {
    grid-template-columns: 1fr;
  }

  me > [data-chat-columns] > [data-chat-panel] > [data-chat-compose] > [data-chat-compose-row] > button {
    width: 100%;
    min-width: 0;
  }
}
"#
);
