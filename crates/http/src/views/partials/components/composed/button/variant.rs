#[derive(Clone, Debug, Default)]
pub enum Variant {
    #[default]
    Primary,
    Secondary,
}

impl Variant {
    pub(super) fn class_name(&self) -> &'static str {
        match self {
            Self::Primary => "button",
            Self::Secondary => "button secondary",
        }
    }
}
