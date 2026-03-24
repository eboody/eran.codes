use super::*;

pub(super) fn validate_home_refs(content: &SiteContent) {
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

pub(super) fn validate_work_refs(content: &SiteContent) {
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

pub(super) fn validate_open_source_refs(content: &SiteContent) {
    validate_open_source_hero_copy_refs(
        content,
        &content.ui_copy.open_source.hero,
        "site.ui_copy.open_source.hero",
    );
}

pub(super) fn validate_lab_refs(content: &SiteContent) {
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

pub(super) fn validate_lab_copy(content: &SiteContent, lab: &LabPageCopy) {
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

pub(super) fn validate_work_case_copy(
    content: &SiteContent,
    case: &WorkCaseCopy,
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
    validate_link_refs(content, &case.action_refs, "site.work_cases[].action_refs");
    assert!(
        !case.action_refs.is_empty(),
        "site.work_cases[].action_refs must contain at least 1 entry",
    );
}

pub(super) fn validate_resume_refs(content: &SiteContent) {
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

pub(super) fn validate_project_refs(content: &SiteContent, slugs: &[WorkCaseSlug], path: &str) {
    assert_min_len(path, slugs, 1);
    for slug in slugs {
        assert!(
            content.projects.iter().any(|project| project.slug == *slug),
            "{path} must resolve {:?}",
            slug,
        );
    }
}

pub(super) fn validate_link_refs(content: &SiteContent, references: &[LinkReference], path: &str) {
    for reference in references {
        validate_link_reference(content, reference, path);
    }
}

pub(super) fn validate_link_reference(
    content: &SiteContent,
    reference: &LinkReference,
    path: &str,
) {
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

pub(super) fn validate_direct_link_reference_shape(
    reference: &DirectLinkReference,
    path: &str,
) {
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

pub(super) fn validate_direct_link_reference_targets(
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

pub(super) fn validate_action_bundle(
    content: &SiteContent,
    bundle: &ActionBundleContent,
    path: &str,
) {
    assert_non_empty(&format!("{path}.id"), &bundle.id);
    assert_min_len(&format!("{path}.references"), &bundle.references, 1);
    for reference in &bundle.references {
        validate_direct_link_reference_targets(
            content,
            reference,
            &format!("{path}.references[]"),
        );
    }
}

pub(super) fn validate_portfolio_hero_copy_refs(
    content: &SiteContent,
    hero: &PortfolioHeroCopy,
    path: &str,
) {
    validate_portfolio_hero_copy_shape(hero, path);
    validate_link_refs(content, &hero.action_refs, &format!("{path}.action_refs"));
}

pub(super) fn validate_open_source_hero_copy_refs(
    content: &SiteContent,
    hero: &PortfolioHeroCopy,
    path: &str,
) {
    validate_open_source_hero_copy_shape(hero, path);
    validate_link_refs(content, &hero.action_refs, &format!("{path}.action_refs"));
}
