use std::collections::HashSet;
use std::str::FromStr;

use crate::paths::Route;

use super::types::{DirectLinkReference, LinkReference, SiteContent, WorkCaseSlug};

pub(super) fn site_content(content: &SiteContent) {
    assert_non_empty_text("site.identity.name", &content.identity.name);
    assert_non_empty_text("site.identity.headline", &content.identity.headline);
    assert_unique_text_ids(
        "site.action_links",
        content.action_links.iter().map(|action| &action.id),
    );
    assert_unique_text_ids(
        "site.action_bundles",
        content.action_bundles.iter().map(|bundle| &bundle.id),
    );
    assert_unique_text_ids(
        "site.contact_methods",
        content.contact_methods.iter().map(|method| &method.id),
    );
    assert_unique_text_ids(
        "site.experience_roles",
        content.experience_roles.iter().map(|role| &role.id),
    );
    assert_unique_text_ids(
        "site.skill_groups",
        content.skill_groups.iter().map(|group| &group.id),
    );
    assert_unique_text_ids(
        "site.open_source_entries",
        content.open_source_entries.iter().map(|entry| &entry.name),
    );
    assert_unique_work_slugs(
        "site.projects",
        content.projects.iter().map(|project| project.slug),
    );
    assert_unique_work_slugs(
        "site.work_cases",
        content.work_cases.iter().map(|case| case.slug),
    );
    assert!(!content.nav_links.is_empty(), "site.nav_links must not be empty");
    assert!(
        !content.open_source_entries.is_empty(),
        "site.open_source_entries must not be empty"
    );
    assert!(!content.projects.is_empty(), "site.projects must not be empty");

    for action in &content.action_links {
        validate_action_link(action);
    }
    for reference in &content.nav_links {
        assert_link_ref_resolves(content, reference, "site.nav_links[]");
    }
    for bundle in &content.action_bundles {
        for reference in &bundle.references {
            assert_direct_link_ref_resolves(
                content,
                reference,
                "site.action_bundles[].references[]",
            );
        }
    }
    assert_ui_copy_refs_resolve(content);

    assert!(
        content
            .work_cases
            .iter()
            .any(|case| case.slug == WorkCaseSlug::SensitiveSync),
        "site.work_cases must include sensitive_sync"
    );
}

fn validate_action_link(action: &super::types::ActionLinkContent) {
    assert_non_empty_text("site.action_links[].label", &action.link.label);
    assert_non_empty_text("site.action_links[].href", &action.link.href);
    if !action.link.kind.is_external() {
        assert_internal_href_valid(&action.link.href, "site.action_links[].href");
    }
}

fn assert_ui_copy_refs_resolve(content: &SiteContent) {
    let home = &content.ui_copy.home;
    assert_link_refs_resolve(
        content,
        &home.hero.action_refs,
        "site.ui_copy.home.hero.action_refs[]",
    );
    assert_project_slugs_resolve(
        content,
        &home.current_proof.project_slugs,
        "site.ui_copy.home.current_proof.project_slugs[]",
    );
    assert_link_refs_resolve(
        content,
        &home.current_proof.action_refs,
        "site.ui_copy.home.current_proof.action_refs[]",
    );

    let work = &content.ui_copy.work;
    assert_project_slugs_resolve(
        content,
        &work.supporting_cases.project_slugs,
        "site.ui_copy.work.supporting_cases.project_slugs[]",
    );
    assert_link_refs_resolve(
        content,
        &work.supporting_cases.action_refs,
        "site.ui_copy.work.supporting_cases.action_refs[]",
    );

    let open_source = &content.ui_copy.open_source;
    assert_link_refs_resolve(
        content,
        &open_source.hero.action_refs,
        "site.ui_copy.open_source.hero.action_refs[]",
    );

    let lab = &content.ui_copy.lab;
    assert_link_refs_resolve(
        content,
        &lab.hero.action_refs,
        "site.ui_copy.lab.hero.action_refs[]",
    );
    assert_link_refs_resolve(
        content,
        &lab.session_card.guest_action_refs,
        "site.ui_copy.lab.session_card.guest_action_refs[]",
    );
    assert_link_refs_resolve(
        content,
        &lab.guest_chat.action_refs,
        "site.ui_copy.lab.guest_chat.action_refs[]",
    );

    let resume = &content.ui_copy.resume;
    assert_contact_method_ids_resolve(
        content,
        &resume.contact_method_ids,
        "site.ui_copy.resume.contact_method_ids[]",
    );
    assert_experience_role_ids_resolve(
        content,
        &resume.experience_role_ids,
        "site.ui_copy.resume.experience_role_ids[]",
    );
    assert_project_slugs_resolve(
        content,
        &resume.featured_project_slugs,
        "site.ui_copy.resume.featured_project_slugs[]",
    );
    assert_skill_group_ids_resolve(
        content,
        &resume.skill_group_ids,
        "site.ui_copy.resume.skill_group_ids[]",
    );

    for case in &content.work_cases {
        assert_link_refs_resolve(
            content,
            &case.content.action_refs,
            "site.work_cases[].action_refs[]",
        );
    }
}

fn assert_link_ref_resolves(content: &SiteContent, reference: &LinkReference, path: &str) {
    match reference {
        LinkReference::Action { id } => {
            let found = content.action_links.iter().any(|action| action.id == *id);
            assert!(found, "{path} must reference an existing action id: {id}");
        }
        LinkReference::ContactMethod { id, .. } => {
            let found = content
                .contact_methods
                .iter()
                .any(|method| method.id == *id);
            assert!(found, "{path} must reference an existing contact id: {id}");
        }
        LinkReference::Bundle { id } => {
            let found = content.action_bundles.iter().any(|bundle| bundle.id == *id);
            assert!(found, "{path} must reference an existing bundle id: {id}");
        }
    }
}

fn assert_link_refs_resolve(content: &SiteContent, references: &[LinkReference], path: &str) {
    for reference in references {
        assert_link_ref_resolves(content, reference, path);
    }
}

fn assert_direct_link_ref_resolves(
    content: &SiteContent,
    reference: &DirectLinkReference,
    path: &str,
) {
    match reference {
        DirectLinkReference::Action { id } => {
            let found = content.action_links.iter().any(|action| action.id == *id);
            assert!(found, "{path} must reference an existing action id: {id}");
        }
        DirectLinkReference::ContactMethod { id, .. } => {
            let found = content
                .contact_methods
                .iter()
                .any(|method| method.id == *id);
            assert!(found, "{path} must reference an existing contact id: {id}");
        }
    }
}

fn assert_contact_method_ids_resolve(content: &SiteContent, ids: &[crate::types::Text], path: &str) {
    for id in ids {
        let found = content
            .contact_methods
            .iter()
            .any(|method| method.id == *id);
        assert!(found, "{path} must reference an existing contact id: {id}");
    }
}

fn assert_experience_role_ids_resolve(content: &SiteContent, ids: &[crate::types::Text], path: &str) {
    for id in ids {
        let found = content
            .experience_roles
            .iter()
            .any(|role| role.id == *id);
        assert!(
            found,
            "{path} must reference an existing experience role id: {id}"
        );
    }
}

fn assert_skill_group_ids_resolve(content: &SiteContent, ids: &[crate::types::Text], path: &str) {
    for id in ids {
        let found = content.skill_groups.iter().any(|group| group.id == *id);
        assert!(
            found,
            "{path} must reference an existing skill group id: {id}"
        );
    }
}

fn assert_project_slugs_resolve(content: &SiteContent, slugs: &[WorkCaseSlug], path: &str) {
    for slug in slugs {
        let found = content.projects.iter().any(|project| project.slug == *slug);
        assert!(found, "{path} must reference an existing project slug: {slug:?}");
    }
}

fn assert_internal_href_valid(href: &crate::types::Text, path: &str) {
    let route = href
        .to_string()
        .split('#')
        .next()
        .unwrap_or_default()
        .split('?')
        .next()
        .unwrap_or_default()
        .to_string();

    assert!(
        Route::from_str(&route).is_ok(),
        "{path} must reference a known internal route: {route}"
    );
}

fn assert_non_empty_text(path: &str, value: &crate::types::Text) {
    assert!(!value.to_string().is_empty(), "{path} must not be empty");
}

fn assert_unique_text_ids<'a, I>(path: &str, ids: I)
where
    I: Iterator<Item = &'a crate::types::Text>,
{
    let mut seen = HashSet::new();
    for id in ids {
        let inserted = seen.insert(id.to_string());
        assert!(inserted, "{path} ids must be unique: {id}");
    }
}

fn assert_unique_work_slugs<I>(path: &str, slugs: I)
where
    I: Iterator<Item = WorkCaseSlug>,
{
    let mut seen = HashSet::new();
    for slug in slugs {
        let inserted = seen.insert(format!("{slug:?}"));
        assert!(inserted, "{path} slugs must be unique: {slug:?}");
    }
}

#[cfg(test)]
mod tests {
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use super::*;

    fn test_site_content() -> SiteContent {
        serde_json::from_str(include_str!("site_content/portfolio.json"))
            .expect("portfolio content should deserialize")
    }

    #[test]
    fn validate_site_content_rejects_missing_ui_action_refs() {
        let mut content = test_site_content();
        content.ui_copy.lab.hero.action_refs = vec![LinkReference::Action {
            id: crate::types::Text::from("missing_action"),
        }];

        let result = catch_unwind(AssertUnwindSafe(|| site_content(&content)));

        assert!(result.is_err(), "invalid ui_copy action refs must fail closed");
    }

    #[test]
    fn validate_site_content_rejects_missing_project_slugs_used_by_ui_copy() {
        let mut content = test_site_content();
        content
            .projects
            .retain(|project| project.slug != WorkCaseSlug::SensitiveSync);

        let result = catch_unwind(AssertUnwindSafe(|| site_content(&content)));

        assert!(result.is_err(), "missing referenced project slugs must fail closed");
    }
}
