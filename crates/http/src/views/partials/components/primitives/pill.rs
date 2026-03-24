use std::str::FromStr;

use bon::Builder;
use maud::Render;
use strum_macros::{Display, EnumString};

use crate::types::Text;

const STYLES: &str = r#"
.ui-pill {
  --pill-padding-block: 0.18rem;
  --pill-padding-inline: 0.55rem;
  display: inline-flex;
  align-items: center;
  padding: var(--pill-padding-block) var(--pill-padding-inline);
  border-radius: var(--radius-pill);
  border: 1px solid var(--pill-accent, var(--ui-border-muted));
  background: color-mix(in srgb, var(--surface-field) 88%, transparent);
  font-size: var(--text-size-label-xs);
  font-weight: 600;
  letter-spacing: var(--text-track-label);
  line-height: var(--text-line-control);
  color: var(--pill-accent, var(--text-muted));
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
  border-color: rgba(120, 190, 255, 0.6);
  color: rgba(140, 210, 255, 0.95);
}

.ui-pill--method-post {
  border-color: rgba(140, 210, 140, 0.7);
  color: rgba(160, 220, 160, 0.95);
}

.ui-pill--method-put,
.ui-pill--method-patch {
  border-color: rgba(255, 196, 80, 0.7);
  color: rgba(255, 196, 80, 0.95);
}

.ui-pill--method-delete {
  border-color: rgba(255, 120, 120, 0.75);
  color: rgba(255, 140, 140, 0.95);
}

.ui-pill--method-other {
  border-color: rgba(180, 180, 200, 0.6);
  color: rgba(200, 200, 220, 0.9);
}

.ui-pill--status {
  font-variant-numeric: tabular-nums;
}

.ui-pill--status-2xx {
  border-color: rgba(120, 210, 140, 0.7);
  color: rgba(150, 220, 160, 0.95);
}

.ui-pill--status-3xx {
  border-color: rgba(120, 190, 255, 0.6);
  color: rgba(140, 210, 255, 0.95);
}

.ui-pill--status-4xx {
  border-color: rgba(255, 196, 80, 0.7);
  color: rgba(255, 196, 80, 0.95);
}

.ui-pill--status-5xx {
  border-color: rgba(255, 120, 120, 0.75);
  color: rgba(255, 140, 140, 0.95);
}

.ui-pill--status-unknown {
  border-color: rgba(180, 180, 200, 0.6);
  color: rgba(200, 200, 220, 0.9);
}

.ui-pill--log-level-info {
  border-color: rgba(80, 160, 255, 0.6);
  color: rgba(120, 190, 255, 0.9);
}

.ui-pill--log-level-warn {
  border-color: rgba(255, 196, 80, 0.7);
  color: rgba(255, 196, 80, 0.95);
}

.ui-pill--log-level-error {
  border-color: rgba(255, 96, 96, 0.7);
  color: rgba(255, 128, 128, 0.95);
}

.ui-pill--log-level-debug {
  border-color: rgba(140, 210, 140, 0.65);
  color: rgba(160, 220, 160, 0.9);
}

.ui-pill--log-level-trace {
  border-color: rgba(160, 160, 180, 0.6);
  color: rgba(180, 180, 200, 0.85);
}

.ui-pill--log-target {
  background: color-mix(
    in srgb,
    var(--ui-surface-card) 86%,
    var(--ui-text-muted) 14%
  );
  border-color: color-mix(in srgb, var(--ui-text-muted) 30%, transparent);
  color: var(--ui-text-muted);
}

.ui-pill--log-fields {
  color: var(--text-subtle);
  border-color: var(--border-default);
}

.ui-pill--badge-secondary {
  background: transparent;
  color: var(--ui-text);
  border: 1px solid var(--ui-border-muted);
}

.ui-pill--badge-you {
  background: #0f766e;
  color: #f8fafc;
  border-color: #0f766e;
}

.ui-pill--badge-demo {
  background: #f59e0b;
  color: #1f2937;
  border-color: #f59e0b;
}

@media (max-width: 768px) {
  .ui-pill--path {
    max-width: 100%;
    overflow-wrap: anywhere;
  }
}

@media (max-width: 20rem) {
  .ui-pill--log-fields,
  .ui-pill--path {
    max-inline-size: 100%;
    align-items: flex-start;
    padding-inline: 0.42rem;
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
pub enum PillVariant {
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

impl PillVariant {
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
    pub variant: PillVariant,
}

impl Pill {
    pub fn level(text: impl Into<Text>) -> Self {
        let text = text.into();
        let kind = LevelKind::try_from(&text).unwrap_or(LevelKind::Info);
        Self {
            text,
            variant: PillVariant::Level(kind),
        }
    }

    pub fn method(text: impl Into<Text>) -> Self {
        let text = text.into();
        let kind = MethodKind::try_from(&text).unwrap_or(MethodKind::Other);
        Self {
            text,
            variant: PillVariant::Method(kind),
        }
    }

    pub fn status(text: impl Into<Text>) -> Self {
        let text = text.into();
        let kind = StatusKind::from(&text);
        Self {
            text,
            variant: PillVariant::Status(kind),
        }
    }

    pub fn path(text: impl Into<Text>) -> Self {
        let text = text.into();
        Self {
            text,
            variant: PillVariant::Path,
        }
    }

    pub fn target(text: impl Into<Text>) -> Self {
        let text = text.into();
        Self {
            text,
            variant: PillVariant::Target,
        }
    }

    pub fn fields(text: impl Into<Text>) -> Self {
        let text = text.into();
        Self {
            text,
            variant: PillVariant::Fields,
        }
    }

    pub fn badge(text: impl Into<Text>, kind: BadgeKind) -> Self {
        let text = text.into();
        Self {
            text,
            variant: PillVariant::Badge(kind),
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
