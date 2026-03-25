use crate::trace_log::demo_chat::Sender as ChatSender;
use crate::types::Text;
use crate::views::partials::components;

pub(super) fn authenticated_user_pill() -> components::Pill {
    components::Pill::fields("user=authenticated (redacted)")
}

pub(super) fn redacted_bind_pill(index: usize) -> components::Pill {
    components::Pill::fields(format!("${index}=(redacted)"))
}

pub(super) fn chat_user_pill(
    sender: ChatSender,
    user_id: Option<&Text>,
) -> components::Pill {
    if user_id.is_none() {
        return components::Pill::fields("user:unknown");
    }

    let (label, kind) = match sender {
        ChatSender::You => (Text::from("You (redacted)"), components::BadgeKind::You),
        ChatSender::Demo => (Text::from("Demo (redacted)"), components::BadgeKind::Demo),
        ChatSender::Unknown => (
            Text::from("User (redacted)"),
            components::BadgeKind::Secondary,
        ),
    };
    components::Pill::badge(label, kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind_values_are_marked_redacted() {
        assert_eq!(
            redacted_bind_pill(2).text.to_string(),
            "$2=(redacted)"
        );
    }

    #[test]
    fn authenticated_user_pill_never_contains_raw_id() {
        assert_eq!(
            authenticated_user_pill().text.to_string(),
            "user=authenticated (redacted)"
        );
    }
}
