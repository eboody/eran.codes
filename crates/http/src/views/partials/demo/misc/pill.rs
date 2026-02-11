use std::str::FromStr;

use bon::Builder;
use maud::Render;
use strum_macros::{Display, EnumString};

use crate::types::Text;

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
    pub fn from_text(value: &Text) -> Self {
        MethodKind::from_str(&value.to_string()).unwrap_or(Self::Other)
    }

    fn class(self) -> &'static str {
        match self {
            MethodKind::Get => "method-get",
            MethodKind::Post => "method-post",
            MethodKind::Put => "method-put",
            MethodKind::Patch => "method-patch",
            MethodKind::Delete => "method-delete",
            MethodKind::Other => "method-other",
        }
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
    pub fn from_str(value: &str) -> Self {
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

    fn class(self) -> &'static str {
        match self {
            StatusKind::S2xx => "status-2xx",
            StatusKind::S3xx => "status-3xx",
            StatusKind::S4xx => "status-4xx",
            StatusKind::S5xx => "status-5xx",
            StatusKind::Unknown => "status-unknown",
        }
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
    pub fn from_text(value: &Text) -> Self {
        LevelKind::from_str(&value.to_string()).unwrap_or(Self::Info)
    }

    fn class(self) -> &'static str {
        match self {
            LevelKind::Info => "log-level-info",
            LevelKind::Warn => "log-level-warn",
            LevelKind::Error => "log-level-error",
            LevelKind::Debug => "log-level-debug",
            LevelKind::Trace => "log-level-trace",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BadgeKind {
    You,
    Demo,
    Secondary,
}

impl BadgeKind {
    fn class(self) -> &'static str {
        match self {
            BadgeKind::You => "badge-you",
            BadgeKind::Demo => "badge-demo",
            BadgeKind::Secondary => "badge-secondary",
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum PillColor {
    Slate,
    Emerald,
    Amber,
    Rose,
    Sky,
}

impl PillColor {
    pub fn to_rgb(self) -> &'static str {
        match self {
            PillColor::Slate => "rgb(148, 163, 184)",
            PillColor::Emerald => "rgb(52, 211, 153)",
            PillColor::Amber => "rgb(251, 191, 36)",
            PillColor::Rose => "rgb(251, 113, 133)",
            PillColor::Sky => "rgb(56, 189, 248)",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PillVariant {
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
    fn class(self) -> Option<&'static str> {
        match self {
            PillVariant::Plain => None,
            PillVariant::Method(kind) => Some(kind.class()),
            PillVariant::Status(kind) => Some(kind.class()),
            PillVariant::Level(kind) => Some(kind.class()),
            PillVariant::Path => Some("path"),
            PillVariant::Target => Some("log-target"),
            PillVariant::Fields => Some("log-fields"),
            PillVariant::Badge(kind) => Some(kind.class()),
        }
    }
}

impl Default for PillVariant {
    fn default() -> Self {
        PillVariant::Plain
    }
}

#[derive(Clone, Debug, Builder)]
pub struct Pill {
    pub text: Text,
    #[builder(default)]
    pub variant: PillVariant,
    pub color: Option<PillColor>,
}

impl Pill {
    pub fn level(text: impl Into<Text>) -> Self {
        let text = text.into();
        let kind = LevelKind::from_text(&text);
        Self {
            text,
            variant: PillVariant::Level(kind),
            color: None,
        }
    }

    pub fn method(text: impl Into<Text>) -> Self {
        let text = text.into();
        let kind = MethodKind::from_text(&text);
        Self {
            text,
            variant: PillVariant::Method(kind),
            color: None,
        }
    }

    pub fn status(text: impl Into<Text>) -> Self {
        let text = text.into();
        let kind = StatusKind::from_str(&text.to_string());
        Self {
            text,
            variant: PillVariant::Status(kind),
            color: None,
        }
    }

    pub fn path(text: impl Into<Text>) -> Self {
        let text = text.into();
        Self {
            text,
            variant: PillVariant::Path,
            color: None,
        }
    }

    pub fn target(text: impl Into<Text>) -> Self {
        let text = text.into();
        Self {
            text,
            variant: PillVariant::Target,
            color: None,
        }
    }

    pub fn fields(text: impl Into<Text>) -> Self {
        let text = text.into();
        Self {
            text,
            variant: PillVariant::Fields,
            color: Some(PillColor::Slate),
        }
    }

    pub fn badge(text: impl Into<Text>, kind: BadgeKind) -> Self {
        let text = text.into();
        Self {
            text,
            variant: PillVariant::Badge(kind),
            color: None,
        }
    }
}

impl Render for Pill {
    fn render(&self) -> maud::Markup {
        let class = match self.variant.class() {
            Some(variant) => format!("pill {}", variant),
            None => "pill".to_string(),
        };
        let style = self
            .color
            .map(|color| format!("--pill-accent: {};", color.to_rgb()));
        maud::html! {
            span class=(class) style=[style] { (&self.text) }
        }
    }
}
