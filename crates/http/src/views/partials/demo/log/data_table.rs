use bon::Builder;
use maud::Render;

#[derive(Clone, Copy, Debug)]
pub enum TableVariant {
    Default,
    ChatFlow,
}

impl TableVariant {
    fn class(self) -> &'static str {
        match self {
            TableVariant::Default => "data-table",
            TableVariant::ChatFlow => "data-table data-table-chat",
        }
    }
}

impl Default for TableVariant {
    fn default() -> Self {
        TableVariant::Default
    }
}

#[derive(Clone, Debug, Builder)]
pub struct DataTable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<maud::Markup>>,
    #[builder(default)]
    pub variant: TableVariant,
}

impl Render for DataTable {
    fn render(&self) -> maud::Markup {
        maud::html! {
            table class=(self.variant.class()) {
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
