use bon::Builder;
use maud::{PreEscaped, Render};
use palette::{FromColor, Hsl, RgbHue, Srgb};

use crate::types::Text;
use crate::views::partials::components::{CodeBlock, CodeLanguage};

#[derive(Clone, Debug, Builder)]
pub struct TabbedShowcaseRow {
    pub label: Text,
    pub value: Text,
}

#[derive(Clone, Debug, Builder)]
pub struct TabbedShowcaseAction {
    pub label: Text,
    pub href: Text,
}

#[derive(Clone, Debug, Builder)]
pub struct TabbedShowcaseMockPanel {
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
        let opposite_hue = (base.hue.into_degrees() + 180.0).rem_euclid(360.0);

        let copy_text = if use_dark_text {
            "hsl(222 47% 11%)"
        } else {
            "hsl(210 40% 98%)"
        };
        let copy_muted = if use_dark_text {
            "hsl(222 47% 11% / 0.82)"
        } else {
            "hsl(210 40% 98% / 0.86)"
        };
        let chip_bg = if use_dark_text {
            "hsl(0 0% 100% / 0.68)"
        } else {
            "hsl(223 47% 11% / 0.28)"
        };
        let chip_border = if use_dark_text {
            "hsl(223 47% 11% / 0.24)"
        } else {
            "hsl(210 40% 98% / 0.34)"
        };

        TabPalette {
            accent: hsl_css(base),
            tab_soft: hsl_css(tab_soft_hsl),
            grad_start: hsl_css(grad_start_hsl),
            grad_end: hsl_css(grad_end_hsl),
            copy_text: copy_text.to_owned(),
            copy_muted: copy_muted.to_owned(),
            chip_bg: chip_bg.to_owned(),
            chip_border: chip_border.to_owned(),
            opposite_hue,
        }
    }
}

#[derive(Clone, Debug)]
struct TabPalette {
    accent: String,
    tab_soft: String,
    grad_start: String,
    grad_end: String,
    copy_text: String,
    copy_muted: String,
    chip_bg: String,
    chip_border: String,
    opposite_hue: f32,
}

#[derive(Clone, Debug, Builder)]
pub struct TabbedShowcaseTab {
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

        let dynamic_rules = self.dynamic_rules();
        let radio_name = format!("{}-tabs", self.id);

        maud::html! {
            section class="tabbed-showcase-section" {
                header class="section-header" {
                    div {
                        h2 { (&self.title) }
                        p class="muted" { (&self.subtitle) }
                    }
                }
                div class="tabbed-showcase" {
                    style { (PreEscaped(dynamic_rules)) }
                    @for (index, _) in self.tabs.iter().enumerate() {
                        @let tab_id = format!("{}-tab-{}", self.id, index);
                        input
                            type="radio"
                            class="tabbed-showcase-toggle"
                            id=(tab_id)
                            name=(radio_name)
                            checked[index == 0];
                    }
                    div class="tabbed-showcase-shell" {
                        nav class="tabbed-showcase-tabs" aria-label="Showcase tabs" {
                            @for (index, tab) in self.tabs.iter().enumerate() {
                                @let tab_id = format!("{}-tab-{}", self.id, index);
                                @let palette = ShowcaseColor::cycle(index).palette();
                                label
                                    class="tabbed-showcase-tab"
                                    data-tab-index=(index)
                                    for=(tab_id)
                                    style={
                                        " --tab-accent: " (palette.accent) ";"
                                        " --tab-accent-soft: " (palette.tab_soft) ";"
                                        " color: " (palette.accent) ";"
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
                                @let palette = ShowcaseColor::cycle(index).palette();
                                article
                                    class=(if tab.mock_panel.is_some() { "tabbed-showcase-panel" } else { "tabbed-showcase-panel tabbed-showcase-panel-full" })
                                    data-tab-index=(index)
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
                                            " --tab-accent: " (palette.accent) ";" 
                                            " --tab-grad-start: " (palette.grad_start) ";"
                                            " --tab-grad-end: " (palette.grad_end) ";"
                                            " --tab-copy-text: " (palette.copy_text) ";"
                                            " --tab-copy-muted: " (palette.copy_muted) ";"
                                            " --tab-chip-bg: " (palette.chip_bg) ";"
                                            " --tab-chip-border: " (palette.chip_border) ";"
                                            " --tab-opposite-h: " (palette.opposite_hue) ";"
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
            }
        }
    }
}

impl TabbedShowcase {
    fn dynamic_rules(&self) -> String {
        let mut css = String::new();

        for index in 0..self.tabs.len() {
            let input_id = format!("{}-tab-{}", self.id, index);
            css.push_str(&format!(
                "#{input_id}:checked ~ .tabbed-showcase-shell .tabbed-showcase-tab[data-tab-index=\"{index}\"] {{ \
                    border-color: var(--tab-accent); \
                    background: var(--tab-accent-soft); \
                    color: var(--tab-accent); \
                    opacity: 1; \
                }}\n"
            ));
            css.push_str(&format!(
                "#{input_id}:checked ~ .tabbed-showcase-shell .tabbed-showcase-panel[data-tab-index=\"{index}\"] {{ \
                    display: grid; \
                }}\n"
            ));
        }

        css
    }
}

fn hsl_css(color: Hsl) -> String {
    format!(
        "hsl({:.1} {:.1}% {:.1}%)",
        color.hue.into_degrees(),
        color.saturation * 100.0,
        color.lightness * 100.0
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
