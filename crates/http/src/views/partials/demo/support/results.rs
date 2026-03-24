use bon::Builder;
use maud::{Markup, Render};

use crate::types::Text;

#[derive(Builder)]
pub struct Results {
    pub target_id: Text,
    pub summary: Markup,
    pub trace: Markup,
}

impl Render for Results {
    fn render(&self) -> Markup {
        maud::html! {
            article
                id=(&self.target_id)
                class="u-support-results"
                data-support-results
            {
                (&self.summary)
                div class="u-support-trace" data-support-trace {
                    (&self.trace)
                }
            }
        }
    }
}

#[derive(Builder)]
pub struct CardGrid {
    pub cards: Vec<Markup>,
}

impl Render for CardGrid {
    fn render(&self) -> Markup {
        maud::html! {
            div class="u-support-card-grid" data-support-card-grid {
                @for card in &self.cards {
                    (card)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_results_wrapper_and_card_grid() {
        let markup = Results::builder()
            .target_id(Text::from("example-target"))
            .summary(
                CardGrid::builder()
                    .cards(vec![
                        maud::html! { div data-status-card { "one" } },
                        maud::html! { div data-status-card { "two" } },
                    ])
                    .build()
                    .render(),
            )
            .trace(maud::html! { div { "trace" } })
            .build()
            .render()
            .into_string();

        assert!(markup.contains("id=\"example-target\""));
        assert!(markup.contains("data-support-results"));
        assert!(markup.contains("data-support-card-grid"));
        assert!(markup.contains("data-support-trace"));
    }
}
