use bon::Builder;
use maud::Render;

use crate::views::partials::{StatusCard, TraceLog};
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct BoundaryCheck {
    pub label: Text,
    pub username: Text,
    pub email: Text,
    pub result: Text,
    pub trace: Vec<crate::trace_log::TraceEntry>,
}

impl Render for BoundaryCheck {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="boundary-target" {
                (StatusCard::builder()
                    .title(self.label.clone())
                    .items(vec![
                        (Text::from("username"), self.username.clone()),
                        (Text::from("email"), self.email.clone()),
                        (Text::from("result"), self.result.clone()),
                    ])
                    .build()
                    .render())
                (TraceLog::builder().entries(&self.trace).build().render())
            }
        }
    }
}
