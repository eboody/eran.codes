use maud::Render;

use crate::trace_log::Entry;

pub struct LiveLog<'a> {
    pub entries: &'a [Entry],
}

impl Render for LiveLog<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="live-log-target" class="demo-result" {
                p { strong { "Live backend log" } }
                @if self.entries.is_empty() {
                    p class="muted" { "No events yet. Trigger a demo action to start streaming." }
                } @else {
                    ul {
                        @for entry in self.entries.iter().rev().take(20) {
                            li {
                                strong { (entry.timestamp.clone()) }
                                " "
                                (entry.level.clone())
                                " "
                                (entry.target.clone())
                                ": "
                                (entry.message.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
