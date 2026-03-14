use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

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
                (partials::StatusCard::builder()
                    .title(Text::from("DB lookup"))
                    .items(vec![
                        partials::StatusCardItem::text("email", self.email.clone()),
                        partials::StatusCardItem::text("result", self.result.clone()),
                    ])
                    .build())
                (partials::RequestTraceLog::builder().entries(&self.trace).build())
            }
        }
    }
}
