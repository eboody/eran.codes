use maud::Render;

use crate::trace_log::Entry;

pub struct TraceLog<'a> {
    pub entries: &'a [Entry],
}

impl Render for TraceLog<'_> {
    fn render(&self) -> maud::Markup {
        if self.entries.is_empty() {
            return maud::html! {
                div class="demo-result muted" { "No trace entries recorded yet." }
            };
        }

        maud::html! {
            div class="demo-result" {
                p { strong { "Trace log" } }
                ul {
                    @for entry in self.entries {
                        li {
                            strong { (entry.timestamp.clone()) }
                            " "
                            (entry.level.clone())
                            " "
                            (entry.target.clone())
                            ": "
                            (entry.message.clone())
                            @if !entry.fields.is_empty() {
                                " "
                                span class="muted" {
                                    "("
                                    @for (idx, field) in entry.fields.iter().enumerate() {
                                        @if idx > 0 { ", " }
                                        (field.0.clone())
                                        "="
                                        (field.1.clone())
                                    }
                                    ")"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
