use std::str::FromStr;

use strum_macros::{AsRefStr, EnumString};

use crate::paths::Route;
use crate::trace_log::store;
use crate::types::{LogFieldKey, Text};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, AsRefStr, EnumString)]
pub(crate) enum Sender {
    #[strum(serialize = "you")]
    You,
    #[strum(serialize = "demo")]
    Demo,
    #[default]
    #[strum(serialize = "-")]
    Unknown,
}

impl Sender {
    pub fn from_entry(entry: &store::TraceEntry) -> Self {
        let Some(sender) = entry.field_text(LogFieldKey::Sender) else {
            return Self::Unknown;
        };
        Self::from_str(&sender.to_string()).unwrap_or_default()
    }
}

impl TryFrom<Route> for Sender {
    type Error = ();

    fn try_from(value: Route) -> core::result::Result<Self, Self::Error> {
        match value {
            Route::ChatMessages => Ok(Self::You),
            Route::ChatMessagesDemo => Ok(Self::Demo),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumString)]
pub(crate) enum Direction {
    #[strum(serialize = "incoming")]
    Incoming,
    #[strum(serialize = "outgoing")]
    Outgoing,
    #[default]
    #[strum(disabled)]
    Unknown,
}

impl Direction {
    pub fn from_entry(entry: &store::TraceEntry) -> Self {
        let Some(direction) = entry.field_text(LogFieldKey::Direction) else {
            return Self::Unknown;
        };
        Self::from_str(&direction.to_string()).unwrap_or_default()
    }
}

pub(crate) fn short_hyphenated_text(value: &Text) -> Text {
    let value = value.to_string();
    let short = value
        .split('-')
        .next()
        .unwrap_or(value.as_str())
        .to_string();
    Text::from(short)
}
