use super::*;

#[test]
fn renders_local_tab_root_with_runtime_selected_by_default() {
    let markup = SupportingProofTabs::builder().build().render().into_string();

    assert!(markup.contains("id=\"supporting-proof-tabs\""));
    assert!(markup.contains("data-local-tabs-root"));
    assert!(markup.contains("data-local-tabs-active=\"runtime_inspection\""));
    assert!(markup.contains("Validate the main proof from other angles"));
    assert!(markup.contains("role=\"tablist\""));
    assert!(markup.contains("role=\"tabpanel\""));
}
