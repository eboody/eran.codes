pub(super) fn format_capabilities(
    capabilities: &[domain::sensitive::AccessCapability],
) -> String {
    if capabilities.is_empty() {
        return "none".to_string();
    }

    capabilities
        .iter()
        .map(|capability| capability.as_ref().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

pub(super) fn format_key_counts(
    counts: &[domain::sensitive::KeyedCiphertextCount],
) -> String {
    if counts.is_empty() {
        return "none".to_string();
    }

    counts
        .iter()
        .map(|count| format!("{}: {}", count.key_id, count.count))
        .collect::<Vec<_>>()
        .join(", ")
}

pub(super) fn event_actor(event: &domain::sensitive::AccessEvent) -> String {
    event.user_id
        .map(|user_id| user_id.as_ref().to_string())
        .unwrap_or_else(|| "guest".to_string())
}

pub(super) fn format_proof_time(value: std::time::SystemTime) -> String {
    let time = time::OffsetDateTime::from(value);
    let format = time::format_description::parse(
        "[year]-[month]-[day] [hour repr:24 padding:zero]:[minute padding:zero]",
    )
    .unwrap_or_else(|_| Vec::new());
    time.format(&format).unwrap_or_else(|_| "--:--".to_string())
}
