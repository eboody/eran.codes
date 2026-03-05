#!/usr/bin/env bash
set -euo pipefail

module_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/mod.rs"
showcase_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/showcase.rs"
panels_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/panels.rs"
render_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/render.rs"
styles_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/styles.rs"
styles_dir="crates/http/src/views/partials/demo/layout/tabbed_showcase/styles"
color_component_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/color.rs"
icon_component_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/icon.rs"
tab_component_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/tab.rs"
tab_bar_component_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/tab_bar.rs"
layout_dir="crates/http/src/views/partials/demo/layout"
capability_file="crates/http/src/views/partials/demo/layout/capability_showcase.rs"
professionalism_file="crates/http/src/views/partials/demo/layout/professionalism_in_practice_tabs.rs"
status=0

require_pattern() {
  local pattern="$1"
  local message="$2"
  local file="$3"
  if ! rg -q -- "$pattern" "$file"; then
    echo "tabbed-showcase-conventions: $message"
    status=1
  fi
}

forbid_pattern() {
  local pattern="$1"
  local message="$2"
  local file="$3"
  if rg -q -- "$pattern" "$file"; then
    echo "tabbed-showcase-conventions: $message"
    status=1
  fi
}

forbid_pattern_any() {
  local pattern="$1"
  local message="$2"
  local target="$3"
  if rg -q --glob '*.rs' -- "$pattern" "$target"; then
    echo "tabbed-showcase-conventions: $message"
    status=1
  fi
}

require_pattern '^// ci: descriptive-module-import crate::views::partials::demo::layout::tabbed_showcase$' \
  "expected descriptive-module-import marker for tabbed_showcase" \
  "$module_file"
forbid_pattern '^mod behavior;' "tab behavior must live in tab.rs; behavior module is disallowed" "$module_file"
require_pattern '^mod panels;' "expected panels component module declaration" "$module_file"
require_pattern '^mod render;' "expected render module declaration" "$module_file"
require_pattern '^mod showcase;' "expected showcase root module declaration" "$module_file"
require_pattern '^mod styles;' "expected styles module declaration" "$module_file"
require_pattern '^mod color;' "expected color value module declaration" "$module_file"
require_pattern '^mod icon;' "expected icon component module declaration" "$module_file"
require_pattern '^mod tab;' "expected tab component module declaration" "$module_file"
require_pattern '^mod tab_bar;' "expected tab bar component module declaration" "$module_file"
require_pattern 'pub\(crate\) fn builder\(\) -> ComponentBuilder \{' \
  "expected module-level builder() API" \
  "$module_file"
require_pattern 'showcase::Component::builder\(\)' \
  "module-level builder() must delegate to showcase::Component::builder()" \
  "$module_file"

require_pattern '\bstruct Component\b' "expected internal Component root model" "$showcase_file"
require_pattern '\benum Theme\b' "expected concise Theme model name" "$showcase_file"
forbid_pattern '\bstruct Tab\b' "tab model must live in tab.rs, not showcase.rs" "$showcase_file"
forbid_pattern '\bstruct Action\b' "action model must live in panels.rs, not showcase.rs" "$showcase_file"
forbid_pattern '\bstruct Row\b' "row model must live in panels.rs, not showcase.rs" "$showcase_file"
forbid_pattern '\bstruct MockPanel\b' "mock panel model must live in panels.rs, not showcase.rs" "$showcase_file"
forbid_pattern '\benum Color\b' "color enum is disallowed; use crate-backed Color value object" "$showcase_file"
forbid_pattern '\benum Icon\b' "icon enum is disallowed; use icon svg text" "$showcase_file"
require_pattern '\bpub tabs: Vec<Tab>,' "showcase root should own tabs list" "$showcase_file"
require_pattern '\bpub panels: Vec<Panel>,' "showcase root should own panels list" "$showcase_file"

require_pattern '\bstruct Tab\b' "expected concise Tab model name" "$tab_component_file"
forbid_pattern '\benum Color\b' "tab color enum is disallowed; use crate-backed Color value" "$tab_component_file"
forbid_pattern '\benum Icon\b' "tab icon enum is disallowed; use Text" "$tab_component_file"
require_pattern '\bpub color: Color,' "expected Tab.color field" "$tab_component_file"
forbid_pattern '\bpub color: Text,' "raw Text color field is disallowed" "$tab_component_file"
require_pattern '\bpub text: Text,' "expected Tab.text field" "$tab_component_file"
forbid_pattern '\btab_label\b' "tab label field is disallowed; use text" "$tab_component_file"
forbid_pattern '\bpub tone: Option<' "tone field is disallowed; use color" "$tab_component_file"
forbid_pattern '\boutline_icon\b' "icon helper should live in icon.rs, not tab.rs" "$tab_component_file"

require_pattern '\bstruct Action\b' "expected concise Action model name" "$panels_file"
require_pattern '\bstruct Row\b' "expected concise Row model name" "$panels_file"
require_pattern '\bstruct MockPanel\b' "expected concise MockPanel model name" "$panels_file"
require_pattern '\bstruct Panel\b' "expected concise Panel model name" "$panels_file"

require_pattern '\bpub icon: Option<Icon>,' \
  "expected Tab.icon field (not tab_icon)" \
  "$tab_component_file"
require_pattern 'style=\(self.color.as_style_attr\(\)\)' \
  "tab component should set accent variable from typed color" \
  "$tab_component_file"
forbid_pattern 'data-showcase-tone=' \
  "tab component should not use data-showcase-tone string attributes" \
  "$tab_component_file"
forbid_pattern '\btab_icon\b' "tab_icon naming is disallowed; use icon" "$tab_component_file"
require_pattern '\bstruct Color\b' "expected dedicated color value object" "$color_component_file"
require_pattern 'use syntect::highlighting::Color as SyntectColor;' \
  "color value object must be backed by crate color type" \
  "$color_component_file"
require_pattern 'impl Render for Color' "color value object should implement Render" "$color_component_file"
require_pattern '\bstruct Icon\b' "expected dedicated icon component struct" "$icon_component_file"
require_pattern 'impl Render for Icon' "icon component should implement Render" "$icon_component_file"
require_pattern 'fn outline<' "icon component should expose Icon::outline(...)" "$icon_component_file"

require_pattern 'mod base;' "expected styles base module" "$styles_file"
require_pattern 'mod panels;' "expected styles panels module" "$styles_file"
require_pattern 'mod responsive;' "expected styles responsive module" "$styles_file"
forbid_pattern '^mod tabs;' "tab styles must live in tab.rs; styles::tabs is disallowed" "$styles_file"
require_pattern 'inline_css!' "expected inline_css! in base styles" "$styles_dir/base.rs"
require_pattern 'inline_css!' "expected inline_css! in panel styles" "$styles_dir/panels.rs"
require_pattern 'inline_css!' "expected inline_css! in responsive styles" "$styles_dir/responsive.rs"
require_pattern 'impl Render for Tab' "tab component should implement Render directly on Tab props" "$tab_component_file"
forbid_pattern '\bstruct Component<' "tab component should not introduce a second wrapper struct" "$tab_component_file"
require_pattern 'data-showcase-tab' "tab button should expose a stable data-showcase-tab hook" "$tab_component_file"
forbid_pattern 'data-tab-index=' "tab button should not require a passed index prop" "$tab_component_file"
require_pattern 'inline_css!' "tab component should own button styling via inline_css!" "$tab_component_file"
require_pattern 'inline_js!' "tab component should own button interaction via inline_js!" "$tab_component_file"
require_pattern '\(css\(\)\)' "tab component should render (css())" "$tab_component_file"
require_pattern '\(js\(\)\)' "tab component should render (js())" "$tab_component_file"
forbid_pattern '\bshowcase_id\b' "tab component should not carry showcase_id" "$tab_component_file"
forbid_pattern 'aria-controls=' "tab component should not wire aria-controls to panels" "$tab_component_file"
forbid_pattern 'role=\"tab\"' "tab component should avoid redundant role tab attribute" "$tab_component_file"
require_pattern '\bstruct Component<' "expected dedicated tab bar component struct" "$tab_bar_component_file"
require_pattern 'impl Render for Component' "tab bar component should implement Render" "$tab_bar_component_file"
require_pattern 'inline_css!' "tab bar should own tab-list styling via inline_css!" "$tab_bar_component_file"
require_pattern 'inline_js!' "tab bar should own tab-list behavior via inline_js!" "$tab_bar_component_file"
require_pattern '\(css\(\)\)' "tab bar should render (css()) at the top" "$tab_bar_component_file"
require_pattern '\(js\(\)\)' "tab bar should render (js()) at the bottom" "$tab_bar_component_file"
require_pattern '\(tab\)' "tab bar should compose tabs by rendering Tab directly" "$tab_bar_component_file"
forbid_pattern 'tab::Component \{' "tab bar should not use tab wrapper props structs" "$tab_bar_component_file"
forbid_pattern 'data-tab-index' "tab bar behavior should target data-showcase-tab hooks" "$tab_bar_component_file"
forbid_pattern '\bshowcase_id\b' "tab bar should not carry showcase_id" "$tab_bar_component_file"
require_pattern 'TabBarComponent::builder\(' "render should compose tab bar via builder" "$render_file"
forbid_pattern 'Behavior\.render\(\)' "render must not call Behavior.render(); tab_bar.rs owns tab-list behavior" "$render_file"
forbid_pattern '\bshowcase_id\b' "panels component should not carry showcase_id" "$panels_file"
forbid_pattern 'data-tab-index' "panels should target data-showcase-tab hooks" "$panels_file"
require_pattern 'style=\(color.as_style_attr\(\)\)' \
  "panels should set accent variable from typed color" \
  "$panels_file"
forbid_pattern 'data-showcase-tone=' \
  "panels should not use data-showcase-tone string attributes" \
  "$panels_file"
forbid_pattern 'aria-labelledby=' "panels should not wire aria-labelledby to tabs" "$panels_file"
forbid_pattern 'role=\"tabpanel\"' "panels should avoid redundant role tabpanel attribute" "$panels_file"

forbid_pattern_any '\bTabbedShowcase(Action|MockPanel|Panel|Row|Tab|Theme)\b' \
  "TabbedShowcase* companion names are disallowed; use tabbed_showcase::Action/MockPanel/Panel/Row/Tab/Theme" \
  "$layout_dir"
forbid_pattern_any 'tabbed_showcase::(TabbedShowcase|Showcase|Component)::builder\(' \
  "redundant root type builders are disallowed; use tabbed_showcase::builder()" \
  "$layout_dir"
forbid_pattern_any '\.tab_icon\(' \
  "tabbed showcase icon setter must be .icon(...)" \
  "$layout_dir"
forbid_pattern_any '\.tone\(' \
  "tabbed showcase color setter must be .color(...)" \
  "$layout_dir"
forbid_pattern_any '\.tab_label\(' \
  "tabbed showcase text setter must be .text(...)" \
  "$layout_dir"
forbid_pattern_any 'tabbed_showcase::outline_icon\(' \
  "tabbed showcase icon helper must be Icon::outline(...)" \
  "$layout_dir"
forbid_pattern_any 'tabbed_showcase::Icon::[A-Z]' \
  "tabbed showcase icon enum-style variants are disallowed; use Icon::outline(...)" \
  "$layout_dir"

require_pattern '^use super::tabbed_showcase;' \
  "capability_showcase must import tabbed_showcase namespace" \
  "$capability_file"
require_pattern '^use super::tabbed_showcase;' \
  "professionalism_in_practice_tabs must import tabbed_showcase namespace" \
  "$professionalism_file"
require_pattern 'tabbed_showcase::builder\(' \
  "capability_showcase must use tabbed_showcase::builder()" \
  "$capability_file"
require_pattern 'tabbed_showcase::builder\(' \
  "professionalism_in_practice_tabs must use tabbed_showcase::builder()" \
  "$professionalism_file"
require_pattern '\.panels\(vec!\[' \
  "capability_showcase must provide panels(...)" \
  "$capability_file"
require_pattern '\.panels\(vec!\[' \
  "professionalism_in_practice_tabs must provide panels(...)" \
  "$professionalism_file"
require_pattern '\.color\(tabbed_showcase::Color::' \
  "capability_showcase tabs should use typed tabbed_showcase::Color values" \
  "$capability_file"

forbid_pattern 'use super::\{[^}]*tabbed_showcase::' \
  "do not leaf-import tabbed_showcase types; import namespace only" \
  "$capability_file"
forbid_pattern 'use super::\{[^}]*tabbed_showcase::' \
  "do not leaf-import tabbed_showcase types; import namespace only" \
  "$professionalism_file"

# Keep render module aligned with concise model names.
forbid_pattern '\bTabbedShowcase\b' \
  "render must not reference TabbedShowcase; use Component/Tab/Panel/MockPanel" \
  "$render_file"

exit "$status"
