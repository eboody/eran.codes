use std::collections::HashSet;
use std::fmt::Write;
use std::str::FromStr;
use std::sync::OnceLock;

use crate::paths::Route;

use super::types::{
    CmsActionLink, ContactMethodContent, CrateSectionContent, ExperienceRoleContent,
    LabPageContent, LinkReference, OpenSourceIndexContent, PortfolioHeroContent,
    PortfolioHomeContent, SessionCardContent, SiteContent, SkillGroupContent, WorkCaseContent,
    WorkCaseSlug, WorkIndexContent, WorkSectionContent,
};

pub fn site_content() -> &'static SiteContent {
    static CONTENT: OnceLock<SiteContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let content: SiteContent = serde_json::from_str(include_str!("site_content/portfolio.json"))
            .unwrap_or_else(|error| panic!("site_content/portfolio.json must be valid JSON: {error}"));
        validate_site_content(&content);
        content
    })
}

pub fn portfolio_nav_links() -> &'static [CmsActionLink] {
    static CONTENT: OnceLock<Vec<CmsActionLink>> = OnceLock::new();

    CONTENT
        .get_or_init(|| resolve_link_refs(site_content(), &site_content().nav_links))
        .as_slice()
}

pub fn lab_page_content() -> &'static LabPageContent {
    static CONTENT: OnceLock<LabPageContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let site = site_content();
        let lab = &site.ui_copy.lab;

        LabPageContent {
            page_title: lab.page_title.clone(),
            hero: build_portfolio_hero(site, &lab.hero),
            session_card: SessionCardContent {
                title: lab.session_card.title.clone(),
                guest_status: lab.session_card.guest_status.clone(),
                guest_summary: lab.session_card.guest_summary.clone(),
                signed_in_action_label: lab.session_card.signed_in_action_label.clone(),
                guest_actions: resolve_link_refs(site, &lab.session_card.guest_action_refs),
            },
            guest_chat: build_closing(site, &lab.guest_chat),
            operations_surface: lab.operations_surface.clone(),
            sensitive_proof: lab.sensitive_proof.clone(),
            engineering_quality: lab.engineering_quality.clone(),
        }
    })
}

pub fn portfolio_home_content() -> &'static PortfolioHomeContent {
    static CONTENT: OnceLock<PortfolioHomeContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let site = site_content();
        let home = &site.ui_copy.home;

        PortfolioHomeContent {
            page_title: home.page_title.clone(),
            hero: build_portfolio_hero(site, &home.hero),
            current_proof_section: WorkSectionContent {
                title: home.current_proof.title.clone(),
                subtitle: home.current_proof.subtitle.clone(),
                cards: project_cards_for_slugs(site, &home.current_proof.project_slugs),
                actions: resolve_link_refs(site, &home.current_proof.action_refs),
            },
        }
    })
}

pub fn work_index_content() -> &'static WorkIndexContent {
    static CONTENT: OnceLock<WorkIndexContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let site = site_content();
        let work = &site.ui_copy.work;

        WorkIndexContent {
            page_title: work.page_title.clone(),
            hero: PortfolioHeroContent {
                eyebrow: work.eyebrow.clone(),
                title: work.title.clone(),
                summary: work.summary.clone(),
                badges: vec![],
                actions: vec![],
            },
            supporting_cases_section: WorkSectionContent {
                title: work.supporting_cases.title.clone(),
                subtitle: work.supporting_cases.subtitle.clone(),
                cards: project_cards_for_slugs(site, &work.supporting_cases.project_slugs),
                actions: resolve_link_refs(site, &work.supporting_cases.action_refs),
            },
        }
    })
}

pub fn open_source_index_content() -> &'static OpenSourceIndexContent {
    static CONTENT: OnceLock<OpenSourceIndexContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let site = site_content();
        let open_source = &site.ui_copy.open_source;

        OpenSourceIndexContent {
            page_title: open_source.page_title.clone(),
            hero: build_portfolio_hero(site, &open_source.hero),
            crate_section: CrateSectionContent {
                title: open_source.crate_section.title.clone(),
                subtitle: open_source.crate_section.subtitle.clone(),
                cards: site.open_source_entries.clone(),
            },
        }
    })
}

pub fn work_case_content(slug: WorkCaseSlug) -> &'static WorkCaseContent {
    static CHAT_REALTIME: OnceLock<WorkCaseContent> = OnceLock::new();
    static COMMAND_SSE: OnceLock<WorkCaseContent> = OnceLock::new();
    static OPERATIONAL_VISIBILITY: OnceLock<WorkCaseContent> = OnceLock::new();
    static SENSITIVE_SYNC: OnceLock<WorkCaseContent> = OnceLock::new();

    match slug {
        WorkCaseSlug::ChatRealtime => CHAT_REALTIME.get_or_init(|| load_work_case(slug)),
        WorkCaseSlug::CommandSse => COMMAND_SSE.get_or_init(|| load_work_case(slug)),
        WorkCaseSlug::OperationalVisibility => {
            OPERATIONAL_VISIBILITY.get_or_init(|| load_work_case(slug))
        }
        WorkCaseSlug::SensitiveSync => SENSITIVE_SYNC.get_or_init(|| load_work_case(slug)),
    }
}

pub fn resume_text() -> &'static str {
    static CONTENT: OnceLock<String> = OnceLock::new();

    CONTENT
        .get_or_init(|| build_resume_text(site_content()))
        .as_str()
}

fn load_work_case(slug: WorkCaseSlug) -> WorkCaseContent {
    let content = site_content()
        .work_cases
        .iter()
        .find(|case| case.slug == slug)
        .unwrap_or_else(|| panic!("site_content.work_cases must include {slug:?}"))
        .content
        .clone();
    let site = site_content();

    WorkCaseContent {
        page_title: content.page_title,
        hero: PortfolioHeroContent {
            eyebrow: content.eyebrow,
            title: content.title,
            summary: content.summary,
            badges: vec![],
            actions: resolve_link_refs(site, &content.action_refs),
        },
        detail_layout: slug.detail_layout(),
        challenge: content.challenge,
        implementation: content.implementation,
        outcomes: content.outcomes,
        stack: content.stack,
    }
}

fn experience_roles_for_ids(
    site: &SiteContent,
    role_ids: &[crate::types::Text],
) -> Vec<ExperienceRoleContent> {
    role_ids
        .iter()
        .map(|role_id| {
            site.experience_roles
                .iter()
                .find(|role| role.id == *role_id)
                .unwrap_or_else(|| {
                    panic!("site_content.experience_roles must include id {}", role_id)
                })
                .clone()
        })
        .collect()
}

fn skill_groups_for_ids(
    site: &SiteContent,
    skill_group_ids: &[crate::types::Text],
) -> Vec<SkillGroupContent> {
    skill_group_ids
        .iter()
        .map(|group_id| {
            site.skill_groups
                .iter()
                .find(|group| group.id == *group_id)
                .unwrap_or_else(|| {
                    panic!("site_content.skill_groups must include id {}", group_id)
                })
                .clone()
        })
        .collect()
}

fn project_cards_for_slugs(
    site: &SiteContent,
    slugs: &[WorkCaseSlug],
) -> Vec<super::types::WorkCardContent> {
    slugs
        .iter()
        .map(|slug| {
            site.projects
                .iter()
                .find(|project| project.slug == *slug)
                .unwrap_or_else(|| panic!("site_content.projects must include {slug:?}"))
                .clone()
        })
        .collect()
}

fn build_resume_text(site: &SiteContent) -> String {
    let resume = &site.ui_copy.resume;
    let mut output = String::new();

    writeln!(&mut output, "# {}", site.identity.name).expect("resume write");
    writeln!(&mut output, "{}", site.identity.location).expect("resume write");
    writeln!(&mut output, "{}", site.identity.headline).expect("resume write");
    for method in contact_methods_for_ids(site, &resume.contact_method_ids) {
        writeln!(&mut output, "{}: {}", method.label, method.value).expect("resume write");
    }
    output.push('\n');

    writeln!(&mut output, "## {}", resume.summary_title).expect("resume write");
    writeln!(&mut output, "{}", resume.summary).expect("resume write");
    output.push('\n');

    writeln!(&mut output, "## {}", resume.experience_title).expect("resume write");
    for role in experience_roles_for_ids(site, &resume.experience_role_ids) {
        writeln!(&mut output, "### {} — {}", role.company, role.title)
            .expect("resume write");
        writeln!(&mut output, "{}", role.tenure).expect("resume write");
        for highlight in role.highlights {
            writeln!(&mut output, "- {}", highlight).expect("resume write");
        }
        output.push('\n');
    }

    writeln!(&mut output, "## {}", resume.projects_title).expect("resume write");
    for project in project_cards_for_slugs(site, &resume.featured_project_slugs) {
        writeln!(&mut output, "### {}", project.title).expect("resume write");
        writeln!(&mut output, "- {}", project.summary).expect("resume write");
        if let Some(outcome) = project.outcome {
            writeln!(&mut output, "- {}", outcome).expect("resume write");
        }
        output.push('\n');
    }

    writeln!(&mut output, "## {}", resume.open_source_title).expect("resume write");
    for entry in &site.open_source_entries {
        writeln!(&mut output, "- {}: {}", entry.name, entry.summary).expect("resume write");
    }
    output.push('\n');

    writeln!(&mut output, "## {}", resume.client_context_title).expect("resume write");
    for client in &resume.client_context {
        writeln!(&mut output, "- {}", client).expect("resume write");
    }
    output.push('\n');

    writeln!(&mut output, "## {}", resume.skills_title).expect("resume write");
    for group in skill_groups_for_ids(site, &resume.skill_group_ids) {
        writeln!(&mut output, "### {}", group.title).expect("resume write");
        writeln!(&mut output, "- {}", join_text_items(&group.items)).expect("resume write");
    }

    output
}

fn contact_methods_for_ids(
    site: &SiteContent,
    contact_method_ids: &[crate::types::Text],
) -> Vec<ContactMethodContent> {
    contact_method_ids
        .iter()
        .map(|contact_id| {
            site.contact_methods
                .iter()
                .find(|method| method.id == *contact_id)
                .unwrap_or_else(|| {
                    panic!(
                        "site_content.contact_methods must include id {}",
                        contact_id
                    )
                })
                .clone()
        })
        .collect()
}

fn join_text_items(items: &[crate::types::Text]) -> String {
    items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn resolve_link_refs(site: &SiteContent, references: &[LinkReference]) -> Vec<CmsActionLink> {
    let mut resolved = Vec::new();

    for reference in references {
        match reference {
            LinkReference::Action { id } => resolved.push(resolve_action_link(site, id)),
            LinkReference::ContactMethod { id, label, tone } => {
                resolved.push(resolve_contact_method_link(site, id, label.as_ref(), *tone));
            }
            LinkReference::Bundle { id } => {
                let bundle = site
                    .action_bundles
                    .iter()
                    .find(|bundle| bundle.id == *id)
                    .unwrap_or_else(|| {
                        panic!("site_content.action_bundles must include id {}", id)
                    });

                for reference in &bundle.references {
                    resolved.push(resolve_direct_link_ref(site, reference));
                }
            }
        }
    }

    resolved
}

fn build_portfolio_hero(
    site: &SiteContent,
    content: &super::types::PortfolioHeroCopy,
) -> PortfolioHeroContent {
    PortfolioHeroContent {
        eyebrow: content.eyebrow.clone(),
        title: content.title.clone(),
        summary: content.summary.clone(),
        badges: content.badges.clone(),
        actions: resolve_link_refs(site, &content.action_refs),
    }
}

fn build_closing(
    site: &SiteContent,
    content: &super::types::ClosingCopy,
) -> super::types::ClosingContent {
    super::types::ClosingContent {
        title: content.title.clone(),
        summary: content.summary.clone(),
        actions: resolve_link_refs(site, &content.action_refs),
    }
}

fn resolve_action_link(site: &SiteContent, id: &crate::types::Text) -> CmsActionLink {
    site.action_links
        .iter()
        .find(|action| action.id == *id)
        .unwrap_or_else(|| panic!("site_content.action_links must include id {}", id))
        .link
        .clone()
}

fn resolve_contact_method_link(
    site: &SiteContent,
    id: &crate::types::Text,
    label: Option<&crate::types::Text>,
    tone: Option<super::types::CtaKind>,
) -> CmsActionLink {
    let method = site
        .contact_methods
        .iter()
        .find(|method| method.id == *id)
        .unwrap_or_else(|| panic!("site_content.contact_methods must include id {}", id));

    CmsActionLink {
        label: label.cloned().unwrap_or_else(|| method.label.clone()),
        href: method.href.clone(),
        kind: method.kind,
        tone: tone.unwrap_or(super::types::CtaKind::Secondary),
    }
}

fn resolve_direct_link_ref(
    site: &SiteContent,
    reference: &super::types::DirectLinkReference,
) -> CmsActionLink {
    match reference {
        super::types::DirectLinkReference::Action { id } => resolve_action_link(site, id),
        super::types::DirectLinkReference::ContactMethod { id, label, tone } => {
            resolve_contact_method_link(site, id, label.as_ref(), *tone)
        }
    }
}

fn validate_site_content(content: &SiteContent) {
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
    reference: &super::types::DirectLinkReference,
    path: &str,
) {
    match reference {
        super::types::DirectLinkReference::Action { id } => {
            let found = content.action_links.iter().any(|action| action.id == *id);
            assert!(found, "{path} must reference an existing action id: {id}");
        }
        super::types::DirectLinkReference::ContactMethod { id, .. } => {
            let found = content
                .contact_methods
                .iter()
                .any(|method| method.id == *id);
            assert!(found, "{path} must reference an existing contact id: {id}");
        }
    }
}

fn assert_contact_method_ids_resolve(
    content: &SiteContent,
    ids: &[crate::types::Text],
    path: &str,
) {
    for id in ids {
        let found = content
            .contact_methods
            .iter()
            .any(|method| method.id == *id);
        assert!(found, "{path} must reference an existing contact id: {id}");
    }
}

fn assert_experience_role_ids_resolve(
    content: &SiteContent,
    ids: &[crate::types::Text],
    path: &str,
) {
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

fn assert_skill_group_ids_resolve(
    content: &SiteContent,
    ids: &[crate::types::Text],
    path: &str,
) {
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

        let result = catch_unwind(AssertUnwindSafe(|| validate_site_content(&content)));

        assert!(result.is_err(), "invalid ui_copy action refs must fail closed");
    }

    #[test]
    fn validate_site_content_rejects_missing_project_slugs_used_by_ui_copy() {
        let mut content = test_site_content();
        content
            .projects
            .retain(|project| project.slug != WorkCaseSlug::SensitiveSync);

        let result = catch_unwind(AssertUnwindSafe(|| validate_site_content(&content)));

        assert!(result.is_err(), "missing referenced project slugs must fail closed");
    }
}
