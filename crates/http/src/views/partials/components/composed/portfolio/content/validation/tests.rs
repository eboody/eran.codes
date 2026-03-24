use super::*;
use super::shared::*;

#[test]
#[should_panic(expected = "site.projects[].slug must be unique")]
fn site_content_rejects_duplicate_project_slugs() {
    let mut content = super::super::fixture_loader::site_content().clone();
    content.projects.push(content.projects[0].clone());

    validate_site_content(&content);
}

#[test]
#[should_panic(expected = "site.ui_copy.home.experience.role_ids")]
fn site_content_requires_resolved_home_experience_refs() {
    let mut content = super::super::fixture_loader::site_content().clone();
    content.ui_copy.home.experience.role_ids[0] = Text::from("missing-role");

    validate_site_content(&content);
}

#[test]
#[should_panic(expected = "site.ui_copy.home.hero.action_refs must resolve action id")]
fn site_content_requires_resolved_action_refs() {
    let mut content = super::super::fixture_loader::site_content().clone();
    content.ui_copy.home.hero.action_refs[0] = super::super::types::LinkReference::Action {
        id: Text::from("missing-action"),
    };

    validate_site_content(&content);
}

#[test]
#[should_panic(
    expected = "site.ui_copy.home.current_proof.action_refs must resolve action bundle id"
)]
fn site_content_requires_resolved_action_bundle_refs() {
    let mut content = super::super::fixture_loader::site_content().clone();
    content.ui_copy.home.current_proof.action_refs[0] =
        super::super::types::LinkReference::Bundle {
            id: Text::from("missing-bundle"),
        };

    validate_site_content(&content);
}

#[test]
#[should_panic(expected = "site.action_bundles[].references[] must resolve action id")]
fn site_content_rejects_bundles_with_unresolved_action_targets() {
    let mut content = super::super::fixture_loader::site_content().clone();
    content.action_bundles[0].references[0] =
        super::super::types::DirectLinkReference::Action {
            id: Text::from("missing-action"),
        };

    validate_site_content(&content);
}

#[test]
#[should_panic(
    expected = "site.action_links[].href must target a known internal route or local anchor"
)]
fn site_content_rejects_invalid_internal_route_targets() {
    let mut content = super::super::fixture_loader::site_content().clone();
    content.action_links[0].link.href = Text::from("/not-a-real-route");

    validate_site_content(&content);
}

#[test]
#[should_panic(expected = "site.ui_copy.work.current_proof.project_slugs must use dedicated proof routes")]
fn site_content_rejects_current_proof_without_dedicated_routes() {
    let mut content = super::super::fixture_loader::site_content().clone();
    content.ui_copy.work.current_proof.project_slugs[0] = super::super::types::WorkCaseSlug::ChatRealtime;

    validate_site_content(&content);
}

#[test]
#[should_panic(expected = "site.ui_copy.work.supporting_cases.project_slugs must use archive anchors")]
fn site_content_rejects_supporting_cases_without_archive_anchor_routes() {
    let mut content = super::super::fixture_loader::site_content().clone();
    content.ui_copy.work.supporting_cases.project_slugs[0] =
        super::super::types::WorkCaseSlug::SensitiveSync;

    validate_site_content(&content);
}

#[test]
#[should_panic(
    expected = "site.contact_methods[].href must target a known internal route or local anchor"
)]
fn site_content_rejects_invalid_internal_contact_method_targets() {
    let mut content = super::super::fixture_loader::site_content().clone();
    content.contact_methods[0].kind = super::super::types::LinkKind::Internal;
    content.contact_methods[0].href = Text::from("/not-a-real-route");

    validate_site_content(&content);
}

#[test]
#[should_panic(expected = "site.action_bundles must not include unused id")]
fn site_content_rejects_unused_action_bundles() {
    let mut content = super::super::fixture_loader::site_content().clone();
    content
        .action_bundles
        .push(super::super::types::ActionBundleContent {
            id: Text::from("unused-bundle"),
            references: vec![super::super::types::DirectLinkReference::Action {
                id: Text::from("sign_in"),
            }],
        });

    validate_site_content(&content);
}

#[test]
#[should_panic(expected = "site.contact_methods must not include unused id")]
fn site_content_rejects_unused_contact_methods() {
    let mut content = super::super::fixture_loader::site_content().clone();
    content.contact_methods.push(super::super::types::ContactMethodContent {
        id: Text::from("unused-contact"),
        label: Text::from("Unused"),
        value: Text::from("unused@example.com"),
        href: Text::from("mailto:unused@example.com"),
        kind: super::super::types::LinkKind::External,
    });

    validate_site_content(&content);
}

#[test]
#[should_panic(expected = "site.work_cases must not include unreachable slug OperationalVisibility")]
fn site_content_rejects_unreachable_work_cases() {
    let mut content = super::super::fixture_loader::site_content().clone();
    content
        .ui_copy
        .home
        .selected_projects
        .project_slugs
        .retain(|slug| *slug != super::super::types::WorkCaseSlug::OperationalVisibility);
    content
        .ui_copy
        .work
        .supporting_cases
        .project_slugs
        .retain(|slug| *slug != super::super::types::WorkCaseSlug::OperationalVisibility);
    content
        .ui_copy
        .resume
        .featured_project_slugs
        .retain(|slug| *slug != super::super::types::WorkCaseSlug::OperationalVisibility);

    validate_site_content(&content);
}

#[test]
#[should_panic(expected = "open_source.crate_section.cards[].docs_label")]
fn open_source_index_requires_docs_label_when_docs_url_is_present() {
    let mut content = super::super::fixture_loader::open_source_index_content().clone();
    content.crate_section.cards[0].docs_label = None;

    validate_open_source_index(&content);
}

#[test]
#[should_panic(expected = "lab.engineering_quality.cards[].points")]
fn lab_page_requires_engineering_quality_points() {
    let mut content = super::super::fixture_loader::lab_page_content().clone();
    content.engineering_quality.cards[0].points.clear();

    validate_lab_page(&content);
}
