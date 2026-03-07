#[derive(Clone, Copy, Debug, Default)]
pub enum CtaTone {
    #[default]
    Primary,
    Secondary,
}

impl CtaTone {
    pub(super) fn class_name(self) -> &'static str {
        match self {
            Self::Primary => "button",
            Self::Secondary => "button secondary",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum CtaButtonType {
    #[default]
    Button,
    Submit,
}

impl CtaButtonType {
    pub(super) fn as_attr(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
        }
    }
}
