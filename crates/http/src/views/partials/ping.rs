use maud_extensions::css;

pub struct Ping;

impl maud::Render for Ping {
    fn render(&self) -> maud::Markup {
        let current_time = jiff::Timestamp::now();
        maud::html! {
            div id="ping-target" class="card pinged" {
                p {
                    (current_time)
                    ": Ping received "
                    em { "(scoped)" }
                }
                ({
                    css! {
                        me {
                          border: 1px dashed var(--accent);
                        }
                        me em {
                          font-style: normal;
                          color: red;
                        }
                    }
                })
            }
        }
    }
}
