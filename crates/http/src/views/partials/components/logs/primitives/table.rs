use bon::Builder;
use maud::Render;

use crate::types::Text;

// ci: markup-slot-exempt table rows require rich typed render cells.
pub type Cell = maud::Markup;

#[derive(Clone, Copy, Debug, Default)]
pub enum Variant {
    #[default]
    Default,
    ChatFlow,
}

impl Variant {
    fn is_chat_flow(self) -> bool {
        matches!(self, Variant::ChatFlow)
    }
}

#[derive(Clone, Debug, Builder)]
pub struct Table {
    pub headers: Vec<Text>,
    pub rows: Vec<Vec<Cell>>,
    #[builder(default)]
    pub variant: Variant,
}

impl Render for Table {
    fn render(&self) -> maud::Markup {
        maud::html! {
            table class="ui-log-table" data-log-table data-chat-flow[self.variant.is_chat_flow()] {
                thead {
                    tr {
                        @for header in &self.headers {
                            th { (header) }
                        }
                    }
                }
                tbody {
                    @for row in &self.rows {
                        tr {
                            @for cell in row {
                                td { (cell.clone()) }
                            }
                        }
                    }
                }
            }
        }
    }
}
