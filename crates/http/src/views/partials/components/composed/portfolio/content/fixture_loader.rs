use std::sync::OnceLock;

use super::types::{
    OpenSourceIndexContent, PortfolioHomeContent, WorkCaseContent, WorkCaseSlug, WorkIndexContent,
};
use super::validation::{
    validate_open_source_index, validate_portfolio_home, validate_work_case, validate_work_index,
};

pub fn portfolio_home_content() -> &'static PortfolioHomeContent {
    static CONTENT: OnceLock<PortfolioHomeContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let content: PortfolioHomeContent =
            serde_json::from_str(include_str!("portfolio_home.json"))
                .expect("portfolio_home fixture must be valid JSON");
        validate_portfolio_home(&content);
        content
    })
}

pub fn work_index_content() -> &'static WorkIndexContent {
    static CONTENT: OnceLock<WorkIndexContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let content: WorkIndexContent =
            serde_json::from_str(include_str!("work_index.json"))
                .expect("work_index fixture must be valid JSON");
        validate_work_index(&content);
        content
    })
}

pub fn open_source_index_content() -> &'static OpenSourceIndexContent {
    static CONTENT: OnceLock<OpenSourceIndexContent> = OnceLock::new();

    CONTENT.get_or_init(|| {
        let content: OpenSourceIndexContent =
            serde_json::from_str(include_str!("open_source_index.json"))
                .expect("open_source_index fixture must be valid JSON");
        validate_open_source_index(&content);
        content
    })
}

pub fn work_case_content(slug: WorkCaseSlug) -> &'static WorkCaseContent {
    static CHAT_REALTIME: OnceLock<WorkCaseContent> = OnceLock::new();
    static COMMAND_SSE: OnceLock<WorkCaseContent> = OnceLock::new();
    static OPERATIONAL_VISIBILITY: OnceLock<WorkCaseContent> = OnceLock::new();

    match slug {
        WorkCaseSlug::ChatRealtime => CHAT_REALTIME.get_or_init(|| {
            load_work_case_fixture(slug, include_str!("work_chat_realtime.json"))
        }),
        WorkCaseSlug::CommandSse => COMMAND_SSE.get_or_init(|| {
            load_work_case_fixture(slug, include_str!("work_command_sse.json"))
        }),
        WorkCaseSlug::OperationalVisibility => OPERATIONAL_VISIBILITY.get_or_init(|| {
            load_work_case_fixture(slug, include_str!("work_operational_visibility.json"))
        }),
    }
}

fn load_work_case_fixture(slug: WorkCaseSlug, raw: &str) -> WorkCaseContent {
    let content: WorkCaseContent =
        serde_json::from_str(raw).expect("work case fixture must be valid JSON");
    validate_work_case(&content, slug);
    content
}
