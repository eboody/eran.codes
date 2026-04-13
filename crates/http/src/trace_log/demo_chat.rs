use strum_macros::{AsRefStr, EnumString};

use crate::paths::Route;
use crate::types::Text;

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

pub(crate) fn short_hyphenated_text(value: &Text) -> Text {
    let value = value.to_string();
    let short = value
        .split('-')
        .next()
        .unwrap_or(value.as_str())
        .to_string();
    Text::from(short)
}
