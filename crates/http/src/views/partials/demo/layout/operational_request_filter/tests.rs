use super::*;

#[test]
fn requests_only_filter_query_and_sse_tab_id() {
    let markup = OperationalRequestFilter::builder()
        .target_id("network-log-target")
        .build()
        .render()
        .into_string();

    let request_action = request_action();

    assert!(markup.contains(request_action));
    assert!(markup.contains("$operations_filter_query = '';"));
    assert_eq!(markup.matches(request_action).count(), 3);
    assert!(markup.contains("/static/operational-timeline-scroll.js"));
}
