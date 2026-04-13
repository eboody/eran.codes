use super::types::{
    ActionBundleContent, ActionLinkContent, ClosingContent, ClosingCopy, CmsActionLink,
    ContactMethodContent, DirectLinkReference, ExperienceRoleContent, LinkReference,
    PortfolioHeroContent, PortfolioHeroCopy, SiteContent, SkillGroupContent, WorkCaseRecord,
    WorkCaseSlug, WorkCardContent,
};
use crate::types::Text;

pub(super) fn link_refs(site: &SiteContent, references: &[LinkReference]) -> Vec<CmsActionLink> {
    let mut resolved = Vec::new();

    for reference in references {
        match reference {
            LinkReference::Action { id } => resolved.push(action_link(site, id)),
            LinkReference::ContactMethod { id, label, tone } => {
                resolved.push(contact_method_link(site, id, label.as_ref(), *tone));
            }
            LinkReference::Bundle { id } => {
                for reference in &action_bundle(site, id).references {
                    resolved.push(direct_link_ref(site, reference));
                }
            }
        }
    }

    resolved
}

pub(super) fn portfolio_hero(
    site: &SiteContent,
    content: &PortfolioHeroCopy,
) -> PortfolioHeroContent {
    PortfolioHeroContent {
        eyebrow: content.eyebrow.clone(),
        title: content.title.clone(),
        summary: content.summary.clone(),
        badges: content.badges.clone(),
        actions: link_refs(site, &content.action_refs),
    }
}

pub(super) fn closing(site: &SiteContent, content: &ClosingCopy) -> ClosingContent {
    ClosingContent {
        title: content.title.clone(),
        summary: content.summary.clone(),
        actions: link_refs(site, &content.action_refs),
    }
}

pub(super) fn contact_methods(
    site: &SiteContent,
    contact_method_ids: &[Text],
) -> Vec<ContactMethodContent> {
    contact_method_ids
        .iter()
        .map(|contact_id| contact_method(site, contact_id).clone())
        .collect()
}

pub(super) fn experience_roles(
    site: &SiteContent,
    role_ids: &[Text],
) -> Vec<ExperienceRoleContent> {
    role_ids
        .iter()
        .map(|role_id| experience_role(site, role_id).clone())
        .collect()
}

pub(super) fn project_cards(site: &SiteContent, slugs: &[WorkCaseSlug]) -> Vec<WorkCardContent> {
    slugs
        .iter()
        .map(|slug| project_card(site, *slug).clone())
        .collect()
}

pub(super) fn skill_groups(
    site: &SiteContent,
    skill_group_ids: &[Text],
) -> Vec<SkillGroupContent> {
    skill_group_ids
        .iter()
        .map(|group_id| skill_group(site, group_id).clone())
        .collect()
}

pub(super) fn work_case(site: &SiteContent, slug: WorkCaseSlug) -> &WorkCaseRecord {
    site.work_cases
        .iter()
        .find(|case| case.slug == slug)
        .expect("validated site content should include requested work case")
}

fn action_link(site: &SiteContent, id: &Text) -> CmsActionLink {
    action_link_content(site, id).link.clone()
}

fn action_link_content<'a>(site: &'a SiteContent, id: &Text) -> &'a ActionLinkContent {
    site.action_links
        .iter()
        .find(|action| action.id == *id)
        .expect("validated site content should include requested action link")
}

fn action_bundle<'a>(site: &'a SiteContent, id: &Text) -> &'a ActionBundleContent {
    site.action_bundles
        .iter()
        .find(|bundle| bundle.id == *id)
        .expect("validated site content should include requested action bundle")
}

fn contact_method<'a>(site: &'a SiteContent, id: &Text) -> &'a ContactMethodContent {
    site.contact_methods
        .iter()
        .find(|method| method.id == *id)
        .expect("validated site content should include requested contact method")
}

fn contact_method_link(
    site: &SiteContent,
    id: &Text,
    label: Option<&Text>,
    tone: Option<super::types::CtaKind>,
) -> CmsActionLink {
    let method = contact_method(site, id);

    CmsActionLink {
        label: label.cloned().unwrap_or_else(|| method.label.clone()),
        href: method.href.clone(),
        kind: method.kind,
        tone: tone.unwrap_or(super::types::CtaKind::Secondary),
    }
}

fn direct_link_ref(site: &SiteContent, reference: &DirectLinkReference) -> CmsActionLink {
    match reference {
        DirectLinkReference::Action { id } => action_link(site, id),
        DirectLinkReference::ContactMethod { id, label, tone } => {
            contact_method_link(site, id, label.as_ref(), *tone)
        }
    }
}

fn experience_role<'a>(site: &'a SiteContent, id: &Text) -> &'a ExperienceRoleContent {
    site.experience_roles
        .iter()
        .find(|role| role.id == *id)
        .expect("validated site content should include requested experience role")
}

fn project_card(site: &SiteContent, slug: WorkCaseSlug) -> &WorkCardContent {
    site.projects
        .iter()
        .find(|project| project.slug == slug)
        .expect("validated site content should include requested project")
}

fn skill_group<'a>(site: &'a SiteContent, id: &Text) -> &'a SkillGroupContent {
    site.skill_groups
        .iter()
        .find(|group| group.id == *id)
        .expect("validated site content should include requested skill group")
}
