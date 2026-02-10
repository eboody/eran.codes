#[derive(Clone, Debug)]
pub enum FieldValue {
    Missing,
    Value(String),
}

impl FieldValue {
    pub fn from_str(value: &str) -> Self {
        match value {
            "-" => Self::Missing,
            _ => Self::Value(value.to_string()),
        }
    }

    pub fn into_option(self) -> Option<String> {
        match self {
            Self::Missing => None,
            Self::Value(value) => Some(value),
        }
    }
}
