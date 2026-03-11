use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::{RequestTraceLog, StatusCard, StatusCardItem};

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
                        StatusCardItem::text("username", self.username.clone()),
                        StatusCardItem::text("email", self.email.clone()),
                        StatusCardItem::text("result", self.result.clone()),
                    ])
                    .build())
                (RequestTraceLog::builder().entries(&self.trace).build())
            }
        }
    }
}
