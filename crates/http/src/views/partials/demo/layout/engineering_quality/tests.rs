use maud::Render;

use super::*;

#[test]
fn keeps_engineering_quality_hooks_local_to_the_surface() {
    let markup = EngineeringQuality::builder().build().render().into_string();

    assert!(markup.contains("data-engineering-quality-grid"));
    assert!(markup.contains("data-engineering-quality-card-summary"));
    assert!(!markup.contains("data-info-grid"));
}
