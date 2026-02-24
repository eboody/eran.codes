use bon::Builder;
use maud::{PreEscaped, Render};
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
    Amber,
    Violet,
    Emerald,
    Cyan,
    Rose,
    Sky,
}

impl ShowcaseColor {
    fn cycle(index: usize) -> Self {
        match index % 6 {
            0 => Self::Amber,
            1 => Self::Violet,
            2 => Self::Emerald,
            3 => Self::Cyan,
            4 => Self::Rose,
            _ => Self::Sky,
        }
    }

    fn base_hsl(self) -> Hsl {
        match self {
            Self::Amber => Hsl::new(RgbHue::from_degrees(43.0), 0.92, 0.51),
            Self::Violet => Hsl::new(RgbHue::from_degrees(262.0), 0.87, 0.66),
            Self::Emerald => Hsl::new(RgbHue::from_degrees(160.0), 0.84, 0.40),
            Self::Cyan => Hsl::new(RgbHue::from_degrees(188.0), 0.95, 0.43),
            Self::Rose => Hsl::new(RgbHue::from_degrees(347.0), 0.89, 0.60),
            Self::Sky => Hsl::new(RgbHue::from_degrees(199.0), 0.88, 0.49),
        }
    }

    fn palette(self) -> TabPalette {
        let base = self.base_hsl();
        let tab_soft_hsl = Hsl::new(
            base.hue,
            (base.saturation * 0.44).clamp(0.0, 1.0),
            0.16,
        );
        let grad_start_hsl = Hsl::new(
            base.hue,
            (base.saturation * 0.86).clamp(0.0, 1.0),
            (base.lightness + 0.24).clamp(0.0, 1.0),
        );
        let grad_end_hsl = Hsl::new(
            base.hue + RgbHue::from_degrees(18.0),
            (base.saturation * 0.76).clamp(0.0, 1.0),
            (base.lightness + 0.08).clamp(0.0, 1.0),
        );
        let avg_bg_hsl = Hsl::new(
            base.hue + RgbHue::from_degrees(9.0),
            ((grad_start_hsl.saturation + grad_end_hsl.saturation) / 2.0)
                .clamp(0.0, 1.0),
            ((grad_start_hsl.lightness + grad_end_hsl.lightness) / 2.0)
                .clamp(0.0, 1.0),
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
            section id=(&self.id) class="tabbed-showcase-section" {
                header class="section-header" {
                    div {
                        h2 { (&self.title) }
                        p class="muted" { (&self.subtitle) }
                    }
                }
                div class="tabbed-showcase" {
                    div class="tabbed-showcase-shell" {
                        nav class="tabbed-showcase-tabs" aria-label="Showcase tabs" role="tablist" {
                            @for (index, tab) in self.tabs.iter().enumerate() {
                                @let tab_id = format!("{}-tab-{}", self.id, index);
                                @let panel_id = format!("{}-panel-{}", self.id, index);
                                @let palette = ShowcaseColor::cycle(index).palette();
                                button
                                    type="button"
                                    class=(if index == 0 { "tabbed-showcase-tab is-active" } else { "tabbed-showcase-tab" })
                                    data-tab-index=(index)
                                    role="tab"
                                    id=(tab_id)
                                    aria-controls=(panel_id)
                                    aria-selected=(if index == 0 { "true" } else { "false" })
                                    tabindex=(if index == 0 { "0" } else { "-1" })
                                    style={
                                        " --tab-accent: " (css_color(palette.accent)) ";"
                                        " --tab-accent-soft: " (css_color(palette.tab_soft)) ";"
                                        " color: " (css_color(palette.accent)) ";"
                                    }
                                {
                                    @if let Some(icon) = &tab.tab_icon {
                                        span class="tabbed-showcase-tab-icon" aria-hidden="true" { (icon) }
                                    }
                                    span class="tabbed-showcase-tab-title" { (&tab.tab_label) }
                                }
                            }
                        }
                        div class="tabbed-showcase-panels" {
                            @for (index, tab) in self.tabs.iter().enumerate() {
                                @let tab_id = format!("{}-tab-{}", self.id, index);
                                @let panel_id = format!("{}-panel-{}", self.id, index);
                                @let palette = ShowcaseColor::cycle(index).palette();
                                article
                                    class=(if tab.mock_panel.is_some() { "tabbed-showcase-panel" } else { "tabbed-showcase-panel tabbed-showcase-panel-full" })
                                    data-tab-index=(index)
                                    id=(panel_id)
                                    role="tabpanel"
                                    aria-labelledby=(tab_id)
                                    tabindex="0"
                                    hidden[index != 0]
                                {
                                    @if let Some(mock_panel) = &tab.mock_panel {
                                        div class="tabbed-showcase-mockup" {
                                            header {
                                                h3 { (&mock_panel.title) }
                                                p class="muted" { (&mock_panel.subtitle) }
                                            }
                                            ul class="tabbed-showcase-rows" {
                                                @for row in &mock_panel.rows {
                                                    li class="tabbed-showcase-row" {
                                                        span class="tabbed-showcase-row-label" { (&row.label) }
                                                        span class="tabbed-showcase-row-value" { (&row.value) }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div
                                        class="tabbed-showcase-copy"
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
                                        p class="muted" { (&tab.subtitle) }
                                        ul class="tabbed-showcase-bullets" {
                                            @for bullet in &tab.bullets {
                                                li { (bullet) }
                                            }
                                        }
                                        @if let Some(action) = &tab.action {
                                            a class="button" href=(&action.href) { (&action.label) }
                                        }
                                        p class="tabbed-showcase-integrations" {
                                            span { (&tab.chips_label) }
                                            @for chip in &tab.chips {
                                                span class="tabbed-showcase-chip" { (chip) }
                                            }
                                        }
                                        @if let Some(path) = &tab.code_path {
                                            p class="professionalism-path" {
                                                "Example: "
                                                code { (path) }
                                            }
                                        }
                                        @if let Some(code) = &tab.code {
                                            (CodeBlock::builder()
                                                .code(code.clone())
                                                .language(CodeLanguage::Rust)
                                                .with_class(Text::from("professionalism-code"))
                                                .build()
                                                .render())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                script { (PreEscaped(tabs_script)) }
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

  const tabs = Array.from(root.querySelectorAll('.tabbed-showcase-tab[role="tab"]'));
  const panels = Array.from(root.querySelectorAll('.tabbed-showcase-panel[role="tabpanel"]'));
  if (!tabs.length || !panels.length) return;

  const lastIndex = tabs.length - 1;

  const activate = (nextIndex, focusTab) => {{
    tabs.forEach((tab, index) => {{
      const isActive = index === nextIndex;
      tab.classList.toggle('is-active', isActive);
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
