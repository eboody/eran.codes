use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

use super::Results;

#[derive(Clone, Debug, Builder)]
pub struct DbCheck {
    pub email: Text,
    pub result: Text,
    pub trace: Vec<crate::trace_log::store::TraceEntry>,
}

impl Render for DbCheck {
    fn render(&self) -> maud::Markup {
        Results::builder()
            .target_id(Text::from("db-check-target"))
            .summary(
                partials::StatusCard::builder()
                    .title(Text::from("DB lookup"))
                    .items(vec![
                        partials::StatusCardItem::text("email", self.email.clone()),
                        partials::StatusCardItem::text("result", self.result.clone()),
                    ])
                    .build()
                    .render(),
            )
            .trace(
                partials::RequestTraceLog::builder()
                    .entries(&self.trace)
                    .build()
                    .render(),
            )
            .build()
            .render()
    }
}
