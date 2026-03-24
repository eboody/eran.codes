# Site Content Authoring Guide

The public site now reads from one checked-in content directory at:

- `crates/http/src/views/partials/components/composed/portfolio/content/site_content/`

Use that directory for authored public claims, section order, shared CTA bundles, and page-level narrative copy.

## Put It In The Content Root When

- the text is part of the public narrative on `/`, `/work`, `/open-source`, `/lab`, or `/resume.txt`
- the value is a reusable CTA, nav label, contact method, project summary, role summary, or other authored public metadata
- the content should stay consistent across multiple public surfaces

## Keep It In Code When

- the text depends on runtime state, auth state, trace output, query results, or background-job state
- the value is derived from enums, request handling, transport contracts, or operational status
- the UI needs behavior-owned fallback text that is tied to execution rather than authored content

## Current Structure

- `identity.json` holds the public name, location, and headline.
- `actions.json` holds shared CTA definitions plus `action_bundles` for repeated action sets.
- `nav.json` holds the portfolio nav references.
- `contact.json`, `experience.json`, `projects.json`, `work_cases.json`, `open_source.json`, and `skills.json` are the main authored collections.
- `pages/home.json`, `pages/work.json`, `pages/open_source.json`, `pages/lab.json`, and `pages/resume.json` hold page order and framing.

## Editing Rules

- Reuse an existing `action_links` entry before adding another near-duplicate CTA.
- Reuse an existing `action_bundles` entry before repeating the same action-ref list across pages or work cases.
- If a new internal link is added, make it a valid repo route. Content validation now rejects invalid internal hrefs.
- Prefer contact-method references over duplicating public contact URLs in multiple places.
- Keep fragment-local IDs unique. Validation now fails with the fragment path when IDs collide or required fragment sections go empty.
- Do not move runtime status strings into the JSON root just because they are user-visible. If the text is owned by execution, it stays in code.

## Maintenance Checklist

- Changing the home narrative:
  Edit `pages/home.json`, then check whether the change should also affect `pages/resume.json`.
- Adding or rewriting a work case:
  Update `projects.json` for the card, `work_cases.json` for the detailed case, then wire the slug into the relevant page fragments.
- Updating repeated CTA behavior:
  Prefer `actions.json` and `action_bundles` before hand-authoring another page-local action list.
- Changing canonical nav or contact paths:
  Update `actions.json`, `nav.json`, and `contact.json` together so shared references stay coherent.

## Route Policy Discipline

- Treat `/`, `/lab`, `/work/sensitive-sync`, and `/open-source` as the canonical reviewer path.
- Treat `/work` as the supporting-proof index and archive surface, not a second home page.
- Treat the legacy `/work/*` case pages as archived supporting proof that redirect to `/work#...` anchors, not current flagship proof.
- Use `/work#chat-realtime`, `/work#command-sse`, and `/work#operational-visibility` for direct archive links instead of the old leaf routes.
- Keep current-proof links pointing to `/work/sensitive-sync` and live-proof links pointing to `/lab`.
