use bon::Builder;
use maud::Render;

use crate::types::Text;

use super::tone::{CtaButtonType, CtaTone};

#[derive(Clone, Debug, Builder)]
pub struct CtaButton {
    pub label: Text,
    #[builder(default)]
    pub button_type: CtaButtonType,
    pub name: Option<Text>,
    pub value: Option<Text>,
    #[builder(default)]
    pub tone: CtaTone,
}

impl Render for CtaButton {
    fn render(&self) -> maud::Markup {
        let class_name = self.tone.class_name();
        let button_type = self.button_type.as_attr();

        maud::html! {
            @if let Some(name) = &self.name {
                @if let Some(value) = &self.value {
                    button type=(button_type) class=(class_name) name=(name) value=(value) {
                        (&self.label)
                    }
                } @else {
                    button type=(button_type) class=(class_name) name=(name) {
                        (&self.label)
                    }
                }
            } @else if let Some(value) = &self.value {
                button type=(button_type) class=(class_name) value=(value) {
                    (&self.label)
                }
            } @else {
                button type=(button_type) class=(class_name) {
                    (&self.label)
                }
            }
        }
    }
}
