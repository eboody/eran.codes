use maud_extensions_macros::{css, js};

pub fn view() -> maud::Markup {
    maud::html! {
        div id="ping-target" class="card" {
            ({
                css! {
                    me { border : 1px dashed var(-- accent); } me em { font - style :
                    normal; color : var(-- accent); }
                }
            })
            p {
                "Ping received "
                em { "(scoped)" }
            }
            ({
                js! {
                    me().class_add("pinged")
                }
            })
        }
    }
}
