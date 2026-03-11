crate::views::scoped::inline_css!(
    r#"
me {
  border: 1px dashed var(--ui-accent-primary);
}

me em {
  font-style: normal;
  color: var(--ui-accent-primary);
}
"#
);

pub struct Ping;

impl maud::Render for Ping {
    fn render(&self) -> maud::Markup {
        let current_time = jiff::Timestamp::now();
        maud::html! {
            article id="ping-target" {
                (css())
                p {
                    (current_time)
                    ": Ping received "
                    em { "(scoped)" }
                }
            }
        }
    }
}
