use bon::Builder;
use maud::Render;

use crate::views::partials::{KeyValueList, TraceLog};

#[derive(Builder)]
pub struct BoundaryCheck<'a> {
    pub label: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub result: &'a str,
    pub trace: Vec<crate::trace_log::TraceEntry>,
}

impl Render for BoundaryCheck<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="boundary-target" {
                div class="demo-result" {
                    p { strong { (self.label) } }
                    (KeyValueList::builder()
                        .items(vec![
                            ("username".to_string(), self.username.to_string()),
                            ("email".to_string(), self.email.to_string()),
                            ("result".to_string(), self.result.to_string()),
                        ])
                        .build()
                        .render())
                }
                (TraceLog::builder().entries(&self.trace).build().render())
            }
        }
    }
}
