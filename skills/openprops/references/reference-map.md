# Open Props Reference Map

Snapshot date: 2026-02-28

## Canonical Sources Captured

- Upstream repo snapshot (source of truth for docs, changelog, and token sources):
  - `upstream/open-props-main/`
  - `upstream/open-props-main.tar.gz`
- Published website mirror (what users see on open-props.style):
  - `site-mirror/open-props.style/index.html`
  - `site-mirror/open-props.style/assets/`
- npm package metadata snapshot:
  - `npm/registry-open-props.json`
  - `npm/tarballs/open-props-1.7.23.tgz` (fallback archive built from `upstream/open-props-main` due intermittent npm DNS failures during snapshot)
- UNPKG listing snapshot:
  - `unpkg/index.html`
- jsDelivr mirror attempt notes:
  - `jsdelivr/NOTES.md`

## Known Gaps

- `open-props.style` does not expose a sitemap at `/sitemap.xml` (captured in `open-props-style/sitemap.xml` note).
- Some CDN metadata endpoints had intermittent DNS failures during snapshot:
  - `jsdelivr/versions/*/meta.json`
  - `unpkg/versions/1.7.23/meta.json`

## Version Signals (from captured files)

- Repo package version: `upstream/open-props-main/package.json` -> `1.7.23`
- npm dist-tags: `npm/registry-open-props.json` ->
  - `latest`: `1.7.23`
  - `beta`: `2.0.0-beta.4`
  - `beta.5`: `2.0.0-beta.5`
- Published site badge in mirrored HTML currently shows: `v1.7.20`

## Task To File Routing

- Installation/import/path questions:
  - `upstream/open-props-main/readme.md`
  - `upstream/open-props-main/package.json` (`exports` map)
- "What token/prop exists?" lookups:
  - `upstream/open-props-main/src/props.*.css`
  - `upstream/open-props-main/src/extra/*.css`
- Full docs/content explanations from official site:
  - `upstream/open-props-main/docsite/index.html`
  - `site-mirror/open-props.style/index.html`
- Release history / changes:
  - `upstream/open-props-main/CHANGELOG.md`
- Build pipeline / generated artifacts details:
  - `upstream/open-props-main/build/*.js`

## Fast Search Patterns

- Find a token definition:
  - `rg --line-number -- '--size-fluid-3|--shadow-2|--animation-' upstream/open-props-main/src`
- Find exports/import subpaths:
  - `rg --line-number '"\./(postcss|normalize|buttons|shadow|colors|sizes)' upstream/open-props-main/package.json`
- Find docs section anchors/topics:
  - `rg --line-number 'id="(getting-started|colors|sizes|typography|animations|media-queries)"' upstream/open-props-main/docsite/index.html`

## Refresh Commands

- Refresh upstream repo snapshot:
  - `wget -O upstream/open-props-main.tar.gz https://codeload.github.com/argyleink/open-props/tar.gz/refs/heads/main`
  - `tar -xzf upstream/open-props-main.tar.gz -C upstream`
- Refresh site mirror:
  - `wget --recursive --level=2 --page-requisites --adjust-extension --convert-links --no-parent --directory-prefix=site-mirror https://open-props.style/`
- Refresh npm metadata:
  - `wget -O npm/registry-open-props.json https://registry.npmjs.org/open-props`
- Refresh npm tarball (preferred, if DNS is healthy):
  - `wget -O npm/tarballs/open-props-1.7.23.tgz https://registry.npmjs.org/open-props/-/open-props-1.7.23.tgz`
