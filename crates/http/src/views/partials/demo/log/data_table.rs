use crate::types::Text;
use bon::Builder;
use maud::Render;

#[derive(Clone, Copy, Debug, Default)]
pub enum TableVariant {
    #[default]
    Default,
    ChatFlow,
}

impl TableVariant {
    fn is_chat_flow(self) -> bool {
        matches!(self, TableVariant::ChatFlow)
    }
}

#[derive(Clone, Debug, Builder)]
pub struct DataTable {
    pub headers: Vec<Text>,
    pub rows: Vec<Vec<maud::Markup>>,
    #[builder(default)]
    pub variant: TableVariant,
}

impl Render for DataTable {
    fn render(&self) -> maud::Markup {
        maud::html! {
            table data-log-table data-chat-flow[self.variant.is_chat_flow()] {
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
