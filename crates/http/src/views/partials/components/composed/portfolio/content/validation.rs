use std::collections::HashSet;

use crate::{paths::Route, types::Text};

use super::CrateGalleryContent;
use super::types::{
    ActionBundleContent, ActionLibraryFragment, ArchiveDetailsContent, CaseListSection,
    ClosingContent, CmsActionLink, CmsImageAsset, ContactMethodContent, CrateCardContent,
    CrateSectionContent, DirectLinkReference, ExperienceRoleContent, ExperienceSectionContent,
    HomePageCopy, IdentityContent, InfoCardContent, InfoSectionContent, LabPageContent,
    LabPageCopy, LabPanelContent, LinkReference, OpenSourceIndexContent, OpenSourceIndexCopy,
    PortfolioHeroContent, PortfolioHomeContent, ResumeDocumentContent, SessionCardContent,
    SiteContent, SkillGroupContent, SkillSectionContent, WorkCardContent, WorkCaseContent,
    WorkCaseRecord, WorkCaseSlug, WorkIndexContent, WorkIndexCopy, WorkSectionContent,
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
        validate_action_bundle(content, bundle, "site.action_bundles[]");
    }

    assert_min_len("site.nav_links", &content.nav_links, 1);
    for link in &content.nav_links {
        validate_link_reference(content, link, "site.nav_links[]");
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

pub(super) fn validate_identity_fragment(content: &IdentityContent) {
    assert_non_empty("site_content/identity.json.name", &content.name);
    assert_non_empty("site_content/identity.json.location", &content.location);
    assert_non_empty("site_content/identity.json.headline", &content.headline);
}

pub(super) fn validate_action_library_fragment(content: &ActionLibraryFragment) {
    assert_min_len(
        "site_content/actions.json.action_links",
        &content.action_links,
        1,
    );
    assert_unique_text_ids(
        "site_content/actions.json.action_links[].id",
        content.action_links.iter().map(|action| &action.id),
    );
    for action in &content.action_links {
        assert_non_empty("site_content/actions.json.action_links[].id", &action.id);
        validate_action(
            &action.link,
            "site_content/actions.json.action_links[]",
        );
    }

    assert_unique_text_ids(
        "site_content/actions.json.action_bundles[].id",
        content.action_bundles.iter().map(|bundle| &bundle.id),
    );
    for bundle in &content.action_bundles {
        assert_non_empty("site_content/actions.json.action_bundles[].id", &bundle.id);
        assert_min_len(
            "site_content/actions.json.action_bundles[].references",
            &bundle.references,
            1,
        );
        for reference in &bundle.references {
            validate_direct_link_reference_shape(
                reference,
                "site_content/actions.json.action_bundles[].references[]",
            );
        }
    }
}

pub(super) fn validate_nav_fragment(content: &[LinkReference]) {
    assert_min_len("site_content/nav.json", content, 1);
}

pub(super) fn validate_contact_fragment(content: &[ContactMethodContent]) {
    assert_min_len("site_content/contact.json", content, 1);
    assert_unique_text_ids(
        "site_content/contact.json[].id",
        content.iter().map(|method| &method.id),
    );
    for method in content {
        validate_contact_method(method, "site_content/contact.json[]");
    }
}

pub(super) fn validate_experience_fragment(content: &[ExperienceRoleContent]) {
    assert_min_len("site_content/experience.json", content, 1);
    assert_unique_text_ids(
        "site_content/experience.json[].id",
        content.iter().map(|role| &role.id),
    );
    for role in content {
        validate_experience_role(role, "site_content/experience.json[]");
    }
}

pub(super) fn validate_projects_fragment(content: &[WorkCardContent]) {
    assert_min_len("site_content/projects.json", content, 1);
    assert_unique_keys(
        "site_content/projects.json[].slug",
        content
            .iter()
            .map(|project| project.slug.route().as_str().to_owned()),
    );
    for project in content {
        validate_work_card(project, "site_content/projects.json[]");
    }
}

pub(super) fn validate_work_cases_fragment(content: &[WorkCaseRecord]) {
    assert_min_len("site_content/work_cases.json", content, 1);
    assert_unique_keys(
        "site_content/work_cases.json[].slug",
        content
            .iter()
            .map(|case| case.slug.route().as_str().to_owned()),
    );
    for case in content {
        assert_non_empty(
            "site_content/work_cases.json[].page_title",
            &case.content.page_title,
        );
        assert_non_empty(
            "site_content/work_cases.json[].eyebrow",
            &case.content.eyebrow,
        );
        assert_non_empty("site_content/work_cases.json[].title", &case.content.title);
        assert_non_empty(
            "site_content/work_cases.json[].summary",
            &case.content.summary,
        );
        validate_case_list(
            &case.content.challenge,
            "site_content/work_cases.json[].challenge",
        );
        validate_case_list(
            &case.content.implementation,
            "site_content/work_cases.json[].implementation",
        );
        validate_case_list(
            &case.content.outcomes,
            "site_content/work_cases.json[].outcomes",
        );
        validate_case_list(&case.content.stack, "site_content/work_cases.json[].stack");
        assert_min_len(
            "site_content/work_cases.json[].action_refs",
            &case.content.action_refs,
            1,
        );
    }
}

pub(super) fn validate_open_source_entries_fragment(content: &[CrateCardContent]) {
    assert_min_len("site_content/open_source.json", content, 1);
    assert_unique_keys(
        "site_content/open_source.json[].name",
        content.iter().map(|entry| entry.name.to_string()),
    );
    for entry in content {
        validate_crate_card(entry, "site_content/open_source.json[]");
    }
}

pub(super) fn validate_skill_groups_fragment(content: &[SkillGroupContent]) {
    assert_min_len("site_content/skills.json", content, 1);
    assert_unique_text_ids(
        "site_content/skills.json[].id",
        content.iter().map(|group| &group.id),
    );
    for group in content {
        validate_skill_group(group, "site_content/skills.json[]");
    }
}

pub(super) fn validate_home_page_fragment(content: &HomePageCopy) {
    assert_non_empty("site_content/pages/home.json.page_title", &content.page_title);
    validate_portfolio_hero_copy_shape(&content.hero, "site_content/pages/home.json.hero");
    assert_non_empty(
        "site_content/pages/home.json.experience.title",
        &content.experience.title,
    );
    assert_non_empty(
        "site_content/pages/home.json.experience.subtitle",
        &content.experience.subtitle,
    );
    assert_min_len(
        "site_content/pages/home.json.experience.role_ids",
        &content.experience.role_ids,
        1,
    );
    validate_project_section_selection_shape(
        &content.selected_projects,
        "site_content/pages/home.json.selected_projects",
    );
    validate_project_section_selection_shape(
        &content.current_proof,
        "site_content/pages/home.json.current_proof",
    );
    validate_closing_copy_shape(
        &content.open_source_teaser,
        "site_content/pages/home.json.open_source_teaser",
    );
    assert_non_empty("site_content/pages/home.json.skills.title", &content.skills.title);
    assert_non_empty(
        "site_content/pages/home.json.skills.subtitle",
        &content.skills.subtitle,
    );
    assert_min_len(
        "site_content/pages/home.json.skills.skill_group_ids",
        &content.skills.skill_group_ids,
        1,
    );
    assert_non_empty("site_content/pages/home.json.contact.title", &content.contact.title);
    assert_non_empty(
        "site_content/pages/home.json.contact.summary",
        &content.contact.summary,
    );
    assert_min_len(
        "site_content/pages/home.json.contact.action_refs",
        &content.contact.action_refs,
        1,
    );
}

pub(super) fn validate_work_page_fragment(content: &WorkIndexCopy) {
    assert_non_empty("site_content/pages/work.json.page_title", &content.page_title);
    assert_non_empty("site_content/pages/work.json.eyebrow", &content.eyebrow);
    assert_non_empty("site_content/pages/work.json.title", &content.title);
    assert_non_empty("site_content/pages/work.json.summary", &content.summary);
    validate_project_section_selection_shape(
        &content.current_proof,
        "site_content/pages/work.json.current_proof",
    );
    validate_project_section_selection_shape(
        &content.supporting_cases,
        "site_content/pages/work.json.supporting_cases",
    );
    validate_archive_details_shape(
        &content.archive_details,
        "site_content/pages/work.json.archive_details",
    );
    validate_closing_copy_shape(
        &content.open_source_teaser,
        "site_content/pages/work.json.open_source_teaser",
    );
}

pub(super) fn validate_open_source_page_fragment(content: &OpenSourceIndexCopy) {
    assert_non_empty(
        "site_content/pages/open_source.json.page_title",
        &content.page_title,
    );
    validate_open_source_hero_copy_shape(
        &content.hero,
        "site_content/pages/open_source.json.hero",
    );
    assert_non_empty(
        "site_content/pages/open_source.json.crate_section.title",
        &content.crate_section.title,
    );
    assert_non_empty(
        "site_content/pages/open_source.json.crate_section.subtitle",
        &content.crate_section.subtitle,
    );
}

pub(super) fn validate_lab_page_fragment(content: &LabPageCopy) {
    assert_non_empty("site_content/pages/lab.json.page_title", &content.page_title);
    validate_portfolio_hero_copy_shape(&content.hero, "site_content/pages/lab.json.hero");
    assert_non_empty(
        "site_content/pages/lab.json.session_card.title",
        &content.session_card.title,
    );
    assert_non_empty(
        "site_content/pages/lab.json.session_card.guest_status",
        &content.session_card.guest_status,
    );
    assert_non_empty(
        "site_content/pages/lab.json.session_card.guest_summary",
        &content.session_card.guest_summary,
    );
    assert_non_empty(
        "site_content/pages/lab.json.session_card.signed_in_action_label",
        &content.session_card.signed_in_action_label,
    );
    assert_min_len(
        "site_content/pages/lab.json.session_card.guest_action_refs",
        &content.session_card.guest_action_refs,
        1,
    );
    validate_closing_copy_shape(
        &content.guest_chat,
        "site_content/pages/lab.json.guest_chat",
    );
    validate_lab_panel(
        &content.operations_surface,
        "site_content/pages/lab.json.operations_surface",
    );
    validate_lab_panel(
        &content.sensitive_proof,
        "site_content/pages/lab.json.sensitive_proof",
    );
    validate_info_section(
        &content.engineering_quality,
        "site_content/pages/lab.json.engineering_quality",
    );
}

pub(super) fn validate_resume_page_fragment(content: &ResumeDocumentContent) {
    assert_non_empty(
        "site_content/pages/resume.json.summary_title",
        &content.summary_title,
    );
    assert_non_empty("site_content/pages/resume.json.summary", &content.summary);
    assert_non_empty(
        "site_content/pages/resume.json.experience_title",
        &content.experience_title,
    );
    assert_non_empty(
        "site_content/pages/resume.json.projects_title",
        &content.projects_title,
    );
    assert_non_empty(
        "site_content/pages/resume.json.open_source_title",
        &content.open_source_title,
    );
    assert_non_empty(
        "site_content/pages/resume.json.client_context_title",
        &content.client_context_title,
    );
    assert_min_len(
        "site_content/pages/resume.json.client_context",
        &content.client_context,
        1,
    );
    assert_non_empty(
        "site_content/pages/resume.json.skills_title",
        &content.skills_title,
    );
    assert_min_len(
        "site_content/pages/resume.json.experience_role_ids",
        &content.experience_role_ids,
        1,
    );
    assert_min_len(
        "site_content/pages/resume.json.featured_project_slugs",
        &content.featured_project_slugs,
        1,
    );
    assert_min_len(
        "site_content/pages/resume.json.skill_group_ids",
        &content.skill_group_ids,
        1,
    );
    assert_min_len(
        "site_content/pages/resume.json.contact_method_ids",
        &content.contact_method_ids,
        1,
    );
}

pub(super) fn validate_portfolio_home(content: &PortfolioHomeContent) {
    assert_non_empty("home.page_title", &content.page_title);
    validate_portfolio_hero(&content.hero, "home.hero");
    validate_experience_section(&content.experience_section, "home.experience_section");
    validate_work_section(&content.project_section, "home.project_section");
    validate_work_section(&content.current_proof_section, "home.current_proof_section");
    validate_closing(&content.open_source_teaser, "home.open_source_teaser");
    validate_skill_section(&content.skill_section, "home.skill_section");
    validate_closing(&content.contact_section, "home.contact_section");
}

pub(super) fn validate_work_index(content: &WorkIndexContent) {
    assert_non_empty("work.page_title", &content.page_title);
    assert_non_empty("work.eyebrow", &content.eyebrow);
    assert_non_empty("work.title", &content.title);
    assert_non_empty("work.summary", &content.summary);
    validate_work_section(&content.current_proof_section, "work.current_proof_section");
    validate_work_section(
        &content.supporting_cases_section,
        "work.supporting_cases_section",
    );
    assert_non_empty("work.archive_details.title", &content.archive_details.title);
    assert_non_empty(
        "work.archive_details.subtitle",
        &content.archive_details.subtitle,
    );
    assert_non_empty(
        "work.archive_details.entry_label",
        &content.archive_details.entry_label,
    );
    validate_closing(&content.open_source_teaser, "work.open_source_teaser");
}

pub(super) fn validate_open_source_index(content: &OpenSourceIndexContent) {
    assert_non_empty("open_source.page_title", &content.page_title);
    validate_open_source_hero(&content.hero, "open_source.hero");
    validate_crate_section(&content.crate_section, "open_source.crate_section");
}

pub(super) fn validate_lab_page(content: &LabPageContent) {
    assert_non_empty("lab.page_title", &content.page_title);
    validate_portfolio_hero(&content.hero, "lab.hero");
    validate_session_card(&content.session_card, "lab.session_card");
    validate_closing(&content.guest_chat, "lab.guest_chat");
    validate_lab_panel(&content.operations_surface, "lab.operations_surface");
    validate_lab_panel(&content.sensitive_proof, "lab.sensitive_proof");
    validate_info_section(&content.engineering_quality, "lab.engineering_quality");
}

pub(super) fn validate_work_case(content: &WorkCaseContent, slug: WorkCaseSlug) {
    assert_non_empty("work_case.page_title", &content.page_title);
    assert_non_empty("work_case.eyebrow", &content.eyebrow);
    assert_non_empty("work_case.title", &content.title);
    assert_non_empty("work_case.summary", &content.summary);
    validate_case_list(&content.challenge, "work_case.challenge");
    validate_case_list(&content.implementation, "work_case.implementation");
    validate_case_list(&content.outcomes, "work_case.outcomes");
    validate_case_list(&content.stack, "work_case.stack");

    for action in &content.actions {
        validate_action(action, "work_case.actions[]");
    }

    let case_route = slug.route();
    assert!(
        content
            .actions
            .iter()
            .any(|action| action_targets_proof_path(action, case_route)),
        "work case {slug:?} should include at least one action to itself, current proof, or /lab",
    );
}

fn validate_home_refs(content: &SiteContent) {
    let home = &content.ui_copy.home;
    validate_portfolio_hero_copy_refs(content, &home.hero, "site.ui_copy.home.hero");
    assert_min_len("site.ui_copy.home.experience.role_ids", &home.experience.role_ids, 1);
    for role_id in &home.experience.role_ids {
        assert!(
            content.experience_roles.iter().any(|role| role.id == *role_id),
            "site.ui_copy.home.experience.role_ids must resolve {}",
            role_id,
        );
    }

    validate_project_refs(
        content,
        &home.selected_projects.project_slugs,
        "site.ui_copy.home.selected_projects.project_slugs",
    );
    validate_link_refs(
        content,
        &home.selected_projects.action_refs,
        "site.ui_copy.home.selected_projects.action_refs",
    );
    validate_project_refs(
        content,
        &home.current_proof.project_slugs,
        "site.ui_copy.home.current_proof.project_slugs",
    );
    validate_link_refs(
        content,
        &home.current_proof.action_refs,
        "site.ui_copy.home.current_proof.action_refs",
    );
    validate_link_refs(
        content,
        &home.open_source_teaser.action_refs,
        "site.ui_copy.home.open_source_teaser.action_refs",
    );
    assert_min_len(
        "site.ui_copy.home.skills.skill_group_ids",
        &home.skills.skill_group_ids,
        1,
    );
    for group_id in &home.skills.skill_group_ids {
        assert!(
            content.skill_groups.iter().any(|group| group.id == *group_id),
            "site.ui_copy.home.skills.skill_group_ids must resolve {}",
            group_id,
        );
    }
    validate_link_refs(
        content,
        &home.contact.action_refs,
        "site.ui_copy.home.contact.action_refs",
    );
}

fn validate_work_refs(content: &SiteContent) {
    let current_slugs = &content.ui_copy.work.current_proof.project_slugs;
    let supporting_slugs = &content.ui_copy.work.supporting_cases.project_slugs;

    validate_project_refs(
        content,
        current_slugs,
        "site.ui_copy.work.current_proof.project_slugs",
    );
    validate_link_refs(
        content,
        &content.ui_copy.work.current_proof.action_refs,
        "site.ui_copy.work.current_proof.action_refs",
    );
    validate_project_refs(
        content,
        supporting_slugs,
        "site.ui_copy.work.supporting_cases.project_slugs",
    );
    validate_link_refs(
        content,
        &content.ui_copy.work.supporting_cases.action_refs,
        "site.ui_copy.work.supporting_cases.action_refs",
    );
    for slug in current_slugs {
        assert!(
            slug.archive_anchor_id().is_none(),
            "site.ui_copy.work.current_proof.project_slugs must use dedicated proof routes",
        );
    }
    for slug in supporting_slugs {
        assert!(
            slug.archive_anchor_id().is_some(),
            "site.ui_copy.work.supporting_cases.project_slugs must use archive anchors",
        );
    }
    for slug in current_slugs {
        assert!(
            !supporting_slugs.contains(slug),
            "site.ui_copy.work current_proof and supporting_cases must stay disjoint",
        );
    }
    validate_link_refs(
        content,
        &content.ui_copy.work.open_source_teaser.action_refs,
        "site.ui_copy.work.open_source_teaser.action_refs",
    );
}

fn validate_open_source_refs(content: &SiteContent) {
    validate_open_source_hero_copy_refs(
        content,
        &content.ui_copy.open_source.hero,
        "site.ui_copy.open_source.hero",
    );
}

fn validate_lab_refs(content: &SiteContent) {
    let lab = &content.ui_copy.lab;
    validate_portfolio_hero_copy_refs(content, &lab.hero, "site.ui_copy.lab.hero");
    validate_link_refs(
        content,
        &lab.session_card.guest_action_refs,
        "site.ui_copy.lab.session_card.guest_action_refs",
    );
    validate_link_refs(
        content,
        &lab.guest_chat.action_refs,
        "site.ui_copy.lab.guest_chat.action_refs",
    );
}

fn validate_lab_copy(content: &SiteContent, lab: &super::types::LabPageCopy) {
    assert_non_empty("site.ui_copy.lab.page_title", &lab.page_title);
    validate_lab_refs(content);
    validate_lab_panel(&lab.operations_surface, "site.ui_copy.lab.operations_surface");
    validate_lab_panel(&lab.sensitive_proof, "site.ui_copy.lab.sensitive_proof");
    validate_info_section(
        &lab.engineering_quality,
        "site.ui_copy.lab.engineering_quality",
    );
    assert_non_empty("site.ui_copy.lab.session_card.title", &lab.session_card.title);
    assert_non_empty(
        "site.ui_copy.lab.session_card.guest_status",
        &lab.session_card.guest_status,
    );
    assert_non_empty(
        "site.ui_copy.lab.session_card.guest_summary",
        &lab.session_card.guest_summary,
    );
    assert_non_empty(
        "site.ui_copy.lab.session_card.signed_in_action_label",
        &lab.session_card.signed_in_action_label,
    );
    assert_min_len(
        "site.ui_copy.lab.session_card.guest_action_refs",
        &lab.session_card.guest_action_refs,
        1,
    );
    assert_non_empty("site.ui_copy.lab.guest_chat.title", &lab.guest_chat.title);
    assert_non_empty("site.ui_copy.lab.guest_chat.summary", &lab.guest_chat.summary);
}

fn validate_work_case_copy(
    content: &SiteContent,
    case: &super::types::WorkCaseCopy,
    _slug: WorkCaseSlug,
) {
    assert_non_empty("site.work_cases[].page_title", &case.page_title);
    assert_non_empty("site.work_cases[].eyebrow", &case.eyebrow);
    assert_non_empty("site.work_cases[].title", &case.title);
    assert_non_empty("site.work_cases[].summary", &case.summary);
    validate_case_list(&case.challenge, "site.work_cases[].challenge");
    validate_case_list(&case.implementation, "site.work_cases[].implementation");
    validate_case_list(&case.outcomes, "site.work_cases[].outcomes");
    validate_case_list(&case.stack, "site.work_cases[].stack");
    validate_link_refs(
        content,
        &case.action_refs,
        "site.work_cases[].action_refs",
    );
    assert!(
        !case.action_refs.is_empty(),
        "site.work_cases[].action_refs must contain at least 1 entry",
    );
}

fn validate_resume_refs(content: &SiteContent) {
    let resume = &content.ui_copy.resume;
    assert_min_len(
        "site.ui_copy.resume.experience_role_ids",
        &resume.experience_role_ids,
        1,
    );
    for role_id in &resume.experience_role_ids {
        assert!(
            content.experience_roles.iter().any(|role| role.id == *role_id),
            "site.ui_copy.resume.experience_role_ids must resolve {}",
            role_id,
        );
    }

    validate_project_refs(
        content,
        &resume.featured_project_slugs,
        "site.ui_copy.resume.featured_project_slugs",
    );

    assert_min_len(
        "site.ui_copy.resume.skill_group_ids",
        &resume.skill_group_ids,
        1,
    );
    for group_id in &resume.skill_group_ids {
        assert!(
            content.skill_groups.iter().any(|group| group.id == *group_id),
            "site.ui_copy.resume.skill_group_ids must resolve {}",
            group_id,
        );
    }

    assert_min_len(
        "site.ui_copy.resume.contact_method_ids",
        &resume.contact_method_ids,
        1,
    );
    for contact_id in &resume.contact_method_ids {
        assert!(
            content
                .contact_methods
                .iter()
                .any(|method| method.id == *contact_id),
            "site.ui_copy.resume.contact_method_ids must resolve {}",
            contact_id,
        );
    }
}

fn validate_project_refs(content: &SiteContent, slugs: &[WorkCaseSlug], path: &str) {
    assert_min_len(path, slugs, 1);
    for slug in slugs {
        assert!(
            content.projects.iter().any(|project| project.slug == *slug),
            "{path} must resolve {:?}",
            slug,
        );
    }
}

fn validate_link_refs(content: &SiteContent, references: &[LinkReference], path: &str) {
    for reference in references {
        validate_link_reference(content, reference, path);
    }
}

fn validate_link_reference(content: &SiteContent, reference: &LinkReference, path: &str) {
    match reference {
        LinkReference::Action { id } => {
            assert!(
                content.action_links.iter().any(|action| action.id == *id),
                "{path} must resolve action id {}",
                id,
            );
        }
        LinkReference::ContactMethod { id, label, .. } => {
            assert!(
                content.contact_methods.iter().any(|method| method.id == *id),
                "{path} must resolve contact method id {}",
                id,
            );
            if let Some(label) = label {
                assert_non_empty(&format!("{path}.label"), label);
            }
        }
        LinkReference::Bundle { id } => {
            assert!(
                content.action_bundles.iter().any(|bundle| bundle.id == *id),
                "{path} must resolve action bundle id {}",
                id,
            );
        }
    }
}

fn validate_direct_link_reference_shape(reference: &DirectLinkReference, path: &str) {
    match reference {
        DirectLinkReference::Action { id } => {
            assert_non_empty(&format!("{path}.id"), id);
        }
        DirectLinkReference::ContactMethod { id, label, .. } => {
            assert_non_empty(&format!("{path}.id"), id);
            if let Some(label) = label {
                assert_non_empty(&format!("{path}.label"), label);
            }
        }
    }
}

fn validate_direct_link_reference_targets(
    content: &SiteContent,
    reference: &DirectLinkReference,
    path: &str,
) {
    match reference {
        DirectLinkReference::Action { id } => {
            assert!(
                content.action_links.iter().any(|action| action.id == *id),
                "{path} must resolve action id {}",
                id,
            );
        }
        DirectLinkReference::ContactMethod { id, label, .. } => {
            assert!(
                content.contact_methods.iter().any(|method| method.id == *id),
                "{path} must resolve contact method id {}",
                id,
            );
            if let Some(label) = label {
                assert_non_empty(&format!("{path}.label"), label);
            }
        }
    }
}

fn validate_action_bundle(content: &SiteContent, bundle: &ActionBundleContent, path: &str) {
    assert_non_empty(&format!("{path}.id"), &bundle.id);
    assert_min_len(&format!("{path}.references"), &bundle.references, 1);
    for reference in &bundle.references {
        validate_direct_link_reference_targets(content, reference, &format!("{path}.references[]"));
    }
}

fn validate_portfolio_hero_copy_shape(
    hero: &super::types::PortfolioHeroCopy,
    path: &str,
) {
    assert_non_empty(&format!("{path}.eyebrow"), &hero.eyebrow);
    assert_non_empty(&format!("{path}.title"), &hero.title);
    assert_non_empty(&format!("{path}.summary"), &hero.summary);
    assert_min_len(&format!("{path}.badges"), &hero.badges, 1);
    for badge in &hero.badges {
        assert_non_empty(&format!("{path}.badges[]"), badge);
    }
    assert_min_len(&format!("{path}.action_refs"), &hero.action_refs, 1);
}

fn validate_open_source_hero_copy_shape(
    hero: &super::types::PortfolioHeroCopy,
    path: &str,
) {
    assert_non_empty(&format!("{path}.eyebrow"), &hero.eyebrow);
    assert_non_empty(&format!("{path}.title"), &hero.title);
    assert_non_empty(&format!("{path}.summary"), &hero.summary);
    for badge in &hero.badges {
        assert_non_empty(&format!("{path}.badges[]"), badge);
    }
}

fn validate_closing_copy_shape(content: &super::types::ClosingCopy, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    assert_min_len(&format!("{path}.action_refs"), &content.action_refs, 1);
}

fn validate_project_section_selection_shape(
    content: &super::types::ProjectSectionSelection,
    path: &str,
) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.project_slugs"), &content.project_slugs, 1);
}

fn validate_archive_details_shape(content: &ArchiveDetailsContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_non_empty(&format!("{path}.entry_label"), &content.entry_label);
}

fn validate_portfolio_hero_copy_refs(
    content: &SiteContent,
    hero: &super::types::PortfolioHeroCopy,
    path: &str,
) {
    validate_portfolio_hero_copy_shape(hero, path);
    validate_link_refs(content, &hero.action_refs, &format!("{path}.action_refs"));
}

fn validate_open_source_hero_copy_refs(
    content: &SiteContent,
    hero: &super::types::PortfolioHeroCopy,
    path: &str,
) {
    validate_open_source_hero_copy_shape(hero, path);
    validate_link_refs(content, &hero.action_refs, &format!("{path}.action_refs"));
}

fn validate_used_entries(content: &SiteContent) {
    let mut usage = ContentUsage::default();

    record_link_refs_usage(content, &content.nav_links, &mut usage);
    record_link_refs_usage(content, &content.ui_copy.home.hero.action_refs, &mut usage);
    record_link_refs_usage(
        content,
        &content.ui_copy.home.selected_projects.action_refs,
        &mut usage,
    );
    record_link_refs_usage(
        content,
        &content.ui_copy.home.current_proof.action_refs,
        &mut usage,
    );
    record_link_refs_usage(
        content,
        &content.ui_copy.home.open_source_teaser.action_refs,
        &mut usage,
    );
    record_link_refs_usage(content, &content.ui_copy.home.contact.action_refs, &mut usage);
    record_link_refs_usage(
        content,
        &content.ui_copy.work.current_proof.action_refs,
        &mut usage,
    );
    record_link_refs_usage(
        content,
        &content.ui_copy.work.supporting_cases.action_refs,
        &mut usage,
    );
    record_link_refs_usage(
        content,
        &content.ui_copy.work.open_source_teaser.action_refs,
        &mut usage,
    );
    record_link_refs_usage(content, &content.ui_copy.open_source.hero.action_refs, &mut usage);
    record_link_refs_usage(content, &content.ui_copy.lab.hero.action_refs, &mut usage);
    record_link_refs_usage(
        content,
        &content.ui_copy.lab.session_card.guest_action_refs,
        &mut usage,
    );
    record_link_refs_usage(content, &content.ui_copy.lab.guest_chat.action_refs, &mut usage);
    for case in &content.work_cases {
        record_link_refs_usage(content, &case.content.action_refs, &mut usage);
    }

    record_work_case_slugs(&content.ui_copy.home.selected_projects.project_slugs, &mut usage);
    record_work_case_slugs(&content.ui_copy.home.current_proof.project_slugs, &mut usage);
    record_work_case_slugs(&content.ui_copy.work.current_proof.project_slugs, &mut usage);
    record_work_case_slugs(&content.ui_copy.work.supporting_cases.project_slugs, &mut usage);
    record_work_case_slugs(&content.ui_copy.resume.featured_project_slugs, &mut usage);

    for method_id in &content.ui_copy.resume.contact_method_ids {
        usage.contact_method_ids.insert(method_id.to_string());
    }

    for action in &content.action_links {
        assert!(
            usage.action_ids.contains(&action.id.to_string()),
            "site.action_links must not include unused id {}",
            action.id,
        );
    }
    for bundle in &content.action_bundles {
        assert!(
            usage.action_bundle_ids.contains(&bundle.id.to_string()),
            "site.action_bundles must not include unused id {}",
            bundle.id,
        );
    }
    for method in &content.contact_methods {
        assert!(
            usage.contact_method_ids.contains(&method.id.to_string()),
            "site.contact_methods must not include unused id {}",
            method.id,
        );
    }
    for case in &content.work_cases {
        assert!(
            usage.work_case_slugs.contains(&case.slug),
            "site.work_cases must not include unreachable slug {:?}",
            case.slug,
        );
    }
}

#[derive(Default)]
struct ContentUsage {
    action_ids: HashSet<String>,
    action_bundle_ids: HashSet<String>,
    contact_method_ids: HashSet<String>,
    work_case_slugs: HashSet<WorkCaseSlug>,
}

fn record_link_refs_usage(content: &SiteContent, references: &[LinkReference], usage: &mut ContentUsage) {
    for reference in references {
        match reference {
            LinkReference::Action { id } => {
                usage.action_ids.insert(id.to_string());
            }
            LinkReference::ContactMethod { id, .. } => {
                usage.contact_method_ids.insert(id.to_string());
            }
            LinkReference::Bundle { id } => {
                usage.action_bundle_ids.insert(id.to_string());
                let bundle = content
                    .action_bundles
                    .iter()
                    .find(|bundle| bundle.id == *id)
                    .unwrap_or_else(|| panic!("site.action_bundles must include id {}", id));
                for reference in &bundle.references {
                    record_direct_link_ref_usage(reference, usage);
                }
            }
        }
    }
}

fn record_direct_link_ref_usage(reference: &DirectLinkReference, usage: &mut ContentUsage) {
    match reference {
        DirectLinkReference::Action { id } => {
            usage.action_ids.insert(id.to_string());
        }
        DirectLinkReference::ContactMethod { id, .. } => {
            usage.contact_method_ids.insert(id.to_string());
        }
    }
}

fn record_work_case_slugs(slugs: &[WorkCaseSlug], usage: &mut ContentUsage) {
    usage.work_case_slugs.extend(slugs.iter().copied());
}

fn action_targets_proof_path(action: &CmsActionLink, case_route: Route) -> bool {
    let href = action.href.to_string();
    let path = href.split(['#', '?']).next().unwrap_or(href.as_str());

    matches!(
        path.parse::<Route>().ok(),
        Some(route)
            if route == case_route || route == Route::Lab || route == Route::WorkSensitiveSync
    )
}

fn validate_portfolio_hero(content: &PortfolioHeroContent, path: &str) {
    assert_non_empty(&format!("{path}.eyebrow"), &content.eyebrow);
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    assert_min_len(&format!("{path}.badges"), &content.badges, 1);
    for badge in &content.badges {
        assert_non_empty(&format!("{path}.badges[]"), badge);
    }
    assert_min_len(&format!("{path}.actions"), &content.actions, 1);
    for action in &content.actions {
        validate_action(action, &format!("{path}.actions[]"));
    }
}

fn validate_open_source_hero(content: &PortfolioHeroContent, path: &str) {
    assert_non_empty(&format!("{path}.eyebrow"), &content.eyebrow);
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    for badge in &content.badges {
        assert_non_empty(&format!("{path}.badges[]"), badge);
    }
    for action in &content.actions {
        validate_action(action, &format!("{path}.actions[]"));
    }
}

fn validate_experience_section(content: &ExperienceSectionContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.roles"), &content.roles, 1);
    for role in &content.roles {
        validate_experience_role(role, &format!("{path}.roles[]"));
    }
}

fn validate_experience_role(content: &ExperienceRoleContent, path: &str) {
    assert_non_empty(&format!("{path}.id"), &content.id);
    assert_non_empty(&format!("{path}.company"), &content.company);
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.tenure"), &content.tenure);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    assert_min_len(&format!("{path}.highlights"), &content.highlights, 1);
    for item in &content.highlights {
        assert_non_empty(&format!("{path}.highlights[]"), item);
    }
    for action in &content.actions {
        validate_action(action, &format!("{path}.actions[]"));
    }
}

fn validate_work_section(content: &WorkSectionContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.cards"), &content.cards, 1);
    for card in &content.cards {
        validate_work_card(card, &format!("{path}.cards[]"));
    }
    for action in &content.actions {
        validate_action(action, &format!("{path}.actions[]"));
    }
}

fn validate_skill_section(content: &SkillSectionContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.groups"), &content.groups, 1);
    for group in &content.groups {
        validate_skill_group(group, &format!("{path}.groups[]"));
    }
}

fn validate_skill_group(content: &SkillGroupContent, path: &str) {
    assert_non_empty(&format!("{path}.id"), &content.id);
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_min_len(&format!("{path}.items"), &content.items, 1);
    for item in &content.items {
        assert_non_empty(&format!("{path}.items[]"), item);
    }
}

fn validate_contact_method(content: &ContactMethodContent, path: &str) {
    assert_non_empty(&format!("{path}.id"), &content.id);
    assert_non_empty(&format!("{path}.label"), &content.label);
    assert_non_empty(&format!("{path}.value"), &content.value);
    assert_non_empty(&format!("{path}.href"), &content.href);
    validate_href(
        path,
        &CmsActionLink {
            label: content.label.clone(),
            href: content.href.clone(),
            kind: content.kind,
            tone: super::types::CtaKind::Secondary,
        },
    );
}

fn validate_lab_panel(content: &LabPanelContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_non_empty(&format!("{path}.empty_message"), &content.empty_message);
    if let Some(label) = &content.action_label {
        assert_non_empty(&format!("{path}.action_label"), label);
    }
}

fn validate_session_card(content: &SessionCardContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.guest_status"), &content.guest_status);
    assert_non_empty(&format!("{path}.guest_summary"), &content.guest_summary);
    assert_non_empty(
        &format!("{path}.signed_in_action_label"),
        &content.signed_in_action_label,
    );
    assert_min_len(&format!("{path}.guest_actions"), &content.guest_actions, 1);
    for action in &content.guest_actions {
        validate_action(action, &format!("{path}.guest_actions[]"));
    }
}

fn validate_info_section(content: &InfoSectionContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.cards"), &content.cards, 1);
    for card in &content.cards {
        validate_info_card(card, &format!("{path}.cards[]"));
    }
}

fn validate_info_card(content: &InfoCardContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    assert_min_len(&format!("{path}.points"), &content.points, 1);
    for point in &content.points {
        assert_non_empty(&format!("{path}.points[]"), point);
    }
}

fn validate_crate_section(content: &CrateSectionContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.cards"), &content.cards, 1);
    for card in &content.cards {
        validate_crate_card(card, &format!("{path}.cards[]"));
    }
}

fn validate_closing(content: &ClosingContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    assert_min_len(&format!("{path}.actions"), &content.actions, 1);
    for action in &content.actions {
        validate_action(action, &format!("{path}.actions[]"));
    }
}

fn validate_case_list(content: &CaseListSection, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_min_len(&format!("{path}.items"), &content.items, 1);
    for item in &content.items {
        assert_non_empty(&format!("{path}.items[]"), item);
    }
}

fn validate_work_card(content: &WorkCardContent, path: &str) {
    assert_non_empty(&format!("{path}.category"), &content.category);
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    if let Some(outcome) = &content.outcome {
        assert_non_empty(&format!("{path}.outcome"), outcome);
    }
    assert_non_empty(&format!("{path}.cta_label"), &content.cta_label);
    assert_min_len(&format!("{path}.highlights"), &content.highlights, 1);
    for item in &content.highlights {
        assert_non_empty(&format!("{path}.highlights[]"), item);
    }
    for tag in &content.stack_tags {
        assert_non_empty(&format!("{path}.stack_tags[]"), tag);
    }
    if let Some(preview) = &content.preview {
        validate_image_asset(preview, &format!("{path}.preview"));
    }
}

fn validate_crate_card(content: &CrateCardContent, path: &str) {
    assert_non_empty(&format!("{path}.name"), &content.name);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    assert_min_len(&format!("{path}.highlights"), &content.highlights, 1);
    for item in &content.highlights {
        assert_non_empty(&format!("{path}.highlights[]"), item);
    }
    if let Some(gallery) = &content.gallery {
        validate_crate_gallery(gallery, &format!("{path}.gallery"));
    }
    assert_non_empty(&format!("{path}.repository_url"), &content.repository_url);
    assert_non_empty(
        &format!("{path}.repository_label"),
        &content.repository_label,
    );
    if content.docs_url.is_some() {
        assert!(
            content.docs_label.is_some(),
            "{path}.docs_label must be present when docs_url is provided",
        );
    }
    if let Some(label) = &content.docs_label {
        assert_non_empty(&format!("{path}.docs_label"), label);
    }
}

fn validate_crate_gallery(content: &CrateGalleryContent, path: &str) {
    assert_non_empty(&format!("{path}.id"), &content.id);
    assert_non_empty(&format!("{path}.aria_label"), &content.aria_label);
    assert_min_len(&format!("{path}.tabs"), &content.tabs, 1);

    let mut tab_ids = HashSet::new();
    for tab in &content.tabs {
        assert_non_empty(&format!("{path}.tabs[].id"), &tab.id);
        assert!(
            tab_ids.insert(tab.id.clone()),
            "{path}.tabs[].id must be unique within the gallery",
        );
        assert_non_empty(&format!("{path}.tabs[].label.primary"), &tab.label.primary);
        if let Some(secondary) = &tab.label.secondary {
            assert_non_empty(&format!("{path}.tabs[].label.secondary"), secondary);
        }

        let preview = &tab.preview;
        validate_gallery_preview(preview, &format!("{path}.tabs[].preview"));

        let body = &tab.body;
        assert_non_empty(&format!("{path}.tabs[].body.title"), &body.title);
        if let Some(subtitle) = &body.subtitle {
            assert_non_empty(&format!("{path}.tabs[].body.subtitle"), subtitle);
        }
        assert_min_len(
            &format!("{path}.tabs[].body.features"),
            &body.features,
            1,
        );
        for feature in &body.features {
            assert_non_empty(&format!("{path}.tabs[].body.features[].text"), &feature.text);
        }
    }
}

fn validate_gallery_preview(
    content: &super::crate_gallery::CrateGalleryPreviewContent,
    path: &str,
) {
    assert_min_len(
        &format!("{path}.code_examples"),
        &content.code_examples,
        1,
    );
    for example in &content.code_examples {
        assert_non_empty(&format!("{path}.code_examples[].code"), &example.code);
        if let Some(label) = &example.label {
            assert_non_empty(&format!("{path}.code_examples[].label"), label);
        }
    }

    if let Some(image) = &content.image {
        assert_non_empty(&format!("{path}.image.asset_ref"), &image.asset_ref);
    }

    if let Some(badge) = &content.badge {
        assert_non_empty(&format!("{path}.badge.text"), &badge.text);
    }
}

fn validate_image_asset(content: &CmsImageAsset, path: &str) {
    assert_non_empty(&format!("{path}.asset_ref"), &content.asset_ref);
    assert_non_empty(&format!("{path}.alt"), &content.alt);
}

fn validate_action(content: &CmsActionLink, path: &str) {
    assert_non_empty(&format!("{path}.label"), &content.label);
    assert_non_empty(&format!("{path}.href"), &content.href);
    validate_href(path, content);
}

fn validate_href(path: &str, action: &CmsActionLink) {
    if action.kind.is_external() {
        return;
    }

    let href = action.href.to_string();
    if href.starts_with('#') {
        assert!(
            href.len() > 1,
            "{path}.href must target a known internal route or local anchor",
        );
        return;
    }

    let route_path = href.split(['#', '?']).next().unwrap_or(href.as_str());
    assert!(
        route_path.parse::<Route>().is_ok(),
        "{path}.href must target a known internal route or local anchor",
    );
}

fn assert_non_empty(path: &str, value: &Text) {
    assert!(
        !value.to_string().trim().is_empty(),
        "{path} must not be empty",
    );
}

fn assert_min_len<T>(path: &str, values: &[T], min_len: usize) {
    assert!(
        values.len() >= min_len,
        "{path} must contain at least {min_len} entries",
    );
}

fn assert_unique_keys(path: &str, values: impl IntoIterator<Item = String>) {
    let mut seen = HashSet::new();
    for value in values {
        assert!(seen.insert(value), "{path} must be unique");
    }
}

fn assert_unique_text_ids<'a>(path: &str, values: impl IntoIterator<Item = &'a Text>) {
    assert_unique_keys(path, values.into_iter().map(ToString::to_string));
}

#[cfg(test)]
mod tests {
    use super::*;

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
    #[should_panic(expected = "site.ui_copy.home.current_proof.action_refs must resolve action bundle id")]
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
        content.action_bundles[0].references[0] = super::super::types::DirectLinkReference::Action {
            id: Text::from("missing-action"),
        };

        validate_site_content(&content);
    }

    #[test]
    #[should_panic(expected = "site.action_links[].href must target a known internal route or local anchor")]
    fn site_content_rejects_invalid_internal_route_targets() {
        let mut content = super::super::fixture_loader::site_content().clone();
        content.action_links[0].link.href = Text::from("/not-a-real-route");

        validate_site_content(&content);
    }

    #[test]
    #[should_panic(expected = "site.ui_copy.work.current_proof.project_slugs must use dedicated proof routes")]
    fn site_content_rejects_current_proof_without_dedicated_routes() {
        let mut content = super::super::fixture_loader::site_content().clone();
        content.ui_copy.work.current_proof.project_slugs[0] =
            super::super::types::WorkCaseSlug::ChatRealtime;

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
    #[should_panic(expected = "site.contact_methods[].href must target a known internal route or local anchor")]
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
}
