use std::sync::LazyLock;

use bon::Builder;
use maud::{PreEscaped, Render};
use maud_extensions::css;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::types::Text;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeLanguage {
    Rust,
}

impl CodeLanguage {
    fn extension(self) -> &'static str {
        match self {
            Self::Rust => "rs",
        }
    }
}

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME: LazyLock<Theme> = LazyLock::new(load_theme);

#[derive(Clone, Debug, Builder)]
pub struct CodeBlock {
    pub code: Text,
    pub language: CodeLanguage,
}

impl Render for CodeBlock {
    fn render(&self) -> maud::Markup {
        let source = self.code.to_string();
        let highlighted = highlight(&source, self.language);

        maud::html! {
            div {
                ({
                    css! {
                        me pre {
                          margin: 0;
                          border: 1px solid rgba(148, 163, 184, 0.24);
                          border-radius: var(--ui-radius-sm);
                          background: rgba(2, 6, 23, 0.82);
                          padding: 0.9rem;
                          overflow-x: auto;
                          white-space: pre;
                          font-size: 0.82rem;
                          line-height: 1.45;
                          isolation: isolate;
                        }
                        me pre > code {
                          display: block;
                          font-family: var(--ui-font-mono);
                        }
                    }
                })
                pre {
                    code {
                        @if let Some(markup) = highlighted {
                            (PreEscaped(markup))
                        } @else {
                            (&source)
                        }
                    }
                }
            }
        }
    }
}

fn highlight(source: &str, language: CodeLanguage) -> Option<String> {
    let syntax = SYNTAX_SET
        .find_syntax_by_extension(language.extension())
        .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
    let mut highlighter = HighlightLines::new(syntax, &THEME);
    let mut html = String::new();

    for line in LinesWithEndings::from(source) {
        let regions = highlighter.highlight_line(line, &SYNTAX_SET).ok()?;
        let line_html =
            styled_line_to_highlighted_html(&regions, IncludeBackground::No).ok()?;
        html.push_str(&line_html);
    }

    Some(html)
}

fn load_theme() -> Theme {
    let themes = ThemeSet::load_defaults();
    if let Some(theme) = themes.themes.get("base16-ocean.dark") {
        return theme.clone();
    }
    themes
        .themes
        .into_values()
        .next()
        .expect("syntect did not provide any bundled theme")
}
