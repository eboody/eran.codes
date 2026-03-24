use maud::Render;

use crate::types::Text;
use crate::views::partials::components::{CodeBlock, CodeLineMode};
use crate::views::partials::components::tab_set;

#[derive(Clone, Debug)]
pub(crate) struct Preview {
    pub asset_ref: Option<Text>,
    pub badge_text: Option<Text>,
    pub code_examples: Vec<CodeExample>,
}

impl From<&tab_set::content::Preview> for Preview {
    fn from(preview: &tab_set::content::Preview) -> Self {
        Self {
            asset_ref: preview.image.as_ref().map(|image| image.asset_ref.clone()),
            badge_text: preview.badge.as_ref().map(|badge| badge.text.clone()),
            code_examples: preview
                .code_examples
                .iter()
                .map(CodeExample::from)
                .collect(),
        }
    }
}

impl Render for Preview {
    fn render(&self) -> maud::Markup {
        let has_code_examples = !self.code_examples.is_empty();

        maud::html! {
            div class="tab-set__preview" {
                div
                    class="tab-set__preview-frame"
                    data-preview-kind=(if has_code_examples { "code" } else { "asset" }) {
                    div class="tab-set__preview-meta" {
                        @if !has_code_examples {
                            p class="tab-set__preview-label" { "Preview" }
                        }
                        @if let Some(badge_text) = &self.badge_text {
                            p class="tab-set__badge" { (badge_text) }
                        }
                    }
                    @if has_code_examples {
                        div class="tab-set__preview-code-stack" {
                            @for example in &self.code_examples {
                                (CodeBlock::builder()
                                    .code(example.code.clone())
                                    .maybe_label(example.label.clone())
                                    .line_mode(CodeLineMode::Wrap)
                                    .with_class(Text::from("tab-set__preview-code"))
                                    .build())
                            }
                        }
                    } @else if let Some(asset_ref) = &self.asset_ref {
                        p class="tab-set__preview-asset" { (asset_ref) }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CodeExample {
    pub label: Option<Text>,
    pub code: Text,
}

impl From<&tab_set::content::CodeExample> for CodeExample {
    fn from(content: &tab_set::content::CodeExample) -> Self {
        Self {
            label: content.label.clone(),
            code: content.code.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_code_examples_when_present() {
        let preview = Preview {
            asset_ref: None,
            badge_text: Some(Text::from("Compile-time guarantees")),
            code_examples: vec![CodeExample {
                label: Some(Text::from("Transition API")),
                code: Text::from("fn publish(self) -> Post<Published> {}"),
            }],
        };
        let markup = preview.render().into_string();

        assert!(markup.contains("data-preview-kind=\"code\""));
        assert!(markup.contains("data-code-block"));
        assert!(markup.contains("Transition API"));
    }
}
