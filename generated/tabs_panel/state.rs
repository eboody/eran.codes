use serde::{Deserialize, Serialize};

// BEGIN MDS GENERATED:state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabsPanelUiState {
    pub active_tab_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabsPanelAppState {
    pub server_connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabsPanelState {
    pub ui: TabsPanelUiState,
    pub app: TabsPanelAppState,
}

impl TabsPanelState {
    pub fn new(active_tab_id: impl Into<String>) -> Self {
        Self {
            ui: TabsPanelUiState {
                active_tab_id: active_tab_id.into(),
            },
            app: TabsPanelAppState {
                server_connected: false,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabsPanelContent {
    pub tabs: Vec<TabContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabContent {
    pub id: String,
    pub label: TabLabel,
    pub icon: Option<AssetRef>,
    pub preview: Option<PreviewContent>,
    pub detail: Option<DetailContent>,
    pub cta: Option<CtaContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabLabel {
    pub line_1: String,
    pub line_2: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewContent {
    pub image: Option<AssetRef>,
    pub badge: Option<BadgeContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeContent {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailContent {
    pub title: String,
    pub subtitle: Option<String>,
    pub features: Vec<FeatureContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureContent {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtaContent {
    pub label: String,
    pub href: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRef {
    pub asset_ref: String,
    pub alt: Option<String>,
}
// END MDS GENERATED:state
