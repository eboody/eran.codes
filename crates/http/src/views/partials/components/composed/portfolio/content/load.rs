use std::sync::OnceLock;

use super::{types::SiteContent, validate};

pub(super) fn site_content() -> &'static SiteContent {
    static CONTENT: OnceLock<SiteContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let content: SiteContent = serde_json::from_str(include_str!("site_content/portfolio.json"))
            .unwrap_or_else(|error| panic!("site_content/portfolio.json must be valid JSON: {error}"));
        validate::site_content(&content);
        content
    })
}
