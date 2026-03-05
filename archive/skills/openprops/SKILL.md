---
name: openprops
description: Open Props documentation and token-reference workflow for answering questions about available CSS custom properties, import/export paths, version differences, and usage guidance. Use when tasks mention Open Props/open-props, token lookup (for example --size-*, --shadow-*, --animation-*), package subpath imports, doc extraction from open-props.style content, or migration/comparison between stable and beta versions.
---

# Openprops

## Quick Start

1. Open `references/reference-map.md`.
2. Route the task to the narrowest source file before loading large files.
3. Use `rg` first, then open only matching lines/sections.
4. For version-sensitive questions, read `references/npm/registry-open-props.json` and `references/upstream/open-props-main/package.json` before answering.

## Workflows

### Token Lookup

1. Search token names in source props files.
2. Confirm the exact token spelling and grouping file.
3. Return token examples plus file location.

Commands:

```bash
rg --line-number -- '--size-fluid-3|--shadow-2|--animation-' references/upstream/open-props-main/src
rg --line-number -- '--[a-z0-9-]+' references/upstream/open-props-main/src/props.*.css
```

### Import Path and Package API Questions

1. Read package `exports` in `references/upstream/open-props-main/package.json`.
2. Map user intent to a concrete subpath (`./normalize`, `./buttons`, `./shadow/*`, `./src/*`, etc).
3. Provide import examples for both CDN and package subpaths when useful.

Commands:

```bash
rg --line-number '"\./' references/upstream/open-props-main/package.json
sed -n '1,260p' references/upstream/open-props-main/readme.md
```

### Docs and Guidance Extraction

1. Prefer authored docs source: `references/upstream/open-props-main/docsite/index.html`.
2. Use mirrored site snapshot `references/site-mirror/open-props.style/index.html` when user asks what is currently published.
3. Search for section IDs first; then read only the surrounding section.

Commands:

```bash
rg --line-number 'id="(getting-started|overview|colors|sizes|typography|animations|media-queries)"' references/upstream/open-props-main/docsite/index.html
rg --line-number 'href="#' references/upstream/open-props-main/docsite/index.html
```

### Version and Migration Checks

1. Compare stable version in `references/upstream/open-props-main/package.json`.
2. Read dist-tags from `references/npm/registry-open-props.json` for latest/beta state.
3. Read `references/upstream/open-props-main/CHANGELOG.md` for behavior changes.
4. Call out when site snapshot and package version differ.

Commands:

```bash
jq '."dist-tags"' references/npm/registry-open-props.json
jq '.version' references/upstream/open-props-main/package.json
rg --line-number '^## ' references/upstream/open-props-main/CHANGELOG.md
```

## Guardrails

- Prefer `references/upstream/open-props-main/src/` for authoritative token definitions.
- Treat `docsite/index.html` as large; never load the whole file unless required.
- If a task asks for the latest state beyond this snapshot, refresh the files listed in `references/reference-map.md` before answering.
- Distinguish stable (`latest`) and beta tags explicitly when recommending package versions.
- Include `sources_used` with exact local file paths consulted.
- When Open Props changes affect visuals in this repo, treat visual baseline handling as mandatory:
  - run `VISUAL_UPDATE_BASELINE=1 scripts/ci/visual-snapshot.sh` for intentional UI changes,
  - run `scripts/ci/visual-snapshot.sh` after,
  - report baseline status in the final response.

## References

- `references/reference-map.md`
- `references/upstream/open-props-main/readme.md`
- `references/upstream/open-props-main/CHANGELOG.md`
- `references/upstream/open-props-main/package.json`
- `references/upstream/open-props-main/docsite/index.html`
- `references/site-mirror/open-props.style/index.html`
- `references/npm/registry-open-props.json`
- `references/unpkg/index.html`
