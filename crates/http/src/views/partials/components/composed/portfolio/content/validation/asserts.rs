use super::shared::*;

pub(super) fn assert_non_empty(path: &str, value: &Text) {
    assert!(
        !value.to_string().trim().is_empty(),
        "{path} must not be empty",
    );
}

pub(super) fn assert_min_len<T>(path: &str, values: &[T], min_len: usize) {
    assert!(
        values.len() >= min_len,
        "{path} must contain at least {min_len} entries",
    );
}

pub(super) fn assert_unique_keys(path: &str, values: impl IntoIterator<Item = String>) {
    let mut seen = HashSet::new();
    for value in values {
        assert!(seen.insert(value), "{path} must be unique");
    }
}

pub(super) fn assert_unique_text_ids<'a>(
    path: &str,
    values: impl IntoIterator<Item = &'a Text>,
) {
    assert_unique_keys(path, values.into_iter().map(ToString::to_string));
}
