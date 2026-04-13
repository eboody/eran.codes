use crate::types::Text;
use crate::views::partials::components;

pub(super) fn burst_metrics(
    endpoint: Text,
    concurrency: usize,
) -> Vec<components::KeyValueItem> {
    vec![
        burst_metric("Endpoint", "data-burst-endpoint", endpoint),
        burst_metric("Workers", "data-burst-workers", concurrency.to_string()),
        burst_metric("Throughput", "data-burst-rate", "—"),
        burst_metric("Duration", "data-burst-duration", "—"),
        burst_metric("OK", "data-burst-ok", "—"),
        burst_metric("Failed", "data-burst-failed", "—"),
        burst_metric("Latency p50", "data-burst-p50", "—"),
        burst_metric("Latency p95", "data-burst-p95", "—"),
        burst_metric("Latency p99", "data-burst-p99", "—"),
        burst_metric("Baseline", "data-burst-previous", "Run once to set a baseline."),
        burst_metric("Delta", "data-burst-delta", "—"),
    ]
}

fn burst_metric(
    label: impl Into<Text>,
    value_attr: impl Into<Text>,
    value: impl Into<Text>,
) -> components::KeyValueItem {
    components::KeyValueItem::builder()
        .label(label.into())
        .value(value.into())
        .value_attrs(vec![components::KeyValueValueAttr::flag(value_attr)])
        .build()
}
