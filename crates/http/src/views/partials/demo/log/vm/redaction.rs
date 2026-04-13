use crate::views::partials::components;

pub(super) fn authenticated_user_pill() -> components::Pill {
    components::Pill::fields("user=authenticated (redacted)")
}

pub(super) fn redacted_bind_pill(index: usize) -> components::Pill {
    components::Pill::fields(format!("${index}=(redacted)"))
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
