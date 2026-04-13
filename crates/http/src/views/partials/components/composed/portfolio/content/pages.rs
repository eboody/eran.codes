use std::fmt::Write;
use std::sync::OnceLock;

use super::load::site_content;
use super::{resolve, types};
use types::{
    CmsActionLink, CrateSectionContent, LabPageContent, OpenSourceIndexContent,
    PortfolioHeroContent, PortfolioHomeContent, SessionCardContent, SiteContent, WorkCaseContent,
    WorkCaseSlug, WorkIndexContent, WorkSectionContent,
};

pub fn portfolio_nav_links() -> &'static [CmsActionLink] {
    static CONTENT: OnceLock<Vec<CmsActionLink>> = OnceLock::new();

    CONTENT
        .get_or_init(|| {
            let site = site_content();
            resolve::link_refs(site, &site.nav_links)
        })
        .as_slice()
}

pub fn lab_page_content() -> &'static LabPageContent {
    static CONTENT: OnceLock<LabPageContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let site = site_content();
        let lab = &site.ui_copy.lab;

        LabPageContent {
            page_title: lab.page_title.clone(),
            hero: resolve::portfolio_hero(site, &lab.hero),
            session_card: SessionCardContent {
                title: lab.session_card.title.clone(),
                guest_status: lab.session_card.guest_status.clone(),
                guest_summary: lab.session_card.guest_summary.clone(),
                signed_in_action_label: lab.session_card.signed_in_action_label.clone(),
                guest_actions: resolve::link_refs(site, &lab.session_card.guest_action_refs),
            },
            guest_chat: resolve::closing(site, &lab.guest_chat),
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
            hero: resolve::portfolio_hero(site, &home.hero),
            current_proof_section: WorkSectionContent {
                title: home.current_proof.title.clone(),
                subtitle: home.current_proof.subtitle.clone(),
                cards: resolve::project_cards(site, &home.current_proof.project_slugs),
                actions: resolve::link_refs(site, &home.current_proof.action_refs),
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
                cards: resolve::project_cards(site, &work.supporting_cases.project_slugs),
                actions: resolve::link_refs(site, &work.supporting_cases.action_refs),
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
            hero: resolve::portfolio_hero(site, &open_source.hero),
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
    let site = site_content();
    let content = resolve::work_case(site, slug).content.clone();

    WorkCaseContent {
        page_title: content.page_title,
        hero: PortfolioHeroContent {
            eyebrow: content.eyebrow,
            title: content.title,
            summary: content.summary,
            badges: vec![],
            actions: resolve::link_refs(site, &content.action_refs),
        },
        detail_layout: slug.detail_layout(),
        challenge: content.challenge,
        implementation: content.implementation,
        outcomes: content.outcomes,
        stack: content.stack,
    }
}

fn build_resume_text(site: &SiteContent) -> String {
    let resume = &site.ui_copy.resume;
    let mut output = String::new();

    writeln!(&mut output, "# {}", site.identity.name).expect("resume write");
    writeln!(&mut output, "{}", site.identity.location).expect("resume write");
    writeln!(&mut output, "{}", site.identity.headline).expect("resume write");
    for method in resolve::contact_methods(site, &resume.contact_method_ids) {
        writeln!(&mut output, "{}: {}", method.label, method.value).expect("resume write");
    }
    output.push('\n');

    writeln!(&mut output, "## {}", resume.summary_title).expect("resume write");
    writeln!(&mut output, "{}", resume.summary).expect("resume write");
    output.push('\n');

    writeln!(&mut output, "## {}", resume.experience_title).expect("resume write");
    for role in resolve::experience_roles(site, &resume.experience_role_ids) {
        writeln!(&mut output, "### {} — {}", role.company, role.title)
            .expect("resume write");
        writeln!(&mut output, "{}", role.tenure).expect("resume write");
        for highlight in role.highlights {
            writeln!(&mut output, "- {}", highlight).expect("resume write");
        }
        output.push('\n');
    }

    writeln!(&mut output, "## {}", resume.projects_title).expect("resume write");
    for project in resolve::project_cards(site, &resume.featured_project_slugs) {
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
    for group in resolve::skill_groups(site, &resume.skill_group_ids) {
        writeln!(&mut output, "### {}", group.title).expect("resume write");
        writeln!(&mut output, "- {}", join_text_items(&group.items)).expect("resume write");
    }

    output
}

fn join_text_items(items: &[crate::types::Text]) -> String {
    items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}
