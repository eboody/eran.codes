// ci: partials-render-file-exempt

use serde::Deserialize;

use crate::paths::Route;
use crate::types::Text;

use super::crate_gallery;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkCaseSlug {
    ChatRealtime,
    CommandSse,
    OperationalVisibility,
}

impl WorkCaseSlug {
    pub fn route(self) -> Route {
        match self {
            Self::ChatRealtime => Route::WorkChatRealtime,
            Self::CommandSse => Route::WorkCommandSse,
            Self::OperationalVisibility => Route::WorkOperationalVisibility,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkKind {
    Internal,
    External,
}

impl LinkKind {
    pub fn is_external(self) -> bool {
        matches!(self, Self::External)
    }
}

impl Default for LinkKind {
    fn default() -> Self {
        Self::Internal
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CtaKind {
    #[default]
    Primary,
    Secondary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofKind {
    Outcome,
    Architecture,
    Reliability,
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
pub struct ProofPointContent {
    pub kind: ProofKind,
    pub title: Text,
    pub text: Text,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProofStripContent {
    pub title: Text,
    pub subtitle: Text,
    pub items: Vec<ProofPointContent>,
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
    pub gallery: Option<crate_gallery::CrateGalleryContent>,
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
pub struct PortfolioHomeContent {
    pub page_title: Text,
    pub hero: PortfolioHeroContent,
    pub proof_strip: ProofStripContent,
    pub work_section: WorkSectionContent,
    pub crate_section: CrateSectionContent,
    pub closing: ClosingContent,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WorkIndexContent {
    pub page_title: Text,
    pub eyebrow: Text,
    pub title: Text,
    pub summary: Text,
    pub cases_title: Text,
    pub cases_subtitle: Text,
    pub cases: Vec<WorkCardContent>,
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

#[derive(Clone, Debug, Deserialize)]
pub struct WorkCaseContent {
    pub page_title: Text,
    pub eyebrow: Text,
    pub title: Text,
    pub summary: Text,
    pub challenge: CaseListSection,
    pub implementation: CaseListSection,
    pub outcomes: CaseListSection,
    pub stack: CaseListSection,
    #[serde(default)]
    pub actions: Vec<CmsActionLink>,
}
