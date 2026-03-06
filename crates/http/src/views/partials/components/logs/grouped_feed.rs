use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::Pill;

use super::EventRow;

#[derive(Clone, Debug, Builder)]
pub struct GroupedFeed {
    pub children: Vec<Group>,
}

impl Render for GroupedFeed {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="ui-log-groups" data-log-groups {
                @for group in &self.children {
                    (group)
                }
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct Group {
    pub request_pill: Pill,
    pub count_label: Text,
    pub rows: Vec<EventRow>,
}

impl Render for Group {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="ui-log-group" data-log-group {
                div class="ui-log-group-header" data-log-group-header {
                    (&self.request_pill)
                    span data-muted { (&self.count_label) }
                }
                ul class="ui-log-entries" data-live-log-entries {
                    @for row in &self.rows {
                        (row)
                    }
                }
            }
        }
    }
}
