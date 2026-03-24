mod asserts;
mod fragments;
mod pages;
mod refs;
#[cfg(test)]
mod tests;
mod usage;

use std::collections::HashSet;

use crate::{paths::Route, types::Text};

use super::CrateGalleryContent;
use super::crate_gallery::CrateGalleryPreviewContent;
use super::types::{
    ActionBundleContent, ActionLibraryFragment, ArchiveDetailsContent, CaseListSection,
    ClosingContent, ClosingCopy, CmsActionLink, CmsImageAsset, ContactMethodContent, CrateCardContent,
    CrateSectionContent, CtaKind, DirectLinkReference, ExperienceRoleContent, ExperienceSectionContent,
    HomePageCopy, IdentityContent, InfoCardContent, InfoSectionContent, LabPageContent,
    LabPageCopy, LabPanelContent, LinkReference, OpenSourceIndexContent, OpenSourceIndexCopy,
    PortfolioHeroContent, PortfolioHeroCopy, PortfolioHomeContent, ProjectSectionSelection,
    ResumeDocumentContent, SessionCardContent, SiteContent, SkillGroupContent, SkillSectionContent,
    WorkCardContent, WorkCaseContent, WorkCaseCopy, WorkCaseRecord, WorkCaseSlug, WorkIndexContent,
    WorkIndexCopy, WorkSectionContent,
};

use self::asserts::*;
pub(super) use self::fragments::{
    validate_action_library_fragment, validate_contact_fragment, validate_experience_fragment,
    validate_home_page_fragment, validate_identity_fragment, validate_lab_page_fragment,
    validate_nav_fragment, validate_open_source_entries_fragment,
    validate_open_source_page_fragment, validate_projects_fragment,
    validate_resume_page_fragment, validate_skill_groups_fragment,
    validate_work_cases_fragment, validate_work_page_fragment,
};
pub(super) use self::pages::{
    validate_lab_page, validate_open_source_index, validate_portfolio_home, validate_work_case,
    validate_work_index,
};
use self::refs::{
    validate_direct_link_reference_shape, validate_home_refs, validate_lab_copy,
    validate_open_source_refs, validate_resume_refs, validate_work_case_copy, validate_work_refs,
};
use self::usage::validate_used_entries;
use self::pages::{
    validate_action, validate_archive_details_shape, validate_case_list,
    validate_closing_copy_shape, validate_contact_method, validate_crate_card,
    validate_experience_role, validate_info_section, validate_lab_panel,
    validate_open_source_hero_copy_shape, validate_portfolio_hero_copy_shape,
    validate_project_section_selection_shape, validate_skill_group, validate_work_card,
};

pub(super) fn validate_site_content(content: &SiteContent) {
    assert_non_empty("site.identity.name", &content.identity.name);
    assert_non_empty("site.identity.location", &content.identity.location);
    assert_non_empty("site.identity.headline", &content.identity.headline);

    assert_min_len("site.action_links", &content.action_links, 1);
    assert_unique_text_ids(
        "site.action_links[].id",
        content.action_links.iter().map(|action| &action.id),
    );
    for action in &content.action_links {
        assert_non_empty("site.action_links[].id", &action.id);
        validate_action(&action.link, "site.action_links[]");
    }
    assert_unique_text_ids(
        "site.action_bundles[].id",
        content.action_bundles.iter().map(|bundle| &bundle.id),
    );
    for bundle in &content.action_bundles {
        refs::validate_action_bundle(content, bundle, "site.action_bundles[]");
    }

    assert_min_len("site.nav_links", &content.nav_links, 1);
    for link in &content.nav_links {
        refs::validate_link_reference(content, link, "site.nav_links[]");
    }

    assert_min_len("site.contact_methods", &content.contact_methods, 1);
    assert_unique_text_ids(
        "site.contact_methods[].id",
        content.contact_methods.iter().map(|method| &method.id),
    );
    for method in &content.contact_methods {
        validate_contact_method(method, "site.contact_methods[]");
    }

    assert_min_len("site.experience_roles", &content.experience_roles, 1);
    assert_unique_text_ids(
        "site.experience_roles[].id",
        content.experience_roles.iter().map(|role| &role.id),
    );
    for role in &content.experience_roles {
        validate_experience_role(role, "site.experience_roles[]");
    }

    assert_min_len("site.projects", &content.projects, 1);
    assert_unique_keys(
        "site.projects[].slug",
        content
            .projects
            .iter()
            .map(|project| project.slug.route().as_str().to_owned()),
    );
    for project in &content.projects {
        validate_work_card(project, "site.projects[]");
    }

    assert_min_len("site.open_source_entries", &content.open_source_entries, 1);
    assert_unique_keys(
        "site.open_source_entries[].name",
        content
            .open_source_entries
            .iter()
            .map(|entry| entry.name.to_string()),
    );
    for entry in &content.open_source_entries {
        validate_crate_card(entry, "site.open_source_entries[]");
    }

    assert_min_len("site.skill_groups", &content.skill_groups, 1);
    assert_unique_text_ids(
        "site.skill_groups[].id",
        content.skill_groups.iter().map(|group| &group.id),
    );
    for group in &content.skill_groups {
        validate_skill_group(group, "site.skill_groups[]");
    }

    assert_min_len("site.work_cases", &content.work_cases, 1);
    assert_unique_keys(
        "site.work_cases[].slug",
        content
            .work_cases
            .iter()
            .map(|case| case.slug.route().as_str().to_owned()),
    );
    for case in &content.work_cases {
        validate_work_case_copy(content, &case.content, case.slug);
    }

    validate_home_refs(content);
    validate_work_refs(content);
    validate_open_source_refs(content);
    validate_resume_refs(content);
    validate_lab_copy(content, &content.ui_copy.lab);
    validate_used_entries(content);
}
