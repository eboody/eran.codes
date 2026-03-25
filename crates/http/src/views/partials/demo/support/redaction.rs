use crate::types::Text;

pub(crate) fn present_redacted(value: &Option<Text>) -> Text {
    redacted_presence(value, "present (redacted)")
}

pub(crate) fn authenticated_redacted(value: &Option<Text>) -> Text {
    redacted_presence(value, "authenticated (redacted)")
}

pub(crate) fn captured_redacted(value: &Option<Text>) -> Text {
    redacted_presence(value, "captured (redacted)")
}

pub(crate) fn viewer_actor_redacted<T>(value: &Option<T>) -> Text {
    if value.is_some() {
        Text::from("authenticated (redacted)")
    } else {
        Text::from("guest")
    }
}

fn redacted_presence(value: &Option<Text>, when_present: &'static str) -> Text {
    if value.is_some() {
        Text::from(when_present)
    } else {
        Text::from("none")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_values_stay_none() {
        assert_eq!(present_redacted(&None), Text::from("none"));
        assert_eq!(authenticated_redacted(&None), Text::from("none"));
        assert_eq!(captured_redacted(&None), Text::from("none"));
        assert_eq!(viewer_actor_redacted::<Text>(&None), Text::from("guest"));
    }

    #[test]
    fn present_values_are_marked_redacted() {
        let value = Some(Text::from("raw"));

        assert_eq!(present_redacted(&value), Text::from("present (redacted)"));
        assert_eq!(
            authenticated_redacted(&value),
            Text::from("authenticated (redacted)")
        );
        assert_eq!(
            captured_redacted(&value),
            Text::from("captured (redacted)")
        );
        assert_eq!(
            viewer_actor_redacted(&value),
            Text::from("authenticated (redacted)")
        );
    }
}
