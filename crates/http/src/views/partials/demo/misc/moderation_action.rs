#[derive(Clone, Copy, Debug)]
pub enum ModerationAction {
    Approve,
    Remove,
}

impl ModerationAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            ModerationAction::Approve => "approve",
            ModerationAction::Remove => "remove",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "approve" => Some(ModerationAction::Approve),
            "remove" => Some(ModerationAction::Remove),
            _ => None,
        }
    }
}
