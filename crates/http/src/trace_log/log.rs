use std::str::FromStr;

use crate::types::{LogMessageText, LogTargetText};
use strum_macros::{Display, EnumString};

pub mod target {
    use super::{Display, EnumString, FromStr, LogTargetText};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
    pub enum Known {
        #[strum(serialize = "demo.request")]
        DemoRequest,
        #[strum(serialize = "demo.request.diagnostic")]
        DemoRequestDiagnostic,
        #[strum(serialize = "demo.db")]
        DemoDb,
        #[strum(serialize = "demo.sse")]
        DemoSse,
        #[strum(serialize = "demo.chat")]
        DemoChat,
        #[strum(serialize = "http::router::layers")]
        HttpRouterLayers,
    }

    impl Known {
        pub const fn as_str(self) -> &'static str {
            match self {
                Self::DemoRequest => "demo.request",
                Self::DemoRequestDiagnostic => "demo.request.diagnostic",
                Self::DemoDb => "demo.db",
                Self::DemoSse => "demo.sse",
                Self::DemoChat => "demo.chat",
                Self::HttpRouterLayers => "http::router::layers",
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum Kind {
        Known(Known),
        Other(LogTargetText),
    }

    impl Kind {
        pub fn parse(value: &str) -> Self {
            Known::from_str(value)
                .map(Self::Known)
                .unwrap_or_else(|_| Self::Other(LogTargetText::new(value)))
        }

        pub fn is_demo(&self) -> bool {
            matches!(
                self,
                Self::Known(Known::DemoRequest)
                    | Self::Known(Known::DemoDb)
                    | Self::Known(Known::DemoSse)
                    | Self::Known(Known::DemoChat)
            )
        }

        pub fn is_diagnostic(&self) -> bool {
            matches!(self, Self::Known(Known::DemoRequestDiagnostic))
        }

        pub fn is_demo_sse(&self) -> bool {
            matches!(self, Self::Known(Known::DemoSse))
        }
    }

    impl From<Known> for LogTargetText {
        fn from(value: Known) -> Self {
            LogTargetText::new(value.as_str())
        }
    }
}

pub mod message {
    use super::{Display, EnumString, FromStr, LogMessageText};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
    pub enum Known {
        #[strum(serialize = "request.start")]
        RequestStart,
        #[strum(serialize = "request.end")]
        RequestEnd,
        #[strum(serialize = "request completed")]
        RequestCompleted,
        #[strum(serialize = "\"db query\"", serialize = "db query")]
        DbQuery,
        #[strum(serialize = "\"db query complete\"", serialize = "db query complete")]
        DbQueryComplete,
        #[strum(serialize = "chat.message.incoming")]
        ChatMessageIncoming,
        #[strum(serialize = "chat message broadcast")]
        ChatMessageBroadcast,
    }

    impl Known {
        pub const fn as_str(self) -> &'static str {
            match self {
                Self::RequestStart => "request.start",
                Self::RequestEnd => "request.end",
                Self::RequestCompleted => "request completed",
                Self::DbQuery => "db query",
                Self::DbQueryComplete => "db query complete",
                Self::ChatMessageIncoming => "chat.message.incoming",
                Self::ChatMessageBroadcast => "chat message broadcast",
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum Kind {
        Known(Known),
        Other(LogMessageText),
    }

    impl Kind {
        pub fn parse(value: &str) -> Self {
            Known::from_str(value)
                .map(Self::Known)
                .unwrap_or_else(|_| Self::Other(LogMessageText::new(value)))
        }
    }

    impl From<Known> for LogMessageText {
        fn from(value: Known) -> Self {
            LogMessageText::new(value.as_str())
        }
    }
}

pub(crate) fn classify(target: &str, message: &str) -> (target::Kind, message::Kind) {
    (target::Kind::parse(target), message::Kind::parse(message))
}

pub(crate) fn should_skip_event(target: &target::Kind, message: &message::Kind) -> bool {
    matches!(
        (target, message),
        (
            target::Kind::Known(target::Known::HttpRouterLayers),
            message::Kind::Known(message::Known::RequestCompleted)
        )
    ) || matches!(
        (target, message),
        (
            target::Kind::Other(target),
            message::Kind::Other(message)
        ) if target.to_string() == "http::handlers::sse"
            && matches!(message.to_string().as_str(), "sse connected" | "sse disconnected")
    )
}

#[cfg(test)]
mod tests {
    use super::{message, should_skip_event, target};

    #[test]
    fn skip_rules_reject_sse_connection_lifecycle_events() {
        assert!(should_skip_event(
            &target::Kind::parse("http::handlers::sse"),
            &message::Kind::parse("sse connected")
        ));
        assert!(should_skip_event(
            &target::Kind::parse("http::handlers::sse"),
            &message::Kind::parse("sse disconnected")
        ));
        assert!(!should_skip_event(
            &target::Kind::parse("http::handlers::sse"),
            &message::Kind::parse("other event")
        ));
    }
}
