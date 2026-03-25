use crate::types::Text;

#[derive(Clone, Debug)]
pub enum Data {
    Flag(Text),
    Value { name: Text, value: Text },
}

impl Data {
    pub fn flag(name: impl Into<Text>) -> Self {
        let name = name.into();
        debug_assert!(name.to_string().starts_with("data-"));
        Self::Flag(name)
    }

    pub fn value(name: impl Into<Text>, value: impl Into<Text>) -> Self {
        let name = name.into();
        debug_assert!(name.to_string().starts_with("data-"));
        Self::Value {
            name,
            value: value.into(),
        }
    }
}
