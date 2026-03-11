use bon::Builder;
use maud::{Markup, Render};

use crate::types::Text;

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  place-items: start center;
  padding-top: clamp(0.8rem, 0.55rem + 1vw, 1.4rem);
}

me > [data-auth-card] {
  inline-size: min(100%, 34rem);
  margin-top: clamp(0.9rem, 0.6rem + 0.8vw, 1.35rem);
  gap: var(--space-4);
}

me > [data-auth-card][data-auth-card-variant='account'] {
  inline-size: min(100%, 30rem);
}

me [data-auth-header] {
  display: grid;
  gap: var(--space-2);
}

me [data-auth-summary] {
  margin: 0;
  max-width: 46ch;
  color: var(--text-muted);
}

me [data-auth-message] {
  margin-top: 1rem;
  padding: 1rem 1.15rem;
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-text-danger);
  background: var(--ui-surface-danger);
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
}

me [data-auth-form] {
  display: grid;
  gap: var(--space-3);
}

me [data-auth-field] {
  display: grid;
  gap: 0.45rem;
}

me [data-auth-field] > span {
  font-size: 0.76rem;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  color: var(--text-subtle);
}

me [data-auth-submit] {
  inline-size: 100%;
}

me [data-auth-note] {
  margin: 0;
  color: var(--text-muted);
}

me [data-auth-note] a {
  color: var(--text-strong);
}

me [data-account-actions] {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
}

me [data-account-actions] > * {
  margin: 0;
}

@media (max-width: 45rem) {
  me > [data-auth-card] {
    inline-size: 100%;
  }
}

@media (max-width: 520px) {
  me > [data-auth-card] {
    gap: var(--space-3);
  }
}
"#
);

#[derive(Clone, Copy, Debug, Default)]
pub enum AuthShellVariant {
    #[default]
    Standard,
    Account,
}

impl AuthShellVariant {
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
    pub variant: AuthShellVariant,
}

impl Render for AuthShell {
    fn render(&self) -> maud::Markup {
        maud::html! {
            main class="u-container" data-auth-shell {
                (css())
                article class="u-surface-card" data-auth-card data-auth-card-variant=(self.variant.attr_value()) {
                    header data-auth-header {
                        h1 { (&self.title) }
                        @if let Some(summary) = &self.summary {
                            p data-auth-summary { (summary) }
                        }
                    }

                    @if let Some(message) = &self.message {
                        p data-auth-message role="alert" { (message) }
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
