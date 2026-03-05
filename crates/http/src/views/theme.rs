#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Theme {
    TabbedShowcase(tabbed_showcase::Theme),
}

impl Theme {
    pub(crate) const fn tabbed_showcase(theme: tabbed_showcase::Theme) -> Self {
        Self::TabbedShowcase(theme)
    }

    pub(crate) const fn netbird() -> Self {
        Self::tabbed_showcase(tabbed_showcase::Theme::Netbird)
    }

    pub(crate) const fn netbird_detail() -> Self {
        Self::tabbed_showcase(tabbed_showcase::Theme::NetbirdDetail)
    }

    pub(crate) const fn as_attr(self) -> &'static str {
        match self {
            Self::TabbedShowcase(theme) => theme.as_attr(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::tabbed_showcase(tabbed_showcase::Theme::default())
    }
}

pub(crate) mod tabbed_showcase {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub(crate) enum Theme {
        #[default]
        Netbird,
        NetbirdDetail,
    }

    impl Theme {
        pub(crate) const fn as_attr(self) -> &'static str {
            match self {
                Self::Netbird => "netbird",
                Self::NetbirdDetail => "netbird-detail",
            }
        }
    }
}
