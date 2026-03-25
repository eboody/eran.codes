use bon::Builder;
use maud::{PreEscaped, Render};

use crate::types::Text;

#[derive(Clone, Copy, Debug, Default)]
pub enum Scope {
    Single,
    #[default]
    All,
}

#[derive(Clone, Debug, Builder)]
pub struct AutoScroll {
    pub root_id: Text,
    pub selector: Text,
    #[builder(default)]
    pub scope: Scope,
}

impl Render for AutoScroll {
    fn render(&self) -> maud::Markup {
        let root_id = escape_js_single_quoted(&self.root_id.to_string());
        let selector = escape_js_single_quoted(&self.selector.to_string());
        let script = match self.scope {
            Scope::Single => format!(
                r#"
(() => {{
  const root = document.getElementById('{root_id}');
  if (!root) return;
  const panel = root.querySelector('{selector}');
  if (!panel) return;
  const scroll = () => {{ panel.scrollTop = panel.scrollHeight; }};
  requestAnimationFrame(scroll);
  const observer = new MutationObserver(scroll);
  observer.observe(panel, {{ childList: true, subtree: true }});
}})();
"#
            ),
            Scope::All => format!(
                r#"
(() => {{
  const root = document.getElementById('{root_id}');
  if (!root) return;
  const panels = root.querySelectorAll('{selector}');
  panels.forEach((panel) => {{
    const scroll = () => {{ panel.scrollTop = panel.scrollHeight; }};
    requestAnimationFrame(scroll);
    const observer = new MutationObserver(scroll);
    observer.observe(panel, {{ childList: true, subtree: true }});
  }});
}})();
"#
            ),
        };

        maud::html! {
            script { (PreEscaped(script)) }
        }
    }
}

fn escape_js_single_quoted(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
