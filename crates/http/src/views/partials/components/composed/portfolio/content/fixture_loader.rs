use std::fmt::Write;
use std::sync::OnceLock;

use serde::de::DeserializeOwned;

use super::types::{
    ActionLibraryFragment, ArchivedWorkCaseContent, ClosingContent, CmsActionLink,
    ContactMethodContent, CrateCardContent, CrateSectionContent, DirectLinkReference,
    ExperienceRoleContent, ExperienceSectionContent, HomePageCopy, IdentityContent,
    LabPageContent, LabPageCopy, LinkReference, OpenSourceIndexContent, OpenSourceIndexCopy,
    PortfolioHeroContent, PortfolioHomeContent, ResumeDocumentContent, SessionCardContent,
    SiteContent, SiteUiCopy, SkillGroupContent, SkillSectionContent, WorkCardContent,
    WorkCaseContent, WorkCaseRecord, WorkCaseSlug, WorkIndexContent, WorkIndexCopy,
    WorkSectionContent,
};
use super::validation::{
    validate_action_library_fragment, validate_contact_fragment, validate_experience_fragment,
    validate_home_page_fragment, validate_identity_fragment, validate_lab_page,
    validate_lab_page_fragment, validate_nav_fragment, validate_open_source_entries_fragment,
    validate_open_source_index, validate_open_source_page_fragment, validate_portfolio_home,
    validate_projects_fragment, validate_resume_page_fragment, validate_site_content,
    validate_skill_groups_fragment, validate_work_case, validate_work_cases_fragment,
    validate_work_index, validate_work_page_fragment,
};

pub fn site_content() -> &'static SiteContent {
    static CONTENT: OnceLock<SiteContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let action_library = load_action_library_fragment();
        let content = SiteContent {
            identity: load_identity_fragment(),
            action_links: action_library.action_links,
            action_bundles: action_library.action_bundles,
            nav_links: load_nav_fragment(),
            contact_methods: load_contact_fragment(),
            experience_roles: load_experience_fragment(),
            projects: load_projects_fragment(),
            open_source_entries: load_open_source_entries_fragment(),
            skill_groups: load_skill_groups_fragment(),
            ui_copy: SiteUiCopy {
                home: load_home_page_fragment(),
                work: load_work_page_fragment(),
                open_source: load_open_source_page_fragment(),
                lab: load_lab_page_fragment(),
                resume: load_resume_page_fragment(),
            },
            work_cases: load_work_cases_fragment(),
        };
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
        let content = LabPageContent {
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
        };
        validate_lab_page(&content);
        content
    })
}

pub fn portfolio_home_content() -> &'static PortfolioHomeContent {
    static CONTENT: OnceLock<PortfolioHomeContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let site = site_content();
        let home = &site.ui_copy.home;
        let content = PortfolioHomeContent {
            page_title: home.page_title.clone(),
            hero: build_portfolio_hero(site, &home.hero),
            experience_section: ExperienceSectionContent {
                title: home.experience.title.clone(),
                subtitle: home.experience.subtitle.clone(),
                roles: experience_roles_for_ids(site, &home.experience.role_ids),
            },
            project_section: WorkSectionContent {
                title: home.selected_projects.title.clone(),
                subtitle: home.selected_projects.subtitle.clone(),
                cards: project_cards_for_slugs(site, &home.selected_projects.project_slugs),
                actions: resolve_link_refs(site, &home.selected_projects.action_refs),
            },
            current_proof_section: WorkSectionContent {
                title: home.current_proof.title.clone(),
                subtitle: home.current_proof.subtitle.clone(),
                cards: project_cards_for_slugs(site, &home.current_proof.project_slugs),
                actions: resolve_link_refs(site, &home.current_proof.action_refs),
            },
            open_source_teaser: build_closing(site, &home.open_source_teaser),
            skill_section: SkillSectionContent {
                title: home.skills.title.clone(),
                subtitle: home.skills.subtitle.clone(),
                groups: skill_groups_for_ids(site, &home.skills.skill_group_ids),
            },
            contact_section: ClosingContent {
                title: home.contact.title.clone(),
                summary: home.contact.summary.clone(),
                actions: resolve_link_refs(site, &home.contact.action_refs),
            },
        };
        validate_portfolio_home(&content);
        content
    })
}

pub fn work_index_content() -> &'static WorkIndexContent {
    static CONTENT: OnceLock<WorkIndexContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let site = site_content();
        let work = &site.ui_copy.work;
        let content = WorkIndexContent {
            page_title: work.page_title.clone(),
            hero: PortfolioHeroContent {
                eyebrow: work.eyebrow.clone(),
                title: work.title.clone(),
                summary: work.summary.clone(),
                badges: vec![],
                actions: vec![],
            },
            current_proof_section: WorkSectionContent {
                title: work.current_proof.title.clone(),
                subtitle: work.current_proof.subtitle.clone(),
                cards: project_cards_for_slugs(site, &work.current_proof.project_slugs),
                actions: resolve_link_refs(site, &work.current_proof.action_refs),
            },
            supporting_cases_section: WorkSectionContent {
                title: work.supporting_cases.title.clone(),
                subtitle: work.supporting_cases.subtitle.clone(),
                cards: project_cards_for_slugs(site, &work.supporting_cases.project_slugs),
                actions: resolve_link_refs(site, &work.supporting_cases.action_refs),
            },
            archive_details: work.archive_details.clone(),
            open_source_teaser: build_closing(site, &work.open_source_teaser),
        };
        validate_work_index(&content);
        content
    })
}

pub fn supporting_archive_cases() -> &'static [ArchivedWorkCaseContent] {
    static CONTENT: OnceLock<Vec<ArchivedWorkCaseContent>> = OnceLock::new();

    CONTENT
        .get_or_init(|| {
            site_content()
                .ui_copy
                .work
                .supporting_cases
                .project_slugs
                .iter()
                .map(|slug| ArchivedWorkCaseContent {
                    slug: *slug,
                    content: work_case_content(*slug).clone(),
                })
                .collect()
        })
        .as_slice()
}

pub fn open_source_index_content() -> &'static OpenSourceIndexContent {
    static CONTENT: OnceLock<OpenSourceIndexContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let site = site_content();
        let open_source = &site.ui_copy.open_source;
        let content = OpenSourceIndexContent {
            page_title: open_source.page_title.clone(),
            hero: build_portfolio_hero(site, &open_source.hero),
            crate_section: CrateSectionContent {
                title: open_source.crate_section.title.clone(),
                subtitle: open_source.crate_section.subtitle.clone(),
                cards: site.open_source_entries.clone(),
            },
        };
        validate_open_source_index(&content);
        content
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
    let content = WorkCaseContent {
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
    };
    validate_work_case(&content, slug);
    content
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
                .unwrap_or_else(|| panic!("site_content.skill_groups must include id {}", group_id))
                .clone()
        })
        .collect()
}

fn project_cards_for_slugs(site: &SiteContent, slugs: &[WorkCaseSlug]) -> Vec<WorkCardContent> {
    slugs.iter()
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
        writeln!(&mut output, "### {} — {}", role.company, role.title).expect("resume write");
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
                    panic!("site_content.contact_methods must include id {}", contact_id)
                })
                .clone()
        })
        .collect()
}

fn join_text_items(items: &[crate::types::Text]) -> String {
    items.iter()
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
                    .unwrap_or_else(|| panic!("site_content.action_bundles must include id {}", id));

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

fn build_closing(site: &SiteContent, content: &super::types::ClosingCopy) -> ClosingContent {
    ClosingContent {
        title: content.title.clone(),
        summary: content.summary.clone(),
        actions: resolve_link_refs(site, &content.action_refs),
    }
}

fn parse_fragment<T: DeserializeOwned>(path: &str, source: &str) -> T {
    serde_json::from_str(source)
        .unwrap_or_else(|error| panic!("{path} fragment must be valid JSON: {error}"))
}

fn load_identity_fragment() -> IdentityContent {
    let content = parse_fragment(
        "site_content/identity.json",
        include_str!("site_content/identity.json"),
    );
    validate_identity_fragment(&content);
    content
}

fn load_action_library_fragment() -> ActionLibraryFragment {
    let content = parse_fragment(
        "site_content/actions.json",
        include_str!("site_content/actions.json"),
    );
    validate_action_library_fragment(&content);
    content
}

fn load_nav_fragment() -> Vec<LinkReference> {
    let content = parse_fragment::<Vec<LinkReference>>(
        "site_content/nav.json",
        include_str!("site_content/nav.json"),
    );
    validate_nav_fragment(&content);
    content
}

fn load_contact_fragment() -> Vec<ContactMethodContent> {
    let content = parse_fragment::<Vec<ContactMethodContent>>(
        "site_content/contact.json",
        include_str!("site_content/contact.json"),
    );
    validate_contact_fragment(&content);
    content
}

fn load_experience_fragment() -> Vec<ExperienceRoleContent> {
    let content = parse_fragment::<Vec<ExperienceRoleContent>>(
        "site_content/experience.json",
        include_str!("site_content/experience.json"),
    );
    validate_experience_fragment(&content);
    content
}

fn load_projects_fragment() -> Vec<WorkCardContent> {
    let content = parse_fragment::<Vec<WorkCardContent>>(
        "site_content/projects.json",
        include_str!("site_content/projects.json"),
    );
    validate_projects_fragment(&content);
    content
}

fn load_work_cases_fragment() -> Vec<WorkCaseRecord> {
    let content = parse_fragment::<Vec<WorkCaseRecord>>(
        "site_content/work_cases.json",
        include_str!("site_content/work_cases.json"),
    );
    validate_work_cases_fragment(&content);
    content
}

fn load_open_source_entries_fragment() -> Vec<CrateCardContent> {
    let content = parse_fragment::<Vec<CrateCardContent>>(
        "site_content/open_source.json",
        include_str!("site_content/open_source.json"),
    );
    validate_open_source_entries_fragment(&content);
    content
}

fn load_skill_groups_fragment() -> Vec<SkillGroupContent> {
    let content = parse_fragment::<Vec<SkillGroupContent>>(
        "site_content/skills.json",
        include_str!("site_content/skills.json"),
    );
    validate_skill_groups_fragment(&content);
    content
}

fn load_home_page_fragment() -> HomePageCopy {
    let content = parse_fragment(
        "site_content/pages/home.json",
        include_str!("site_content/pages/home.json"),
    );
    validate_home_page_fragment(&content);
    content
}

fn load_work_page_fragment() -> WorkIndexCopy {
    let content = parse_fragment(
        "site_content/pages/work.json",
        include_str!("site_content/pages/work.json"),
    );
    validate_work_page_fragment(&content);
    content
}

fn load_open_source_page_fragment() -> OpenSourceIndexCopy {
    let content = parse_fragment(
        "site_content/pages/open_source.json",
        include_str!("site_content/pages/open_source.json"),
    );
    validate_open_source_page_fragment(&content);
    content
}

fn load_lab_page_fragment() -> LabPageCopy {
    let content = parse_fragment(
        "site_content/pages/lab.json",
        include_str!("site_content/pages/lab.json"),
    );
    validate_lab_page_fragment(&content);
    content
}

fn load_resume_page_fragment() -> ResumeDocumentContent {
    let content = parse_fragment(
        "site_content/pages/resume.json",
        include_str!("site_content/pages/resume.json"),
    );
    validate_resume_page_fragment(&content);
    content
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

fn resolve_direct_link_ref(site: &SiteContent, reference: &DirectLinkReference) -> CmsActionLink {
    match reference {
        DirectLinkReference::Action { id } => resolve_action_link(site, id),
        DirectLinkReference::ContactMethod { id, label, tone } => {
            resolve_contact_method_link(site, id, label.as_ref(), *tone)
        }
    }
}
