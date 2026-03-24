use super::shared::*;

pub(in super::super) fn validate_identity_fragment(content: &IdentityContent) {
    assert_non_empty("site_content/identity.json.name", &content.name);
    assert_non_empty("site_content/identity.json.location", &content.location);
    assert_non_empty("site_content/identity.json.headline", &content.headline);
}

pub(in super::super) fn validate_action_library_fragment(content: &ActionLibraryFragment) {
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
        validate_action(&action.link, "site_content/actions.json.action_links[]");
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

pub(in super::super) fn validate_nav_fragment(content: &[LinkReference]) {
    assert_min_len("site_content/nav.json", content, 1);
}

pub(in super::super) fn validate_contact_fragment(content: &[ContactMethodContent]) {
    assert_min_len("site_content/contact.json", content, 1);
    assert_unique_text_ids(
        "site_content/contact.json[].id",
        content.iter().map(|method| &method.id),
    );
    for method in content {
        validate_contact_method(method, "site_content/contact.json[]");
    }
}

pub(in super::super) fn validate_experience_fragment(content: &[ExperienceRoleContent]) {
    assert_min_len("site_content/experience.json", content, 1);
    assert_unique_text_ids(
        "site_content/experience.json[].id",
        content.iter().map(|role| &role.id),
    );
    for role in content {
        validate_experience_role(role, "site_content/experience.json[]");
    }
}

pub(in super::super) fn validate_projects_fragment(content: &[WorkCardContent]) {
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

pub(in super::super) fn validate_work_cases_fragment(content: &[WorkCaseRecord]) {
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

pub(in super::super) fn validate_open_source_entries_fragment(content: &[CrateCardContent]) {
    assert_min_len("site_content/open_source.json", content, 1);
    assert_unique_keys(
        "site_content/open_source.json[].name",
        content.iter().map(|entry| entry.name.to_string()),
    );
    for entry in content {
        validate_crate_card(entry, "site_content/open_source.json[]");
    }
}

pub(in super::super) fn validate_skill_groups_fragment(content: &[SkillGroupContent]) {
    assert_min_len("site_content/skills.json", content, 1);
    assert_unique_text_ids(
        "site_content/skills.json[].id",
        content.iter().map(|group| &group.id),
    );
    for group in content {
        validate_skill_group(group, "site_content/skills.json[]");
    }
}

pub(in super::super) fn validate_home_page_fragment(content: &HomePageCopy) {
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

pub(in super::super) fn validate_work_page_fragment(content: &WorkIndexCopy) {
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

pub(in super::super) fn validate_open_source_page_fragment(content: &OpenSourceIndexCopy) {
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

pub(in super::super) fn validate_lab_page_fragment(content: &LabPageCopy) {
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

pub(in super::super) fn validate_resume_page_fragment(content: &ResumeDocumentContent) {
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
