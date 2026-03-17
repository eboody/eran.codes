use maud::Render;

crate::views::scoped::inline_css!(
    r#"
me {
  --inset-card-border: var(--ui-text-danger);
  --inset-card-bg: var(--ui-surface-danger);
  --inset-card-padding: var(--ui-inset-card-padding-cozy);
}

me strong {
  color: var(--ui-text-danger);
}
"#
);

#[derive(Clone, Copy, Debug)]
pub struct Error;

impl Render for Error {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div id="error-target" data-show="$transportErrorMessage" style="display:none;" {
                (css())
                article
                    class="u-inset-card"
                    role="alert"
                    data-transport-error=""
                    data-attr:data-transport-error-kind="$transportErrorKind"
                    data-attr:data-transport-error-status="$transportErrorStatus || ''"
                {
                    strong data-text="$transportErrorTitle || 'Something went wrong'" { "Something went wrong" }
                    p data-text="$transportErrorMessage" {}
                }
            }
        }
    }
}
