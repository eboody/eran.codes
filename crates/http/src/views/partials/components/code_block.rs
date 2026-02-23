use bon::Builder;
use maud::Render;

use crate::types::Text;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeLanguage {
    Rust,
}

#[derive(Clone, Debug, Builder)]
pub struct CodeBlock {
    pub code: Text,
    pub language: CodeLanguage,
    #[builder(setters(name = with_class))]
    pub class: Option<Text>,
}

impl Render for CodeBlock {
    fn render(&self) -> maud::Markup {
        let class_name = self.class.as_ref().map(Text::to_string).unwrap_or_default();
        let source = self.code.to_string();
        let tokens = tokenize(&source, self.language);

        maud::html! {
            pre class={ "code-block " (class_name) } {
                code class="code-block-content" {
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
            Self::Keyword => "code-token-keyword",
            Self::TypeName => "code-token-type",
            Self::Number => "code-token-number",
            Self::String => "code-token-string",
            Self::Comment => "code-token-comment",
            Self::Macro => "code-token-macro",
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
    let mut i = 0;

    while i < bytes.len() {
        let ch = bytes[i] as char;

        if ch.is_whitespace() {
            let start = i;
            i += 1;
            while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                i += 1;
            }
            push_token(&mut tokens, &source[start..i], CodeTokenKind::Plain);
            continue;
        }

        if ch == '/' && i + 1 < bytes.len() && bytes[i + 1] as char == '/' {
            let start = i;
            i += 2;
            while i < bytes.len() && bytes[i] as char != '\n' {
                i += 1;
            }
            push_token(&mut tokens, &source[start..i], CodeTokenKind::Comment);
            continue;
        }

        if ch == '"' {
            let start = i;
            i += 1;
            while i < bytes.len() {
                let curr = bytes[i] as char;
                if curr == '\\' {
                    i += 2;
                    continue;
                }
                i += 1;
                if curr == '"' {
                    break;
                }
            }
            push_token(&mut tokens, &source[start..i.min(bytes.len())], CodeTokenKind::String);
            continue;
        }

        if ch.is_ascii_digit() {
            let start = i;
            i += 1;
            while i < bytes.len() {
                let curr = bytes[i] as char;
                if curr.is_ascii_alphanumeric() || curr == '_' || curr == '.' {
                    i += 1;
                } else {
                    break;
                }
            }
            push_token(&mut tokens, &source[start..i], CodeTokenKind::Number);
            continue;
        }

        if is_ident_start(ch) {
            let start = i;
            i += 1;
            while i < bytes.len() && is_ident_continue(bytes[i] as char) {
                i += 1;
            }

            let ident = &source[start..i];
            if i < bytes.len() && bytes[i] as char == '!' {
                i += 1;
                push_token(&mut tokens, &source[start..i], CodeTokenKind::Macro);
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

        let start = i;
        i += 1;
        push_token(&mut tokens, &source[start..i], CodeTokenKind::Plain);
    }

    tokens
}

fn push_token<'a>(tokens: &mut Vec<CodeToken<'a>>, value: &'a str, kind: CodeTokenKind) {
    if value.is_empty() {
        return;
    }
    tokens.push(CodeToken {
        value,
        kind,
    });
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
