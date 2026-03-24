use bon::Builder;
use maud::{Markup, Render};

use crate::types::Text;

#[derive(Builder)]
pub struct ResultCard {
    pub content: Markup,
    pub target_id: Option<Text>,
    pub extra_class: Option<&'static str>,
    #[builder(default)]
    pub muted: bool,
    #[builder(default)]
    pub status_card: bool,
}

impl Render for ResultCard {
    fn render(&self) -> Markup {
        let mut class_name = String::from("u-demo-result-card u-inset-card");
        if self.muted {
            class_name.push_str(" u-muted");
        }
        if let Some(extra_class) = &self.extra_class {
            class_name.push(' ');
            class_name.push_str(extra_class);
        }

        match (&self.target_id, self.status_card) {
            (Some(target_id), true) => maud::html! {
                div id=(target_id) class=(class_name) data-demo-result data-status-card {
                    (&self.content)
                }
            },
            (Some(target_id), false) => maud::html! {
                div id=(target_id) class=(class_name) data-demo-result {
                    (&self.content)
                }
            },
            (None, true) => maud::html! {
                div class=(class_name) data-demo-result data-status-card {
                    (&self.content)
                }
            },
            (None, false) => maud::html! {
                div class=(class_name) data-demo-result {
                    (&self.content)
                }
            },
        }
    }
}
