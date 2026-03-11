use crate::types::Text;

#[derive(Clone, Debug, Default)]
pub enum Role {
    #[default]
    Button,
    Submit {
        name: Option<Text>,
        value: Option<Text>,
    },
    Link {
        href: Text,
        external: bool,
    },
}

impl Role {
    pub fn submit() -> Self {
        Self::Submit {
            name: None,
            value: None,
        }
    }

    pub fn submit_with(name: impl Into<Text>, value: impl Into<Text>) -> Self {
        Self::Submit {
            name: Some(name.into()),
            value: Some(value.into()),
        }
    }

    pub fn link(href: impl Into<Text>) -> Self {
        Self::Link {
            href: href.into(),
            external: false,
        }
    }

    pub fn external_link(href: impl Into<Text>) -> Self {
        Self::Link {
            href: href.into(),
            external: true,
        }
    }
}
