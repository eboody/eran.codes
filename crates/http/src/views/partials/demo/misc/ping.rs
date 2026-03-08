pub struct Ping;

impl maud::Render for Ping {
    fn render(&self) -> maud::Markup {
        let current_time = jiff::Timestamp::now();
        maud::html! {
            article id="ping-target" class="ui-ping-target" {
                p {
                    (current_time)
                    ": Ping received "
                    em { "(scoped)" }
                }
            }
        }
    }
}
