use std::collections::HashSet;

use crate::{paths::Route, types::Text};

use super::types::{
    CaseListSection, ClosingContent, CmsActionLink, CmsImageAsset, CrateCardContent,
    CrateSectionContent, OpenSourceIndexContent, PortfolioHeroContent, PortfolioHomeContent,
    ProofPointContent, ProofStripContent, WorkCardContent, WorkCaseContent, WorkCaseSlug,
    WorkIndexContent,
    WorkSectionContent,
};
use super::CrateGalleryContent;

pub(super) fn validate_portfolio_home(content: &PortfolioHomeContent) {
    assert_non_empty("home.page_title", &content.page_title);
    validate_portfolio_hero(&content.hero, "home.hero");
    validate_proof_strip(&content.proof_strip, "home.proof_strip");
    validate_work_section(&content.work_section, "home.work_section");
    validate_crate_section(&content.crate_section, "home.crate_section");
    validate_closing(&content.closing, "home.closing");
}

pub(super) fn validate_work_index(content: &WorkIndexContent) {
    assert_non_empty("work.page_title", &content.page_title);
    assert_non_empty("work.eyebrow", &content.eyebrow);
    assert_non_empty("work.title", &content.title);
    assert_non_empty("work.summary", &content.summary);
    assert_non_empty("work.cases_title", &content.cases_title);
    assert_non_empty("work.cases_subtitle", &content.cases_subtitle);
    assert_min_len("work.cases", &content.cases, 1);
    for case in &content.cases {
        validate_work_card(case, "work.cases[]");
    }
    validate_closing(&content.open_source_teaser, "work.open_source_teaser");
}

pub(super) fn validate_open_source_index(content: &OpenSourceIndexContent) {
    assert_non_empty("open_source.page_title", &content.page_title);
    validate_open_source_hero(&content.hero, "open_source.hero");
    validate_crate_section(&content.crate_section, "open_source.crate_section");
}

pub(super) fn validate_work_case(content: &WorkCaseContent, slug: WorkCaseSlug) {
    assert_non_empty("work_case.page_title", &content.page_title);
    assert_non_empty("work_case.eyebrow", &content.eyebrow);
    assert_non_empty("work_case.title", &content.title);
    assert_non_empty("work_case.summary", &content.summary);
    validate_case_list(&content.challenge, "work_case.challenge");
    validate_case_list(&content.implementation, "work_case.implementation");
    validate_case_list(&content.outcomes, "work_case.outcomes");
    validate_case_list(&content.stack, "work_case.stack");

    for action in &content.actions {
        validate_action(action, "work_case.actions[]");
    }

    let case_route = slug.route();
    assert!(
        content
            .actions
            .iter()
            .any(|action| action_targets_case_or_lab(action, case_route)),
        "work case {slug:?} should include at least one action to itself or /lab",
    );
}

fn action_targets_case_or_lab(action: &CmsActionLink, case_route: Route) -> bool {
    let href = action.href.to_string();
    let path = href.split(['#', '?']).next().unwrap_or(href.as_str());

    matches!(
        path.parse::<Route>().ok(),
        Some(route) if route == case_route || route == Route::Lab
    )
}

fn validate_portfolio_hero(content: &PortfolioHeroContent, path: &str) {
    assert_non_empty(&format!("{path}.eyebrow"), &content.eyebrow);
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    assert_min_len(&format!("{path}.badges"), &content.badges, 1);
    for badge in &content.badges {
        assert_non_empty(&format!("{path}.badges[]"), badge);
    }
    assert_min_len(&format!("{path}.actions"), &content.actions, 1);
    for action in &content.actions {
        validate_action(action, &format!("{path}.actions[]"));
    }
}

fn validate_proof_strip(content: &ProofStripContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.items"), &content.items, 1);
    for item in &content.items {
        validate_proof_point(item, &format!("{path}.items[]"));
    }
}

fn validate_open_source_hero(content: &PortfolioHeroContent, path: &str) {
    assert_non_empty(&format!("{path}.eyebrow"), &content.eyebrow);
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    for badge in &content.badges {
        assert_non_empty(&format!("{path}.badges[]"), badge);
    }
    for action in &content.actions {
        validate_action(action, &format!("{path}.actions[]"));
    }
}

fn validate_proof_point(content: &ProofPointContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.text"), &content.text);
}

fn validate_work_section(content: &WorkSectionContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.cards"), &content.cards, 1);
    for card in &content.cards {
        validate_work_card(card, &format!("{path}.cards[]"));
    }
    for action in &content.actions {
        validate_action(action, &format!("{path}.actions[]"));
    }
}

fn validate_crate_section(content: &CrateSectionContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.subtitle"), &content.subtitle);
    assert_min_len(&format!("{path}.cards"), &content.cards, 1);
    for card in &content.cards {
        validate_crate_card(card, &format!("{path}.cards[]"));
    }
}

fn validate_closing(content: &ClosingContent, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    assert_min_len(&format!("{path}.actions"), &content.actions, 1);
    for action in &content.actions {
        validate_action(action, &format!("{path}.actions[]"));
    }
}

fn validate_case_list(content: &CaseListSection, path: &str) {
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_min_len(&format!("{path}.items"), &content.items, 1);
    for item in &content.items {
        assert_non_empty(&format!("{path}.items[]"), item);
    }
}

fn validate_work_card(content: &WorkCardContent, path: &str) {
    assert_non_empty(&format!("{path}.category"), &content.category);
    assert_non_empty(&format!("{path}.title"), &content.title);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    if let Some(outcome) = &content.outcome {
        assert_non_empty(&format!("{path}.outcome"), outcome);
    }
    assert_non_empty(&format!("{path}.cta_label"), &content.cta_label);
    assert_min_len(&format!("{path}.highlights"), &content.highlights, 1);
    for item in &content.highlights {
        assert_non_empty(&format!("{path}.highlights[]"), item);
    }
    for tag in &content.stack_tags {
        assert_non_empty(&format!("{path}.stack_tags[]"), tag);
    }
    if let Some(preview) = &content.preview {
        validate_image_asset(preview, &format!("{path}.preview"));
    }
}

fn validate_crate_card(content: &CrateCardContent, path: &str) {
    assert_non_empty(&format!("{path}.name"), &content.name);
    assert_non_empty(&format!("{path}.summary"), &content.summary);
    assert_min_len(&format!("{path}.highlights"), &content.highlights, 1);
    for item in &content.highlights {
        assert_non_empty(&format!("{path}.highlights[]"), item);
    }
    if let Some(gallery) = &content.gallery {
        validate_crate_gallery(gallery, &format!("{path}.gallery"));
    }
    assert_non_empty(&format!("{path}.repository_url"), &content.repository_url);
    assert_non_empty(
        &format!("{path}.repository_label"),
        &content.repository_label,
    );
    if content.docs_url.is_some() {
        assert!(
            content.docs_label.is_some(),
            "{path}.docs_label must be present when docs_url is provided",
        );
    }
    if let Some(label) = &content.docs_label {
        assert_non_empty(&format!("{path}.docs_label"), label);
    }
}

fn validate_crate_gallery(content: &CrateGalleryContent, path: &str) {
    assert_non_empty(&format!("{path}.id"), &content.id);
    assert_non_empty(&format!("{path}.aria_label"), &content.aria_label);
    assert_min_len(&format!("{path}.tabs"), &content.tabs, 1);

    let mut tab_ids = HashSet::new();
    for tab in &content.tabs {
        assert_non_empty(&format!("{path}.tabs[].id"), &tab.id);
        assert!(
            tab_ids.insert(tab.id.clone()),
            "{path}.tabs[].id must be unique within the gallery",
        );
        assert_non_empty(&format!("{path}.tabs[].label.primary"), &tab.label.primary);
        if let Some(secondary) = &tab.label.secondary {
            assert_non_empty(&format!("{path}.tabs[].label.secondary"), secondary);
        }

        let preview = &tab.preview;
        validate_gallery_preview(preview, &format!("{path}.tabs[].preview"));

        let body = &tab.body;
        assert_non_empty(&format!("{path}.tabs[].body.title"), &body.title);
        if let Some(subtitle) = &body.subtitle {
            assert_non_empty(&format!("{path}.tabs[].body.subtitle"), subtitle);
        }
        assert_min_len(
            &format!("{path}.tabs[].body.features"),
            &body.features,
            1,
        );
        for feature in &body.features {
            assert_non_empty(&format!("{path}.tabs[].body.features[].text"), &feature.text);
        }
    }
}

fn validate_gallery_preview(
    content: &super::crate_gallery::CrateGalleryPreviewContent,
    path: &str,
) {
    assert_min_len(
        &format!("{path}.code_examples"),
        &content.code_examples,
        1,
    );
    for example in &content.code_examples {
        assert_non_empty(&format!("{path}.code_examples[].code"), &example.code);
        if let Some(label) = &example.label {
            assert_non_empty(&format!("{path}.code_examples[].label"), label);
        }
    }

    if let Some(image) = &content.image {
        assert_non_empty(&format!("{path}.image.asset_ref"), &image.asset_ref);
    }

    if let Some(badge) = &content.badge {
        assert_non_empty(&format!("{path}.badge.text"), &badge.text);
    }
}

fn validate_image_asset(content: &CmsImageAsset, path: &str) {
    assert_non_empty(&format!("{path}.asset_ref"), &content.asset_ref);
    assert_non_empty(&format!("{path}.alt"), &content.alt);
}

fn validate_action(content: &CmsActionLink, path: &str) {
    assert_non_empty(&format!("{path}.label"), &content.label);
    assert_non_empty(&format!("{path}.href"), &content.href);
}

fn assert_non_empty(path: &str, value: &Text) {
    assert!(
        !value.to_string().trim().is_empty(),
        "{path} must not be empty",
    );
}

fn assert_min_len<T>(path: &str, values: &[T], min_len: usize) {
    assert!(
        values.len() >= min_len,
        "{path} must contain at least {min_len} entries",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "open_source.crate_section.cards[].highlights")]
    fn open_source_index_rejects_crate_cards_without_highlights() {
        let mut content = super::super::fixture_loader::open_source_index_content().clone();
        content.crate_section.cards[0].highlights.clear();

        validate_open_source_index(&content);
    }

    #[test]
    #[should_panic(expected = "open_source.crate_section.cards[].docs_label")]
    fn open_source_index_requires_docs_label_when_docs_url_is_present() {
        let mut content = super::super::fixture_loader::open_source_index_content().clone();
        content.crate_section.cards[0].docs_label = None;

        validate_open_source_index(&content);
    }

    #[test]
    #[should_panic(expected = "open_source.crate_section.cards[].gallery.tabs[].preview.code_examples")]
    fn open_source_index_rejects_gallery_tabs_without_code_examples() {
        let mut content = super::super::fixture_loader::open_source_index_content().clone();
        content.crate_section.cards[0]
            .gallery
            .as_mut()
            .unwrap()
            .tabs[0]
            .preview
            .code_examples
            .clear();

        validate_open_source_index(&content);
    }

    #[test]
    #[should_panic(expected = "work.cases_title")]
    fn work_index_requires_cases_title() {
        let mut content = super::super::fixture_loader::work_index_content().clone();
        content.cases_title = Text::from("");

        validate_work_index(&content);
    }
}
