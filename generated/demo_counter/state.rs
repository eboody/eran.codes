use serde::{Deserialize, Serialize};

// BEGIN MDS GENERATED:state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterUiState {
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterAppState {
    pub server_count: i64,
    pub server_connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterState {
    pub ui: CounterUiState,
    pub app: CounterAppState,
}

impl CounterState {
    pub fn new(count: i64) -> Self {
        Self {
            ui: CounterUiState { count },
            app: CounterAppState {
                server_count: 0,
                server_connected: false,
            },
        }
    }
}
// END MDS GENERATED:state
