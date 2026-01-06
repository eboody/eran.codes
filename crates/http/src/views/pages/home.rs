use maud_exts::{css, inline_css, inline_js};

pub struct HomePage;

impl maud::Render for HomePage {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="page" {
                h1 { "Hello from Maud" }
                p { "This page is server-rendered; HTMX handles the small interactions." }

                div class="card" {
                    ({
                        css! {
                            me {
                              border: 1px solid blue;
                              border-radius: 10px;
                            }
                        }
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

                div class="card clickable" {
                    p { "Click to run Surreal inline script." }
                    button class="btn" { "Run script" }
                }
                (js())
            }
        };

        crate::views::layout::PageLayout {
            title: "Home",
            content,
        }
        .render()
    }
}

inline_js! {
    me("div.clickable.card").on("click", (el) => {
      me(el).textContent = "Surreal says hi!";
    });
}

inline_css! {
    me {
      border: 1px solid var(--accent);
      border-radius: 10px;
      padding: 12px;
    }
    me strong {
      color: var(--accent);
    }
}
