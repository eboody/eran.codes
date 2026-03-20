use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::{BadgeKind, Pill};

crate::views::scoped::inline_css!(
    r#"
me {
  --_code-block-padding: clamp(0.85rem, 0.76rem + 0.42vw, 1.05rem);
  --_code-block-font-size: var(--text-size-meta-sm);
  --_code-block-line-height: var(--text-line-code);
  --_code-block-border-color: color-mix(
    in srgb,
    var(--portfolio-accent-a) 46%,
    var(--ui-border-soft)
  );
  --_code-block-background:
    linear-gradient(
      180deg,
      color-mix(in srgb, white 18%, transparent),
      transparent 28%
    ),
    color-mix(in srgb, var(--ui-surface-soft-alt) 84%, var(--ui-surface-card));
  --_code-block-token-keyword: color-mix(
    in srgb,
    var(--accent-signal) 82%,
    var(--text-strong)
  );
  --_code-block-token-type: color-mix(
    in srgb,
    var(--accent-warm) 76%,
    var(--text-strong)
  );
  --_code-block-token-number: color-mix(
    in srgb,
    var(--status-success) 82%,
    var(--text-strong)
  );
  --_code-block-token-string: color-mix(
    in srgb,
    var(--accent-warm) 68%,
    var(--text-strong)
  );
  --_code-block-token-comment: color-mix(
    in srgb,
    var(--ui-text-muted) 88%,
    var(--ui-text) 12%
  );
  --_code-block-token-macro: color-mix(
    in srgb,
    var(--accent-signal) 64%,
    var(--accent-warm) 36%
  );

  display: grid;
  gap: var(--space-2);
  margin: 0;
  padding: var(--_code-block-padding);
  border: var(--border-size-1) solid var(--_code-block-border-color);
  border-radius: var(--ui-radius-md);
  background: var(--_code-block-background);
  box-shadow:
    inset 0 1px 0 color-mix(in srgb, white 38%, transparent),
    inset 0 0 0 1px color-mix(in srgb, black 2%, transparent);
  overflow: clip;
}

me [data-code-block-header] {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-2);
  padding-block-end: var(--space-2);
  border-bottom: 1px solid color-mix(in srgb, var(--ui-border-soft) 88%, transparent);
}

me [data-code-block-label] {
  margin: 0;
  font-size: var(--text-size-label-xs);
  font-weight: 500;
  letter-spacing: var(--text-track-label);
  color: var(--ui-text-muted);
}

me pre {
  margin: 0;
  padding-block-start: var(--space-1);
  overflow: auto;
  font-family: var(--ui-font-mono);
  font-size: var(--_code-block-font-size);
  line-height: var(--_code-block-line-height);
  color: color-mix(in srgb, var(--ui-text) 94%, var(--text-strong) 6%);
  scrollbar-gutter: stable both-edges;
}

me code {
  display: block;
  min-width: max-content;
  white-space: pre;
}

me .ui-code-block__token--keyword {
  color: var(--_code-block-token-keyword);
  font-weight: 600;
}

me .ui-code-block__token--type {
  color: var(--_code-block-token-type);
}

me .ui-code-block__token--number {
  color: var(--_code-block-token-number);
}

me .ui-code-block__token--string {
  color: var(--_code-block-token-string);
}

me .ui-code-block__token--comment {
  color: var(--_code-block-token-comment);
  font-style: italic;
}

me .ui-code-block__token--macro {
  color: var(--_code-block-token-macro);
}

@media (max-width: 48rem) {
  me {
    --_code-block-padding: 0.78rem;
    --_code-block-font-size: var(--text-size-meta-xs);
    --_code-block-line-height: var(--text-line-reading);
    gap: var(--space-1);
  }

  me [data-code-block-header] {
    padding-block-end: var(--space-1);
  }
}

@media (prefers-color-scheme: dark) {
  me {
    --_code-block-border-color: color-mix(
      in srgb,
      var(--ui-border-soft) 88%,
      transparent
    );
    --_code-block-background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 22%),
      color-mix(in srgb, var(--surface-panel) 95%, black 5%);

    box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  }
}
"#
);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, strum_macros::AsRefStr)]
pub enum CodeLanguage {
    #[default]
    #[strum(serialize = "Rust")]
    Rust,
}

#[derive(Clone, Debug, Builder)]
// ci: style-system-component
pub struct CodeBlock {
    pub code: Text,
    #[builder(default)]
    pub language: CodeLanguage,
    pub label: Option<Text>,
    #[builder(setters(name = with_class))]
    pub class: Option<Text>,
}

impl Render for CodeBlock {
    fn render(&self) -> maud::Markup {
        let source = self.code.to_string();
        let tokens = tokenize(&source, self.language);
        let class_attr = self.class_attr();

        maud::html! {
            div class=(class_attr) data-code-block {
                (css())
                div data-code-block-header {
                    (Pill::badge(self.language.as_ref(), BadgeKind::Secondary))
                    @if let Some(label) = &self.label {
                        p data-code-block-label { (label) }
                    }
                }
                pre {
                    code {
                        @for token in tokens {
                            @if token.kind.class_name().is_empty() {
                                (token.value)
                            } @else {
                                span class=(token.kind.class_name()) { (token.value) }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl CodeBlock {
    fn class_attr(&self) -> String {
        match &self.class {
            Some(class) => format!("u-inset-card ui-code-block {}", class),
            None => String::from("u-inset-card ui-code-block"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CodeTokenKind {
    Plain,
    Keyword,
    TypeName,
    Number,
    String,
    Comment,
    Macro,
}

impl CodeTokenKind {
    fn class_name(self) -> &'static str {
        match self {
            Self::Plain => "",
            Self::Keyword => "ui-code-block__token--keyword",
            Self::TypeName => "ui-code-block__token--type",
            Self::Number => "ui-code-block__token--number",
            Self::String => "ui-code-block__token--string",
            Self::Comment => "ui-code-block__token--comment",
            Self::Macro => "ui-code-block__token--macro",
        }
    }
}

#[derive(Clone, Debug)]
struct CodeToken<'a> {
    value: &'a str,
    kind: CodeTokenKind,
}

fn tokenize<'a>(source: &'a str, language: CodeLanguage) -> Vec<CodeToken<'a>> {
    match language {
        CodeLanguage::Rust => tokenize_rust(source),
    }
}

fn tokenize_rust<'a>(source: &'a str) -> Vec<CodeToken<'a>> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        let ch = bytes[index] as char;

        if ch.is_whitespace() {
            let start = index;
            index += 1;
            while index < bytes.len() && (bytes[index] as char).is_whitespace() {
                index += 1;
            }
            push_token(&mut tokens, &source[start..index], CodeTokenKind::Plain);
            continue;
        }

        if ch == '/' && index + 1 < bytes.len() && bytes[index + 1] as char == '/' {
            let start = index;
            index += 2;
            while index < bytes.len() && bytes[index] as char != '\n' {
                index += 1;
            }
            push_token(&mut tokens, &source[start..index], CodeTokenKind::Comment);
            continue;
        }

        if ch == '"' {
            let start = index;
            index += 1;
            while index < bytes.len() {
                let current = bytes[index] as char;
                if current == '\\' {
                    index += 2;
                    continue;
                }
                index += 1;
                if current == '"' {
                    break;
                }
            }
            push_token(
                &mut tokens,
                &source[start..index.min(bytes.len())],
                CodeTokenKind::String,
            );
            continue;
        }

        if ch.is_ascii_digit() {
            let start = index;
            index += 1;
            while index < bytes.len() {
                let current = bytes[index] as char;
                if current.is_ascii_alphanumeric() || current == '_' || current == '.' {
                    index += 1;
                } else {
                    break;
                }
            }
            push_token(&mut tokens, &source[start..index], CodeTokenKind::Number);
            continue;
        }

        if is_ident_start(ch) {
            let start = index;
            index += 1;
            while index < bytes.len() && is_ident_continue(bytes[index] as char) {
                index += 1;
            }

            let ident = &source[start..index];
            if index < bytes.len() && bytes[index] as char == '!' {
                index += 1;
                push_token(&mut tokens, &source[start..index], CodeTokenKind::Macro);
                continue;
            }

            let kind = if is_rust_keyword(ident) {
                CodeTokenKind::Keyword
            } else if is_rust_type(ident) {
                CodeTokenKind::TypeName
            } else {
                CodeTokenKind::Plain
            };
            push_token(&mut tokens, ident, kind);
            continue;
        }

        let start = index;
        index += 1;
        push_token(&mut tokens, &source[start..index], CodeTokenKind::Plain);
    }

    tokens
}

fn push_token<'a>(tokens: &mut Vec<CodeToken<'a>>, value: &'a str, kind: CodeTokenKind) {
    if value.is_empty() {
        return;
    }
    tokens.push(CodeToken { value, kind });
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_ident_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

fn is_rust_keyword(ident: &str) -> bool {
    matches!(
        ident,
        "as"
            | "async"
            | "await"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
    )
}

fn is_rust_type(ident: &str) -> bool {
    matches!(
        ident,
        "String"
            | "Result"
            | "Option"
            | "Vec"
            | "Box"
            | "Arc"
            | "HashMap"
            | "HashSet"
            | "usize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "bool"
            | "str"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_label_and_language_chip() {
        let markup = CodeBlock::builder()
            .code(Text::from("fn main() {}"))
            .label(Text::from("Transition API"))
            .build()
            .render()
            .into_string();

        assert!(markup.contains("data-code-block"));
        assert!(markup.contains("Rust"));
        assert!(markup.contains("Transition API"));
    }

    #[test]
    fn highlights_keywords_comments_and_macros() {
        let markup = CodeBlock::builder()
            .code(Text::from("pub fn main() {\n    println!(\"hi\"); // comment\n}"))
            .build()
            .render()
            .into_string();

        assert!(markup.contains("ui-code-block__token--keyword\">pub</span>"));
        assert!(markup.contains("ui-code-block__token--macro\">println!</span>"));
        assert!(markup.contains("ui-code-block__token--comment\">// comment</span>"));
    }
}
