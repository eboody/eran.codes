use std::fmt;

use nutype::nutype;
use serde::{Deserialize, Serialize};

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize)
)]
pub struct Text(String);

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Text::new(value)
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text::new(value)
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize)
)]
pub struct RequestId(String);

impl From<&str> for RequestId {
    fn from(value: &str) -> Self {
        RequestId::new(value)
    }
}

impl RequestId {
    pub fn unknown() -> Self {
        RequestId::new("unknown")
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize)
)]
pub struct SessionId(String);

impl From<&str> for SessionId {
    fn from(value: &str) -> Self {
        SessionId::new(value)
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize)
)]
pub struct UserIdText(String);

impl From<&str> for UserIdText {
    fn from(value: &str) -> Self {
        UserIdText::new(value)
    }
}

impl From<String> for UserIdText {
    fn from(value: String) -> Self {
        UserIdText::new(value)
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize)
)]
pub struct ClientIp(String);

impl From<&str> for ClientIp {
    fn from(value: &str) -> Self {
        ClientIp::new(value)
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize)
)]
pub struct UserAgent(String);

impl From<&str> for UserAgent {
    fn from(value: &str) -> Self {
        UserAgent::new(value)
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize)
)]
pub struct RoutePath(String);

impl From<&str> for RoutePath {
    fn from(value: &str) -> Self {
        RoutePath::new(value)
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize)
)]
pub struct TimestampText(String);

impl From<&str> for TimestampText {
    fn from(value: &str) -> Self {
        TimestampText::new(value)
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize)
)]
pub struct LogLevelText(String);

impl From<&str> for LogLevelText {
    fn from(value: &str) -> Self {
        LogLevelText::new(value)
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize)
)]
pub struct LogTargetText(String);

impl From<&str> for LogTargetText {
    fn from(value: &str) -> Self {
        LogTargetText::new(value)
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize)
)]
pub struct LogMessageText(String);

impl From<&str> for LogMessageText {
    fn from(value: &str) -> Self {
        LogMessageText::new(value)
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize)
)]
pub struct LogFieldName(String);

impl From<&str> for LogFieldName {
    fn from(value: &str) -> Self {
        LogFieldName::new(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogFieldValue {
    Missing,
    Text(Text),
}

impl LogFieldValue {
    pub fn new(value: impl Into<Text>) -> Self {
        Self::Text(value.into())
    }

    pub fn missing() -> Self {
        Self::Missing
    }

    pub fn as_text(&self) -> Option<&Text> {
        match self {
            Self::Missing => None,
            Self::Text(value) => Some(value),
        }
    }
}

impl fmt::Display for LogFieldValue {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Missing => write!(f, "-"),
            Self::Text(value) => write!(f, "{value}"),
        }
    }
}

impl From<&str> for LogFieldValue {
    fn from(value: &str) -> Self {
        LogFieldValue::new(value)
    }
}
