use super::*;

pub(super) fn validate_used_entries(content: &SiteContent) {
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

pub(super) fn action_targets_proof_path(action: &CmsActionLink, case_route: Route) -> bool {
    let href = action.href.to_string();
    let path = href.split(['#', '?']).next().unwrap_or(href.as_str());

    matches!(
        path.parse::<Route>().ok(),
        Some(route)
            if route == case_route || route == Route::Lab || route == Route::WorkSensitiveSync
    )
}
