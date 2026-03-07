use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::logs;

#[derive(Clone, Copy, Debug, Default)]
pub enum SurfaceLayout {
    #[default]
    Stack,
    Panels,
}

impl SurfaceLayout {
    fn class_name(self) -> &'static str {
        match self {
            SurfaceLayout::Stack => "ui-log-surface",
            SurfaceLayout::Panels => "ui-log-surface ui-log-panels",
        }
    }

    fn is_panels(self) -> bool {
        matches!(self, SurfaceLayout::Panels)
    }
}

// ci: render-composition-component
#[derive(Clone, Debug, Builder)]
pub struct Surface {
    pub target_id: Option<Text>,
    #[builder(default)]
    pub layout: SurfaceLayout,
    #[builder(default)]
    pub classes: Vec<Text>,
    pub children: Vec<maud::Markup>,
    pub auto_scroll: Option<logs::primitives::AutoScroll>,
}

impl Render for Surface {
    fn render(&self) -> maud::Markup {
        let mut class_names = vec![self.layout.class_name().to_string()];
        class_names.extend(self.classes.iter().map(ToString::to_string));
        let class_attr = class_names.join(" ");

        if let Some(target_id) = &self.target_id {
            maud::html! {
                section
                    id=(target_id)
                    class=(class_attr)
                    data-log-panels[self.layout.is_panels()] {
                    @for child in &self.children {
                        (child)
                    }
                    @if let Some(auto_scroll) = &self.auto_scroll {
                        (auto_scroll)
                    }
                }
            }
        } else {
            maud::html! {
                section class=(class_attr) data-log-panels[self.layout.is_panels()] {
                    @for child in &self.children {
                        (child)
                    }
                    @if let Some(auto_scroll) = &self.auto_scroll {
                        (auto_scroll)
                    }
                }
            }
        }
    }
}
