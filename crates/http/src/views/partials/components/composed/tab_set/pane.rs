use maud::Render;

use crate::types::Text;
use crate::views::partials::components::Tab;

use super::content;

// ci: style-system-component
// ci: render-composition-component
// ci: bon-builder-exempt
#[derive(Clone, Debug)]
pub(crate) struct List<'a> {
    pub children: &'a [Item],
}

impl Render for List<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            @for pane in self.children {
                (pane)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Item {
    pub tab_dom_id: Text,
    pub panel_dom_id: Text,
    pub tab_value: Text,
    pub preview: Option<Preview>,
    pub body: Option<Body>,
    pub action: Option<Action>,
}

impl Item {
    pub(crate) fn from_content(tab: &Tab, tab_value: Text, tab_content: &content::Tab) -> Self {
        Self {
            tab_dom_id: tab.id.clone(),
            panel_dom_id: tab.controls.clone(),
            tab_value,
            preview: tab_content.preview.as_ref().map(Preview::from_content),
            body: tab_content.body.as_ref().map(Body::from_content),
            action: tab_content.action.as_ref().map(Action::from_content),
        }
    }
}

impl Render for Item {
    fn render(&self) -> maud::Markup {
        let show_expr = show_expr(&self.tab_value);
        let tabindex_expr = format!("{} ? '0' : '-1'", show_expr);

        maud::html! {
            section
                id=(&self.panel_dom_id)
                class="tab-set__panel ui-panel"
                role="tabpanel"
                aria-labelledby=(&self.tab_dom_id)
                data-show=(show_expr)
                data-attr:tabindex=(tabindex_expr) {
                @if let Some(preview) = &self.preview {
                    (preview)
                }
                div class="tab-set__copy" {
                    @if let Some(body) = &self.body {
                        (body)
                    }
                    @if let Some(action) = &self.action {
                        (action)
                    }
                }
            }
        }
    }
}

fn json_literal(value: &Text) -> String {
    serde_json::to_string(&value.to_string()).unwrap_or_else(|_| "\"\"".to_string())
}

pub(super) fn show_expr(value: &Text) -> String {
    format!("$active_tab_id == {}", json_literal(value))
}

#[derive(Clone, Debug)]
pub(crate) struct Preview {
    pub asset_ref: Option<Text>,
    pub badge_text: Option<Text>,
}

impl Preview {
    pub(crate) fn from_content(preview: &content::Preview) -> Self {
        Self {
            asset_ref: preview.image.as_ref().map(|image| image.asset_ref.clone()),
            badge_text: preview.badge.as_ref().map(|badge| badge.text.clone()),
        }
    }
}

impl Render for Preview {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="tab-set__preview" {
                div class="tab-set__preview-frame ui-preview-frame" {
                    p class="tab-set__preview-label" { "Preview" }
                    @if let Some(asset_ref) = &self.asset_ref {
                        p class="tab-set__preview-asset" { (asset_ref) }
                    }
                    @if let Some(badge_text) = &self.badge_text {
                        p class="tab-set__badge" { (badge_text) }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Body {
    pub title: Text,
    pub subtitle: Option<Text>,
    pub features: FeatureList,
}

impl Body {
    pub(crate) fn from_content(body: &content::Body) -> Self {
        Self {
            title: body.title.clone(),
            subtitle: body.subtitle.clone(),
            features: FeatureList {
                children: body.features.iter().map(Feature::from_content).collect(),
            },
        }
    }
}

impl Render for Body {
    fn render(&self) -> maud::Markup {
        maud::html! {
            h2 { (&self.title) }
            @if let Some(subtitle) = &self.subtitle {
                p class="tab-set__subtitle" { (subtitle) }
            }
            (self.features)
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FeatureList {
    pub children: Vec<Feature>,
}

impl Render for FeatureList {
    fn render(&self) -> maud::Markup {
        maud::html! {
            ul class="tab-set__features ui-feature-list" {
                @for feature in &self.children {
                    (feature)
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Feature {
    pub text: Text,
}

impl Feature {
    pub(crate) fn from_content(feature: &content::Feature) -> Self {
        Self {
            text: feature.text.clone(),
        }
    }
}

impl Render for Feature {
    fn render(&self) -> maud::Markup {
        maud::html! {
            li { (&self.text) }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Action {
    pub label: Text,
    pub href: Option<Text>,
}

impl Action {
    pub(crate) fn from_content(action: &content::Action) -> Self {
        Self {
            label: action.label.clone(),
            href: action.href.clone(),
        }
    }
}

impl Render for Action {
    fn render(&self) -> maud::Markup {
        maud::html! {
            @if let Some(href) = &self.href {
                a class="button tab-set__cta ui-cta" href=(href) {
                    (&self.label)
                }
            } @else {
                button class="button tab-set__cta ui-cta" type="button" {
                    (&self.label)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pane_show_expression_uses_json_literal() {
        assert_eq!(
            show_expr(&Text::from("quote'\"id")),
            "$active_tab_id == \"quote'\\\"id\""
        );
    }
}
