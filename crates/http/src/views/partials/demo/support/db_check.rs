use bon::Builder;
use maud::Render;

use crate::views::partials::{StatusCard, TraceLog};
use crate::types::Text;

#[derive(Clone, Copy, Debug)]
enum DbLookupStatus {
    Found,
    NotFound,
}

impl From<DbLookupStatus> for Text {
    fn from(value: DbLookupStatus) -> Self {
        match value {
            DbLookupStatus::Found => Text::from("found"),
            DbLookupStatus::NotFound => Text::from("not found"),
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct DbCheck {
    pub email: Text,
    pub exists: bool,
    pub trace: Vec<crate::trace_log::TraceEntry>,
}

impl Render for DbCheck {
    fn render(&self) -> maud::Markup {
        let status = if self.exists {
            DbLookupStatus::Found
        } else {
            DbLookupStatus::NotFound
        };
        maud::html! {
            article id="db-check-target" {
                (StatusCard::builder()
                    .title(Text::from("DB lookup"))
                    .items(vec![
                        (Text::from("email"), self.email.clone()),
                        (Text::from("result"), Text::from(status)),
                    ])
                    .build()
                    .render())
                (TraceLog::builder().entries(&self.trace).build().render())
            }
        }
    }
}
