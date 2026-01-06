use maud_exts::{css, js};

pub struct PingPartial;

impl maud::Render for PingPartial {
    fn render(&self) -> maud::Markup {
        let current_time = jiff::Timestamp::now();
        maud::html! {
            div id="ping-target" class="card" {
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
                p {
                    (current_time)
                    ": Ping received "
                    em { "(scoped)" }
                }
                ({
                    js! {
                        me().class_add("pinged");
                    }
                })
            }
        }
    }
}
