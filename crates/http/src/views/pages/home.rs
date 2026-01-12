use maud_extensions::{css, inline_css, inline_js};

pub struct Home;

impl maud::Render for Home {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="container" {
                header {
                    h1 { "Hello from Maud" }
                    p { "This page is server-rendered; Datastar handles the small interactions." }
                }

                section {
                    div class="grid" {
                        article {
                            ({
                                css! {
                                    me {
                                      border: 1px solid var(--pico-primary);
                                    }
                                }
                            })
                            p {
                                strong { "Scoped CSS" }
                                " via css-scope-inline."
                            }
                        }
                        article {
                            h3 { "Ping" }
                            div id="ping-target" {
                                p { "No pings yet." }
                            }
                            button data-on:click="@get('/partials/ping')" { "Ping" }
                        }
                    }
                }

                section {
                    article class="clickable" {
                        h3 { "Surreal" }
                        p { "Click to run Surreal inline script." }
                        button { "Run script" }
                    }
                }

                section {
                    article {
                        button class="secondary" data-on:click="@get('/error-test')" {
                            "Trigger error"
                        }
                    }
                }

                (js())
            }
        };

        crate::views::page::Layout {
            title: "Home",
            content,
        }
        .render()
    }
}

inline_js! {
    me().on("surreal_hi", async (ev) => {
      let hi_running_name = "surreal_hi_running";
      let element = me(".clickable button");
      element.disabled = true;

      let previous_text = element.textContent;
      element.textContent = "Surreal says hi!";
      await sleep(1000);
      element.textContent = previous_text;

      element.disabled = false;
    });

    me(".clickable button").on("click", (el) => {
      if (el.disabled) {
        return;
      } else {
        me(el).send("surreal_hi");
      }
    });
}

inline_css! {
    me {
      border: 1px solid var(--pico-primary);
      border-radius: var(--pico-border-radius);
      padding: var(--pico-spacing);
    }
    me strong {
      color: var(--pico-primary);
    }
}
