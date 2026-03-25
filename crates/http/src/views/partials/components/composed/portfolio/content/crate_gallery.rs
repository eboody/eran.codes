use serde::Deserialize;

use crate::types::Text;
use crate::views::partials::components::primitives::Icon;
use crate::views::partials::components::tab_set;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Content {
    pub(crate) id: Text,
    pub(crate) aria_label: Text,
    pub(crate) tabs: Vec<TabContent>,
}

impl Content {
    pub(crate) fn tab_set_content(&self) -> tab_set::content::TabSet {
        tab_set::content::TabSet {
            tabs: self.tabs.iter().map(TabContent::tab_set_tab).collect(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct TabContent {
    pub(crate) id: Text,
    pub(crate) label: LabelContent,
    pub(crate) icon: Option<Icon>,
    pub(crate) preview: PreviewContent,
    pub(crate) body: BodyContent,
}

impl TabContent {
    fn tab_set_tab(&self) -> tab_set::content::Tab {
        tab_set::content::Tab {
            id: self.id.clone(),
            label: self.label.tab_set_label(),
            icon: self.icon.clone(),
            preview: Some(self.preview.tab_set_preview()),
            body: Some(self.body.tab_set_body()),
            action: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct LabelContent {
    pub(crate) primary: Text,
    #[serde(default)]
    pub(crate) secondary: Option<Text>,
}

impl LabelContent {
    fn tab_set_label(&self) -> tab_set::content::Label {
        tab_set::content::Label {
            primary: self.primary.clone(),
            secondary: self.secondary.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct PreviewContent {
    #[serde(default)]
    pub(crate) code_examples: Vec<CodeExampleContent>,
    #[serde(default)]
    pub(crate) image: Option<ImageContent>,
    #[serde(default)]
    pub(crate) badge: Option<BadgeContent>,
}

impl PreviewContent {
    fn tab_set_preview(&self) -> tab_set::content::Preview {
        tab_set::content::Preview {
            code_examples: self
                .code_examples
                .iter()
                .map(CodeExampleContent::tab_set_code_example)
                .collect(),
            image: self.image.as_ref().map(ImageContent::tab_set_image),
            badge: self.badge.as_ref().map(BadgeContent::tab_set_badge),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct CodeExampleContent {
    #[serde(default)]
    pub(crate) label: Option<Text>,
    pub(crate) code: Text,
}

impl CodeExampleContent {
    fn tab_set_code_example(&self) -> tab_set::content::CodeExample {
        tab_set::content::CodeExample {
            label: self.label.clone(),
            code: self.code.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ImageContent {
    pub(crate) asset_ref: Text,
    #[serde(default, rename = "alt")]
    pub(crate) alt: Option<Text>,
}

impl ImageContent {
    fn tab_set_image(&self) -> tab_set::content::Image {
        tab_set::content::Image {
            asset_ref: self.asset_ref.clone(),
            _alt: self.alt.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct BadgeContent {
    pub(crate) text: Text,
}

impl BadgeContent {
    fn tab_set_badge(&self) -> tab_set::content::Badge {
        tab_set::content::Badge {
            text: self.text.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct BodyContent {
    pub(crate) title: Text,
    #[serde(default)]
    pub(crate) subtitle: Option<Text>,
    pub(crate) features: Vec<FeatureContent>,
}

impl BodyContent {
    fn tab_set_body(&self) -> tab_set::content::Body {
        tab_set::content::Body {
            title: self.title.clone(),
            subtitle: self.subtitle.clone(),
            features: self
                .features
                .iter()
                .map(FeatureContent::tab_set_feature)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct FeatureContent {
    pub(crate) text: Text,
}

impl FeatureContent {
    fn tab_set_feature(&self) -> tab_set::content::Feature {
        tab_set::content::Feature {
            text: self.text.clone(),
        }
    }
}
