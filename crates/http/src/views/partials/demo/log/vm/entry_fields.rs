use crate::types::Text;

pub fn short_request_id(value: &Text) -> Text {
    crate::trace_log::demo_chat::short_hyphenated_text(value)
}
