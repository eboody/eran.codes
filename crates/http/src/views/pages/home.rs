use maud_extensions::{css, inline_css};

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
                    article data-signals="{surrealMessage: 'Ready.', originalSurrealMessage: 'Ready.', surrealStatus: 'idle'}" {
                        h3 { "Signals" }
                        p data-text="$surrealMessage" {}
                        small data-text="$surrealStatus" {}
                        div class="grid" {
                            button
                                data-on:click="$surrealMessage = 'Front-end says hi!'; setTimeout(() => { $surrealMessage = $originalSurrealMessage; }, 1000)"
                            { "Front-end update" }
                            button data-on:click="@get('/partials/surreal-message-guarded')" {
                                "Backend guarded"
                            }
                            button data-on:click="@get('/partials/surreal-message-cancel')" {
                                "Backend cancel"
                            }
                        }
                    }
                }

                section {
                    article {
                        button class="secondary" data-on:click="@get('/error-test')" {
                            "Trigger error"
                        }
                    }
                }
            }
        };

        crate::views::page::Layout {
            title: "Home",
            content,
        }
        .render()
    }
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
