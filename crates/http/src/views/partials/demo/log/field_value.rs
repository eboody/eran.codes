use crate::types::{LogFieldValue, Text};

#[derive(Clone, Debug)]
pub enum FieldValue {
    Missing,
    Value(Text),
}

impl FieldValue {
    pub fn from_log_value(value: Option<&LogFieldValue>) -> Self {
        match value {
            Some(LogFieldValue::Text(text)) => Self::Value(text.clone()),
            _ => Self::Missing,
        }
    }

    pub fn into_option(self) -> Option<Text> {
        match self {
            Self::Missing => None,
            Self::Value(value) => Some(value),
        }
    }
}
