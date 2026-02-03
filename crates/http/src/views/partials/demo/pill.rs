use bon::Builder;
use maud::Render;

#[derive(Clone, Debug, Builder)]
pub struct Pill {
    pub text: String,
    pub extra_class: Option<String>,
}

impl Render for Pill {
    fn render(&self) -> maud::Markup {
        let class = match &self.extra_class {
            Some(extra) => format!("pill {}", extra),
            None => "pill".to_string(),
        };
        maud::html! {
            span class=(class) { (&self.text) }
        }
    }
}
