# Package Catalog

This file documents the current shared `ui-*` package surface in `crates/http/static/app.css`.

## Core Surfaces

- `.ui-surface-card`: generic shared surface chrome
- `.ui-panel`: panel body shell for tab and content regions
- `.ui-preview-frame`: media and preview frame shell
- `.ui-feature-list`: shared feature list treatment
- `.ui-cta`: primary shared call-to-action surface

## Navigation

- `.ui-nav-shell`: outer sticky shell
- `.ui-nav`: nav layout container
- `.ui-nav-list`: shared nav list packaging
- `.ui-nav-brand`: brand cluster
- `.ui-nav-links`: primary nav link cluster
- `.ui-nav-auth`: auth action cluster
- `.ui-nav-link`: reusable nav link affordance
- `.ui-nav-auth-action`: signed-in auth button treatment

## Sections And Layout

- `.ui-section-header`: section heading package
- `.ui-section-meta`: section metadata row
- `.ui-cta-row`: grouped CTA row layout
- `.ui-grid-two-column`: generic two-column layout shell

## Tabs And Information Panels

- `.ui-tabs`: tabs rail container
- `.ui-tab`: shared tab affordance
- `.ui-info-grid`: information grid layout
- `.ui-info-card`: information card surface

## Domain Families

- `.ui-portfolio-*`: portfolio surfaces and sections
- `.ui-log-*`: log viewer shells, rows, tables, and detail panes
- `.ui-chat-*`: chat demo and moderation layouts
- `.ui-burst-*`: request burst demo controls and results
- `.ui-op-filter*`: operational filter controls

Add a new shared package only when the role is reusable across components. If the rule is still tied to one component's content or layout, keep it scoped.
