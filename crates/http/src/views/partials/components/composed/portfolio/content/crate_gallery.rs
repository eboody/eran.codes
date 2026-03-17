use serde::Deserialize;

use crate::types::Text;
use crate::views::partials::components::primitives::Icon;
use crate::views::partials::components::tab_set;

#[derive(Clone, Debug, Deserialize)]
pub struct CrateGalleryContent {
    pub id: Text,
    pub aria_label: Text,
    pub signal_name: Text,
    pub tabs: Vec<CrateGalleryTabContent>,
}

impl CrateGalleryContent {
    pub(crate) fn tab_set_content(&self) -> tab_set::content::TabSetContent {
        tab_set::content::TabSetContent {
            tabs: self
                .tabs
                .iter()
                .map(CrateGalleryTabContent::tab_set_tab)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrateGalleryTabContent {
    pub id: Text,
    pub label: CrateGalleryLabelContent,
    pub icon: Option<Icon>,
    pub preview: CrateGalleryPreviewContent,
    pub body: CrateGalleryBodyContent,
}

impl CrateGalleryTabContent {
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
pub struct CrateGalleryLabelContent {
    pub primary: Text,
    #[serde(default)]
    pub secondary: Option<Text>,
}

impl CrateGalleryLabelContent {
    fn tab_set_label(&self) -> tab_set::content::Label {
        tab_set::content::Label {
            primary: self.primary.clone(),
            secondary: self.secondary.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrateGalleryPreviewContent {
    #[serde(default)]
    pub code_examples: Vec<CrateGalleryCodeExampleContent>,
    #[serde(default)]
    pub image: Option<CrateGalleryImageContent>,
    #[serde(default)]
    pub badge: Option<CrateGalleryBadgeContent>,
}

impl CrateGalleryPreviewContent {
    fn tab_set_preview(&self) -> tab_set::content::Preview {
        tab_set::content::Preview {
            code_examples: self
                .code_examples
                .iter()
                .map(CrateGalleryCodeExampleContent::tab_set_code_example)
                .collect(),
            image: self
                .image
                .as_ref()
                .map(CrateGalleryImageContent::tab_set_image),
            badge: self
                .badge
                .as_ref()
                .map(CrateGalleryBadgeContent::tab_set_badge),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrateGalleryCodeExampleContent {
    #[serde(default)]
    pub label: Option<Text>,
    pub code: Text,
}

impl CrateGalleryCodeExampleContent {
    fn tab_set_code_example(&self) -> tab_set::content::CodeExample {
        tab_set::content::CodeExample {
            label: self.label.clone(),
            code: self.code.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrateGalleryImageContent {
    pub asset_ref: Text,
    #[serde(default, rename = "alt")]
    pub alt: Option<Text>,
}

impl CrateGalleryImageContent {
    fn tab_set_image(&self) -> tab_set::content::Image {
        tab_set::content::Image {
            asset_ref: self.asset_ref.clone(),
            _alt: self.alt.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrateGalleryBadgeContent {
    pub text: Text,
}

impl CrateGalleryBadgeContent {
    fn tab_set_badge(&self) -> tab_set::content::Badge {
        tab_set::content::Badge {
            text: self.text.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrateGalleryBodyContent {
    pub title: Text,
    #[serde(default)]
    pub subtitle: Option<Text>,
    pub features: Vec<CrateGalleryFeatureContent>,
}

impl CrateGalleryBodyContent {
    fn tab_set_body(&self) -> tab_set::content::Body {
        tab_set::content::Body {
            title: self.title.clone(),
            subtitle: self.subtitle.clone(),
            features: self
                .features
                .iter()
                .map(CrateGalleryFeatureContent::tab_set_feature)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrateGalleryFeatureContent {
    pub text: Text,
}

impl CrateGalleryFeatureContent {
    fn tab_set_feature(&self) -> tab_set::content::Feature {
        tab_set::content::Feature {
            text: self.text.clone(),
        }
    }
}
