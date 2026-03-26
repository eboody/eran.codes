// ci: partials-render-file-exempt

use serde::Deserialize;

use crate::paths::Route;
use crate::types::Text;

use super::crate_gallery;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkCaseSlug {
    ChatRealtime,
    CommandSse,
    OperationalVisibility,
    SensitiveSync,
}

impl WorkCaseSlug {
    pub fn route(self) -> Route {
        match self {
            Self::ChatRealtime => Route::WorkChatRealtime,
            Self::CommandSse => Route::WorkCommandSse,
            Self::OperationalVisibility => Route::WorkOperationalVisibility,
            Self::SensitiveSync => Route::WorkSensitiveSync,
        }
    }

    pub const fn archive_anchor_id(self) -> Option<&'static str> {
        match self {
            Self::ChatRealtime => Some("chat-realtime"),
            Self::CommandSse => Some("command-sse"),
            Self::OperationalVisibility => Some("operational-visibility"),
            Self::SensitiveSync => None,
        }
    }

    pub const fn public_href(self) -> &'static str {
        match self {
            Self::ChatRealtime => "/work#chat-realtime",
            Self::CommandSse => "/work#command-sse",
            Self::OperationalVisibility => "/work#operational-visibility",
            Self::SensitiveSync => Route::WorkSensitiveSync.as_str(),
        }
    }

    pub const fn detail_layout(self) -> WorkCaseDetailLayout {
        match self {
            Self::SensitiveSync => WorkCaseDetailLayout::CurrentProof,
            Self::ChatRealtime | Self::CommandSse | Self::OperationalVisibility => {
                WorkCaseDetailLayout::ArchiveGrid
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkKind {
    #[default]
    Internal,
    External,
}

impl LinkKind {
    pub fn is_external(self) -> bool {
        matches!(self, Self::External)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CtaKind {
    #[default]
    Primary,
    Secondary,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CmsImageAsset {
    pub asset_ref: Text,
    pub alt: Text,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CmsActionLink {
    pub label: Text,
    pub href: Text,
    #[serde(default)]
    pub kind: LinkKind,
    #[serde(default)]
    pub tone: CtaKind,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ActionLinkContent {
    pub id: Text,
    #[serde(flatten)]
    pub link: CmsActionLink,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum DirectLinkReference {
    Action {
        id: Text,
    },
    ContactMethod {
        id: Text,
        #[serde(default)]
        label: Option<Text>,
        #[serde(default)]
        tone: Option<CtaKind>,
    },
}

#[derive(Clone, Debug, Deserialize)]
pub struct ActionBundleContent {
    pub id: Text,
    #[serde(default)]
    pub references: Vec<DirectLinkReference>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum LinkReference {
    Action {
        id: Text,
    },
    ContactMethod {
        id: Text,
        #[serde(default)]
        label: Option<Text>,
        #[serde(default)]
        tone: Option<CtaKind>,
    },
    Bundle {
        id: Text,
    },
}

#[derive(Clone, Debug, Deserialize)]
pub struct ActionLibraryFragment {
    #[serde(default)]
    pub action_links: Vec<ActionLinkContent>,
    #[serde(default)]
    pub action_bundles: Vec<ActionBundleContent>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PortfolioHeroContent {
    pub eyebrow: Text,
    pub title: Text,
    pub summary: Text,
    #[serde(default)]
    pub badges: Vec<Text>,
    #[serde(default)]
    pub actions: Vec<CmsActionLink>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PortfolioHeroCopy {
    pub eyebrow: Text,
    pub title: Text,
    pub summary: Text,
    #[serde(default)]
    pub badges: Vec<Text>,
    #[serde(default)]
    pub action_refs: Vec<LinkReference>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WorkCardContent {
    pub slug: WorkCaseSlug,
    pub category: Text,
    pub title: Text,
    pub summary: Text,
    pub outcome: Option<Text>,
    pub highlights: Vec<Text>,
    #[serde(default)]
    pub stack_tags: Vec<Text>,
    pub cta_label: Text,
    pub preview: Option<CmsImageAsset>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WorkSectionContent {
    pub title: Text,
    pub subtitle: Text,
    pub cards: Vec<WorkCardContent>,
    #[serde(default)]
    pub actions: Vec<CmsActionLink>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrateCardContent {
    pub name: Text,
    pub summary: Text,
    pub highlights: Vec<Text>,
    pub gallery: Option<crate_gallery::Content>,
    pub repository_url: Text,
    pub repository_label: Text,
    pub docs_url: Option<Text>,
    pub docs_label: Option<Text>,
    #[serde(default)]
    pub tags: Vec<Text>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrateSectionContent {
    pub title: Text,
    pub subtitle: Text,
    pub cards: Vec<CrateCardContent>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ClosingContent {
    pub title: Text,
    pub summary: Text,
    #[serde(default)]
    pub actions: Vec<CmsActionLink>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ClosingCopy {
    pub title: Text,
    pub summary: Text,
    #[serde(default)]
    pub action_refs: Vec<LinkReference>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExperienceRoleContent {
    pub id: Text,
    pub company: Text,
    pub title: Text,
    pub tenure: Text,
    pub summary: Text,
    pub highlights: Vec<Text>,
    #[serde(default)]
    pub actions: Vec<CmsActionLink>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExperienceSectionContent {
    pub title: Text,
    pub subtitle: Text,
    pub roles: Vec<ExperienceRoleContent>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SkillGroupContent {
    pub id: Text,
    pub title: Text,
    pub items: Vec<Text>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SkillSectionContent {
    pub title: Text,
    pub subtitle: Text,
    pub groups: Vec<SkillGroupContent>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContactMethodContent {
    pub id: Text,
    pub label: Text,
    pub value: Text,
    pub href: Text,
    #[serde(default)]
    pub kind: LinkKind,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PortfolioHomeContent {
    pub page_title: Text,
    pub hero: PortfolioHeroContent,
    pub experience_section: ExperienceSectionContent,
    pub project_section: WorkSectionContent,
    pub current_proof_section: WorkSectionContent,
    pub open_source_teaser: ClosingContent,
    pub skill_section: SkillSectionContent,
    pub contact_section: ClosingContent,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WorkIndexContent {
    pub page_title: Text,
    pub hero: PortfolioHeroContent,
    pub current_proof_section: WorkSectionContent,
    pub supporting_cases_section: WorkSectionContent,
    pub archive_details: ArchiveDetailsContent,
    pub open_source_teaser: ClosingContent,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenSourceIndexContent {
    pub page_title: Text,
    pub hero: PortfolioHeroContent,
    pub crate_section: CrateSectionContent,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CaseListSection {
    pub title: Text,
    pub items: Vec<Text>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkCaseDetailLayout {
    ArchiveGrid,
    CurrentProof,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WorkCaseContent {
    pub page_title: Text,
    pub hero: PortfolioHeroContent,
    pub detail_layout: WorkCaseDetailLayout,
    pub challenge: CaseListSection,
    pub implementation: CaseListSection,
    pub outcomes: CaseListSection,
    pub stack: CaseListSection,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IdentityContent {
    pub name: Text,
    pub location: Text,
    pub headline: Text,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SectionIntro {
    pub title: Text,
    pub subtitle: Text,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ArchiveDetailsContent {
    pub title: Text,
    pub subtitle: Text,
    pub entry_label: Text,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExperienceSectionSelection {
    pub title: Text,
    pub subtitle: Text,
    pub role_ids: Vec<Text>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProjectSectionSelection {
    pub title: Text,
    pub subtitle: Text,
    pub project_slugs: Vec<WorkCaseSlug>,
    #[serde(default)]
    pub action_refs: Vec<LinkReference>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SkillSectionSelection {
    pub title: Text,
    pub subtitle: Text,
    pub skill_group_ids: Vec<Text>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContactSectionSelection {
    pub title: Text,
    pub summary: Text,
    #[serde(default)]
    pub action_refs: Vec<LinkReference>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HomePageCopy {
    pub page_title: Text,
    pub hero: PortfolioHeroCopy,
    pub experience: ExperienceSectionSelection,
    pub selected_projects: ProjectSectionSelection,
    pub current_proof: ProjectSectionSelection,
    pub open_source_teaser: ClosingCopy,
    pub skills: SkillSectionSelection,
    pub contact: ContactSectionSelection,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WorkIndexCopy {
    pub page_title: Text,
    pub eyebrow: Text,
    pub title: Text,
    pub summary: Text,
    pub current_proof: ProjectSectionSelection,
    pub supporting_cases: ProjectSectionSelection,
    pub archive_details: ArchiveDetailsContent,
    pub open_source_teaser: ClosingCopy,
}

#[derive(Clone, Debug)]
pub struct ArchivedWorkCaseContent {
    pub slug: WorkCaseSlug,
    pub content: WorkCaseContent,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenSourceIndexCopy {
    pub page_title: Text,
    pub hero: PortfolioHeroCopy,
    pub crate_section: SectionIntro,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SessionCardCopy {
    pub title: Text,
    pub guest_status: Text,
    pub guest_summary: Text,
    pub signed_in_action_label: Text,
    #[serde(default)]
    pub guest_action_refs: Vec<LinkReference>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SessionCardContent {
    pub title: Text,
    pub guest_status: Text,
    pub guest_summary: Text,
    pub signed_in_action_label: Text,
    #[serde(default)]
    pub guest_actions: Vec<CmsActionLink>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LabPanelContent {
    pub title: Text,
    pub subtitle: Text,
    pub empty_message: Text,
    pub action_label: Option<Text>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoCardContent {
    pub title: Text,
    pub summary: Text,
    pub points: Vec<Text>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoSectionContent {
    pub title: Text,
    pub subtitle: Text,
    pub cards: Vec<InfoCardContent>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LabPageCopy {
    pub page_title: Text,
    pub hero: PortfolioHeroCopy,
    pub session_card: SessionCardCopy,
    pub guest_chat: ClosingCopy,
    pub operations_surface: LabPanelContent,
    pub sensitive_proof: LabPanelContent,
    pub engineering_quality: InfoSectionContent,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LabPageContent {
    pub page_title: Text,
    pub hero: PortfolioHeroContent,
    pub session_card: SessionCardContent,
    pub guest_chat: ClosingContent,
    pub operations_surface: LabPanelContent,
    pub sensitive_proof: LabPanelContent,
    pub engineering_quality: InfoSectionContent,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResumeDocumentContent {
    pub summary_title: Text,
    pub summary: Text,
    pub experience_title: Text,
    pub projects_title: Text,
    pub open_source_title: Text,
    pub client_context_title: Text,
    pub client_context: Vec<Text>,
    pub skills_title: Text,
    pub experience_role_ids: Vec<Text>,
    pub featured_project_slugs: Vec<WorkCaseSlug>,
    pub skill_group_ids: Vec<Text>,
    pub contact_method_ids: Vec<Text>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SiteUiCopy {
    pub home: HomePageCopy,
    pub work: WorkIndexCopy,
    pub open_source: OpenSourceIndexCopy,
    pub lab: LabPageCopy,
    pub resume: ResumeDocumentContent,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WorkCaseCopy {
    pub page_title: Text,
    pub eyebrow: Text,
    pub title: Text,
    pub summary: Text,
    pub challenge: CaseListSection,
    pub implementation: CaseListSection,
    pub outcomes: CaseListSection,
    pub stack: CaseListSection,
    #[serde(default)]
    pub action_refs: Vec<LinkReference>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WorkCaseRecord {
    pub slug: WorkCaseSlug,
    #[serde(flatten)]
    pub content: WorkCaseCopy,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SiteContent {
    pub identity: IdentityContent,
    #[serde(default)]
    pub action_links: Vec<ActionLinkContent>,
    #[serde(default)]
    pub action_bundles: Vec<ActionBundleContent>,
    #[serde(default)]
    pub nav_links: Vec<LinkReference>,
    #[serde(default)]
    pub contact_methods: Vec<ContactMethodContent>,
    #[serde(default)]
    pub experience_roles: Vec<ExperienceRoleContent>,
    #[serde(default)]
    pub projects: Vec<WorkCardContent>,
    #[serde(default)]
    pub open_source_entries: Vec<CrateCardContent>,
    #[serde(default)]
    pub skill_groups: Vec<SkillGroupContent>,
    pub ui_copy: SiteUiCopy,
    #[serde(default)]
    pub work_cases: Vec<WorkCaseRecord>,
}
