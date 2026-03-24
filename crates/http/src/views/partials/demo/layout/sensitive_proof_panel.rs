use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

#[derive(Clone, Debug, Builder)]
pub struct SensitiveProofPanel {}

impl Render for SensitiveProofPanel {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::lab_page_content();
        let request_action = request_action();

        maud::html! {
            section
                id="sensitive-proof"
                class="u-surface-card"
                data-sensitive-proof
                data-init=(request_action) {
                ({
                    partials::SectionHeader::builder()
                        .title(content.sensitive_proof.title.clone())
                        .subtitle(content.sensitive_proof.subtitle.clone())
                        .action(partials::button::Button::builder()
                            .label(
                                content
                                    .sensitive_proof
                                    .action_label
                                    .clone()
                                    .unwrap_or_else(|| Text::from("Refresh proof")),
                            )
                            .variant(partials::button::Variant::Secondary)
                            .data_attrs(vec![
                                partials::button::DataAttr::value("data-on:click", request_action),
                            ])
                            .build())
                        .build()
                })
                ({
                    partials::DemoResultPlaceholder::builder()
                        .target_id(Text::from("sensitive-proof-target"))
                        .message(content.sensitive_proof.empty_message.clone())
                        .build()
                })
            }
        }
    }
}

fn request_action() -> &'static str {
    "@get('/partials/sensitive-proof', {filterSignals: {include: /^sseTabId$/}})"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_only_sse_tab_id_when_refreshing() {
        let markup = SensitiveProofPanel::builder().build().render().into_string();

        assert!(markup.contains(request_action()));
        assert!(markup.contains("sensitive-proof-target"));
    }
}
