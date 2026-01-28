use bon::Builder;
use maud::Render;

use crate::views::partials::{KeyValueList, TraceLog};

#[derive(Clone, Copy, Debug)]
enum DbLookupStatus {
    Found,
    NotFound,
}

impl DbLookupStatus {
    fn as_str(self) -> &'static str {
        match self {
            DbLookupStatus::Found => "found",
            DbLookupStatus::NotFound => "not found",
        }
    }
}

#[derive(Builder)]
pub struct DbCheck<'a> {
    pub email: &'a str,
    pub exists: bool,
    pub trace: Vec<crate::trace_log::TraceEntry>,
}

impl Render for DbCheck<'_> {
    fn render(&self) -> maud::Markup {
        let status = if self.exists {
            DbLookupStatus::Found
        } else {
            DbLookupStatus::NotFound
        };
        maud::html! {
            article id="db-check-target" {
                div class="demo-result" {
                    p { strong { "DB lookup" } }
                    (KeyValueList::builder()
                        .items(vec![
                            ("email".to_string(), self.email.to_string()),
                            ("result".to_string(), status.as_str().to_string()),
                        ])
                        .build()
                        .render())
                }
                (TraceLog::builder().entries(&self.trace).build().render())
            }
        }
    }
}
