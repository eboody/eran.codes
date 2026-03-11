use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::{RequestTraceLog, StatusCard, StatusCardItem};

#[derive(Clone, Debug, Builder)]
pub struct DbCheck {
    pub email: Text,
    pub result: Text,
    pub trace: Vec<crate::trace_log::TraceEntry>,
}

impl Render for DbCheck {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="db-check-target" {
                (StatusCard::builder()
                    .title(Text::from("DB lookup"))
                    .items(vec![
                        StatusCardItem::text("email", self.email.clone()),
                        StatusCardItem::text("result", self.result.clone()),
                    ])
                    .build())
                (RequestTraceLog::builder().entries(&self.trace).build())
            }
        }
    }
}
