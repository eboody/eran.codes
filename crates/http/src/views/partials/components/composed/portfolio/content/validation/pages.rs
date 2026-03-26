use super::shared::*;

pub(in super::super) fn validate_portfolio_home(content: &PortfolioHomeContent) {
    assert_non_empty("home.page_title", &content.page_title);
    validate_portfolio_hero(&content.hero, "home.hero");
    validate_experience_section(&content.experience_section, "home.experience_section");
    validate_work_section(&content.project_section, "home.project_section");
    validate_work_section(&content.current_proof_section, "home.current_proof_section");
    validate_closing(&content.open_source_teaser, "home.open_source_teaser");
    validate_skill_section(&content.skill_section, "home.skill_section");
    validate_closing(&content.contact_section, "home.contact_section");
}

pub(in super::super) fn validate_work_index(content: &WorkIndexContent) {
    assert_non_empty("work.page_title", &content.page_title);
    validate_optional_portfolio_hero(&content.hero, "work.hero");
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

pub(in super::super) fn validate_open_source_index(content: &OpenSourceIndexContent) {
    assert_non_empty("open_source.page_title", &content.page_title);
    validate_optional_portfolio_hero(&content.hero, "open_source.hero");
    validate_crate_section(&content.crate_section, "open_source.crate_section");
}

pub(in super::super) fn validate_lab_page(content: &LabPageContent) {
    assert_non_empty("lab.page_title", &content.page_title);
    validate_portfolio_hero(&content.hero, "lab.hero");
    validate_session_card(&content.session_card, "lab.session_card");
    validate_closing(&content.guest_chat, "lab.guest_chat");
    validate_lab_panel(&content.operations_surface, "lab.operations_surface");
    validate_lab_panel(&content.sensitive_proof, "lab.sensitive_proof");
    validate_info_section(&content.engineering_quality, "lab.engineering_quality");
}

pub(in super::super) fn validate_work_case(content: &WorkCaseContent, slug: WorkCaseSlug) {
    assert_non_empty("work_case.page_title", &content.page_title);
    validate_optional_portfolio_hero(&content.hero, "work_case.hero");
    assert_eq!(
        content.detail_layout,
        slug.detail_layout(),
        "work_case.detail_layout must match slug {:?}",
        slug,
    );
    validate_case_list(&content.challenge, "work_case.challenge");
    validate_case_list(&content.implementation, "work_case.implementation");
    validate_case_list(&content.outcomes, "work_case.outcomes");
    validate_case_list(&content.stack, "work_case.stack");

    let case_route = slug.route();
    assert!(
        content
            .hero
            .actions
            .iter()
            .any(|action| super::usage::action_targets_proof_path(action, case_route)),
        "work case {slug:?} should include at least one action to itself, current proof, or /lab",
    );
}

pub(super) fn validate_portfolio_hero_copy_shape(
    hero: &PortfolioHeroCopy,
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

pub(super) fn validate_open_source_hero_copy_shape(
    hero: &PortfolioHeroCopy,
    path: &str,
) {
    assert_non_empty(&format!("{path}.eyebrow"), &hero.eyebrow);
    assert_non_empty(&format!("{path}.title"), &hero.title);
    assert_non_empty(&format!("{path}.summary"), &hero.summary);
    for badge in &hero.badges {
        assert_non_empty(&format!("{path}.badges[]"), badge);
    }
}

pub(super) fn validate_closing_copy_shape(content: &ClosingCopy, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    assert_min_len(&format!("{path}.action_refs"), &content.action_refs, 1);
}

pub(super) fn validate_project_section_selection_shape(
    content: &ProjectSectionSelection,
    path: &str,
) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.project_slugs"), &content.project_slugs, 1);
}

pub(super) fn validate_archive_details_shape(content: &ArchiveDetailsContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_non_empty(&format!("{path}.entry_label"), &content.entry_label);
}

pub(super) fn validate_portfolio_hero(content: &PortfolioHeroContent, path: &str) {
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

pub(super) fn validate_optional_portfolio_hero(content: &PortfolioHeroContent, path: &str) {
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

pub(super) fn validate_experience_section(content: &ExperienceSectionContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.roles"), &content.roles, 1);
    for role in &content.roles {
        validate_experience_role(role, &format!("{path}.roles[]"));
    }
}

pub(super) fn validate_experience_role(content: &ExperienceRoleContent, path: &str) {
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

pub(super) fn validate_work_section(content: &WorkSectionContent, path: &str) {
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

pub(super) fn validate_skill_section(content: &SkillSectionContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.groups"), &content.groups, 1);
    for group in &content.groups {
        validate_skill_group(group, &format!("{path}.groups[]"));
    }
}

pub(super) fn validate_skill_group(content: &SkillGroupContent, path: &str) {
    assert_non_empty(&format!("{path}.id"), &content.id);
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_min_len(&format!("{path}.items"), &content.items, 1);
    for item in &content.items {
        assert_non_empty(&format!("{path}.items[]"), item);
    }
}

pub(super) fn validate_contact_method(content: &ContactMethodContent, path: &str) {
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
            tone: CtaKind::Secondary,
        },
    );
}

pub(super) fn validate_lab_panel(content: &LabPanelContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_non_empty(&format!("{path}.empty_message"), &content.empty_message);
    if let Some(label) = &content.action_label {
        assert_non_empty(&format!("{path}.action_label"), label);
    }
}

pub(super) fn validate_session_card(content: &SessionCardContent, path: &str) {
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

pub(super) fn validate_info_section(content: &InfoSectionContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.cards"), &content.cards, 1);
    for card in &content.cards {
        validate_info_card(card, &format!("{path}.cards[]"));
    }
}

pub(super) fn validate_info_card(content: &InfoCardContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    assert_min_len(&format!("{path}.points"), &content.points, 1);
    for point in &content.points {
        assert_non_empty(&format!("{path}.points[]"), point);
    }
}

pub(super) fn validate_crate_section(content: &CrateSectionContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.cards"), &content.cards, 1);
    for card in &content.cards {
        validate_crate_card(card, &format!("{path}.cards[]"));
    }
}

pub(super) fn validate_closing(content: &ClosingContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    assert_min_len(&format!("{path}.actions"), &content.actions, 1);
    for action in &content.actions {
        validate_action(action, &format!("{path}.actions[]"));
    }
}

pub(super) fn validate_case_list(content: &CaseListSection, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_min_len(&format!("{path}.items"), &content.items, 1);
    for item in &content.items {
        assert_non_empty(&format!("{path}.items[]"), item);
    }
}

pub(super) fn validate_work_card(content: &WorkCardContent, path: &str) {
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

pub(super) fn validate_crate_card(content: &CrateCardContent, path: &str) {
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
    assert_non_empty(&format!("{path}.repository_label"), &content.repository_label);
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

pub(super) fn validate_crate_gallery(content: &CrateGalleryContent, path: &str) {
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
        assert_min_len(&format!("{path}.tabs[].body.features"), &body.features, 1);
        for feature in &body.features {
            assert_non_empty(&format!("{path}.tabs[].body.features[].text"), &feature.text);
        }
    }
}

pub(super) fn validate_gallery_preview(
    content: &PreviewContent,
    path: &str,
) {
    assert_min_len(&format!("{path}.code_examples"), &content.code_examples, 1);
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

pub(super) fn validate_image_asset(content: &CmsImageAsset, path: &str) {
    assert_non_empty(&format!("{path}.asset_ref"), &content.asset_ref);
    assert_non_empty(&format!("{path}.alt"), &content.alt);
}

pub(super) fn validate_action(content: &CmsActionLink, path: &str) {
    assert_non_empty(&format!("{path}.label"), &content.label);
    assert_non_empty(&format!("{path}.href"), &content.href);
    validate_href(path, content);
}

pub(super) fn validate_href(path: &str, action: &CmsActionLink) {
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
