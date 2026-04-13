use super::*;
use crate::views::partials::components::portfolio::content::{
    work_case_content, WorkCaseSlug,
};

fn assert_in_order(markup: &str, titles: &[String]) {
    let mut cursor = 0;

    for title in titles {
        let next = markup[cursor..]
            .find(title)
            .unwrap_or_else(|| panic!("missing title in markup: {title}"));
        cursor += next + title.len();
    }
}

#[test]
fn archive_grid_renders_sections_in_configured_order() {
    let content = work_case_content(WorkCaseSlug::ChatRealtime);
    let markup = Work { content }.render().into_string();
    let section_titles = section_refs(content, ARCHIVE_GRID_ORDER)
        .map(|section| section.title().to_string())
        .into_iter()
        .collect::<Vec<_>>();

    assert!(markup.contains("ui-portfolio-case-grid"));
    assert!(!markup.contains("Boundary and scope"));
    assert_in_order(&markup, &section_titles);
}

#[test]
fn current_proof_layout_uses_override_title_and_stack_badges() {
    let content = work_case_content(WorkCaseSlug::SensitiveSync);
    let markup = Work { content }.render().into_string();
    let main_titles = section_refs(content, CURRENT_PROOF_MAIN_ORDER)
        .map(|section| section.title().to_string())
        .into_iter()
        .collect::<Vec<_>>();

    assert!(markup.contains("ui-portfolio-current-proof-detail"));
    assert!(markup.contains("Boundary and scope"));
    assert!(markup.contains("ui-portfolio-current-proof-stack"));
    assert_in_order(&markup, &main_titles);
    assert!(!markup.contains("class=\"ui-portfolio-card-grid ui-portfolio-case-grid\""));
}
