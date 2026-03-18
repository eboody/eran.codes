use crate::types::TimestampText;

pub fn now_timestamp_short() -> TimestampText {
    let raw = jiff::Timestamp::now().to_string();
    format_timestamp(TimestampText::new(raw))
}

fn format_timestamp(raw: TimestampText) -> TimestampText {
    let raw_value = raw.to_string();
    let mut parts = raw_value.split('T');
    let Some(date) = parts.next() else {
        return raw;
    };
    let Some(time) = parts.next() else {
        return raw;
    };
    let time = time.trim_end_matches('Z');
    let time = time.split('.').next().unwrap_or(time);
    TimestampText::new(format!("{date} {time}"))
}
