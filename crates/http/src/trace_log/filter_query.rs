use crate::types::Text;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FlowFilterTerms(Vec<Text>);

impl FlowFilterTerms {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_slice(&self) -> &[Text] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<Text> {
        self.0
    }
}

impl From<&str> for FlowFilterTerms {
    fn from(query: &str) -> Self {
        Self(
            query
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| Text::from(value.to_lowercase()))
                .collect(),
        )
    }
}

impl From<&Text> for FlowFilterTerms {
    fn from(value: &Text) -> Self {
        Self::from(value.to_string().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::FlowFilterTerms;

    #[test]
    fn from_str_trims_commas_and_normalizes_case() {
        let terms = FlowFilterTerms::from(" /events,  POST , , /HEALTH ");
        let values: Vec<String> = terms
            .into_vec()
            .into_iter()
            .map(|value| value.to_string())
            .collect();
        assert_eq!(values, vec!["/events", "post", "/health"]);
    }
}
