use bon::Builder;
use maud::Render;

#[derive(Clone, Debug, Builder)]
pub struct FlowMap {
    pub steps: Vec<String>,
}

impl Render for FlowMap {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="flow-map" {
                @for (idx, step) in self.steps.iter().enumerate() {
                    @if idx > 0 {
                        span class="arrow" { "→" }
                    }
                    span class="step" { (step) }
                }
            }
        }
    }
}
