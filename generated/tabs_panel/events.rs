use serde::{Deserialize, Serialize};

// BEGIN MDS GENERATED:events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TabsPanelUiEvent {
    TabSelect { tab_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TabsPanelAppEvent {
    DatastarPatchSignals { server_connected: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventsStreamInput {
    pub sse_tab_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabsPanelSseEvent {
    pub server_connected: bool,
}
// END MDS GENERATED:events
