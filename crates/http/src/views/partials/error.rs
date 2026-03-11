use maud::Render;

crate::views::scoped::inline_css!(
    r#"
me {
  padding: 1rem 1.15rem;
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-text-danger);
  background: var(--ui-surface-danger);
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
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
