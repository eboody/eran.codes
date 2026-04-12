mod crate_gallery;
mod fixture_loader;
#[path = "types.rs"]
pub(super) mod model;

pub(crate) use crate_gallery::Content as CrateGalleryContent;
pub use fixture_loader::{
    lab_page_content, open_source_index_content, portfolio_home_content, portfolio_nav_links,
    resume_text, work_case_content, work_index_content,
};
pub(super) use model as types;
pub use model::{
    CmsActionLink, CrateCardContent, CrateSectionContent, CtaKind, OpenSourceIndexContent,
    PortfolioHeroContent, PortfolioHomeContent, WorkCardContent, WorkCaseContent,
    WorkCaseDetailLayout, WorkCaseSlug, WorkIndexContent, WorkSectionContent,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn find_crate<'a>(cards: &'a [CrateCardContent], name: &str) -> &'a CrateCardContent {
        cards
            .iter()
            .find(|card| card.name == crate::types::Text::from(name))
            .expect("crate card should exist")
    }

    fn gallery_code_examples(card: &CrateCardContent) -> Vec<String> {
        card.gallery
            .as_ref()
            .expect("crate card should include a gallery")
            .tabs
            .iter()
            .flat_map(|tab| tab.preview.code_examples.iter())
            .map(|example| example.code.to_string())
            .collect()
    }

    #[test]
    fn portfolio_home_fixture_loads() {
        let content = portfolio_home_content();

        assert!(!content.current_proof_section.cards.is_empty());
    }

    #[test]
    fn work_index_fixture_loads() {
        let content = work_index_content();

        assert!(!content.hero.title.to_string().is_empty());
        assert!(!content.supporting_cases_section.cards.is_empty());
        assert!(
            !content
                .supporting_cases_section
                .title
                .to_string()
                .is_empty()
        );
    }

    #[test]
    fn site_content_deserializes_from_one_portfolio_file() {
        let content: super::types::SiteContent = serde_json::from_str(include_str!(
            "site_content/portfolio.json"
        ))
        .expect("portfolio content should deserialize");

        assert_eq!(content.identity.name.to_string(), "Eran Boodnero");
        assert!(!content.action_links.is_empty());
        assert!(!content.action_bundles.is_empty());
        assert!(!content.nav_links.is_empty());
        assert!(!content.contact_methods.is_empty());
        assert!(!content.experience_roles.is_empty());
        assert!(!content.projects.is_empty());
        assert!(!content.work_cases.is_empty());
        assert!(!content.open_source_entries.is_empty());
        assert!(!content.skill_groups.is_empty());
        assert!(!content.ui_copy.home.current_proof.action_refs.is_empty());
        assert!(!content.ui_copy.work.supporting_cases.project_slugs.is_empty());
        assert!(!content.ui_copy.open_source.crate_section.title.to_string().is_empty());
        assert!(!content.ui_copy.lab.hero.action_refs.is_empty());
        assert!(!content.ui_copy.resume.contact_method_ids.is_empty());
    }

    #[test]
    fn open_source_index_fixture_loads() {
        let content = open_source_index_content();

        assert!(!content.hero.title.to_string().is_empty());
        assert!(!content.crate_section.cards.is_empty());
    }

    #[test]
    fn open_source_statum_gallery_uses_current_published_surfaces() {
        let content = open_source_index_content();
        let statum_card = find_crate(&content.crate_section.cards, "statum");
        let code_examples = gallery_code_examples(statum_card);

        assert!(
            code_examples
                .iter()
                .any(|code| code.contains("task_machine::SomeState::InReview")),
            "statum gallery should show the current SomeState surface",
        );
        assert!(
            !code_examples
                .iter()
                .any(|code| code.contains("task_machine::State::InReview")),
            "statum gallery should avoid the compatibility alias in primary examples",
        );
        assert!(
            code_examples.iter().any(|code| code
                .contains("self.reviewer.clone().ok_or(statum::Error::InvalidState)")),
            "statum gallery should match the published validator shape",
        );
        assert!(
            !code_examples
                .iter()
                .any(|code| code.contains("reviewer-for-{client}")),
            "statum gallery should not keep stale placeholder examples",
        );
    }

    #[test]
    fn open_source_nestum_gallery_uses_current_rewrite_surfaces() {
        let content = open_source_index_content();
        let nestum_card = find_crate(&content.crate_section.cards, "nestum");
        let code_examples = gallery_code_examples(nestum_card);

        assert!(
            code_examples
                .iter()
                .any(|code| code.contains("Event::Document::Created")),
            "nestum gallery should use the current nested constructor surface",
        );
        assert!(
            code_examples
                .iter()
                .any(|code| code.contains("#[nestum_scope]")),
            "nestum gallery should show scope-level rewrites",
        );
        assert!(
            code_examples
                .iter()
                .any(|code| code.contains("nested! { Outer::Wrap::Struct { x: 5 } }")),
            "nestum gallery should cover named-field constructor rewrites",
        );
    }

    #[test]
    fn open_source_modum_gallery_uses_current_cli_and_config_surfaces() {
        let content = open_source_index_content();
        let modum_card = find_crate(&content.crate_section.cards, "modum");
        let code_examples = gallery_code_examples(modum_card);

        assert!(
            code_examples.iter().any(|code| code
                .contains("cargo modum check --root . --mode warn --format json")),
            "modum gallery should show the current CLI surface for machine-readable diagnostics",
        );
        assert!(
            code_examples
                .iter()
                .any(|code| code.contains("namespace_preserving_modules")),
            "modum gallery should show namespace-preserving module configuration",
        );
    }

    #[test]
    fn portfolio_home_open_source_cards_link_docs_for_all_published_crates() {
        let content = open_source_index_content();

        for crate_name in ["statum", "nestum", "modum"] {
            let card = find_crate(&content.crate_section.cards, crate_name);
            assert!(
                card.docs_url.is_some(),
                "{crate_name} should expose docs on the open-source card",
            );
            assert!(
                card.docs_label.is_some(),
                "{crate_name} should expose a docs label on the open-source card",
            );
        }
    }

    #[test]
    fn work_case_fixtures_load() {
        let content = work_case_content(WorkCaseSlug::SensitiveSync);

        assert!(!content.hero.title.to_string().is_empty());
        assert!(!content.outcomes.items.is_empty());
    }

    #[test]
    fn shared_action_bundles_expand_into_expected_ctas() {
        let home = portfolio_home_content();
        let lab = lab_page_content();
        let current_case = work_case_content(WorkCaseSlug::SensitiveSync);

        let home_labels = home
            .current_proof_section
            .actions
            .iter()
            .map(|action| action.label.to_string())
            .collect::<Vec<_>>();
        let lab_labels = lab
            .hero
            .actions
            .iter()
            .map(|action| action.label.to_string())
            .collect::<Vec<_>>();
        let current_case_labels = current_case
            .hero
            .actions
            .iter()
            .map(|action| action.label.to_string())
            .collect::<Vec<_>>();

        assert_eq!(
            home_labels,
            vec!["Inspect live lab", "Read case study"],
        );
        assert_eq!(lab_labels, home_labels);
        assert_eq!(
            current_case_labels,
            vec!["Inspect live lab", "Browse archive"],
        );
    }

    #[test]
    fn shared_site_content_exposes_resume_text_and_lab_copy() {
        assert!(resume_text().contains("## Professional Summary"));
        assert_eq!(lab_page_content().page_title.to_string(), "Lab");
        assert!(
            portfolio_nav_links()
                .iter()
                .any(|link| link.href.to_string() == "/resume.txt")
        );
    }
}
