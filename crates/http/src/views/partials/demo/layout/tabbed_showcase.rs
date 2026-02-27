use bon::Builder;
use maud::{PreEscaped, Render};
use maud_extensions::css;
use palette::{FromColor, Hsl, RgbHue, Srgb};

use crate::types::Text;
use crate::views::partials::components::{CodeBlock, CodeLanguage};

#[derive(Clone, Debug, Builder)]
pub(crate) struct TabbedShowcaseRow {
    pub label: Text,
    pub value: Text,
}

#[derive(Clone, Debug, Builder)]
pub(crate) struct TabbedShowcaseAction {
    pub label: Text,
    pub href: Text,
}

#[derive(Clone, Debug, Builder)]
pub(crate) struct TabbedShowcaseMockPanel {
    pub title: Text,
    pub subtitle: Text,
    pub rows: Vec<TabbedShowcaseRow>,
}

#[derive(Clone, Copy, Debug)]
enum ShowcaseColor {
    Indigo,
    Blue,
    Teal,
    Emerald,
    Slate,
    Amber,
}

impl ShowcaseColor {
    fn cycle(index: usize) -> Self {
        match index % 6 {
            0 => Self::Indigo,
            1 => Self::Blue,
            2 => Self::Teal,
            3 => Self::Emerald,
            4 => Self::Slate,
            _ => Self::Amber,
        }
    }

    fn base_hsl(self) -> Hsl {
        match self {
            Self::Indigo => Hsl::new(RgbHue::from_degrees(228.0), 0.62, 0.56),
            Self::Blue => Hsl::new(RgbHue::from_degrees(205.0), 0.60, 0.50),
            Self::Teal => Hsl::new(RgbHue::from_degrees(184.0), 0.50, 0.44),
            Self::Emerald => Hsl::new(RgbHue::from_degrees(158.0), 0.46, 0.40),
            Self::Slate => Hsl::new(RgbHue::from_degrees(214.0), 0.28, 0.56),
            Self::Amber => Hsl::new(RgbHue::from_degrees(32.0), 0.56, 0.52),
        }
    }

    fn palette(self) -> TabPalette {
        let base = self.base_hsl();
        let tab_soft_hsl =
            Hsl::new(base.hue, (base.saturation * 0.42).clamp(0.0, 1.0), 0.2);
        let grad_start_hsl = Hsl::new(
            base.hue,
            (base.saturation * 0.58).clamp(0.0, 1.0),
            (base.lightness + 0.06).clamp(0.0, 1.0),
        );
        let grad_end_hsl = Hsl::new(
            base.hue + RgbHue::from_degrees(10.0),
            (base.saturation * 0.44).clamp(0.0, 1.0),
            (base.lightness - 0.05).clamp(0.0, 1.0),
        );
        let avg_bg_hsl = Hsl::new(
            base.hue + RgbHue::from_degrees(9.0),
            ((grad_start_hsl.saturation + grad_end_hsl.saturation) / 2.0).clamp(0.0, 1.0),
            ((grad_start_hsl.lightness + grad_end_hsl.lightness) / 2.0).clamp(0.0, 1.0),
        );
        let avg_bg = Srgb::from_color(avg_bg_hsl);
        let black = Srgb::new(0.043, 0.071, 0.125);
        let white = Srgb::new(0.973, 0.98, 0.988);
        let use_dark_text = contrast_ratio(avg_bg, black) >= contrast_ratio(avg_bg, white);

        let copy_text = if use_dark_text {
            CssColor::hsl(Hsl::new(RgbHue::from_degrees(222.0), 0.47, 0.11))
        } else {
            CssColor::hsl(Hsl::new(RgbHue::from_degrees(210.0), 0.40, 0.98))
        };
        let copy_muted = if use_dark_text {
            CssColor::hsla(Hsl::new(RgbHue::from_degrees(222.0), 0.47, 0.11), 0.82)
        } else {
            CssColor::hsla(Hsl::new(RgbHue::from_degrees(210.0), 0.40, 0.98), 0.86)
        };
        let chip_bg = if use_dark_text {
            CssColor::hsla(Hsl::new(RgbHue::from_degrees(0.0), 0.0, 1.0), 0.68)
        } else {
            CssColor::hsla(Hsl::new(RgbHue::from_degrees(223.0), 0.47, 0.11), 0.28)
        };
        let chip_border = if use_dark_text {
            CssColor::hsla(Hsl::new(RgbHue::from_degrees(223.0), 0.47, 0.11), 0.24)
        } else {
            CssColor::hsla(Hsl::new(RgbHue::from_degrees(210.0), 0.40, 0.98), 0.34)
        };

        TabPalette {
            accent: CssColor::hsl(base),
            tab_soft: CssColor::hsl(tab_soft_hsl),
            grad_start: CssColor::hsl(grad_start_hsl),
            grad_end: CssColor::hsl(grad_end_hsl),
            copy_text,
            copy_muted,
            chip_bg,
            chip_border,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct CssColor {
    hsl: Hsl,
    alpha: Option<f32>,
}

impl CssColor {
    fn hsl(hsl: Hsl) -> Self {
        Self { hsl, alpha: None }
    }

    fn hsla(hsl: Hsl, alpha: f32) -> Self {
        Self {
            hsl,
            alpha: Some(alpha.clamp(0.0, 1.0)),
        }
    }
}

#[derive(Clone, Debug)]
struct TabPalette {
    accent: CssColor,
    tab_soft: CssColor,
    grad_start: CssColor,
    grad_end: CssColor,
    copy_text: CssColor,
    copy_muted: CssColor,
    chip_bg: CssColor,
    chip_border: CssColor,
}

#[derive(Clone, Debug, Builder)]
pub(crate) struct TabbedShowcaseTab {
    pub tab_icon: Option<Text>,
    pub tab_label: Text,
    pub title: Text,
    pub subtitle: Text,
    pub bullets: Vec<Text>,
    pub mock_panel: Option<TabbedShowcaseMockPanel>,
    pub chips_label: Text,
    pub chips: Vec<Text>,
    pub action: Option<TabbedShowcaseAction>,
    pub code_path: Option<Text>,
    pub code: Option<Text>,
}

#[derive(Clone, Debug, Builder)]
pub struct TabbedShowcase {
    pub id: Text,
    pub title: Text,
    pub subtitle: Text,
    pub tabs: Vec<TabbedShowcaseTab>,
}

impl Render for TabbedShowcase {
    fn render(&self) -> maud::Markup {
        if self.tabs.is_empty() {
            return maud::html! {};
        }

        let tabs_script = tabs_script(&self.id);

        maud::html! {
            section id=(&self.id) {
                header data-section-header {
                    div {
                        h2 { (&self.title) }
                        p data-muted { (&self.subtitle) }
                    }
                }
                div data-showcase-root {
                    div data-showcase-shell {
                        nav data-showcase-tabs aria-label="Showcase tabs" role="tablist" {
                            @for (index, tab) in self.tabs.iter().enumerate() {
                                @let tab_id = format!("{}-tab-{}", self.id, index);
                                @let panel_id = format!("{}-panel-{}", self.id, index);
                                @let palette = ShowcaseColor::cycle(index).palette();
                                button
                                    type="button"
                                    data-tab-index=(index)
                                    role="tab"
                                    id=(tab_id)
                                    aria-controls=(panel_id)
                                    aria-selected=(if index == 0 { "true" } else { "false" })
                                    tabindex=(if index == 0 { "0" } else { "-1" })
                                    style={
                                        " --tab-accent: " (css_color(palette.accent)) ";"
                                        " --tab-accent-soft: " (css_color(palette.tab_soft)) ";"
                                    }
                                {
                                    @if let Some(icon) = &tab.tab_icon {
                                        span aria-hidden="true" { (icon) }
                                    }
                                    span { (&tab.tab_label) }
                                }
                            }
                        }
                        div data-showcase-panels {
                            @for (index, tab) in self.tabs.iter().enumerate() {
                                @let tab_id = format!("{}-tab-{}", self.id, index);
                                @let panel_id = format!("{}-panel-{}", self.id, index);
                                @let palette = ShowcaseColor::cycle(index).palette();
                                article
                                    data-showcase-panel
                                    data-panel-full[tab.mock_panel.is_none()]
                                    data-tab-index=(index)
                                    id=(panel_id)
                                    role="tabpanel"
                                    aria-labelledby=(tab_id)
                                    tabindex="0"
                                    hidden[index != 0]
                                {
                                    @if let Some(mock_panel) = &tab.mock_panel {
                                        div data-showcase-mockup {
                                            header {
                                                h3 { (&mock_panel.title) }
                                                p data-muted { (&mock_panel.subtitle) }
                                            }
                                            ul data-showcase-rows {
                                                @for row in &mock_panel.rows {
                                                    li {
                                                        span data-showcase-row-label { (&row.label) }
                                                        span data-showcase-row-value { (&row.value) }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div
                                        data-showcase-copy
                                        style={
                                            " --tab-accent: " (css_color(palette.accent)) ";"
                                            " --tab-grad-start: " (css_color(palette.grad_start)) ";"
                                            " --tab-grad-end: " (css_color(palette.grad_end)) ";"
                                            " --tab-copy-text: " (css_color(palette.copy_text)) ";"
                                            " --tab-copy-muted: " (css_color(palette.copy_muted)) ";"
                                            " --tab-chip-bg: " (css_color(palette.chip_bg)) ";"
                                            " --tab-chip-border: " (css_color(palette.chip_border)) ";"
                                        }
                                    {
                                        h3 { (&tab.title) }
                                        p data-muted { (&tab.subtitle) }
                                        ul data-showcase-bullets {
                                            @for bullet in &tab.bullets {
                                                li { (bullet) }
                                            }
                                        }
                                        @if let Some(action) = &tab.action {
                                            a class="button" href=(&action.href) { (&action.label) }
                                        }
                                        p data-showcase-integrations {
                                            span { (&tab.chips_label) }
                                            @for chip in &tab.chips {
                                                span data-showcase-chip { (chip) }
                                            }
                                        }
                                        @if let Some(path) = &tab.code_path {
                                            p data-code-path {
                                                "Example: "
                                                code { (path) }
                                            }
                                        }
                                        @if let Some(code) = &tab.code {
                                            (CodeBlock::builder()
                                                .code(code.clone())
                                                .language(CodeLanguage::Rust)
                                                .build())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                script { (PreEscaped(tabs_script)) }
                ({
                    css! {
                        me {
                          margin-top: 2.8rem;
                          border: 1px solid var(--portfolio-surface-border);
                          border-radius: 18px;
                          padding: 1.35rem 1.35rem 1.45rem;
                          background: var(--portfolio-surface);
                          box-shadow: 0 6px 16px color-mix(in srgb, black 8%, transparent);
                        }
                        me [data-section-header] {
                          display: flex;
                          flex-wrap: wrap;
                          align-items: center;
                          justify-content: space-between;
                          gap: 0.9rem 1.2rem;
                          margin-bottom: 1.1rem;
                        }
                        me [data-section-header] h2 {
                          margin-bottom: 0.28rem;
                          font-size: clamp(1.5rem, 1.2rem + 1.1vw, 2rem);
                          line-height: 1.18;
                        }
                        me [data-section-header] [data-muted] {
                          margin-bottom: 0;
                          max-width: 70ch;
                          color: color-mix(in srgb, var(--pico-muted-color) 94%, var(--pico-color) 6%);
                        }
                        me [data-showcase-root] {
                          position: relative;
                          margin-top: 1.15rem;
                        }
                        me [data-showcase-shell] {
                          border: 1px solid color-mix(in srgb, var(--tab-shell-border) 82%, transparent);
                          border-radius: 16px;
                          padding: 0.95rem;
                          background:
                            radial-gradient(circle at 0% 0%, var(--tab-shell-glow-a), transparent 55%),
                            linear-gradient(
                              180deg,
                              color-mix(in srgb, var(--tab-shell-bg) 96%, transparent),
                              color-mix(in srgb, var(--tab-shell-bg) 90%, transparent)
                            );
                        }
                        me [data-showcase-tabs] {
                          display: flex;
                          align-items: stretch;
                          gap: 0.45rem;
                          overflow-x: auto;
                          padding-bottom: 0.72rem;
                          border-bottom: 1px solid color-mix(in srgb, var(--pico-muted-color) 30%, transparent);
                        }
                        me [data-showcase-tabs] > button[role="tab"] {
                          cursor: pointer;
                          border-radius: 10px;
                          border: 1px solid transparent;
                          background: var(--tab-tab-bg);
                          color: var(--tab-tab-text);
                          padding: 0.56rem 0.76rem;
                          min-width: 170px;
                          font-size: 0.81rem;
                          font-weight: 600;
                          display: inline-flex;
                          align-items: center;
                          gap: 0.45rem;
                          transition: border-color 0.2s ease, background 0.2s ease, color 0.2s ease;
                          opacity: 1;
                        }
                        me [data-showcase-tabs] > button[role="tab"][aria-selected="true"] {
                          border-color: color-mix(in srgb, var(--tab-accent) 52%, transparent);
                          background: color-mix(in srgb, var(--tab-accent-soft) 44%, var(--tab-tab-bg) 56%);
                          color: color-mix(in srgb, var(--tab-accent) 44%, var(--pico-color) 56%);
                        }
                        me [data-showcase-tabs] > button[role="tab"]:focus-visible {
                          outline: 2px solid color-mix(in srgb, var(--tab-accent) 65%, white);
                          outline-offset: 2px;
                        }
                        me [data-showcase-tabs] > button[role="tab"] > span[aria-hidden="true"] {
                          display: inline-flex;
                          align-items: center;
                          justify-content: center;
                          min-width: 1.2rem;
                          font-size: 0.9rem;
                          line-height: 1;
                          color: inherit;
                        }
                        me [data-showcase-panels] {
                          margin-top: 1rem;
                        }
                        me [data-showcase-panel] {
                          display: grid;
                          gap: 1.2rem;
                          min-width: 0;
                        }
                        me [data-showcase-panel][hidden] {
                          display: none;
                        }
                        @media (min-width: 980px) {
                          me [data-showcase-panel] {
                            grid-template-columns: 1.1fr 0.9fr;
                            align-items: stretch;
                          }
                          me [data-showcase-panel][data-panel-full] {
                            grid-template-columns: 1fr;
                          }
                        }
                        me [data-showcase-panel] > * {
                          min-width: 0;
                        }
                        me [data-showcase-mockup] {
                          border: 1px solid var(--tab-surface-border);
                          border-radius: var(--ui-radius-md);
                          padding: 1rem;
                          background: var(--tab-surface-bg);
                        }
                        me [data-showcase-mockup] h3 {
                          margin-bottom: 0.3rem;
                        }
                        me [data-showcase-rows] {
                          list-style: none;
                          margin: 1rem 0 0;
                          padding: 0;
                          display: grid;
                          gap: 0.55rem;
                        }
                        me [data-showcase-rows] > li {
                          border: 1px solid var(--tab-row-border);
                          border-radius: 9px;
                          background: var(--tab-row-bg);
                          padding: 0.5rem 0.65rem;
                          display: flex;
                          justify-content: space-between;
                          gap: 0.6rem;
                        }
                        me [data-showcase-row-label] {
                          color: var(--pico-muted-color);
                          font-size: 0.78rem;
                        }
                        me [data-showcase-row-value] {
                          font-size: 0.8rem;
                          font-weight: 600;
                          text-align: right;
                        }
                        me [data-showcase-copy] {
                          border: 1px solid color-mix(in srgb, var(--tab-accent) 30%, transparent);
                          border-radius: var(--ui-radius-md);
                          padding: 1.05rem;
                          background:
                            linear-gradient(
                              180deg,
                              color-mix(in srgb, var(--tab-accent-soft) 36%, var(--pico-card-background-color) 64%),
                              color-mix(in srgb, var(--pico-card-background-color) 96%, black 4%)
                            );
                          color: var(--pico-color);
                          overflow: hidden;
                        }
                        me [data-showcase-copy] h3 {
                          color: inherit;
                          margin-bottom: 0.35rem;
                        }
                        me [data-showcase-copy] [data-muted] {
                          color: color-mix(in srgb, var(--pico-muted-color) 92%, var(--pico-color) 8%);
                        }
                        me [data-showcase-bullets] {
                          margin: 1rem 0 1.1rem;
                          padding-left: 1.1rem;
                          display: grid;
                          gap: 0.35rem;
                          color: var(--pico-color);
                        }
                        me [data-showcase-bullets] li {
                          color: inherit;
                        }
                        me [data-showcase-bullets] li::marker {
                          color: color-mix(in srgb, var(--tab-accent) 58%, transparent);
                        }
                        me [data-showcase-copy] .button {
                          background: color-mix(in srgb, var(--pico-primary) 86%, black 14%);
                          color: var(--pico-primary-inverse);
                        }
                        me [data-showcase-integrations] {
                          margin: 0.95rem 0 0;
                          display: flex;
                          flex-wrap: wrap;
                          align-items: center;
                          gap: 0.4rem;
                          font-size: 0.76rem;
                          color: var(--pico-muted-color);
                        }
                        me [data-showcase-chip] {
                          border: 1px solid color-mix(in srgb, var(--tab-accent) 26%, var(--pico-muted-color) 74%);
                          border-radius: 999px;
                          padding: 0.2rem 0.5rem;
                          font-weight: 600;
                          background: color-mix(in srgb, var(--pico-card-background-color) 90%, var(--tab-accent-soft) 10%);
                        }
                        me [data-code-path] {
                          margin: 0.45rem 0 0.65rem;
                          color: var(--pico-muted-color);
                          font-size: 0.85rem;
                        }
                        me [data-code-path] code {
                          word-break: break-all;
                        }
                        me [data-showcase-copy] ::selection {
                          background: hsl(223 47% 11% / 0.72);
                          color: hsl(210 40% 98%);
                        }
                        me [data-showcase-copy] ::-moz-selection {
                          background: hsl(223 47% 11% / 0.72);
                          color: hsl(210 40% 98%);
                        }
                        @media (max-width: 768px) {
                          me {
                            margin-top: 1.8rem;
                            padding: 1rem 0.95rem 1.1rem;
                            border-radius: 16px;
                          }
                        }
                    }
                })
            }
        }
    }
}

fn tabs_script(showcase_id: &Text) -> String {
    let showcase_id_json = serde_json::to_string(&showcase_id.to_string())
        .unwrap_or_else(|_| "\"\"".to_owned());
    format!(
        r#"
(() => {{
  const root = document.getElementById({showcase_id_json});
  if (!root) return;

  const tabs = Array.from(root.querySelectorAll('[role="tab"]'));
  const panels = Array.from(root.querySelectorAll('[data-showcase-panel][role="tabpanel"]'));
  if (!tabs.length || !panels.length) return;

  const lastIndex = tabs.length - 1;

  const activate = (nextIndex, focusTab) => {{
    tabs.forEach((tab, index) => {{
      const isActive = index === nextIndex;
      tab.setAttribute('aria-selected', isActive ? 'true' : 'false');
      tab.tabIndex = isActive ? 0 : -1;
      if (focusTab && isActive) tab.focus();
    }});

    panels.forEach((panel, index) => {{
      panel.hidden = index !== nextIndex;
    }});
  }};

  tabs.forEach((tab, index) => {{
    tab.addEventListener('click', () => activate(index, false));
    tab.addEventListener('keydown', (event) => {{
      let next = null;
      if (event.key === 'ArrowRight' || event.key === 'ArrowDown') {{
        next = index === lastIndex ? 0 : index + 1;
      }} else if (event.key === 'ArrowLeft' || event.key === 'ArrowUp') {{
        next = index === 0 ? lastIndex : index - 1;
      }} else if (event.key === 'Home') {{
        next = 0;
      }} else if (event.key === 'End') {{
        next = lastIndex;
      }}

      if (next !== null) {{
        event.preventDefault();
        activate(next, true);
      }}
    }});
  }});

  const selectedIndex = tabs.findIndex((tab) => tab.getAttribute('aria-selected') === 'true');
  activate(selectedIndex >= 0 ? selectedIndex : 0, false);
}})();
    "#
    )
}

fn hsl_css(color: Hsl) -> String {
    format!(
        "hsl({:.1} {:.1}% {:.1}%)",
        color.hue.into_degrees(),
        color.saturation * 100.0,
        color.lightness * 100.0
    )
}

fn css_color(color: CssColor) -> String {
    match color.alpha {
        Some(alpha) => hsla_css(color.hsl, alpha),
        None => hsl_css(color.hsl),
    }
}

fn hsla_css(color: Hsl, alpha: f32) -> String {
    format!(
        "hsl({:.1} {:.1}% {:.1}% / {:.2})",
        color.hue.into_degrees(),
        color.saturation * 100.0,
        color.lightness * 100.0,
        alpha
    )
}

fn relative_luminance(color: Srgb<f32>) -> f32 {
    let channel = |value: f32| {
        if value <= 0.03928 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    };

    let r = channel(color.red);
    let g = channel(color.green);
    let b = channel(color.blue);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn contrast_ratio(a: Srgb<f32>, b: Srgb<f32>) -> f32 {
    let a_lum = relative_luminance(a);
    let b_lum = relative_luminance(b);
    let (lighter, darker) = if a_lum >= b_lum {
        (a_lum, b_lum)
    } else {
        (b_lum, a_lum)
    };
    (lighter + 0.05) / (darker + 0.05)
}
