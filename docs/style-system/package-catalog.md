# Global CSS Catalog

`crates/http/static/app.css` is the shared layer for theme tokens, element defaults, and utilities.

## Permanent Utilities

- `.u-container`: shared page-width wrapper
- `.u-muted`: muted text utility
- `.u-surface-card`: reusable card/surface shell

## Migration Holdovers

- none

## Removed From Global CSS

- `.ui-button` and `.ui-button-row` base package styles
- `.ui-nav-*`
- `.ui-auth-*` and `.ui-account-*`
- `.ui-section-header` and `.ui-section-meta`
- `.ui-demo-result`
- `.ui-error-alert`
- `.ui-key-value-list`
- `.ui-status-card`
- `.ui-op-filter*`
- `.ui-burst-*`
- `.ui-home-hero*`
- `.ui-chat-connection-row`
- `.ui-ping-target`
- `.ui-info-*`
- `.ui-panel`
- `.ui-preview-frame`
- `.ui-feature-list`
- `.ui-chat-page*`
- `.ui-chat-moderation-*`
- `.ui-lab-tab-set`
- `.ui-tabs` and `.ui-tab`
- `.tab-set__tab-icon`
- `.ui-lab-main`
- `.ui-lab-chat-surface`
- `.ui-portfolio-*`
- `.ui-log-*`
- `.ui-surface-card`
- `.ui-pill*`
- `.ui-icon`
- shared `.button[data-button]` styles
- `.container`
- `.muted`
- `[data-muted]`

If a style only belongs to one component, keep it with that component via `inline_css!` instead of adding it back to `app.css`.
