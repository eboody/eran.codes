mod crate_gallery;
mod fixture_loader;
#[path = "types.rs"]
pub(super) mod model;
mod validation;

pub(crate) use crate_gallery::Content as CrateGalleryContent;
pub use fixture_loader::{
    lab_page_content, open_source_index_content, portfolio_home_content, portfolio_nav_links,
    resume_text, supporting_archive_cases, work_case_content, work_index_content,
};
pub(super) use model as types;
pub use model::{
    ArchiveDetailsContent, ArchivedWorkCaseContent, CmsActionLink, CrateCardContent,
    CrateSectionContent, CtaKind,
    ExperienceRoleContent, ExperienceSectionContent, OpenSourceIndexContent, PortfolioHeroContent,
    PortfolioHomeContent, WorkCardContent, WorkCaseContent, WorkCaseSlug, WorkIndexContent,
    WorkSectionContent,
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::DeserializeOwned;

    fn parse_fragment<T: DeserializeOwned>(path: &str, source: &str) -> T {
        serde_json::from_str(source)
            .unwrap_or_else(|error| panic!("{path} should deserialize: {error}"))
    }

    fn find_crate<'a>(cards: &'a [CrateCardContent], name: &str) -> &'a CrateCardContent {
        cards.iter()
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

        assert!(!content.experience_section.roles.is_empty());
        assert!(!content.project_section.cards.is_empty());
        assert!(!content.current_proof_section.cards.is_empty());
        assert!(!content.skill_section.groups.is_empty());
    }

    #[test]
    fn work_index_fixture_loads() {
        let content = work_index_content();

        assert!(!content.current_proof_section.cards.is_empty());
        assert!(!content.supporting_cases_section.cards.is_empty());
        assert!(!content.current_proof_section.title.to_string().is_empty());
        assert!(!content.supporting_cases_section.title.to_string().is_empty());
        assert!(!content.archive_details.title.to_string().is_empty());
        assert!(!content.archive_details.entry_label.to_string().is_empty());
        assert!(!content.open_source_teaser.title.to_string().is_empty());
        assert!(!content.open_source_teaser.actions.is_empty());
    }

    #[test]
    fn site_content_fragments_deserialize_from_split_files() {
        let identity: super::types::IdentityContent = parse_fragment(
            "site_content/identity.json",
            include_str!("site_content/identity.json"),
        );
        let actions: super::types::ActionLibraryFragment = parse_fragment(
            "site_content/actions.json",
            include_str!("site_content/actions.json"),
        );
        let nav: Vec<super::types::LinkReference> = parse_fragment(
            "site_content/nav.json",
            include_str!("site_content/nav.json"),
        );
        let contact: Vec<super::types::ContactMethodContent> = parse_fragment(
            "site_content/contact.json",
            include_str!("site_content/contact.json"),
        );
        let experience: Vec<super::types::ExperienceRoleContent> = parse_fragment(
            "site_content/experience.json",
            include_str!("site_content/experience.json"),
        );
        let projects: Vec<super::types::WorkCardContent> = parse_fragment(
            "site_content/projects.json",
            include_str!("site_content/projects.json"),
        );
        let work_cases: Vec<super::types::WorkCaseRecord> = parse_fragment(
            "site_content/work_cases.json",
            include_str!("site_content/work_cases.json"),
        );
        let open_source: Vec<super::types::CrateCardContent> = parse_fragment(
            "site_content/open_source.json",
            include_str!("site_content/open_source.json"),
        );
        let skills: Vec<super::types::SkillGroupContent> = parse_fragment(
            "site_content/skills.json",
            include_str!("site_content/skills.json"),
        );
        let home: super::types::HomePageCopy = parse_fragment(
            "site_content/pages/home.json",
            include_str!("site_content/pages/home.json"),
        );
        let work: super::types::WorkIndexCopy = parse_fragment(
            "site_content/pages/work.json",
            include_str!("site_content/pages/work.json"),
        );
        let open_source_page: super::types::OpenSourceIndexCopy = parse_fragment(
            "site_content/pages/open_source.json",
            include_str!("site_content/pages/open_source.json"),
        );
        let lab: super::types::LabPageCopy = parse_fragment(
            "site_content/pages/lab.json",
            include_str!("site_content/pages/lab.json"),
        );
        let resume: super::types::ResumeDocumentContent = parse_fragment(
            "site_content/pages/resume.json",
            include_str!("site_content/pages/resume.json"),
        );

        assert_eq!(identity.name.to_string(), "Eran Boodnero");
        assert!(!actions.action_links.is_empty());
        assert!(!actions.action_bundles.is_empty());
        assert!(!nav.is_empty());
        assert!(!contact.is_empty());
        assert!(!experience.is_empty());
        assert!(!projects.is_empty());
        assert!(!work_cases.is_empty());
        assert!(!open_source.is_empty());
        assert!(!skills.is_empty());
        assert!(!home.current_proof.action_refs.is_empty());
        assert!(!work.archive_details.entry_label.to_string().is_empty());
        assert!(!open_source_page.crate_section.title.to_string().is_empty());
        assert!(!lab.hero.action_refs.is_empty());
        assert!(!resume.contact_method_ids.is_empty());
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
            code_examples
                .iter()
                .any(|code| code.contains("self.reviewer.clone().ok_or(statum::Error::InvalidState)")),
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
            code_examples
                .iter()
                .any(|code| code.contains("cargo modum check --root . --mode warn --format json")),
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
        for slug in [
            WorkCaseSlug::ChatRealtime,
            WorkCaseSlug::CommandSse,
            WorkCaseSlug::OperationalVisibility,
            WorkCaseSlug::SensitiveSync,
        ] {
            let content = work_case_content(slug);
            assert!(!content.outcomes.items.is_empty());
        }
    }

    #[test]
    fn supporting_archive_cases_follow_work_archive_order() {
        let cases = supporting_archive_cases();

        assert_eq!(cases.len(), 3);
        assert_eq!(cases[0].slug, WorkCaseSlug::ChatRealtime);
        assert_eq!(cases[1].slug, WorkCaseSlug::CommandSse);
        assert_eq!(cases[2].slug, WorkCaseSlug::OperationalVisibility);
    }

    #[test]
    fn shared_action_bundles_expand_into_expected_ctas() {
        let home = portfolio_home_content();
        let work = work_index_content();
        let lab = lab_page_content();
        let archived = work_case_content(WorkCaseSlug::ChatRealtime);

        let home_labels = home
            .current_proof_section
            .actions
            .iter()
            .map(|action| action.label.to_string())
            .collect::<Vec<_>>();
        let work_labels = work
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
        let archived_labels = archived
            .actions
            .iter()
            .map(|action| action.label.to_string())
            .collect::<Vec<_>>();

        assert_eq!(
            home_labels,
            vec!["Inspect sensitive proof", "Read current proof case"],
        );
        assert_eq!(work_labels, home_labels);
        assert_eq!(lab_labels, home_labels);
        assert_eq!(
            archived_labels,
            vec!["Review current proof case", "Back to supporting proof archive"],
        );
    }

    #[test]
    fn shared_site_content_exposes_resume_text_and_lab_copy() {
        assert!(resume_text().contains("## Professional Summary"));
        assert_eq!(lab_page_content().page_title.to_string(), "Live Proof");
        assert!(
            portfolio_nav_links()
                .iter()
                .any(|link| link.href.to_string() == "/resume.txt")
        );
    }
}
