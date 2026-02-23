moddef::moddef!(mod { demo_section, demo_result, cta_row, section_header, home_hero, feature_gallery, tabbed_showcase, capability_showcase, professionalism_in_practice_tabs });

pub use demo_section::DemoSection;
pub use demo_result::DemoResultPlaceholder;
pub use cta_row::CtaRow;
pub use section_header::SectionHeader;
pub use home_hero::HomeHero;
pub use feature_gallery::{DiagramPanel, DiagramRow, DiagramStatus, FeatureAccent, FeatureCard, FeatureGallery};
pub use tabbed_showcase::TabbedShowcase;
pub(crate) use tabbed_showcase::{
    TabbedShowcaseAction, TabbedShowcaseMockPanel, TabbedShowcaseRow, TabbedShowcaseTab,
};
pub use capability_showcase::CapabilityShowcase;
pub use professionalism_in_practice_tabs::ProfessionalismInPracticeTabs;
