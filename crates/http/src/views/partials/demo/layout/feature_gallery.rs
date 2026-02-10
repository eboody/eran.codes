use bon::Builder;
use maud::Render;

use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct FeatureGallery {
    pub title: Text,
    pub subtitle: Text,
    pub features: Vec<FeatureCard>,
    pub diagrams: Vec<DiagramPanel>,
}

impl Render for FeatureGallery {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section class="feature-gallery" {
                header class="feature-header" {
                    div {
                        h2 { (self.title.to_string()) }
                        p class="muted" { (self.subtitle.to_string()) }
                    }
                }
                div class="feature-layout" {
                    div class="feature-grid" {
                        @for card in &self.features {
                            (card.render())
                        }
                    }
                    div class="diagram-grid" {
                        @for panel in &self.diagrams {
                            (panel.render())
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct FeatureCard {
    pub title: Text,
    pub description: Text,
    pub bullets: Vec<Text>,
    pub accent: FeatureAccent,
    pub badge: Option<Text>,
}

impl Render for FeatureCard {
    fn render(&self) -> maud::Markup {
        let accent_class = self.accent.class();
        maud::html! {
            article class={ "feature-card " (accent_class) } {
                @if let Some(badge) = &self.badge {
                    span class="feature-badge" { (badge.to_string()) }
                }
                h3 { (self.title.to_string()) }
                p class="muted" { (self.description.to_string()) }
                ul class="feature-bullets" {
                    @for bullet in &self.bullets {
                        li { (bullet.to_string()) }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FeatureAccent {
    Indigo,
    Emerald,
    Amber,
    Rose,
}

impl FeatureAccent {
    fn class(self) -> &'static str {
        match self {
            FeatureAccent::Indigo => "accent-indigo",
            FeatureAccent::Emerald => "accent-emerald",
            FeatureAccent::Amber => "accent-amber",
            FeatureAccent::Rose => "accent-rose",
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct DiagramPanel {
    pub title: Text,
    pub description: Text,
    pub rows: Vec<DiagramRow>,
}

impl Render for DiagramPanel {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article class="diagram-panel" {
                header {
                    h3 { (self.title.to_string()) }
                    p class="muted" { (self.description.to_string()) }
                }
                div class="diagram-rows" {
                    @for row in &self.rows {
                        (row.render())
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct DiagramRow {
    pub label: Text,
    pub value: Text,
    pub status: DiagramStatus,
}

impl Render for DiagramRow {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="diagram-row" {
                span class={ "status-dot " (self.status.class()) } {}
                span class="diagram-label" { (self.label.to_string()) }
                span class="diagram-value" { (self.value.to_string()) }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DiagramStatus {
    Active,
    Passive,
    Warning,
    Info,
}

impl DiagramStatus {
    fn class(self) -> &'static str {
        match self {
            DiagramStatus::Active => "status-active",
            DiagramStatus::Passive => "status-passive",
            DiagramStatus::Warning => "status-warning",
            DiagramStatus::Info => "status-info",
        }
    }
}
