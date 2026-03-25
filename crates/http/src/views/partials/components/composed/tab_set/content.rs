use serde::Deserialize;

use crate::types::Text;
use crate::views::partials::components::primitives::Icon;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TabSet {
    pub tabs: Vec<Tab>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Tab {
    pub id: Text,
    pub label: Label,
    pub icon: Option<Icon>,
    pub preview: Option<Preview>,
    #[serde(default, alias = "detail")]
    pub body: Option<Body>,
    #[serde(default, alias = "cta")]
    pub action: Option<Action>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Label {
    #[serde(alias = "line_1")]
    pub primary: Text,
    #[serde(default, alias = "line_2")]
    pub secondary: Option<Text>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Preview {
    #[serde(default)]
    pub code_examples: Vec<CodeExample>,
    pub image: Option<Image>,
    pub badge: Option<Badge>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CodeExample {
    pub label: Option<Text>,
    pub code: Text,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Image {
    pub asset_ref: Text,
    #[serde(default, rename = "alt")]
    pub _alt: Option<Text>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Badge {
    pub text: Text,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Body {
    pub title: Text,
    pub subtitle: Option<Text>,
    pub features: Vec<Feature>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Feature {
    pub text: Text,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Action {
    pub label: Text,
    pub href: Option<Text>,
}
