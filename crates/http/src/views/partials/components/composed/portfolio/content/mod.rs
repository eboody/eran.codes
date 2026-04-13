mod crate_gallery;
mod load;
mod pages;
mod resolve;
#[cfg(test)]
mod tests;
mod validate;
#[path = "types.rs"]
pub(super) mod model;

pub(crate) use crate_gallery::Content as CrateGalleryContent;
pub use pages::{
    lab_page_content, open_source_index_content, portfolio_home_content, portfolio_nav_links,
    resume_text, work_case_content, work_index_content,
};
pub(super) use model as types;
pub use model::{
    CmsActionLink, CrateCardContent, CrateSectionContent, CtaKind, OpenSourceIndexContent,
    PortfolioHeroContent, PortfolioHomeContent, WorkCardContent, WorkCaseContent,
    WorkCaseDetailLayout, WorkCaseSlug, WorkIndexContent, WorkSectionContent,
};
