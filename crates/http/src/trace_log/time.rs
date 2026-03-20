use crate::types::TimestampText;

pub fn now_timestamp_short() -> TimestampText {
    TimestampText::new(jiff::Timestamp::now().strftime("%F %T").to_string())
}
