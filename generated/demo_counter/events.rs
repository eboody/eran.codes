use serde::{Deserialize, Serialize};

// BEGIN MDS GENERATED:events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CounterUiEvent {
    IncrementClick,
    DecrementClick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CounterAppEvent {
    CounterUpdate { count: i64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncCounterInput {
    pub delta: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncCounterCommandResult {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CounterSignalPatch {
    pub server_count: i64,
    pub server_connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterStreamInput {
    pub component_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterSseEvent {
    pub count: i64,
}
// END MDS GENERATED:events
