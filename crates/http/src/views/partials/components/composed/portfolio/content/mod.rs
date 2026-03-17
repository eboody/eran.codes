mod crate_gallery;
mod fixture_loader;
mod types;
mod validation;

pub use crate_gallery::CrateGalleryContent;
pub use fixture_loader::{
    open_source_index_content, portfolio_home_content, work_case_content, work_index_content,
};
pub use types::{
    ClosingContent, CmsActionLink, CrateCardContent, CrateSectionContent, CtaKind,
    PortfolioHeroContent, ProofKind, ProofStripContent, WorkCardContent, WorkCaseContent,
    WorkCaseSlug, WorkIndexContent, WorkSectionContent,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portfolio_home_fixture_loads() {
        let content = portfolio_home_content();

        assert!(!content.work_section.cards.is_empty());
        assert!(!content.crate_section.cards.is_empty());
    }

    #[test]
    fn work_index_fixture_loads() {
        let content = work_index_content();

        assert!(!content.cases.is_empty());
        assert!(!content.cases_title.to_string().is_empty());
        assert!(!content.open_source_teaser.title.to_string().is_empty());
        assert!(!content.open_source_teaser.actions.is_empty());
    }

    #[test]
    fn open_source_index_fixture_loads() {
        let content = open_source_index_content();

        assert!(!content.hero.title.to_string().is_empty());
        assert!(!content.crate_section.cards.is_empty());
    }

    #[test]
    fn work_case_fixtures_load() {
        for slug in [
            WorkCaseSlug::ChatRealtime,
            WorkCaseSlug::CommandSse,
            WorkCaseSlug::OperationalVisibility,
        ] {
            let content = work_case_content(slug);
            assert!(!content.outcomes.items.is_empty());
        }
    }
}
