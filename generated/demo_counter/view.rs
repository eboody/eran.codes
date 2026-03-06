use maud::{html, Markup};

// BEGIN MDS GENERATED:component
pub fn counter_widget_view(initial_count: i64) -> Markup {
    html! {
        section id="counter-widget"
            class="counter-widget"
            data-signals={ format!("{{count: {}, delta: 0, server_count: 0, server_connected: false}}", initial_count) } {
            h3 class="counter-title" { "Counter Widget" }
            output id="counter-value" class="counter-value" data-text="$count" { (initial_count) }
            output id="counter-server-value" class="counter-server-value" data-text="$server_count" { "0" }
            div class="counter-controls" {
                button
                    id="counter-dec"
                    type="button"
                    class="counter-dec"
                    data-on:click="$count = $count - 1; $delta = -1; @post('/api/counter/sync')" {
                    "-"
                }
                button
                    id="counter-inc"
                    type="button"
                    class="counter-inc"
                    data-on:click="$count = $count + 1; $delta = 1; @post('/api/counter/sync')" {
                    "+"
                }
            }
            small class="counter-status" data-text="$server_connected ? 'synced' : 'syncing'" {}
        }
    }
}
// END MDS GENERATED:component
