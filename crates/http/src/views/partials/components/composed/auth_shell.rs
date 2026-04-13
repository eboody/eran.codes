mod styles;

use bon::Builder;
use maud::{Markup, Render};

use crate::types::Text;

#[derive(Clone, Copy, Debug, Default)]
pub enum Variant {
    #[default]
    Standard,
    Account,
}

impl Variant {
    fn attr_value(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Account => "account",
        }
    }
}

#[derive(Debug, Builder)]
// ci: style-system-component
pub struct AuthShell {
    pub title: Text,
    pub summary: Option<Text>,
    pub message: Option<Text>,
    pub body: Markup,
    pub footer: Option<Markup>,
    #[builder(default)]
    pub variant: Variant,
}

impl Render for AuthShell {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-auth-shell data-page-section {
                (styles::render())
                article class="u-surface-card" data-auth-card data-auth-card-variant=(self.variant.attr_value()) {
                    header data-auth-header {
                        h1 { (&self.title) }
                        @if let Some(summary) = &self.summary {
                            p data-auth-summary { (summary) }
                        }
                    }

                    @if let Some(message) = &self.message {
                        p class="u-inset-card" data-auth-message role="alert" { (message) }
                    }

                    (&self.body)

                    @if let Some(footer) = &self.footer {
                        (footer)
                    }
                }
            }
        }
    }
}
