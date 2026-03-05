use axum::response::sse::Event;

use crate::generated::tabs_panel::events::TabsPanelSseEvent;

// BEGIN MDS GENERATED:handler
pub fn tabs_panel_patch_event(update: TabsPanelSseEvent) -> Event {
    Event::default().event("datastar-patch-signals").data(format!(
        "signals {{server_connected: {}}}",
        update.server_connected
    ))
}
// END MDS GENERATED:handler
