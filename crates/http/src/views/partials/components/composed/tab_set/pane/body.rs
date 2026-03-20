use maud::Render;

use crate::types::Text;
use crate::views::partials::components::tab_set;

#[derive(Clone, Debug)]
pub(crate) struct Body {
    pub title: Text,
    pub subtitle: Option<Text>,
    pub features: FeatureList,
}

impl From<&tab_set::content::Body> for Body {
    fn from(body: &tab_set::content::Body) -> Self {
        Self {
            title: body.title.clone(),
            subtitle: body.subtitle.clone(),
            features: FeatureList {
                children: body.features.iter().map(Feature::from).collect(),
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
            ul class="tab-set__features" {
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

impl From<&tab_set::content::Feature> for Feature {
    fn from(feature: &tab_set::content::Feature) -> Self {
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
