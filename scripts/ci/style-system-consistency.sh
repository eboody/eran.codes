#!/usr/bin/env bash
set -euo pipefail

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required for style-system checks."
  exit 1
fi

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for style-system checks."
  exit 1
fi

status=0
RAW_OPEN_PROPS_PATTERN='--(size|gray|stone|sand|red|pink|purple|indigo|blue|cyan|teal|green|lime|yellow|orange|choco)-|--shadow-[0-9]+|--radius-[0-9]+'

DOCS_DIR="docs/style-system"
DOCS_INDEX="$DOCS_DIR/index.md"
DOCS_CATALOG="$DOCS_DIR/package-catalog.md"

for required_doc in "$DOCS_INDEX" "$DOCS_CATALOG"; do
  if [[ ! -f "$required_doc" ]]; then
    echo "error: expected style-system doctrine file at ${required_doc}."
    exit 1
  fi
done

APP_CSS="crates/http/static/app.css"
if [[ ! -f "$APP_CSS" ]]; then
  echo "error: expected global stylesheet at ${APP_CSS}."
  exit 1
fi

for selector in ".u-container" ".u-muted" ".u-surface-card"; do
  if ! rg -q "^\s*\\${selector}\b|^\s*${selector}\b" "$APP_CSS"; then
    echo "${APP_CSS}: missing global utility ${selector}."
    status=1
  fi

  if ! rg -Fq "${selector}" "$DOCS_CATALOG"; then
    echo "${DOCS_CATALOG}: missing package catalog entry for ${selector}."
    status=1
  fi
done

for removed_selector in \
  ".button" \
  ".container" \
  ".muted" \
  ".ui-button" \
  ".ui-nav-shell" \
  ".ui-nav" \
  ".ui-nav-list" \
  ".ui-nav-link" \
  ".ui-nav-brand" \
  ".ui-nav-brand-link" \
  ".ui-nav-brand-picture" \
  ".ui-nav-brand-mark-wrap" \
  ".ui-nav-brand-mark" \
  ".ui-nav-brand-text" \
  ".ui-nav-links" \
  ".ui-nav-auth" \
  ".ui-nav-auth-text" \
  ".ui-nav-auth-action" \
  ".ui-auth-main" \
  ".ui-auth-card" \
  ".ui-auth-header" \
  ".ui-auth-summary" \
  ".ui-auth-form" \
  ".ui-auth-field" \
  ".ui-auth-submit" \
  ".ui-auth-note" \
  ".ui-account-card" \
  ".ui-account-actions" \
  ".ui-section-header" \
  ".ui-section-meta" \
  ".ui-demo-result" \
  ".ui-error-alert" \
  ".ui-surface-card" \
  ".ui-pill" \
  ".ui-pill--method" \
  ".ui-pill--path" \
  ".ui-pill--method-get" \
  ".ui-pill--method-post" \
  ".ui-pill--method-put" \
  ".ui-pill--method-patch" \
  ".ui-pill--method-delete" \
  ".ui-pill--method-other" \
  ".ui-pill--status" \
  ".ui-pill--status-2xx" \
  ".ui-pill--status-3xx" \
  ".ui-pill--status-4xx" \
  ".ui-pill--status-5xx" \
  ".ui-pill--status-unknown" \
  ".ui-pill--log-level-info" \
  ".ui-pill--log-level-warn" \
  ".ui-pill--log-level-error" \
  ".ui-pill--log-level-debug" \
  ".ui-pill--log-level-trace" \
  ".ui-pill--log-target" \
  ".ui-pill--log-fields" \
  ".ui-pill--badge-secondary" \
  ".ui-pill--badge-you" \
  ".ui-pill--badge-demo" \
  ".ui-icon" \
  ".ui-key-value-list" \
  ".ui-status-card" \
  ".ui-op-filter" \
  ".ui-op-filter-label" \
  ".ui-op-filter-row" \
  ".ui-op-filter-meta" \
  ".ui-burst-controls" \
  ".ui-burst-slider" \
  ".ui-burst-selected" \
  ".ui-burst-actions" \
  ".ui-burst-result" \
  ".ui-home-hero" \
  ".ui-home-hero-copy" \
  ".ui-home-hero-kicker" \
  ".ui-home-hero-tags" \
  ".ui-home-hero-card" \
  ".ui-chat-connection-row" \
  ".ui-info-grid" \
  ".ui-info-card" \
  ".ui-panel" \
  ".ui-preview-frame" \
  ".ui-feature-list" \
  ".ui-tabs" \
  ".ui-tab" \
  ".tab-set__tab-icon" \
  ".ui-chat-page" \
  ".ui-chat-page-surface" \
  ".ui-chat-moderation-hero" \
  ".ui-chat-moderation-flow" \
  ".ui-chat-moderation-stack" \
  ".ui-chat-moderation-card" \
  ".ui-lab-main" \
  ".ui-lab-chat-surface" \
  ".ui-lab-tab-set" \
  ".ui-portfolio-main" \
  ".ui-portfolio-hero" \
  ".ui-portfolio-case-hero" \
  ".ui-portfolio-eyebrow" \
  ".ui-portfolio-summary" \
  ".ui-portfolio-section-copy" \
  ".ui-portfolio-badges" \
  ".ui-portfolio-proof-strip" \
  ".ui-portfolio-proof-item" \
  ".ui-portfolio-card-grid" \
  ".ui-portfolio-card" \
  ".ui-portfolio-card-kicker" \
  ".ui-portfolio-card-summary" \
  ".ui-portfolio-card-outcome" \
  ".ui-portfolio-card-preview" \
  ".ui-portfolio-preview-key" \
  ".ui-portfolio-preview-alt" \
  ".ui-portfolio-list" \
  ".ui-portfolio-card-links" \
  ".ui-portfolio-section-actions" \
  ".ui-portfolio-closing" \
  ".ui-portfolio-case-grid" \
  ".ui-portfolio-case-section" \
  ".ui-log-surface" \
  ".ui-log-panels" \
  ".ui-log-panel" \
  ".ui-log-scroll" \
  ".ui-log-empty" \
  ".ui-log-entries" \
  ".ui-log-entry" \
  ".ui-pill-cluster" \
  ".ui-log-table" \
  ".ui-log-groups" \
  ".ui-log-group" \
  ".ui-log-group-header" \
  ".ui-log-flow-shell" \
  ".ui-log-flow-list" \
  ".ui-log-flow-item" \
  ".ui-log-flow-item-id" \
  ".ui-log-flow-item-title" \
  ".ui-log-flow-item-meta" \
  ".ui-log-flow-item-time" \
  ".ui-log-flow-details" \
  ".ui-log-flow-detail" \
  ".ui-log-flow-detail-header" \
  ".ui-log-flow-detail-title" \
  ".ui-log-flow-event" \
  ".ui-log-flow-event-head" \
  ".ui-log-flow-event-summary" \
  ".ui-log-flow-event-summary-inline" \
  ".ui-ping-target"; do
  if rg -q "^\s*\\${removed_selector}\b|^\s*${removed_selector}\b" "$APP_CSS"; then
    echo "${APP_CSS}: ${removed_selector} should be component-scoped, not defined globally."
    status=1
  fi
done

if rg -q '^\s*\[data-muted\]' "$APP_CSS"; then
  echo "${APP_CSS}: [data-muted] should not be defined globally."
  status=1
fi

if rg -q '^\s*button\b|^\s*a\.button\b' "$APP_CSS"; then
  echo "${APP_CSS}: shared button selectors should live with the button component, not in app.css."
  status=1
fi

if rg --no-heading --line-number 'class="([^"]* )?container( [^"]*)?"' crates/http/src/views >/dev/null; then
  echo "crates/http/src/views: use u-container instead of the legacy container alias."
  status=1
fi

if rg --no-heading --line-number 'class="([^"]* )?muted( [^"]*)?"' crates/http/src/views >/dev/null; then
  echo "crates/http/src/views: use u-muted instead of the legacy muted alias."
  status=1
fi

if rg --no-heading --line-number '\bdata-muted\b' crates/http/src/views >/dev/null; then
  echo "crates/http/src/views: use u-muted instead of data-muted."
  status=1
fi

if ! python3 - <<'PY'
from pathlib import Path
import re
import sys

allowed_prefixes = (
    "var(",
    "inherit",
    "normal",
    "unset",
    "initial",
    "revert",
)
pattern = re.compile(r"^\s*(font-size|line-height|letter-spacing):\s*([^;]+);")
matches = []

for path in sorted(Path("crates/http/src/views").rglob("*.rs")):
    for line_no, line in enumerate(path.read_text().splitlines(), start=1):
        match = pattern.search(line)
        if not match:
            continue
        value = match.group(2).strip()
        if value.startswith(allowed_prefixes):
            continue
        matches.append(f"{path}:{line_no}: {match.group(1)}: {value}")

if matches:
    print(
        "crates/http/src/views: typography declarations must resolve through semantic or local token aliases, not raw literals."
    )
    for item in matches:
        print(item)
    sys.exit(1)
PY
then
  status=1
fi

if ! python3 - <<'PY'
from pathlib import Path
import re
import sys

target_props = {
    "gap",
    "row-gap",
    "column-gap",
    "margin",
    "margin-top",
    "margin-bottom",
    "margin-block",
    "margin-inline",
    "padding",
    "padding-top",
    "padding-bottom",
    "padding-left",
    "padding-right",
    "padding-block",
    "padding-inline",
    "padding-inline-end",
    "outline-offset",
    "top",
    "bottom",
    "left",
    "right",
}
allowed_prefixes = ("var(", "calc(", "clamp(", "min(", "max(")
pattern = re.compile(r"^\s*([a-zA-Z-]+)\s*:\s*([^;]+);")
length_pattern = re.compile(r"(?<![\w-])(\d*\.\d+|\d+)(rem|px|em)\b")
matches = []

for path in sorted(Path("crates/http/src/views").rglob("*.rs")):
    for line_no, line in enumerate(path.read_text().splitlines(), start=1):
        stripped = line.strip()
        match = pattern.match(stripped)
        if not match:
            continue
        prop, value = match.groups()
        if prop.startswith("--") or prop not in target_props:
            continue
        if value.strip().startswith(allowed_prefixes):
            continue
        if not length_pattern.search(value):
            continue
        matches.append(f"{path}:{line_no}: {prop}: {value.strip()}")

if matches:
    print(
        "crates/http/src/views: rhythm and spacing literals should live in shared or local tokens, not direct declarations."
    )
    for item in matches:
        print(item)
    sys.exit(1)
PY
then
  status=1
fi

if ! python3 - <<'PY'
from pathlib import Path
import re
import sys

definition_pattern = re.compile(
    r"^\s*(--_[A-Za-z0-9-]+)\s*:\s*(var\(--[A-Za-z0-9-]+\))\s*;\s*$"
)
all_lines = {}

for path in sorted(Path("crates/http/src/views").rglob("*.rs")):
    all_lines[path] = path.read_text().splitlines()

repo_occurrences = {}
for lines in all_lines.values():
    for line in lines:
        for match in re.finditer(r"--_[A-Za-z0-9-]+", line):
            name = match.group(0)
            repo_occurrences[name] = repo_occurrences.get(name, 0) + 1

matches = []
for path, lines in all_lines.items():
    for line_no, line in enumerate(lines, start=1):
        match = definition_pattern.match(line.strip())
        if not match:
            continue
        name, value = match.groups()
        file_occurrences = sum(1 for candidate in lines if name in candidate)
        file_definitions = sum(
            1 for candidate in lines if re.search(rf"^\s*{re.escape(name)}\s*:", candidate.strip())
        )
        if file_occurrences != 2:
            continue
        if file_definitions != 1:
            continue
        if repo_occurrences.get(name, 0) != file_occurrences:
            continue
        matches.append(f"{path}:{line_no}: {name}: {value}")

if matches:
    print(
        "crates/http/src/views: remove single-use pass-through local aliases that only rename one shared token."
    )
    for item in matches:
        print(item)
    sys.exit(1)
PY
then
  status=1
fi

while IFS= read -r spec; do
  if ! jq -e '((.meta.target // []) | index("datastar")) != null' "$spec" >/dev/null 2>&1; then
    continue
  fi

  if ! jq -e '.styling.mode == "hybrid"' "$spec" >/dev/null; then
    echo "${spec}: styling.mode must be \"hybrid\"."
    status=1
  fi

  if ! jq -e '(.styling.global_packages // [] | length) > 0' "$spec" >/dev/null; then
    echo "${spec}: styling.global_packages must include reusable classes."
    status=1
  fi

  if ! jq -e '(.styling.tokens_used // [] | length) > 0' "$spec" >/dev/null; then
    echo "${spec}: styling.tokens_used must declare consumed tokens."
    status=1
  fi

  if ! jq -e 'all((.styling.global_packages // [])[]; startswith("ui-") or startswith("u-"))' "$spec" >/dev/null; then
    echo "${spec}: styling.global_packages should use documented shared utility/package names."
    status=1
  fi

  if ! jq -e '
    all(
      (.styling.tokens_used // [])[];
      test("^--(size|gray|stone|sand|red|pink|purple|indigo|blue|cyan|teal|green|lime|yellow|orange|choco)-|^--shadow-[0-9]+|^--radius-[0-9]+")
      | not
    )
  ' "$spec" >/dev/null; then
    echo "${spec}: styling.tokens_used should list semantic workspace aliases, not raw Open Props primitives."
    status=1
  fi

  if ! jq -e 'all((.styling.scoped_exceptions // [])[]; (type == "string") and (length > 0))' "$spec" >/dev/null; then
    echo "${spec}: styling.scoped_exceptions entries must be non-empty strings when present."
    status=1
  fi

  if ! jq -e '(.pipeline.required_agents // []) | index("mds-styling-system") != null' "$spec" >/dev/null; then
    echo "${spec}: pipeline.required_agents must include mds-styling-system."
    status=1
  fi

  if ! jq -e '
    (.pipeline.execution_order // []) as $order
    | ($order | index("mds-codegen")) as $codegen
    | ($order | index("mds-styling-system")) as $styling
    | ($order | index("mds-verifier")) as $verifier
    | ($codegen != null and $styling != null and $verifier != null and $codegen < $styling and $styling < $verifier)
  ' "$spec" >/dev/null; then
    echo "${spec}: execution_order must place mds-styling-system after mds-codegen and before mds-verifier."
    status=1
  fi
done < <(find tests/fixtures generated -type f \( -name '*.component_spec.json' -o -name 'resolved.component_spec.json' \) 2>/dev/null | sort)

# Files opting into the style-system component policy:
#   // ci: style-system-component
mapfile -t style_components < <(
  rg --no-heading --line-number -g '*.rs' \
    '^\s*//\s*ci:\s*style-system-component\b' \
    crates/http/src/views \
    | cut -d: -f1 \
    | sort -u
)

for file in "${style_components[@]}"; do
  if rg --no-heading --line-number -- "$RAW_OPEN_PROPS_PATTERN" "$file" >/dev/null; then
    echo "${file}: style-system components should consume semantic workspace tokens, not raw Open Props primitives."
    status=1
  fi
done

if rg --no-heading --line-number 'site-nav|site-brand-links|portfolio-nav-links|site-auth-links' \
  crates/http/src/views/page.rs >/dev/null; then
  echo "crates/http/src/views/page.rs: navbar should use reusable ui-nav-* classes, not legacy site-* selectors."
  status=1
fi

exit "$status"
