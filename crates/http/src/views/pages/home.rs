use maud_extensions_macros::{css, js};

pub struct HomePage;

impl maud::Render for HomePage {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="page" {
                h1 { "Hello from Maud" }
                p { "This page is server-rendered; HTMX handles the small interactions." }

                div class="card" {
                    (css! {
                        me { border: 1px solid var(--accent); border-radius: 10px; padding: 12px; }
                        me strong { color: var(--accent); }
                    })
                    p {
                        strong { "Scoped CSS" }
                        " via css-scope-inline."
                    }
                }

                div id="ping-target" class="card" {
                    p { "No pings yet." }
                }

                button
                    class="btn"
                    hx-get="/partials/ping"
                    hx-target="#ping-target"
                    hx-swap="outerHTML"
                { "Ping" }

                div class="card" {
                    p { "Click to run Surreal inline script." }
                    button class="btn" { "Run script" }
                    (js! {
                        me('-').on('click', () => { me('-').textContent = 'Surreal says hi.' })
                    })
                }
            }
        };

        crate::views::layout::PageLayout {
            title: "Home",
            content,
        }
        .render()
    }
}
