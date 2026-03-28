use std::str::FromStr;

use bon::Builder;
use maud::Render;
use strum_macros::{Display, EnumString};

use crate::types::Text;

const STYLES: &str = r#"
.ui-pill {
  --pill-padding-block: 0.18rem;
  --pill-padding-inline: 0.55rem;
  --pill-padding-inline-narrow: 0.42rem;
  --pill-tone: var(--ui-text-muted);
  --pill-border: color-mix(in srgb, var(--pill-tone) 36%, transparent);
  --pill-color: var(--pill-tone);
  --pill-background: color-mix(in srgb, var(--surface-field) 88%, transparent);
  display: inline-flex;
  align-items: center;
  min-width: 0;
  max-inline-size: 100%;
  padding: var(--pill-padding-block) var(--pill-padding-inline);
  border-radius: var(--radius-pill);
  border: 1px solid var(--pill-border);
  background: var(--pill-background);
  font-size: var(--text-size-label-xs);
  font-weight: 600;
  letter-spacing: var(--text-track-label);
  line-height: var(--text-line-control);
  color: var(--pill-color);
}

.ui-pill--method {
  text-transform: uppercase;
}

.ui-pill--path {
  font-family: var(--ui-font-mono);
  font-size: var(--text-size-label-2xs);
  letter-spacing: var(--text-track-label);
}

.ui-pill--method-get {
  --pill-tone: color-mix(in srgb, var(--accent-signal) 90%, var(--text-body));
}

.ui-pill--method-post {
  --pill-tone: color-mix(in srgb, var(--status-success) 92%, var(--text-body));
}

.ui-pill--method-put,
.ui-pill--method-patch {
  --pill-tone: color-mix(in srgb, var(--status-warn) 92%, var(--text-body));
}

.ui-pill--method-delete {
  --pill-tone: color-mix(in srgb, var(--status-danger) 92%, var(--text-body));
}

.ui-pill--method-other {
  --pill-tone: color-mix(in srgb, var(--text-muted) 88%, var(--text-strong));
}

.ui-pill--status {
  font-variant-numeric: tabular-nums;
}

.ui-pill--status-2xx {
  --pill-tone: color-mix(in srgb, var(--status-success) 90%, var(--text-body));
}

.ui-pill--status-3xx {
  --pill-tone: color-mix(in srgb, var(--accent-signal) 90%, var(--text-body));
}

.ui-pill--status-4xx {
  --pill-tone: color-mix(in srgb, var(--status-warn) 92%, var(--text-body));
}

.ui-pill--status-5xx {
  --pill-tone: color-mix(in srgb, var(--status-danger) 92%, var(--text-body));
}

.ui-pill--status-unknown {
  --pill-tone: color-mix(in srgb, var(--text-muted) 88%, var(--text-strong));
}

.ui-pill--log-level-info {
  --pill-tone: color-mix(in srgb, var(--accent-signal) 84%, var(--text-body));
}

.ui-pill--log-level-warn {
  --pill-tone: color-mix(in srgb, var(--status-warn) 92%, var(--text-body));
}

.ui-pill--log-level-error {
  --pill-tone: color-mix(in srgb, var(--status-danger) 92%, var(--text-body));
}

.ui-pill--log-level-debug {
  --pill-tone: color-mix(in srgb, var(--status-success) 88%, var(--text-body));
}

.ui-pill--log-level-trace {
  --pill-tone: color-mix(in srgb, var(--text-subtle) 88%, var(--text-strong));
}

.ui-pill--log-target {
  --pill-background: color-mix(
    in srgb,
    var(--ui-surface-card) 86%,
    var(--ui-text-muted) 14%
  );
  --pill-border: color-mix(in srgb, var(--ui-text-muted) 30%, transparent);
  --pill-color: var(--ui-text-muted);
}

.ui-pill--log-fields {
  --pill-color: var(--text-subtle);
  --pill-border: var(--border-default);
}

.ui-pill--badge-secondary {
  --pill-background: transparent;
  --pill-color: var(--ui-text);
  --pill-border: var(--ui-border-muted);
}

.ui-pill--badge-you {
  --pill-background: color-mix(in srgb, var(--ui-accent-primary) 90%, var(--surface-field));
  --pill-color: var(--ui-text-on-accent);
  --pill-border: color-mix(in srgb, var(--ui-accent-primary) 82%, transparent);
}

.ui-pill--badge-demo {
  --pill-background: color-mix(in srgb, var(--status-warn) 86%, var(--surface-field));
  --pill-color: color-mix(in srgb, var(--text-strong) 94%, black);
  --pill-border: color-mix(in srgb, var(--status-warn) 80%, transparent);
}

@media (max-width: 768px) {
  .ui-pill--path {
    max-width: 100%;
    overflow-wrap: anywhere;
  }
}

@media (max-width: 20rem) {
  .ui-pill--log-fields,
  .ui-pill--log-target,
  .ui-pill--path {
    max-inline-size: 100%;
    align-items: flex-start;
    padding-inline: var(--pill-padding-inline-narrow);
    font-size: var(--text-size-label-2xs);
    line-height: var(--text-line-reading);
    text-align: left;
    white-space: normal;
    overflow-wrap: anywhere;
    word-break: break-word;
  }
}
"#;

pub(crate) fn head_styles() -> maud::Markup {
    crate::views::scoped::style(STYLES)
}

#[derive(Clone, Copy, Debug, Display, EnumString)]
pub enum MethodKind {
    #[strum(serialize = "GET")]
    Get,
    #[strum(serialize = "POST")]
    Post,
    #[strum(serialize = "PUT")]
    Put,
    #[strum(serialize = "PATCH")]
    Patch,
    #[strum(serialize = "DELETE")]
    Delete,
    Other,
}

impl MethodKind {
    fn classes(self) -> Vec<&'static str> {
        match self {
            Self::Get => vec!["ui-pill--method", "ui-pill--method-get"],
            Self::Post => vec!["ui-pill--method", "ui-pill--method-post"],
            Self::Put => vec!["ui-pill--method", "ui-pill--method-put"],
            Self::Patch => vec!["ui-pill--method", "ui-pill--method-patch"],
            Self::Delete => vec!["ui-pill--method", "ui-pill--method-delete"],
            Self::Other => vec!["ui-pill--method", "ui-pill--method-other"],
        }
    }
}

impl TryFrom<&Text> for MethodKind {
    type Error = strum::ParseError;

    fn try_from(value: &Text) -> Result<Self, strum::ParseError> {
        Self::from_str(&value.to_string())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StatusKind {
    S2xx,
    S3xx,
    S4xx,
    S5xx,
    Unknown,
}

impl StatusKind {
    fn classes(self) -> Vec<&'static str> {
        match self {
            Self::S2xx => vec!["ui-pill--status", "ui-pill--status-2xx"],
            Self::S3xx => vec!["ui-pill--status", "ui-pill--status-3xx"],
            Self::S4xx => vec!["ui-pill--status", "ui-pill--status-4xx"],
            Self::S5xx => vec!["ui-pill--status", "ui-pill--status-5xx"],
            Self::Unknown => vec!["ui-pill--status", "ui-pill--status-unknown"],
        }
    }
}

impl From<&str> for StatusKind {
    fn from(value: &str) -> Self {
        if let Ok(code) = value.parse::<u16>() {
            if code >= 500 {
                return Self::S5xx;
            }
            if code >= 400 {
                return Self::S4xx;
            }
            if code >= 300 {
                return Self::S3xx;
            }
            if code >= 200 {
                return Self::S2xx;
            }
        }
        Self::Unknown
    }
}

impl From<&Text> for StatusKind {
    fn from(value: &Text) -> Self {
        let value = value.to_string();
        Self::from(value.as_str())
    }
}

#[derive(Clone, Copy, Debug, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum LevelKind {
    #[strum(serialize = "info")]
    Info,
    #[strum(serialize = "warn", serialize = "warning")]
    Warn,
    #[strum(serialize = "error")]
    Error,
    #[strum(serialize = "debug")]
    Debug,
    #[strum(serialize = "trace")]
    Trace,
}

impl LevelKind {
    fn classes(self) -> Vec<&'static str> {
        match self {
            Self::Info => vec!["ui-pill--log-level-info"],
            Self::Warn => vec!["ui-pill--log-level-warn"],
            Self::Error => vec!["ui-pill--log-level-error"],
            Self::Debug => vec!["ui-pill--log-level-debug"],
            Self::Trace => vec!["ui-pill--log-level-trace"],
        }
    }
}

impl TryFrom<&Text> for LevelKind {
    type Error = strum::ParseError;

    fn try_from(value: &Text) -> Result<Self, strum::ParseError> {
        Self::from_str(&value.to_string())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BadgeKind {
    You,
    Demo,
    Secondary,
}

impl BadgeKind {
    fn classes(self) -> Vec<&'static str> {
        match self {
            Self::You => vec!["ui-pill--badge-you"],
            Self::Demo => vec!["ui-pill--badge-demo"],
            Self::Secondary => vec!["ui-pill--badge-secondary"],
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Variant {
    #[default]
    Plain,
    Method(MethodKind),
    Status(StatusKind),
    Level(LevelKind),
    Path,
    Target,
    Fields,
    Badge(BadgeKind),
}

impl Variant {
    fn classes(self) -> Vec<&'static str> {
        match self {
            Self::Plain => Vec::new(),
            Self::Method(kind) => kind.classes(),
            Self::Status(kind) => kind.classes(),
            Self::Level(kind) => kind.classes(),
            Self::Path => vec!["ui-pill--path"],
            Self::Target => vec!["ui-pill--log-target"],
            Self::Fields => vec!["ui-pill--log-fields"],
            Self::Badge(kind) => kind.classes(),
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct Pill {
    pub text: Text,
    #[builder(default)]
    pub variant: Variant,
}

impl Pill {
    pub fn level(text: impl Into<Text>) -> Self {
        let text = text.into();
        let kind = LevelKind::try_from(&text).unwrap_or(LevelKind::Info);
        Self {
            text,
            variant: Variant::Level(kind),
        }
    }

    pub fn method(text: impl Into<Text>) -> Self {
        let text = text.into();
        let kind = MethodKind::try_from(&text).unwrap_or(MethodKind::Other);
        Self {
            text,
            variant: Variant::Method(kind),
        }
    }

    pub fn status(text: impl Into<Text>) -> Self {
        let text = text.into();
        let kind = StatusKind::from(&text);
        Self {
            text,
            variant: Variant::Status(kind),
        }
    }

    pub fn path(text: impl Into<Text>) -> Self {
        let text = text.into();
        Self {
            text,
            variant: Variant::Path,
        }
    }

    pub fn target(text: impl Into<Text>) -> Self {
        let text = text.into();
        Self {
            text,
            variant: Variant::Target,
        }
    }

    pub fn fields(text: impl Into<Text>) -> Self {
        let text = text.into();
        Self {
            text,
            variant: Variant::Fields,
        }
    }

    pub fn badge(text: impl Into<Text>, kind: BadgeKind) -> Self {
        let text = text.into();
        Self {
            text,
            variant: Variant::Badge(kind),
        }
    }
}

impl Render for Pill {
    fn render(&self) -> maud::Markup {
        let mut classes = vec!["ui-pill"];
        classes.extend(self.variant.classes());

        maud::html! {
            span class=(classes.join(" ")) { (&self.text) }
        }
    }
}
