use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

#[derive(Clone, Debug, Builder)]
pub struct BoundaryCheck {
    pub label: Text,
    pub username: Text,
    pub email: Text,
    pub result: Text,
    pub trace: Vec<crate::trace_log::store::TraceEntry>,
}

impl Render for BoundaryCheck {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="boundary-target" {
                (partials::StatusCard::builder()
                    .title(self.label.clone())
                    .items(vec![
                        partials::StatusCardItem::text("username", self.username.clone()),
                        partials::StatusCardItem::text("email", self.email.clone()),
                        partials::StatusCardItem::text("result", self.result.clone()),
                    ])
                    .build())
                (partials::RequestTraceLog::builder().entries(&self.trace).build())
            }
        }
    }
}
